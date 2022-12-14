use crate::sprite::Action;
use crate::types::{Image, Rect};
use std::collections::hash_map::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub struct Animation {
    pub frames: Vec<Rect>,
    pub frame_timings: Vec<usize>,
    pub loops: bool,
}

#[allow(dead_code)]
impl Animation {
    // Should hold some data...
    // Be used to decide what frame to use...
    // Could have a query function like current_frame(&self, start_time:usize, now:usize, speedup_factor:usize)
    // Or could be ticked in-place with a function like tick(&self)
    pub fn initial_frame(&self) -> Rect {
        self.frames[0]
    }

    pub fn current_frame(&self, start_time: usize, now: usize, speedup_factor: &usize) -> Rect {
        let frame_timing = (now - start_time) / speedup_factor;
        self.frames[frame_timing]
    }

    #[allow(dead_code)]
    fn is_finished(&self, start_time: usize, now: usize, speedup_factor: &usize) -> bool {
        // return true if the end time of this animation is passed.
        (now - start_time) / speedup_factor >= self.frames.len()
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub struct AnimationState {
    // Here you'll need to track how far along in the animation you are.
    // You can also choose to have an Rc<Animation> to refer to the animation in use.
    // But you could also decide to just pass around the animation and state together
    // where needed.
    pub start_time: usize,
    pub now: usize,
    pub action: Action,
    pub animation: Rc<Animation>,
}

impl AnimationState {
    #[allow(dead_code)]
    pub fn tick(&mut self, speedup_factor: &usize) -> Rect {
        self.now += 1;
        if self
            .animation
            .is_finished(self.start_time, self.now, speedup_factor)
            && self.animation.loops
        {
            self.now = 0;
        }
        self.animation
            .current_frame(self.start_time, self.now, speedup_factor)
    }
}

pub struct AnimationSet {
    pub image: Image,
    pub animations: HashMap<Action, Rc<Animation>>,
}

impl AnimationSet {
    pub fn get_animation(&self, action: Action) -> &Rc<Animation> {
        // let this return an AnimationState, clone
        self.animations.get(&action).unwrap()
    }

    pub fn play_animation(&self, action: Action) -> AnimationState {
        AnimationState {
            start_time: 0,
            now: 0,
            action,
            animation: self.animations.get(&action).unwrap().clone(),
        }
    }

    pub fn get_image(&self) -> &Image {
        &self.image
    }

    pub fn get_reversed_image(&self) -> &Image {
        &self.image
    }

    pub fn set_animation(&mut self, animations: HashMap<Action, Rc<Animation>>) {
        self.animations = animations;
    }

    pub fn set_image(&mut self, image: Image) {
        self.image = image;
    }

    pub fn new(path: &std::path::Path, animations: HashMap<Action, Rc<Animation>>) -> Self {
        AnimationSet {
            image: Image::from_file(path),
            animations,
        }
    }
}
