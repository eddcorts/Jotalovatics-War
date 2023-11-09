use crate::assets::{WarriorAssets, WARRIOR_IN_GAME_SPRITE_SIZE};
use crate::player::Player;
use crate::scenery::FLOOR_HEIGHT;
use crate::{HALF_WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod sprites;
mod states;
mod stats;

pub use self::{sprites::*, states::*, stats::*};

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateWarriorHitbox>()
            .register_type::<WarriorPositionState>()
            .register_type::<WarriorPositionStateTransition>()
            .register_type::<FacingPosition>()
            .register_type::<WarriorKind>()
            .register_type::<Speed>()
            .register_type::<SpriteAnimationTimer>()
            .register_type::<WarriorJumpingTimer>()
            .add_systems(Startup, (spawn_warrior,))
            .add_systems(
                Update,
                (
                    update_warriors_sprites,
                    update_warriors_hitbox
                        .run_if(on_event::<UpdateWarriorHitbox>())
                        .after(update_warriors_sprites),
                ),
            );
    }
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Warrior;

fn spawn_warrior(mut commands: Commands, warrior_assets: Res<WarriorAssets>) {
    let selected_warrior = WarriorKind::Jotaile;
    let default_position_state = WarriorPositionState::default();
    let hitbox = default_position_state.get_position_hitbox();

    commands.spawn((
        Name::new("Jotaile"),
        Warrior,
        Player,
        selected_warrior.clone(),
        default_position_state,
        WarriorPositionStateTransition::default(),
        Speed {
            walk: 180.,
            jump: 450.,
        },
        FacingPosition::default(),
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(WARRIOR_IN_GAME_SPRITE_SIZE),
                ..default()
            },
            texture_atlas: warrior_assets.jotaile_sprites.clone(),
            transform: Transform::from_xyz(
                -WINDOW_WIDTH / 3.,
                -HALF_WINDOW_HEIGHT + FLOOR_HEIGHT + WARRIOR_IN_GAME_SPRITE_SIZE.y / 2.,
                0.,
            ),
            ..default()
        },
        SpriteAnimationTimer {
            timer: Timer::from_seconds(0.75, TimerMode::Repeating),
        },
        Collider::cuboid(hitbox.0, hitbox.1),
        KinematicCharacterController::default(),
    ));
}
