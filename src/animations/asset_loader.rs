use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "sprites/gangster.png")]
    pub player: Handle<Image>,
    #[asset(path = "sprites/RTS_Crate.png")]
    pub wacky_crate: Handle<Image>,
}
