use bevy::prelude::*;

use crate::assets::IncrementSpriteIndex;

use super::{UpdateWarriorHitbox, Warrior, WarriorPositionState};

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct SpriteAnimationTimer {
    pub timer: Timer,
}

pub fn update_warriors_sprites(
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
