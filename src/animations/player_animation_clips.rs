use crate::animations::player_animation_state::*;
use crate::animations::sprite_animation::*;
use crate::engine::player::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationClips {
    pub idle_1: AnimationClip,
    pub idle_2: AnimationClip,
    pub walk: AnimationClip,
    pub run: AnimationClip,
    pub jump: AnimationClip,
    pub attack_1: AnimationClip,
    pub attack_2: AnimationClip,
    pub attack_3: AnimationClip,
    pub hurt: AnimationClip,
    pub dead: AnimationClip,
}

pub struct AnimationClip {
    pub start: usize,
    pub end: usize,
    pub frame_duration: f32,
}

pub fn setup_animation_clips(mut commands: Commands) {
    // Define animation indices based on your frame counts
    commands.insert_resource(AnimationClips {
        idle_1: AnimationClip {
            start: 0,
            end: 6,
            frame_duration: 1.0 / 12.0,
        },
        idle_2: AnimationClip {
            start: 7,
            end: 19,
            frame_duration: 1.0 / 12.0,
        },
        walk: AnimationClip {
            start: 20,
            end: 29,
            frame_duration: 1.0 / 10.0,
        },
        run: AnimationClip {
            start: 30,
            end: 39,
            frame_duration: 1.0 / 10.0,
        },
        jump: AnimationClip {
            start: 40,
            end: 49,
            frame_duration: 1.0 / 10.0,
        },
        attack_1: AnimationClip {
            start: 50,
            end: 55,
            frame_duration: 1.0 / 15.0,
        },
        attack_2: AnimationClip {
            start: 56,
            end: 59,
            frame_duration: 1.0 / 15.0,
        },
        attack_3: AnimationClip {
            start: 60,
            end: 65,
            frame_duration: 1.0 / 15.0,
        },
        hurt: AnimationClip {
            start: 66,
            end: 69,
            frame_duration: 1.0 / 10.0,
        },
        dead: AnimationClip {
            start: 70,
            end: 74,
            frame_duration: 1.0 / 5.0,
        },
    });
}

pub fn update_player_animation(
    time: Res<Time>,
    player_animations: Res<AnimationClips>,
    mut query: Query<
        (
            &mut SpriteAnimState,
            &mut Sprite,
            &PlayerState,
            &mut PlayerStatus,
        ),
        Changed<PlayerState>,
    >,
) {
    for (mut anim_state, mut sprite, state, mut status) in query.iter_mut() {
        if matches!(state.current_state(), PlayerStateKind::Idle) {
            status.idle_timer.tick(time.delta());
        } else {
            status.idle_timer.reset();
        }
        let animation = match state.current_state() {
            PlayerStateKind::Idle => {
                if status.idle_timer.finished() {
                    &player_animations.idle_2
                } else {
                    &player_animations.idle_1
                }
            }
            PlayerStateKind::Walk(_) => &player_animations.walk,
            PlayerStateKind::Run(_) => &player_animations.run,
            PlayerStateKind::Jump(_) => &player_animations.jump,
            PlayerStateKind::Attack => match status.attack_combo {
                0 => &player_animations.attack_1,
                1 => &player_animations.attack_2,
                _ => &player_animations.attack_3,
            },
            PlayerStateKind::Hurt => &player_animations.hurt,
            PlayerStateKind::Dead => &player_animations.dead,
            //_ => continue,
        };
        anim_state.start_index = animation.start;
        anim_state.end_index = animation.end;
        anim_state.timer = Timer::from_seconds(animation.frame_duration, TimerMode::Repeating);
        // Reset texture atlas
        if let Some(texture_atlas) = &mut sprite.texture_atlas {
            texture_atlas.index = animation.start;
        }
    }
}
