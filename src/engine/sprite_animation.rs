use bevy::prelude::*;

pub enum AnimationEventKind {
    Finished,
    Charging,
}

#[derive(Event)]
pub struct AnimationEvent {
    pub kind: AnimationEventKind,
    pub _entity: Entity,
}

#[derive(Component)]
pub struct SpriteAnimState {
    pub start_index: usize,
    pub end_index: usize,
    pub frame_size: UVec2,
    pub timer: Timer,
}

pub struct Animation {
    pub frames: usize,
    pub frame_size: UVec2,
    pub texture_handle: Handle<Image>,
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(Entity, &mut TextureAtlas, &mut SpriteAnimState)>,
    mut event_writer_anim: EventWriter<AnimationEvent>,
)
{
    for (_entity, mut atlas, mut anim_state) in query.iter_mut() {
        anim_state.timer.tick(time.delta());
        if anim_state.timer.finished() {
            //println!("Current Index: {} End Index: {}", atlas.index, anim_state.end_index);
            atlas.index += 1;
            if atlas.index > anim_state.end_index {
                atlas.index = anim_state.start_index;
                event_writer_anim.send(AnimationEvent {
                    kind: AnimationEventKind::Finished,
                    _entity,
            });
        }
        }
    }
}

