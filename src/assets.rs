use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// tamanho de cada sprite no atlas eh de 250x450
// pub const WARRIOR_ORIGINAL_SPRITE_SIZE: Vec2 = Vec2::new(141., 453.);
pub const WARRIOR_SPRITE_TILE_PROPORTION: f32 = 250. / 450.;
pub const WARRIOR_IN_GAME_HEIGHT: f32 = 260.;
pub const WARRIOR_IN_GAME_SPRITE_SIZE: Vec2 = Vec2::new(
    WARRIOR_IN_GAME_HEIGHT * WARRIOR_SPRITE_TILE_PROPORTION,
    WARRIOR_IN_GAME_HEIGHT,
);
// pub const WARRIOR_NOT_STOOD_UP_SPRIZE_SIZE: Vec2 = Vec2::new(135., 450.);

// pub let ATLAS_SPRITES_INDEX: HashMap<&WarriorPositionState, Vec<usize>> = HashMap::new();
//     ha
// //     &WarriorPositionState::Idle: (0,1)
// };

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
    // pub state_indexes: f32,
}

// fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.insert_resource(ImageAssets {
//         jotaile_sprite: asset_server.load("jotaile/idle1.png"),
//         background_sprite: asset_server.load("scenery/cicest.jpeg"),
//     });
// }
