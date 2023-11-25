use crate::warrior::{Speed, WarriorPositionState, WarriorPositionStateTransition};
use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
use bevy_tnua::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_player.in_set(TnuaUserControlsSystemSet),
                // process_jump
            ),
        );
    }
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

fn move_player(
    // mut commands: Commands,
    mut player: Query<
        (
            // Entity,
            &Speed,
            &mut WarriorPositionState,
            // &mut KinematicCharacterController,
            // Option<&KinematicCharacterControllerOutput>,
            &mut TnuaController,
            &mut WarriorPositionStateTransition,
        ),
        &Player,
    >,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // avaliar se precisa do delta seconds pro movimento
    //! PENDING TESTS
    //! should only jump when press w + idle or walking
    //! should only crouch when press s + idle or walking
    //! should idle when stop moving and not in the air nor crouching
    //! should idle when stop crouching and not walking
    //! should walk when stop crouching and walking
    //! should be jumping when not grounded

    let (
        //
        // entity,
        player_speed,
        mut warrior_position_state,
        mut tnua_controller,
        // mut kinematic_controller,
        // kinematic_output,
        mut position_state_transition,
    ) = player.single_mut();

    let mut to_move = Vec3::ZERO;

    // if let Some(kinematic_output) = kinematic_output {
    //     if *warrior_position_state != WarriorPositionState::Idle && kinematic_output.grounded {
    //         position_state_transition.previous = warrior_position_state.clone();
    //         *warrior_position_state = WarriorPositionState::Idle;
    //         commands
    //             .get_entity(entity)
    //             .unwrap()
    //             .remove::<WarriorJumpingTimer>();
    //     }
    // }

    if matches!(
        *warrior_position_state,
        WarriorPositionState::Idle | WarriorPositionState::Walking
    ) {
        if keyboard.pressed(KeyCode::W) {
            position_state_transition.previous = warrior_position_state.clone();
            *warrior_position_state = WarriorPositionState::Jumping;
        }

        if keyboard.pressed(KeyCode::S) {
            position_state_transition.previous = warrior_position_state.clone();
            *warrior_position_state = WarriorPositionState::Crouching;
            // todo!("Fazer o estado do jogador pra agachar etc");
        }
    }

    if keyboard.pressed(KeyCode::A) {
        if *warrior_position_state == WarriorPositionState::Idle {
            position_state_transition.previous = warrior_position_state.clone();
            *warrior_position_state = WarriorPositionState::Walking;
        }

        to_move.x -= player_speed.walk * time.delta_seconds();
    } else if keyboard.pressed(KeyCode::D) {
        if *warrior_position_state == WarriorPositionState::Idle {
            position_state_transition.previous = warrior_position_state.clone();
            *warrior_position_state = WarriorPositionState::Walking;
        }
        to_move.x += player_speed.walk * time.delta_seconds();
    }

    // Return to idle
    if (
        // Stopped walking
        *warrior_position_state == WarriorPositionState::Walking
            && (keyboard.just_released(KeyCode::A) || keyboard.just_released(KeyCode::D))
    ) || (
        // Stopped crouching
        *warrior_position_state == WarriorPositionState::Crouching
            && keyboard.just_released(KeyCode::S)
    ) {
        position_state_transition.previous = warrior_position_state.clone();
        *warrior_position_state = WarriorPositionState::Idle
    }

    // kinematic_controller.translation = Some(to_move);
    if keyboard.pressed(KeyCode::Space) {
        to_move += Vec3::new(10., 0., 0.) * 40.;
        tnua_controller.basis(TnuaBuiltinWalk {
            desired_velocity: to_move,
            float_height: 200.0,
            ..default()
        });
    }
    if keyboard.pressed(KeyCode::H) {
        to_move += Vec3::new(10., 0., 0.) * 40.;
        tnua_controller.basis(TnuaBuiltinWalk {
            float_height: 100.0,
            ..default()
        });
    }
}
