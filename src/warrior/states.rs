use bevy::prelude::*;

#[derive(Debug, Component, Clone, Default, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum WarriorPositionState {
    #[default]
    Idle,
    Walking,
    Jumping,
    Crouching,
    Fallen,
}

#[derive(Debug, Component, Clone, Default, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct WarriorPositionStateTransition {
    pub previous: WarriorPositionState,
}

#[derive(Debug, Component, Clone, Default, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum FacingPosition {
    #[default]
    Right,
    Left,
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct WarriorJumpingTimer {
    pub timer: Timer,
}
