use bevy::prelude::*;

use crate::gameplay::state::GameplayState;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<BattleState>()
            .register_type::<CurrentBattleData>()
            .add_systems(
                OnEnter(BattleState::Begin),
                (battle_begin).run_if(in_state(GameplayState::Battle)),
            )
            .add_systems(
                OnEnter(BattleState::End),
                (battle_end).run_if(in_state(GameplayState::Battle)),
            );
    }
}

#[derive(Resource, Reflect)]
pub struct CurrentBattleData {
    turn_number: u8,
}

impl CurrentBattleData {
    fn new() -> Self {
        Self { turn_number: 0 }
    }
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BattleState {
    #[default]
    Begin,
    End,
    PlayerChoices,
    AllyTurn,
    OpponentTurn,
}

fn battle_begin(mut cmd: Commands, mut next_battle_state: ResMut<NextState<BattleState>>) {
    info!("Battle has begun!");

    cmd.insert_resource(CurrentBattleData::new());
    next_battle_state.set(BattleState::PlayerChoices);
}

fn battle_end(mut cmd: Commands, mut next_gameplay_state: ResMut<NextState<GameplayState>>) {
    info!("Battle has ended.");

    cmd.remove_resource::<CurrentBattleData>();
    next_gameplay_state.set(GameplayState::Overworld);
}
