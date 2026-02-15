use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::sample::AudioSample;

#[derive(AssetCollection, Resource)]
pub struct FontsCollection {
    #[asset(path = "excalifont/Excalifont-Regular.ttf")]
    pub title: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct SpritesFgCollection {
    #[asset(path = "sprites/fg/pawn.png")]
    pub pawn: Handle<Image>,
    #[asset(path = "sprites/fg/knight.png")]
    pub knight: Handle<Image>,
    #[asset(path = "sprites/fg/bishop.png")]
    pub bishop: Handle<Image>,
    #[asset(path = "sprites/fg/rook.png")]
    pub rook: Handle<Image>,
    #[asset(path = "sprites/fg/queen.png")]
    pub queen: Handle<Image>,
    #[asset(path = "sprites/fg/king.png")]
    pub king: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SpritesBgCollection {
    #[asset(path = "sprites/bg/pawn.png")]
    pub pawn: Handle<Image>,
    #[asset(path = "sprites/bg/knight.png")]
    pub knight: Handle<Image>,
    #[asset(path = "sprites/bg/bishop.png")]
    pub bishop: Handle<Image>,
    #[asset(path = "sprites/bg/rook.png")]
    pub rook: Handle<Image>,
    #[asset(path = "sprites/bg/queen.png")]
    pub queen: Handle<Image>,
    #[asset(path = "sprites/bg/king.png")]
    pub king: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SoundsCollection {
    #[asset(path = "sound/passturn.wav")]
    pub passturn: Handle<AudioSample>,
    #[asset(path = "sound/vineboom.wav")]
    pub vineboom: Handle<AudioSample>,
}
