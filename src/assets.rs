use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

// pub const WARRIOR_ORIGINAL_SPRITE_SIZE: Vec2 = Vec2::new(141., 453.);
pub const WARRIOR_IN_GAME_SPRITE_SIZE: Vec2 = Vec2::new(83., 266.);
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
    #[asset(path = "jotaile", collection(typed, mapped))]
    pub jotaile_sprites: HashMap<String, Handle<Image>>,
}

// fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.insert_resource(ImageAssets {
//         jotaile_sprite: asset_server.load("jotaile/idle1.png"),
//         background_sprite: asset_server.load("scenery/cicest.jpeg"),
//     });
// }
