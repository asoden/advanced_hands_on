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
    let atlas = TextureAtlasLayout::from_grid(

    );
}

fn display_score() {}

fn player(){}

fn cpu(){}

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
