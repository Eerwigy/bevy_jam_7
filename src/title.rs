use crate::{AppState, Typewriter, assets::FontsCollection};
use bevy::prelude::*;

const TITLE: &str = "Fever Dream Chess";
const PRESS_TO_PLAY: &str = "Press [SPACE] to PLay";

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Title), setup);
    app.add_systems(Update, press_space.run_if(in_state(AppState::Title)));
}

#[derive(Component)]
struct TitleText;

#[derive(Component)]
struct PressToPlayText;

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
                Text::default(),
                TitleText,
                TextFont {
                    font: fonts.title.clone(),
                    font_size: 70.0,
                    ..default()
                },
                Typewriter {
                    full_text: TITLE.to_string(),
                    visible_chars: 0,
                    timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                }
            ),
            (
                Name::new("Play Button"),
                Text::default(),
                PressToPlayText,
                TextFont {
                    font: fonts.title.clone(),
                    font_size: 20.0,
                    ..default()
                },
                Typewriter {
                    full_text: PRESS_TO_PLAY.to_string(),
                    visible_chars: 0,
                    timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                }
            ),
        ],
    ));
}

fn press_space(mut state: ResMut<NextState<AppState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        state.set(AppState::Main);
    }
}
