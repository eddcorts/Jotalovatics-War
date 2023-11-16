use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_systems(Startup, (spawn_camera, disable_picking_mod_logs));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        // RaycastPickCamera::default()
    ));
}

fn disable_picking_mod_logs(
    mut picking_mod_logging_next_state: ResMut<NextState<debug::DebugPickingMode>>,
) {
    picking_mod_logging_next_state.set(debug::DebugPickingMode::Disabled);
}
