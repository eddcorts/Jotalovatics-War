use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod assets;
mod camera;
mod player;
mod scenery;
mod warrior;

const WINDOW_HEIGHT: f32 = 540.0;
const WINDOW_WIDTH: f32 = 960.0;
const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.;
const HALF_WINDOW_WIDTH: f32 = -WINDOW_WIDTH / 2.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.5)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        title: "Jotalovatics War".to_string(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default(),
            RapierPhysicsPlugin::<()>::pixels_per_meter(100.0),
            #[cfg(debug_assertions)]
            RapierDebugRenderPlugin {
                mode: DebugRenderMode::all(),
                ..default()
            },
            camera::CameraPlugin,
            warrior::WarriorPlugin,
            player::PlayerPlugin,
            scenery::SceneryPlugin,
            assets::AssetsPlugin,
        ))
        // .add_plugins((InspectableRapierPlugin,))
        .run();
}
