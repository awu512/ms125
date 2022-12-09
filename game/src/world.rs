use engine::animations::{Animation};
use engine::npc::{NPC, NPCSet};
use engine::sprite::Action;
use engine::types::{PSZ, TILE_SZ};
use engine::types::{Rect, Vec2i};

use std::fs::read_to_string;
use std::collections::HashMap;
use std::path::Path;
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
    let raw = read_to_string(Path::new("game/content/dlg01.json")).unwrap();
    let dlg: HashMap<String, String> = serde_json::from_str::<HashMap<String, String>>(&raw).unwrap();

    let npcs = vec![
        NPC::new(0, 0, Vec2i { x: 15, y: 18 }, dlg["BOY"].to_string()),
        NPC::new(1, 2, Vec2i { x: 7,  y: 13 }, dlg["WOMAN"].to_string()),
        NPC::new(2, 3, Vec2i { x: 20, y: 15 }, dlg["OAK"].to_string()),
        NPC::new(3, 0, Vec2i { x: 10, y: 10 }, dlg["MOM"].to_string()),
        NPC::new(4, 0, Vec2i { x: 7,  y: 9  }, dlg["HSIGN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 15, y: 9  }, dlg["RSIGN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 11, y: 13 }, dlg["TSIGN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 17, y: 17 }, dlg["BSIGN"].to_string()),
    ];

    NPCSet::new(
        "game/content/npcs01.png",
        npcs
    )
}

pub fn coords01(c: usize) -> Vec2i {
    let coords = if (65..=90).contains(&c) {
        Vec2i { x: (c - 65) as i32, y: 0 }
    } else if (97..=122).contains(&c) {
        Vec2i { x: (c - 97) as i32, y: 1 }
    } else if (48..=57).contains(&c) {
        Vec2i { x: (c - 48) as i32, y: 2 }
    } else {
        match c {
            1 => Vec2i { x: 0, y: 3 }, // top left
            2 => Vec2i { x: 2, y: 3 }, // top right
            3 => Vec2i { x: 5, y: 3 }, // bottom right
            4 => Vec2i { x: 4, y: 3 }, // bottom left
            5 => Vec2i { x: 1, y: 3 }, // horizontal bar
            6 => Vec2i { x: 3, y: 3 }, // vertical bar
            7 => Vec2i { x: 21, y: 3 }, // down arrow
            233 => Vec2i { x: 10, y: 2 }, // é
            256 => Vec2i { x: 11, y: 2 }, // 'd
            257 => Vec2i { x: 12, y: 2 }, // 'l
            258 => Vec2i { x: 13, y: 2 }, // 's
            259 => Vec2i { x: 14, y: 2 }, // 't
            260 => Vec2i { x: 15, y: 2 }, // 'v
            261 => Vec2i { x: 16, y: 2 }, // 'r
            262 => Vec2i { x: 17, y: 2 }, // 'm
            263 => Vec2i { x: 18, y: 3 }, // " open
            264 => Vec2i { x: 19, y: 3 }, // " close
            40 => Vec2i { x: 6, y: 3 }, // Vec2i { x: 
            41 => Vec2i { x: 7, y: 3 }, // )
            58 => Vec2i { x: 8, y: 3 }, // :
            59 => Vec2i { x: 9, y: 3 }, // ;
            91 => Vec2i { x: 10, y: 3 }, // [
            93 => Vec2i { x: 11, y: 3 }, // ]
            45 => Vec2i { x: 12, y: 3 }, // -
            63 => Vec2i { x: 13, y: 3 }, // ?
            33 => Vec2i { x: 14, y: 3 }, // !
            46 => Vec2i { x: 15, y: 3 }, // .
            47 => Vec2i { x: 16, y: 3 }, // /
            44 => Vec2i { x: 17, y: 3 }, // ,
            42 => Vec2i { x: 20, y: 3 }, // * -> ...
            _ => Vec2i { x: 22, y: 3 } // space (default)
        }
    };
    Vec2i { x: TILE_SZ * coords.x, y: TILE_SZ * coords.y } // space (default)
}