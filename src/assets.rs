use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct FontsCollection {
    #[asset(path = "Frijole/Frijole-Regular.ttf")]
    pub title: Handle<Font>,
}
