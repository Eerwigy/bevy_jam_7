use crate::AppState;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Title), setup);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Node"),
        DespawnOnExit(AppState::Title),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            ..default()
        },
        children![(Name::new("Play Button")), Node { ..default() }],
    ));
}
