use bevy::ecs::schedule::States;

/// Global state for the gameplay.
///
/// ### Overworld
/// The player will be walking around and interacting with stuff.
///
/// ### Battle
/// The player will be in the beautiful turn-based battle system we all love.
#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameplayState {
    Overworld,
    #[default]
    Battle,
}
