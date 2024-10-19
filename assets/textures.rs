use bevy::prelude::*;

pub struct TextureAssets {
    pub player_texture: Handle<Texture>,
}

pub fn load_textures(asset_server: Res<AssetServer>, mut textures: ResMut<Assets<Texture>>) {
    let player_texture = asset_server.load("knight.png");
    textures.add(player_texture.clone());

    // Store the handles in a resource if needed
    // texture_assets.player_texture = player_texture;
}