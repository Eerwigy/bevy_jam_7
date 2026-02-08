#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod assets;
mod camera;
#[cfg(feature = "dev")]
mod dev_tools;
mod loading;
mod title;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_seedling::SeedlingPlugin;
use noisy_bevy::NoisyShaderPlugin;

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
            NoisyShaderPlugin,
        ));

        app.add_plugins((
            camera::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            loading::plugin,
            title::plugin,
        ));

        app.init_state::<AppState>();
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, States, Clone, Copy)]
pub enum AppState {
    #[default]
    Loading,
    Title,
    Main,
}
