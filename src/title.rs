use crate::{AppState, assets::FontsCollection};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Title), setup);
}

fn setup(mut commands: Commands, fonts: Res<FontsCollection>) {
    commands.spawn((
        Name::new("Main Node"),
        DespawnOnExit(AppState::Title),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Name::new("Title"),
                Text::new("Fever Dream Chess"),
                TextFont {
                    font: fonts.title.clone(),
                    font_size: 70.0,
                    ..default()
                }
            ),
            (
                Name::new("Play Button"),
                Text::new("Press [SPACE] to PLay"),
                TextFont {
                    font: fonts.title.clone(),
                    font_size: 20.0,
                    ..default()
                }
            ),
        ],
    ));
}
