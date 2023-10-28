use crate::assets::{IncrementSpriteIndex, WarriorAssets, WARRIOR_IN_GAME_SPRITE_SIZE};
use crate::player::Player;
use crate::scenery::FLOOR_HEIGHT;
use crate::{HALF_WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WarriorPlugin;

impl Plugin for WarriorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_warrior,))
            .add_systems(Update, update_warriors_sprites);
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
pub enum FacingPosition {
    #[default]
    Right,
    Left,
}

#[derive(Debug, Component, Clone)]
pub enum WarriorKind {
    Jotaile,
    // Ed,
    // Rod,
    // Turi,
    // Fred,
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed {
    pub walk: f32,
    pub jump: f32,
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Warrior;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct SpriteAnimationTimer {
    pub timer: Timer,
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct WarriorJumpingTimer {
    pub timer: Timer,
}

fn spawn_warrior(mut commands: Commands, warrior_assets: Res<WarriorAssets>) {
    let selected_warrior = WarriorKind::Jotaile;
    let default_position_state = WarriorPositionState::default();

    commands.spawn((
        Name::new("Jotaile"),
        selected_warrior.clone(),
        Player,
        Warrior,
        default_position_state,
        Speed {
            walk: 180.,
            jump: 400.,
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
        Collider::cuboid(
            WARRIOR_IN_GAME_SPRITE_SIZE.x / 2.,
            WARRIOR_IN_GAME_SPRITE_SIZE.y / 2.,
        ),
        KinematicCharacterController::default(),
    ));
}

fn update_warriors_sprites(
    mut animated_sprites: Query<
        (
            &mut SpriteAnimationTimer,
            &WarriorPositionState,
            &mut TextureAtlasSprite,
            Changed<WarriorPositionState>,
        ),
        &Warrior,
    >,
    time: Res<Time>,
) {
    for (
        //
        mut sprite_animation_timer,
        position_state,
        mut sprite_atlas,
        changed_position_state,
    ) in &mut animated_sprites
    {
        if changed_position_state {
            sprite_animation_timer.timer.reset();
        }

        sprite_animation_timer.timer.tick(time.delta());

        if changed_position_state || sprite_animation_timer.timer.just_finished() {
            sprite_atlas.update_sprite_idx(position_state);
        }
    }
}
