use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::warrior::{DamagedState, WarriorPositionState};

// tamanho de cada sprite no atlas eh de 250x450
pub const WARRIOR_SPRITE_TILE_PROPORTION: f32 = 250. / 450.;
pub const WARRIOR_IN_GAME_HEIGHT: f32 = 260.;
pub const WARRIOR_IN_GAME_SPRITE_SIZE: Vec2 = Vec2::new(
    WARRIOR_IN_GAME_HEIGHT * WARRIOR_SPRITE_TILE_PROPORTION,
    WARRIOR_IN_GAME_HEIGHT,
);
pub const ATLAS_COLUMNS_AMOUNT: usize = 6;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(PreStartup, (load_assets,));
            .init_collection::<WarriorAssets>();
    }
}

#[derive(Debug, AssetCollection, Resource)]
pub struct WarriorAssets {
    #[asset(texture_atlas(
        tile_size_x = 250.,
        tile_size_y = 450.,
        columns = 6,
        rows = 2,
        // padding_x = 0.,
        // padding_y = 0.,
        // offset_x = 0.,
        // offset_y = 0.
    ))]
    #[asset(path = "jotaile/jotaile_atlas.png")]
    pub jotaile_sprites: Handle<TextureAtlas>,
}

pub trait IncrementSpriteIndex {
    fn update_sprite_idx(
        &mut self,
        warrior_position_state: &WarriorPositionState,
        damaged_state: &DamagedState,
    );
}

impl IncrementSpriteIndex for TextureAtlasSprite {
    fn update_sprite_idx(
        &mut self,
        warrior_position_state: &WarriorPositionState,
        damaged_state: &DamagedState,
    ) {
        let column_offset = match damaged_state {
            DamagedState::None => 0,
            _ => ATLAS_COLUMNS_AMOUNT,
        };

        let sprites_idx = match warrior_position_state {
            WarriorPositionState::Idle | WarriorPositionState::Walking => vec![0, 1],
            WarriorPositionState::Jumping => vec![3],
            WarriorPositionState::Crouching => vec![2],
            WarriorPositionState::Fallen => vec![4],
        }
        .iter()
        .map(|idx| idx + column_offset)
        .collect::<Vec<usize>>();

        let sprites_amount = sprites_idx.len();
        let min_index = sprites_idx[0];

        if sprites_amount == 1 {
            self.index = min_index;
            return;
        }

        let current_atlas_idx = self.index;
        let max_index = sprites_idx[1];

        if current_atlas_idx < min_index || current_atlas_idx > max_index {
            self.index = min_index;
            return;
        }

        self.index = min_index + (current_atlas_idx + 1) % sprites_amount;
    }
}
