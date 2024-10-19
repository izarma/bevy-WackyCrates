use bevy::prelude::*;

struct Animation {
    frames: usize,
    frame_size: UVec2,
    texture_size: Vec2,
    texture_handle: Handle<Image>,
}