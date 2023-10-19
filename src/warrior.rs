use crate::assets::{WarriorAssets, WARRIOR_IN_GAME_SPRITE_SIZE};
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

impl WarriorKind {
    fn get_idle_sprite(
        &self,
        warrior_assets: &WarriorAssets,
        idx: Option<usize>,
    ) -> (Handle<Image>, usize) {
        let sprites = match self {
            WarriorKind::Jotaile => &warrior_assets.jotaile_sprites,
        };
        let idle_names: Vec<&String> = sprites.keys().filter(|&key| key.contains("idle")).collect();
        let sprites_amount = idle_names.len();
        let sprite_idx = idx.unwrap_or(0) % sprites_amount;

        (sprites[idle_names[sprite_idx]].clone(), sprites_amount)
    }
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed(pub f32);

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Warrior;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct SpriteAnimationTimer {
    pub timer: Timer,
    pub frame: usize,
    pub frames_amount: usize,
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct WarriorJumpingTimer {
    pub timer: Timer,
}

fn spawn_warrior(mut commands: Commands, warrior_assets: Res<WarriorAssets>) {
    let selected_warrior = WarriorKind::Jotaile;
    let (image_handle, frames_amount): (Handle<Image>, usize) =
        selected_warrior.get_idle_sprite(&warrior_assets, Some(0));

    commands.spawn((
        Name::new("Jotaile"),
        selected_warrior.clone(),
        Player,
        Warrior,
        WarriorPositionState::default(),
        Speed(4.),
        FacingPosition::default(),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(WARRIOR_IN_GAME_SPRITE_SIZE),
                ..default()
            },
            texture: image_handle,
            transform: Transform::from_xyz(
                -WINDOW_WIDTH / 3.,
                -HALF_WINDOW_HEIGHT + FLOOR_HEIGHT + WARRIOR_IN_GAME_SPRITE_SIZE.y / 2.,
                0.,
            ),
            ..default()
        },
        SpriteAnimationTimer {
            timer: Timer::from_seconds(0.75, TimerMode::Repeating),
            frame: 0,
            frames_amount: frames_amount,
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
        (&mut SpriteAnimationTimer, &WarriorKind, &mut Handle<Image>),
        &Warrior,
    >,
    warrior_assets: Res<WarriorAssets>,
    time: Res<Time>,
) {
    for (mut sprite_animation_timer, warrior_kind, mut sprite) in &mut animated_sprites {
        sprite_animation_timer.timer.tick(time.delta());

        if sprite_animation_timer.timer.just_finished() {
            let current_frame_idx = sprite_animation_timer.frame;
            let (image_handle, frames_amount) =
                warrior_kind.get_idle_sprite(&warrior_assets, Some(current_frame_idx + 1));
            sprite_animation_timer.frame = (current_frame_idx + 1) % frames_amount;

            *sprite = image_handle;
        }
    }
}
