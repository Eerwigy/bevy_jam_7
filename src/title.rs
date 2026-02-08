use crate::{AppState, assets::FontsCollection};
use bevy::prelude::*;

const TITLE: &str = "Fever Dream Chess";
const PRESS_TO_PLAY: &str = "Press [SPACE] to PLay";

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Title), setup);
    app.add_systems(
        Update,
        (animate_text, press_space).run_if(in_state(AppState::Title)),
    );
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
                }
            ),
        ],
    ));
}

fn animate_text(
    mut title_done: Local<bool>,
    mut timer: Local<Timer>,
    mut char_index: Local<usize>,
    mut title_q: Query<&mut Text, (With<TitleText>, Without<PressToPlayText>)>,
    mut press_to_play_q: Query<&mut Text, (With<PressToPlayText>, Without<TitleText>)>,
    time: Res<Time>,
) {
    if timer.is_finished() && timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.05, TimerMode::Repeating);
        *char_index = 0;
        *title_done = false;
    }

    timer.tick(time.delta());

    if !timer.is_finished() {
        return;
    }

    if !*title_done {
        let mut text = title_q.single_mut().unwrap();
        let end = (*char_index + 1).min(TITLE.len());
        text.0 = TITLE[..end].to_string();
        *char_index += 1;

        if *char_index >= TITLE.len() {
            *title_done = true;
            *char_index = 0;
        }
    } else {
        let mut text = press_to_play_q.single_mut().unwrap();
        let end = (*char_index + 1).min(PRESS_TO_PLAY.len());
        text.0 = PRESS_TO_PLAY[..end].to_string();
        *char_index += 1;
    }
}

fn press_space(mut state: ResMut<NextState<AppState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        state.set(AppState::Main);
    }
}
