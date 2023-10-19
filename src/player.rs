use crate::warrior::{Speed, WarriorJumpingTimer, WarriorPositionState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const UP: Vec2 = Vec2::new(0., 1.);
const JUMP_SPEED: f32 = 7.;
const JUMP_TIMER_DURATION: f32 = 0.3;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player, process_jump));
    }
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

fn move_player(
    mut commands: Commands,
    mut player: Query<
        (
            Entity,
            &Speed,
            &mut WarriorPositionState,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        &Player,
    >,
    // output: Query<(Entity, Option<&KinematicCharacterControllerOutput>), &Player>,
    keyboard: Res<Input<KeyCode>>,
) {
    // avaliar se precisa do delta seconds pro movimento
    //! PENDING TESTS
    //! should only jump when press w + idle or walking
    //! should only crouch when press s + idle or walking
    //! should idle when stop moving and not in the air nor crouching

    let (
        //
        entity,
        player_speed,
        mut warrior_state,
        mut kinematic_controller,
        kinematic_output,
    ) = player.single_mut();
    let mut to_move = Vec2::ZERO;

    if let Some(kinematic_output) = kinematic_output {
        if kinematic_output.grounded {
            *warrior_state = WarriorPositionState::Idle;
            commands
                .get_entity(entity)
                .unwrap()
                .remove::<WarriorJumpingTimer>();
        }
    }

    if matches!(
        *warrior_state,
        WarriorPositionState::Idle | WarriorPositionState::Walking
    ) {
        if keyboard.pressed(KeyCode::W) {
            *warrior_state = WarriorPositionState::Jumping;

            commands
                .get_entity(entity)
                .unwrap()
                .insert(WarriorJumpingTimer {
                    timer: Timer::from_seconds(JUMP_TIMER_DURATION, TimerMode::Once),
                });
        }

        if keyboard.pressed(KeyCode::S) {
            *warrior_state = WarriorPositionState::Crouching;
            // todo!("Fazer o estado do jogador pra agachar etc");
        }
    }

    if keyboard.pressed(KeyCode::A) {
        if *warrior_state == WarriorPositionState::Idle {
            *warrior_state = WarriorPositionState::Walking;
        }

        to_move.x -= player_speed.0;
    } else if keyboard.pressed(KeyCode::D) {
        if *warrior_state == WarriorPositionState::Idle {
            *warrior_state = WarriorPositionState::Walking;
        }
        to_move.x += player_speed.0;
    }

    if *warrior_state == WarriorPositionState::Walking
        && (keyboard.just_released(KeyCode::A) || keyboard.just_released(KeyCode::D))
    {
        *warrior_state = WarriorPositionState::Idle
    }

    kinematic_controller.translation = Some(to_move);
}

fn process_jump(
    mut player: Query<(&mut WarriorJumpingTimer, &mut KinematicCharacterController), &Player>,
    time: Res<Time>,
) {
    for (
        //
        mut jumping_timer,
        mut kinematic_controller,
    ) in &mut player
    {
        jumping_timer.timer.tick(time.delta());

        let direction = if !jumping_timer.timer.finished() {
            1.
        } else {
            -1.
        };

        kinematic_controller.translation = Some(
            kinematic_controller.translation.unwrap_or(Vec2::ZERO) + UP * direction * JUMP_SPEED,
        );
    }
}
