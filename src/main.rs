#![allow(unused)]

mod gameplay;
use bevy::prelude::*;
use bevy_smooth_pixel_camera::prelude::*;
use gameplay::state::GameplayState;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins))
        .init_state::<GameplayState>()
        .add_systems(Startup, (setup))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((MainCamera, Camera2dBundle::default()));
}
