use bevy::{prelude::*, sprite::TextureAtlas};

use crate::binary::BinaryAsset;

#[derive(Resource, Default)]
pub struct State {
    pub font_handle: Handle<BinaryAsset>,
    pub char_atlas: Handle<TextureAtlas>,
    pub font_loaded: bool,
    pub tiles_added: bool,
}
