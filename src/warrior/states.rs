use std::marker::PhantomData;

use bevy::prelude::*;

/// For timers only with Once timer mode
///
/// TODO: transform into derive macro
pub trait StateTimer: Component {
    fn get_timer(&mut self) -> &mut Timer;
}

pub trait RegisterStateTimerSystems {
    fn register_state_timer_systems(&mut self) -> &mut App;
}

impl RegisterStateTimerSystems for App {
    fn register_state_timer_systems(&mut self) -> &mut Self {
        self.add_event::<StateTimerFinished<DamagedTimer>>()
            .add_event::<StateTimerFinished<WarriorJumpingTimer>>()
            .add_systems(
                Update,
                (
                    tick_states_timers::<DamagedTimer>,
                    tick_states_timers::<WarriorJumpingTimer>,
                ),
            )
    }
}

pub fn tick_states_timers<T: StateTimer>(
    mut timer_finished_event: EventWriter<StateTimerFinished<T>>,
    mut state_timers: Query<(Entity, &mut T)>,
    time: Res<Time>,
) {
    for (warrior_entity, mut timer) in &mut state_timers {
        if timer.get_timer().tick(time.delta()).finished() {
            timer_finished_event.send(StateTimerFinished::new(warrior_entity));
        }
    }
}

#[derive(Event)]
pub struct StateTimerFinished<T: StateTimer>(pub Entity, PhantomData<T>);

impl<T: StateTimer> StateTimerFinished<T> {
    fn new(entity: Entity) -> Self {
        StateTimerFinished(entity, PhantomData)
    }
}

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

impl StateTimer for WarriorJumpingTimer {
    fn get_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct DamagedTimer {
    pub timer: Timer,
}

impl StateTimer for DamagedTimer {
    fn get_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }
}

#[derive(Debug, Component, Clone, Default, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum DamagedState {
    #[default]
    None,
    Hit,
    Stunned,
}
