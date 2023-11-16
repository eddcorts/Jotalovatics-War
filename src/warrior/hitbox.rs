use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::assets::WARRIOR_IN_GAME_SPRITE_SIZE;

use super::{
    states::{WarriorPositionState, WarriorPositionStateTransition},
    Warrior,
};

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Hitbox;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct CrouchHitbox;

#[derive(Event)]
pub struct UpdateWarriorHitbox {
    pub warrior_entity: Entity,
    pub position_state: WarriorPositionState,
}

pub trait PositionStateHitbox {
    const STOOD_UP_HITBOX_HALF_EXTENT: (f32, f32);
    const CROUCH_HITBOX_HALF_EXTENT: (f32, f32);
    fn get_position_hitbox(&self) -> (f32, f32);
}

impl PositionStateHitbox for WarriorPositionState {
    const STOOD_UP_HITBOX_HALF_EXTENT: (f32, f32) = (
        WARRIOR_IN_GAME_SPRITE_SIZE.x / 4.,
        WARRIOR_IN_GAME_SPRITE_SIZE.y / 2.,
    );

    const CROUCH_HITBOX_HALF_EXTENT: (f32, f32) = (
        WARRIOR_IN_GAME_SPRITE_SIZE.x / 2.5,
        WARRIOR_IN_GAME_SPRITE_SIZE.y / 4.,
    );

    fn get_position_hitbox(&self) -> (f32, f32) {
        match self {
            WarriorPositionState::Idle
            | WarriorPositionState::Walking
            | WarriorPositionState::Jumping => Self::STOOD_UP_HITBOX_HALF_EXTENT,
            WarriorPositionState::Crouching => Self::CROUCH_HITBOX_HALF_EXTENT,
            WarriorPositionState::Fallen => (
                Self::STOOD_UP_HITBOX_HALF_EXTENT.1,
                Self::STOOD_UP_HITBOX_HALF_EXTENT.0,
            ),
        }
    }
}

pub fn update_warriors_hitbox(
    mut update_hitbox_events: EventReader<UpdateWarriorHitbox>,
    mut warrior_collider_components: Query<
        (
            &mut Collider,
            &mut Transform,
            &WarriorPositionStateTransition,
        ),
        &Warrior,
    >,
) {
    // TODO: consider using collision groups and child entities for crouch hitbox
    //! bugs with walking and crouching, jumping and crouching

    for warrior_hitbox_update in update_hitbox_events.read() {
        let (
            //
            mut warrior_collider,
            mut transform,
            position_state_transition,
        ) = warrior_collider_components
            .get_mut(warrior_hitbox_update.warrior_entity)
            .unwrap();

        let position_state = &warrior_hitbox_update.position_state;

        let half_extents = position_state.get_position_hitbox();
        *warrior_collider = Collider::cuboid(half_extents.0, half_extents.1);

        if position_state == &WarriorPositionState::Crouching {
            transform.translation.y -= WarriorPositionState::Crouching.get_position_hitbox().1;
        } else if position_state_transition.previous == WarriorPositionState::Crouching {
            transform.translation.y += WarriorPositionState::Crouching.get_position_hitbox().1;
        }
    }
}
