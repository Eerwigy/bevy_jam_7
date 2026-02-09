use crate::{
    AppState,
    assets::{FontsCollection, SpritesBgCollection, SpritesFgCollection},
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(AppState::Loading)
            .continue_to_state(AppState::Title)
            .load_collection::<FontsCollection>()
            .load_collection::<SpritesFgCollection>()
            .load_collection::<SpritesBgCollection>(),
    );

    app.add_systems(OnEnter(AppState::Loading), spawn_loading_screen);
    app.add_systems(
        Update,
        animate_loading_text.run_if(in_state(AppState::Loading)),
    );
    app.add_systems(OnExit(AppState::Loading), || {
        info!("Finished Loading");
    });
}

#[derive(Component)]
struct LoadingText;

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        Name::new("Loading Screen Ui Root"),
        DespawnOnExit(AppState::Loading),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
            ..default()
        },
        Pickable::IGNORE,
        children![(
            Name::new("Loading Text"),
            Text::new("Loading"),
            TextFont {
                font_size: 40.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            Pickable::IGNORE,
            LoadingText,
        )],
    ));
}

fn animate_loading_text(
    mut query: Query<&mut Text, With<LoadingText>>,
    mut timer: Local<Timer>,
    time: Res<Time>,
) {
    if timer.is_finished() && timer.elapsed_secs() == 0.0 {
        *timer = Timer::from_seconds(0.5, TimerMode::Repeating);
    }

    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let Ok(mut text) = query.single_mut() else {
        warn!("loading text missing");
        return;
    };

    text.0 = match text.0.as_str() {
        "Loading" => "Loading.".to_string(),
        "Loading." => "Loading..".to_string(),
        "Loading.." => "Loading...".to_string(),
        "Loading..." => "Loading".to_string(),
        _ => "Loading".to_string(),
    };
}
