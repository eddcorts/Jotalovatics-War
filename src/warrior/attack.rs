use bevy::{ecs::query::Has, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::player::Player;

use super::{DamagedState, DamagedTimer, StateTimerFinished, Warrior};

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Attack;

#[derive(Debug, Component, Clone, Default, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum AttackParty {
    #[default]
    Ally,
    Enemy,
}

#[derive(Event)]
pub struct AttackHit {
    pub warrior_entity: Entity,
    pub attack_entity: Entity,
}

pub fn tmp_spawn_fixed_attack(mut commands: Commands) {
    commands.spawn((
        Name::new("attack"),
        Attack,
        AttackParty::Enemy,
        Transform::from_xyz(0., 0., 0.),
        Collider::cuboid(5., 5.),
    ));
}

pub fn attack_collides_player(
    warriors: Query<(Entity, &KinematicCharacterControllerOutput, Has<Player>), &Warrior>,
    attacks: Query<(Entity, &AttackParty), &Attack>,
    // query: Query<&Name>,
    mut collision_events: EventWriter<AttackHit>,
) {
    let mut ally_attacks: Vec<Entity> = vec![];
    let mut enemy_attacks: Vec<Entity> = vec![];

    for (entity, attack_party) in &attacks {
        (if attack_party == &AttackParty::Ally {
            &mut ally_attacks
        } else {
            &mut enemy_attacks
        })
        .push(entity);
    }

    for (warrior_entity, warrior_kinematic_output, is_ally_warrior) in &warriors {
        for collision in &warrior_kinematic_output.collisions {
            // query.get(collision.entity).and_then(|x| {
            //     dbg!(x);
            //     Ok(x)
            // });
            if (if is_ally_warrior {
                &enemy_attacks
            } else {
                &ally_attacks
            })
            .contains(&collision.entity)
            {
                collision_events.send(AttackHit {
                    warrior_entity,
                    attack_entity: collision.entity,
                });
            }
        }
    }
}

pub fn attack_hits(
    mut commands: Commands,
    mut warrior_damaged_state: Query<&mut DamagedState>,
    mut attack_hit_events: EventReader<AttackHit>,
) {
    for attack_hit in attack_hit_events.read() {
        // TODO: refactor to keep this logic to not happen for _every_ hit, but an aggregated one
        commands
            .entity(attack_hit.warrior_entity)
            .remove::<DamagedTimer>()
            .insert(DamagedTimer {
                timer: Timer::from_seconds(1., TimerMode::Once),
            });

        *warrior_damaged_state
            .get_mut(attack_hit.warrior_entity)
            .unwrap() = DamagedState::Hit;
    }
}

pub fn damage_timer_finished(
    mut damage_timer_finished_events: EventReader<StateTimerFinished<DamagedTimer>>,
    mut warrior_state_query: Query<&mut DamagedState>,
) {
    //! this dont cover cases like: if damaged in air, then will have to fall to recover character control
    for damage_timer_finished_event in damage_timer_finished_events.read() {
        *warrior_state_query
            .get_mut(damage_timer_finished_event.0)
            .unwrap() = DamagedState::None;
    }
}
