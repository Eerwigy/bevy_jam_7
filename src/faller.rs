use crate::{
    AppState,
    assets::{SpritesBgCollection, SpritesFgCollection},
};
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::{FRAC_PI_3, PI};

const FALLER_SPEED: f32 = 100.0;
const FALLER_ROTATION: f32 = 2.0 * FRAC_PI_3;
const FALLER_HALF: f32 = 100.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Title), spawn_fallers);
    app.add_systems(Update, update_fallers);
}

#[derive(Component)]
struct Faller;

fn spawn_fallers(
    mut commands: Commands,
    bg: Res<SpritesBgCollection>,
    fg: Res<SpritesFgCollection>,
    camera: Query<&Projection, With<Camera2d>>,
) {
    let mut rng = rand::rng();

    let Ok(Projection::Orthographic(proj)) = camera.single() else {
        return;
    };

    let width = proj.area.width();
    let height = proj.area.height();
    let half_width = width * 0.5;
    let half_height = height * 0.5;
    let half_height_faller = half_height + FALLER_HALF;

    for _ in 0..10 {
        commands.spawn((
            Name::new("Faller"),
            Faller,
            Transform::from_xyz(
                rng.random_range(-half_width..half_width),
                rng.random_range(-half_height_faller..half_height_faller),
                -10.0,
            )
            .with_rotation(Quat::from_rotation_z(rng.random_range(-PI..PI))),
            children![
                Sprite::from_image(bg.pawn.clone()),
                Sprite::from_image(fg.pawn.clone()),
            ],
        ));
    }
}

fn update_fallers(
    mut query: Query<&mut Transform, (With<Faller>)>,
    time: Res<Time>,
    camera: Query<&Projection, With<Camera2d>>,
) {
    let dt = time.delta_secs();

    let displacement = FALLER_SPEED * dt;
    let rotation = FALLER_ROTATION * dt;

    let mut rng = rand::rng();

    let Ok(Projection::Orthographic(proj)) = camera.single() else {
        return;
    };

    let width = proj.area.width();
    let height = proj.area.height();
    let half_width = width * 0.5;
    let half_height = height * 0.5;
    let half_height_faller = half_height + FALLER_HALF;

    for mut faller in &mut query {
        faller.translation.y -= displacement;
        faller.rotate_local_z(rotation);

        if faller.translation.y < -half_height_faller {
            faller.translation.x = rng.random_range(-half_width..half_width);
            faller.translation.y = half_height_faller;
        }
    }
}
