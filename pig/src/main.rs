use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, egui};
use my_library::RandomNumberGenerator;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
    #[default]
    Player,
    Cpu,
}

#[derive(Resource)]
struct GameAssets {
    dice_image: Handle<Image>,
    dice_layout: Handle<TextureAtlasLayout>,
}

#[derive(Clone, Copy, Resource)]
struct Scores {
    player: usize,
    cpu: usize,
}

#[derive(Component)]
struct HandDie;

#[derive(Resource)]
struct Random(RandomNumberGenerator);

#[derive(Resource)]
struct HandTimer(Timer);

fn setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("dice.png");
    let atlas = TextureAtlasLayout::from_grid(UVec2::new(52, 52), 6, 1, None, None);
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(GameAssets {
        dice_image: texture,
        dice_layout: atlas_handle,
    });
    commands.insert_resource(Scores { cpu: 0, player: 0 });
    commands.insert_resource(Random(RandomNumberGenerator::new()));
    commands.insert_resource(HandTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

fn display_score(scores: Res<Scores>, mut egui_context: EguiContexts) {
    egui::Window::new("Total Scores").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Player: {}", scores.player));
        ui.label(format!("CPU: {}", scores.cpu));
    });
}

fn spawn_die(
    hand_query: &Query<Entity, With<HandDie>>,
    commands: &mut Commands,
    assets: &GameAssets,
    new_roll: usize,
    color: Color,
) {
    let rolled_die = hand_query.iter().count() as f32 * 52.0;
    commands.spawn((
        Sprite {
            image: assets.dice_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.dice_layout.clone(),
                index: new_roll - 1,
            }),
            color,
            ..default()
        },
        Transform::from_xyz(rolled_die - 400.0, 60.0, 1.0),
    ));
}

fn clear_die(hand_query: &Query<Entity, With<HandDie>>, commands: &mut Commands) {
    hand_query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
}

fn player() {}

fn cpu() {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .init_state::<GamePhase>()
        .add_systems(Update, display_score)
        .add_systems(Update, player.run_if(in_state(GamePhase::Player)))
        .add_systems(Update, cpu.run_if(in_state(GamePhase::Cpu)))
        .run();
}
