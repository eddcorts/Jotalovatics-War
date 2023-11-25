use crate::assets::{WarriorAssets, WARRIOR_IN_GAME_SPRITE_SIZE};
use crate::player::Player;
use crate::scenery::FLOOR_HEIGHT;
use crate::{HALF_WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_rapier2d::TnuaRapier2dIOBundle;

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

pub fn spawn_warrior_bundle<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    entity_name: Option<&'static str>,
    selected_warrior: WarriorKind,
    warrior_assets: Res<WarriorAssets>,
    player: Option<Player>,
) -> EntityCommands<'w, 's, 'a> {
    let default_position_state = WarriorPositionState::default();
    let hitbox = default_position_state.get_position_hitbox();

    let mut warrior_commands = commands.spawn((
        Warrior,
        selected_warrior,
        default_position_state,
        WarriorPositionStateTransition::default(),
        Speed {
            walk: 180.,
            jump: 450.,
        },
        DamagedState::None,
        FacingPosition::default(),
        SpriteAnimationTimer {
            timer: Timer::from_seconds(0.75, TimerMode::Repeating),
        },
        RigidBody::Dynamic,
        Collider::cuboid(hitbox.0, hitbox.1),
        // KinematicCharacterController::default(),
        // CollisionGroups::new(Group::NONE, Group::NONE),
        // SolverGroups::new(Group::NONE, Group::NONE),
        // ColliderFlags {
        //     enabled: ColliderEnabled::Disabled,
        //     ..default()
        // },
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(WARRIOR_IN_GAME_SPRITE_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(
                -WINDOW_WIDTH / 3.,
                -HALF_WINDOW_HEIGHT + FLOOR_HEIGHT + WARRIOR_IN_GAME_SPRITE_SIZE.y / 2. + 40.,
                0.,
            ),
            texture_atlas: warrior_assets.jotaile_sprites.clone(),
            ..default()
        },
        TnuaControllerBundle::default(),
        TnuaRapier2dIOBundle::default(),
        TnuaSimpleAirActionsCounter::default(),
        // TnuaRapier2dSensorShape(Collider::cuboid(hitbox.0, hitbox.1)),
    ));

    // warrior_commands.with_children(|parent| {
    //     parent.spawn((
    //         Sprite,
    //         SpriteSheetBundle {
    //             sprite: TextureAtlasSprite {
    //                 index: 0,
    //                 custom_size: Some(WARRIOR_IN_GAME_SPRITE_SIZE),
    //                 anchor: Anchor::BottomCenter,
    //                 ..default()
    //             },
    //             transform: Transform::from_xyz(0., -WARRIOR_IN_GAME_SPRITE_SIZE.y / 2., 0.),
    //             texture_atlas: warrior_assets.jotaile_sprites.clone(),
    //             ..default()
    //         },
    //     ));

    // parent.spawn((
    //     Name::new("Hitbox"),
    //     Hitbox,
    //     Collider::cuboid(hitbox.0, hitbox.1),
    //     Sensor,
    //     TransformBundle::from_transform(Transform::from_xyz(-1., 0., 0.)),
    //     CollisionGroups::new(Group::NONE, Group::NONE),
    // ));

    // let crouch_hitbox = WarriorPositionState::Crouching.get_position_hitbox();

    // parent.spawn((
    //     Name::new("CrouchHitbox"),
    //     CrouchHitbox,
    //     Collider::cuboid(crouch_hitbox.0, crouch_hitbox.1),
    //     Sensor,
    //     TransformBundle::from_transform(Transform::from_xyz(0., -crouch_hitbox.1, 0.)),
    //     // CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
    // ));
    // });

    if player.is_some() {
        warrior_commands.insert(Player);
    }

    warrior_commands.insert(Name::new(entity_name.unwrap_or("Warrior")));

    warrior_commands
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
