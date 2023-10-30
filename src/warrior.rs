use crate::assets::{IncrementSpriteIndex, WarriorAssets, WARRIOR_IN_GAME_SPRITE_SIZE};
use crate::player::Player;
use crate::scenery::FLOOR_HEIGHT;
use crate::{HALF_WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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

impl WarriorPositionState {
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

#[derive(Debug, Component, Clone, Reflect)]
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

#[derive(Event)]
struct UpdateWarriorHitbox {
    warrior_entity: Entity,
    position_state: WarriorPositionState,
}

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

fn update_warriors_sprites(
    mut animated_sprites: Query<
        (
            Entity,
            &mut SpriteAnimationTimer,
            &WarriorPositionState,
            &mut TextureAtlasSprite,
            Changed<WarriorPositionState>,
        ),
        &Warrior,
    >,
    time: Res<Time>,
    mut update_hitbox_event: EventWriter<UpdateWarriorHitbox>,
) {
    for (
        //
        entity,
        mut sprite_animation_timer,
        position_state,
        mut sprite_atlas,
        changed_position_state,
    ) in &mut animated_sprites
    {
        if changed_position_state {
            sprite_animation_timer.timer.reset();
            update_hitbox_event.send(UpdateWarriorHitbox {
                warrior_entity: entity,
                position_state: position_state.clone(),
            });
        }

        sprite_animation_timer.timer.tick(time.delta());

        if changed_position_state || sprite_animation_timer.timer.just_finished() {
            sprite_atlas.update_sprite_idx(position_state);
        }
    }
}

fn update_warriors_hitbox(
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
    for warrior_hitbox_update in update_hitbox_events.iter() {
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
