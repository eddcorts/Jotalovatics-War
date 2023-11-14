use crate::assets::{WarriorAssets, WARRIOR_IN_GAME_SPRITE_SIZE};
use crate::player::Player;
use crate::scenery::FLOOR_HEIGHT;
use crate::{HALF_WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::prelude::*;

mod attack;
mod hitbox;
mod sprites;
mod states;
mod stats;

pub use self::{attack::*, hitbox::*, sprites::*, states::*, stats::*};

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateWarriorHitbox>()
            .add_event::<AttackHit>()
            .register_type::<DamagedState>()
            .register_type::<WarriorPositionState>()
            .register_type::<WarriorPositionStateTransition>()
            .register_type::<FacingPosition>()
            .register_type::<WarriorKind>()
            .register_type::<Speed>()
            .register_type::<SpriteAnimationTimer>()
            .register_type::<WarriorJumpingTimer>()
            .register_type::<Attack>()
            .register_type::<AttackParty>()
            .register_type::<DamagedTimer>()
            .add_systems(Startup, (spawn_warrior, tmp_spawn_fixed_attack))
            .add_systems(
                Update,
                (
                    update_warriors_sprites,
                    update_warriors_hitbox
                        .run_if(on_event::<UpdateWarriorHitbox>())
                        .after(update_warriors_sprites),
                    attack_collides_player,
                    attack_hits
                        .run_if(on_event::<AttackHit>())
                        .after(attack_collides_player),
                    damage_timer_finished.run_if(on_event::<StateTimerFinished<DamagedTimer>>()),
                ),
            )
            .register_state_timer_systems();
    }
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Warrior;

pub fn spawn_warrior_bundle(
    commands: &mut Commands,
    entity_name: Option<&'static str>,
    selected_warrior: WarriorKind,
    warrior_assets: Res<WarriorAssets>,
    player: Option<Player>,
) {
    let default_position_state = WarriorPositionState::default();
    let hitbox = default_position_state.get_position_hitbox();

    let mut warrior_commands = commands.spawn((
        Warrior,
        DamagedState::None,
        selected_warrior,
        default_position_state,
        WarriorPositionStateTransition::default(),
        Speed {
            walk: 180.,
            jump: 450.,
        },
        FacingPosition::default(),
        SpriteAnimationTimer {
            timer: Timer::from_seconds(0.75, TimerMode::Repeating),
        },
        KinematicCharacterController::default(),
        SpatialBundle {
            transform: Transform::from_xyz(
                -WINDOW_WIDTH / 3.,
                -HALF_WINDOW_HEIGHT + FLOOR_HEIGHT + WARRIOR_IN_GAME_SPRITE_SIZE.y / 2.,
                0.,
            ),
            ..default()
        },
    ));

    warrior_commands.with_children(|parent| {
        parent.spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(WARRIOR_IN_GAME_SPRITE_SIZE),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_xyz(0., -WARRIOR_IN_GAME_SPRITE_SIZE.y / 2., 0.),
            texture_atlas: warrior_assets.jotaile_sprites.clone(),
            ..default()
        });

        parent.spawn((
            Collider::cuboid(hitbox.0, hitbox.1),
            SpatialBundle::default(),
        ));
    });

    if player.is_some() {
        warrior_commands.insert(Player);
    }

    warrior_commands.insert(Name::new(entity_name.unwrap_or("Warrior")));
}

pub fn spawn_warrior(mut commands: Commands, warrior_assets: Res<WarriorAssets>) {
    let selected_warrior = WarriorKind::Jotaile;

    spawn_warrior_bundle(
        &mut commands,
        Some("Jotaile"),
        selected_warrior,
        warrior_assets,
        Some(Player),
    );
}
