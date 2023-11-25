use crate::{HALF_WINDOW_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;

pub const FLOOR_HEIGHT: f32 = 50.0;
pub struct SceneryPlugin;

impl Plugin for SceneryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_collection::<SceneryAssets>()
            .add_systems(Startup, (spawn_floor, apply_gravity))
            .add_systems(
                Update,
                spawn_background.run_if(on_event::<AssetEvent<Image>>()),
            );
    }
}

#[derive(Debug, AssetCollection, Resource)]
pub struct SceneryAssets {
    #[asset(path = "scenery/cicest.jpeg")]
    pub cicest_sprite: Handle<Image>,
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Floor;

fn spawn_background(
    mut asset_events: EventReader<AssetEvent<Image>>,
    mut commands: Commands,
    scenery_assets: Res<SceneryAssets>,
    assets: Res<Assets<Image>>,
) {
    for event in asset_events.read() {
        match event {
            AssetEvent::Added { id } => {
                // a texture was just loaded or changed!
                if *id != scenery_assets.cicest_sprite.id() {
                    continue;
                }

                let scenery_size = assets.get(*id).unwrap().size().as_vec2();
                let scale_proportion = WINDOW_HEIGHT / scenery_size.y;

                commands.spawn((
                    Name::new("Background"),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(scenery_size * scale_proportion),
                            ..default()
                        },
                        texture: scenery_assets.cicest_sprite.clone(),
                        // transform: Transform::from_scale(Vec3::new(1.65, 1.65, 1.)),
                        ..default()
                    },
                ));
                break;
            }
            _ => {}
        }
    }
}

fn spawn_floor(mut commands: Commands) {
    commands.spawn((
        Name::new("Floor"),
        Floor,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH, FLOOR_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0., -HALF_WINDOW_HEIGHT + FLOOR_HEIGHT / 2., 0.1),
            visibility: Visibility::Hidden,
            ..default()
        },
        // Collider::cuboid(WINDOW_WIDTH / 2., FLOOR_HEIGHT / 2.),
        Collider::halfspace(Vec2::Y).unwrap(),
    ));
}

fn apply_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::Y * -9.81;
}
