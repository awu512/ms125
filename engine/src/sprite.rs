use crate::animations::AnimationState;
use crate::types::Rect;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Action {
    StandD,
    StandU,
    StandL,
    StandR,
    WalkD,
    WalkU,
    WalkL,
    WalkR,
}

#[allow(dead_code)]
pub struct Sprite {
    pub animation_state: AnimationState,
    pub shape: Rect,
}

impl Sprite {
    pub fn play_animation(&mut self, speedup_factor: &usize) -> Rect {
        self.animation_state.tick(speedup_factor) // you can use types to choose which animations to play
    }

    // pub fn set_animation(&mut self, animation_state: AnimationState) {
    //     self.action = animation_state.action;
    //     self.animation_state = animation_state;
    // }

    // #[allow(dead_code)]
    // fn draw() {}
    // #[allow(dead_code)]
    // fn tick_animation() {}
}
