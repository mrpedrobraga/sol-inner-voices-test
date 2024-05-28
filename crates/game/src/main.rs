#![allow(unused)]

mod gameplay;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use gameplay::InnerVoicesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InnerVoicesPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
