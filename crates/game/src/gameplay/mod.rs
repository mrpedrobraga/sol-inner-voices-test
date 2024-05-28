use battle::plugin::BattlePlugin;
use bevy::prelude::*;
use state::GameplayState;

pub mod battle;
pub mod state;

pub struct InnerVoicesPlugin;

impl Plugin for InnerVoicesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<GameplayState>()
            .add_plugins(BattlePlugin)
            .add_systems(Startup, (setup));
    }
}

fn setup(mut cmd: Commands) {}
