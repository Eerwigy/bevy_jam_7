#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod assets;
mod behaviour;
mod camera;
mod chessboard;
#[cfg(feature = "dev")]
mod dev_tools;
mod faller;
mod loading;
mod stats;
mod title;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_seedling::SeedlingPlugin;
use rand::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bevy Jam 7".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            SeedlingPlugin::default(),
        ));

        app.add_plugins((
            camera::plugin,
            chessboard::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            faller::plugin,
            loading::plugin,
            title::plugin,
        ));

        app.init_state::<AppState>();
        app.insert_resource(ClearColor(Color::hsl(200.0, 0.9, 0.1)));
        app.add_systems(Update, update_typewriters);
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, States, Clone, Copy)]
pub enum AppState {
    #[default]
    Loading,
    Title,
    Main,
}

#[derive(Component)]
pub struct Typewriter {
    pub full_text: String,
    pub visible_chars: usize,
    pub timer: Timer,
}

fn update_typewriters(mut query: Query<(&mut Text, &mut Typewriter)>, time: Res<Time>) {
    for (mut text, mut writer) in &mut query {
        writer.timer.tick(time.delta());

        if writer.timer.just_finished() {
            if writer.visible_chars < writer.full_text.len() {
                writer.visible_chars += 1;
                text.0 = writer.full_text[..writer.visible_chars].to_string();
            }
        }
    }
}

pub fn generate_character_text() -> String {
    const CHARS: &str = "GERNIAFDBM  ";
    const LEN: usize = CHARS.len();
    let mut string = String::new();
    let mut rng = rand::rng();
    let str_len = rng.random_range(25..40);
    for _ in 0..str_len {
        let x = CHARS.chars().nth(rng.random_range(..LEN)).unwrap();
        string.push(x);
    }
    string.push('!');

    string
}
