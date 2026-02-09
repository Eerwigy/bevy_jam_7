use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

#[derive(Resource, Default, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct CameraTarget(Vec2);

#[derive(Resource, Default, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct CameraSmoothing(f32);

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
