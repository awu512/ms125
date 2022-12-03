use engine::animations::{Animation};
use engine::npc::{NPC, NPCSet};
use engine::sprite::Action;
use engine::types::{Rect, Vec2i, PSZ};

use std::collections::HashMap;
use std::rc::Rc;

pub fn anims01() -> HashMap<Action, Rc<Animation>> {
    let mut animations: HashMap<Action, Rc<Animation>> = HashMap::new();

    animations.insert(
        Action::StandD,
        Rc::new(Animation {
            frames: vec![Rect { pos: Vec2i { x: 0, y: 0 }, sz: PSZ }],
            frame_timings: vec![0],
            loops: true,
        }),
    );
    animations.insert(
        Action::StandU,
        Rc::new(Animation {
            frames: vec![Rect { pos: Vec2i { x: 0, y: 16 }, sz: PSZ }],
            frame_timings: vec![0],
            loops: true,
        }),
    );
    animations.insert(
        Action::StandL,
        Rc::new(Animation {
            frames: vec![Rect { pos: Vec2i { x: 0, y: 32 }, sz: PSZ }],
            frame_timings: vec![0],
            loops: true,
        }),
    );
    animations.insert(
        Action::StandR,
        Rc::new(Animation {
            frames: vec![Rect { pos: Vec2i { x: 0, y: 48 }, sz: PSZ }],
            frame_timings: vec![0],
            loops: true,
        }),
    );

    animations.insert(
        Action::WalkD,
        Rc::new(Animation {
            frames: vec![
                Rect { pos: Vec2i { x: 0, y: 0 }, sz: PSZ },
                Rect { pos: Vec2i { x: 16, y: 0 }, sz: PSZ },
                Rect { pos: Vec2i { x: 0, y: 0 }, sz: PSZ },
                Rect { pos: Vec2i { x: 32, y: 0 }, sz: PSZ },
            ],
            frame_timings: vec![0, 15, 30, 45],
            loops: true,
        }),
    );
    animations.insert(
        Action::WalkU,
        Rc::new(Animation {
            frames: vec![
                Rect { pos: Vec2i { x: 0, y: 16 }, sz: PSZ },
                Rect { pos: Vec2i { x: 16, y: 16 }, sz: PSZ },
                Rect { pos: Vec2i { x: 0, y: 16 }, sz: PSZ },
                Rect { pos: Vec2i { x: 32, y: 16 }, sz: PSZ },
            ],
            frame_timings: vec![0, 15, 30, 45],
            loops: true,
        }),
    );
    animations.insert(
        Action::WalkL,
        Rc::new(Animation {
            frames: vec![
                Rect { pos: Vec2i { x: 0, y: 32 }, sz: PSZ },
                Rect { pos: Vec2i { x: 16, y: 32 }, sz: PSZ },
            ],
            frame_timings: vec![0, 15],
            loops: true,
        }),
    );
    animations.insert(
        Action::WalkR,
        Rc::new(Animation {
            frames: vec![
                Rect { pos: Vec2i { x: 0, y: 48 }, sz: PSZ },
                Rect { pos: Vec2i { x: 16, y: 48 }, sz: PSZ },
            ],
            frame_timings: vec![0, 15],
            loops: true,
        }),
    );

    animations
}

pub fn npcs01() -> NPCSet {
    let npcs = vec![
        NPC::new(0, 0, Vec2i { x: 15, y: 18 }) // BOY
    ];

    NPCSet::new(
        "game/content/npcs01.png",
        npcs
    )
}