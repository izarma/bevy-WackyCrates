use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteAnimState {
    pub start_index: usize,
    pub end_index: usize,
    pub frame_size: UVec2,
    pub texture_size: Vec2,
    pub timer: Timer,
}

pub struct Animation {
    pub frames: usize,
    pub frame_size: UVec2,
    pub texture_size: Vec2,
    pub texture_handle: Handle<Image>,
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &mut SpriteAnimState)>,
)
{
    for (mut atlas, mut anim_state) in query.iter_mut() {
        anim_state.timer.tick(time.delta());
        if anim_state.timer.finished() {
            atlas.index += 1;
            if atlas.index > anim_state.end_index {
                atlas.index = anim_state.start_index;
            }
        }
    }
}

