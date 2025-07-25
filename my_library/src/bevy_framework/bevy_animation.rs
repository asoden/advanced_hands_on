use bevy::{log, prelude::*, utils::HashMap};

pub enum AnimationOption {
    None,
    NextFrame,
    GoToFrame(usize),
    SwitchToAnimation(String),
    PlaySound(String),
}

pub struct AnimationFrame {
    sprite_index: usize,
    delay_ms: u128,
    action: Vec<AnimationOption>,
}

impl AnimationFrame {
    pub fn new(sprite_index: usize, delay_ms: u128, action: Vec<AnimationOption>) -> Self {
        Self {
            sprite_index,
            delay_ms,
            action,
        }
    }
}

pub struct PerFrameAnimation {
    pub frames: Vec<AnimationFrame>,
}

impl PerFrameAnimation {
    pub fn new(frames: Vec<AnimationFrame>) -> Self {
        Self { frames }
    }
}

#[derive(Resource)]
pub struct Animations(HashMap<String, PerFrameAnimation>);

impl Animations {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_animation<S: ToString>(mut self, tag: S, animation: PerFrameAnimation) -> Self {
        self.0.insert(tag.to_string(), animation);
        self
    }
}

#[derive(Component)]
pub struct AnimationCycle {
    animation_tag: String,
    current_frame: usize,
    timer: u128,
}

impl AnimationCycle {
    pub fn new<S: ToString>(tag: S) -> Self {
        Self {
            animation_tag: tag.to_string(),
            current_frame: 0,
            timer: 0,
        }
    }

    pub fn switch<S: ToString>(&mut self, new: S) {
        let new = new.to_string();
        if new != self.animation_tag {
            self.animation_tag = new;
            self.current_frame = 0;
            self.timer = 0;
        }
    }
}

pub fn cycle_animations(
    animations: Res<Animations>,
    mut animated: Query<(&mut AnimationCycle, &mut Sprite)>,
    time: Res<Time>,
    assets: Res<crate::AssetStore>,
    mut commands: Commands,
    loaded_assets: Res<crate::LoadedAssets>,
) {
    let ms_since_last_call = time.delta().as_millis();
    animated.iter_mut().for_each(|(mut animation, mut sprite)| {
        animation.timer += ms_since_last_call;
        if let Some(cycle) = animations.0.get(&animation.animation_tag) {
            let current_frame = &cycle.frames[animation.current_frame];
            if animation.timer > current_frame.delay_ms {
                animation.timer = 0;
                for action in current_frame.action.iter() {
                    match action {
                        AnimationOption::None => {}
                        AnimationOption::NextFrame => {
                            animation.current_frame += 1;
                        }
                        AnimationOption::GoToFrame(frame) => {
                            animation.current_frame = *frame;
                        }
                        AnimationOption::SwitchToAnimation(new) => {
                            animation.animation_tag = new.to_string();
                            animation.current_frame = 0;
                        }
                        AnimationOption::PlaySound(tag) => {
                            assets.play(tag, &mut commands, &loaded_assets);
                        }
                    }
                    if let Some(ta) = &mut sprite.texture_atlas {
                        ta.index = cycle.frames[animation.current_frame].sprite_index;
                    }
                }
            }
        } else {
            log::warn!("Animation Cycle [{}] not found!", animation.animation_tag);
        }
    });
}

#[macro_export]
macro_rules! spawn_animated_sprite {
    ($assets:expr, $commands:expr, $index:expr, $x:expr, $y:expr, $z:expr,
        $animation_name:expr, $($component:expr),*) => {
        let Some((img, atlas)) = $assets.get_atlas_handle($index) else { panic!() };
        $commands.spawn((
            Sprite::from_atlas_image(img.clone(), TextureAtlas {
                layout: atlas.clone(),
                index: 0,
            }),
            Transform::from_xyz($x, $y, $z),
            AnimationCycle::new($animation_name),
        ))
        $(
            .insert($component)
        )*;
    };
}

#[derive(Component)]
pub struct ContinualParallax {
    image_width: f32,
    move_every_ms: u128,
    scroll_speed: Vec2,
    timer: u128
}

impl ContinualParallax {
    pub fn new(image_width: f32, move_every_ms: u128, scroll_speed: Vec2) -> Self {
        Self {
            image_width, move_every_ms, scroll_speed, timer: 0
        }
    }
}

pub fn continual_parallax(
    mut animated: Query<(&mut ContinualParallax, &mut Transform)>,
    time: Res<Time>
) {
    let ms_since_last_call = time.delta().as_millis();
    animated.iter_mut().for_each(|(mut parallax, mut transform)| {
        parallax.timer += ms_since_last_call;
        if parallax.timer >= parallax.move_every_ms {
            parallax.timer = 0;
            transform.translation.x -= parallax.scroll_speed.x;
            transform.translation.y -= parallax.scroll_speed.y;
            if transform.translation.x <= (0.0 - parallax.image_width) {
                transform.translation.x = parallax.image_width;
            }
        }
    });
}