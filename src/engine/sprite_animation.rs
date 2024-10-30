use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteAnimState {
    pub start_index: usize,
    pub end_index: usize,
    pub frame_size: UVec2,
    pub texture_size: Vec2,
    pub timer: Timer,
}

struct Animation {
    frames: usize,
    frame_size: UVec2,
    texture_size: Vec2,
    texture_handle: Handle<Image>,
}

fn animate_sprite(
    time: Res<Time>,
    mut q: Query<(&mut TextureAtlas, &mut SpriteAnimState)>,
)
{
    for (mut atlas, mut anim) in q.iter_mut() {
        anim.timer.tick(time.delta());
        if anim.timer.finished() {
            atlas.index += 1;
            if atlas.index > anim.end_index {
                atlas.index = anim.start_index;
            }
        }
    }
}

