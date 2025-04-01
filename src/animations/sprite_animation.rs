use bevy::prelude::*;

pub enum AnimationEventKind {
    Finished,
    _Charging,
}

#[derive(Event)]
pub struct AnimationEvent {
    pub kind: AnimationEventKind,
    pub entity: Entity,
}

#[derive(Component)]
pub struct SpriteAnimState {
    pub start_index: usize,
    pub end_index: usize,
    pub timer: Timer,
}

impl Default for SpriteAnimState {
    fn default() -> Self {
        Self {
            start_index: 0,
            end_index: 6, // Assumes idle_frames = 13
            timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &mut SpriteAnimState)>,
    mut event_writer_anim: EventWriter<AnimationEvent>,
) {
    for (entity, mut sprite, mut anim_state) in query.iter_mut() {
        anim_state.timer.tick(time.delta());
        if anim_state.timer.finished() {
            if let Some(texture_atlas) = &mut sprite.texture_atlas {
                //println!("Current Index: {} End Index: {}",texture_atlas.index, anim_state.end_index);
                texture_atlas.index += 1;
                if texture_atlas.index > anim_state.end_index {
                    texture_atlas.index = anim_state.start_index;
                    event_writer_anim.send(AnimationEvent {
                        kind: AnimationEventKind::Finished,
                        entity,
                    });
                }
            }
        }
    }
}
