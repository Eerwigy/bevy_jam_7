use crate::AppState;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CameraTarget>();
    app.register_type::<CameraSmoothing>();
    app.init_resource::<CameraTarget>();
    app.init_resource::<CameraSmoothing>();
    app.add_systems(Startup, spawn_camera);
    app.add_systems(PostUpdate, follow_camera.run_if(in_state(AppState::Main)));
}

#[derive(Resource, Default, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct CameraTarget(Vec2);

#[derive(Resource, Default, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct CameraSmoothing(f32);

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 150.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn follow_camera(
    mut query: Query<&mut Transform, With<Camera2d>>,
    target: Res<CameraTarget>,
    smoothing: Res<CameraSmoothing>,
) {
    let Ok(mut camera) = query.single_mut() else {
        return;
    };

    camera.translation = camera.translation.lerp(
        vec3(target.0.x, target.0.y, camera.translation.z),
        0.0 - smoothing.0,
    );
}
