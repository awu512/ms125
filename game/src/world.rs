use engine::animations::{Animation};
use engine::npc::{NPC, NPCSet};
use engine::sprite::Action;
use engine::tiles::*;
use engine::types::*;

use std::fs::read_to_string;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

pub fn anims(sz: Vec2i) -> HashMap<Action, Rc<Animation>> {
    let mut animations: HashMap<Action, Rc<Animation>> = HashMap::new();

    animations.insert(
        Action::StandD,
        Rc::new(Animation {
            frames: vec![Rect { pos: Vec2i { x: 0, y: 0 }, sz }],
            frame_timings: vec![0],
            loops: true,
        }),
    );
    animations.insert(
        Action::StandU,
        Rc::new(Animation {
            frames: vec![Rect { pos: Vec2i { x: 0, y: sz.y }, sz }],
            frame_timings: vec![0],
            loops: true,
        }),
    );
    animations.insert(
        Action::StandL,
        Rc::new(Animation {
            frames: vec![Rect { pos: Vec2i { x: 0, y: 2*sz.y }, sz }],
            frame_timings: vec![0],
            loops: true,
        }),
    );
    animations.insert(
        Action::StandR,
        Rc::new(Animation {
            frames: vec![Rect { pos: Vec2i { x: 0, y: 3*sz.y }, sz }],
            frame_timings: vec![0],
            loops: true,
        }),
    );

    animations.insert(
        Action::WalkD,
        Rc::new(Animation {
            frames: vec![
                Rect { pos: Vec2i { x: 0, y: 0 }, sz },
                Rect { pos: Vec2i { x: sz.x, y: 0 }, sz },
                Rect { pos: Vec2i { x: 0, y: 0 }, sz },
                Rect { pos: Vec2i { x: 2*sz.x, y: 0 }, sz },
            ],
            frame_timings: vec![0, 15, 30, 45],
            loops: true,
        }),
    );
    animations.insert(
        Action::WalkU,
        Rc::new(Animation {
            frames: vec![
                Rect { pos: Vec2i { x: 0, y: sz.y }, sz },
                Rect { pos: Vec2i { x: sz.x, y: sz.y }, sz },
                Rect { pos: Vec2i { x: 0, y: sz.y }, sz },
                Rect { pos: Vec2i { x: 2*sz.x, y: sz.y }, sz },
            ],
            frame_timings: vec![0, 15, 30, 45],
            loops: true,
        }),
    );
    animations.insert(
        Action::WalkL,
        Rc::new(Animation {
            frames: vec![
                Rect { pos: Vec2i { x: 0, y: 2*sz.y }, sz },
                Rect { pos: Vec2i { x: sz.x, y: 2*sz.y }, sz },
            ],
            frame_timings: vec![0, 15],
            loops: true,
        }),
    );
    animations.insert(
        Action::WalkR,
        Rc::new(Animation {
            frames: vec![
                Rect { pos: Vec2i { x: 0, y: 3*sz.y }, sz },
                Rect { pos: Vec2i { x: sz.x, y: 3*sz.y }, sz },
            ],
            frame_timings: vec![0, 15],
            loops: true,
        }),
    );

    animations
}

pub fn text_coords(c: usize) -> Vec2i {
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
            6 => Vec2i { x: 1, y: 3 }, // horizontal bar
            7 => Vec2i { x: 3, y: 3 }, // vertical bar
            8 => Vec2i { x: 3, y: 3 }, // vertical bar
            9 => Vec2i { x: 21, y: 3 }, // down arrow
            11 => Vec2i { x: 0, y: 4 }, // top left
            12 => Vec2i { x: 2, y: 4 }, // top right
            13 => Vec2i { x: 4, y: 4 }, // bottom right
            14 => Vec2i { x: 6, y: 4 }, // bottom left
            15 => Vec2i { x: 1, y: 4 }, // horizontal bar
            16 => Vec2i { x: 1, y: 4 }, // horizontal bar
            17 => Vec2i { x: 7, y: 4 }, // vertical bar left
            18 => Vec2i { x: 3, y: 4 }, // vertical bar right
            19 => Vec2i { x: 21, y: 3 }, // down arrow
            21 => Vec2i { x: 8, y: 4 }, // top left
            22 => Vec2i { x: 10, y: 4 }, // top right
            23 => Vec2i { x: 12, y: 4 }, // bottom right
            24 => Vec2i { x: 14, y: 4 }, // bottom left
            25 => Vec2i { x: 9, y: 4 }, // horizontal bar top
            26 => Vec2i { x: 13, y: 4 }, // horizontal bar bottom
            27 => Vec2i { x: 15, y: 4 }, // vertical bar left
            28 => Vec2i { x: 11, y: 4 }, // vertical bar right
            29 => Vec2i { x: 21, y: 3 }, // down arrow
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
            40 => Vec2i { x: 6, y: 3 }, // (
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

pub fn npcs(level: usize) -> NPCSet {
    match level {
        0 => npcs01(),
        1 => npcs02(),
        2 => npcs03(),
        _ => panic!("Invalid level")
    }
}

pub fn map01() -> Tilemap {
    let tilesheet = Rc::new(Image::from_file(std::path::Path::new(
        "game/content/ts01.png",
    )));
    let solid = (0..96)
        .map(|x| Tile { solid: !(x == 0 || x == 3 || x == 44 || x == 57) })
        .collect::<Vec<Tile>>();
    let tileset = Rc::new(Tileset::new(
        solid,
        tilesheet,
    ));
    Tilemap::from_csv(
        Vec2i { x: PPOS.x - MOVE_SZ * START.x, y: PPOS.y - MOVE_SZ * START.y },
        (56, 54),
        tileset,
        Path::new("game/content/tm01.csv"),
        2,
        vec![0, 3, 44, 57],
    )
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
        npcs,
        Vec2i { x: 16, y: 16 },
        String::from("Looks like you've talked to everyone here in PALLET TOWN! Why don't you walk around a bit?")
    )
}

pub fn map02() -> Tilemap {
    let tilesheet = Rc::new(Image::from_file(std::path::Path::new(
        "game/content/ts02.png",
    )));
    let solid = (0..96)
        .map(|x| Tile { solid: !(x == 0 || x == 3 || x == 5 || x == 6) })
        .collect::<Vec<Tile>>();
    let tileset = Rc::new(Tileset::new(
        solid,
        tilesheet,
    ));
    Tilemap::from_csv(
        Vec2i { x: PPOS.x - MOVE_SZ * START.x, y: PPOS.y - MOVE_SZ * START.y },
        (56, 54),
        tileset,
        Path::new("game/content/tm02.csv"),
        2,
        vec![0, 3, 5, 6],
    )
}

pub fn npcs02() -> NPCSet {
    let raw = read_to_string(Path::new("game/content/dlg02.json")).unwrap();
    let dlg: HashMap<String, String> = serde_json::from_str::<HashMap<String, String>>(&raw).unwrap();

    let npcs = vec![
        NPC::new(0, 0, Vec2i { x: 16, y: 14 }, dlg["BOY"].to_string()),
        NPC::new(1, 3, Vec2i { x: 7,  y: 7  }, dlg["RIVAL"].to_string()),
        NPC::new(2, 3, Vec2i { x: 21, y: 12 }, dlg["ELM"].to_string()),
        NPC::new(3, 1, Vec2i { x: 10, y: 13 }, dlg["WOMAN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 7,  y: 8  }, dlg["LSIGN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 15, y: 10 }, dlg["HSIGN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 12, y: 13 }, dlg["MSIGN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 13, y: 18 }, dlg["BSIGN"].to_string()),
    ];

    NPCSet::new(
        "game/content/npcs02.png",
        npcs,
        Vec2i { x: 16, y: 16 },
        String::from("Looks like you've talked to everyone here in PALLET TOWN! Why don't you try walking around a bit?")
    )
}

pub fn map03() -> Tilemap {
    let tilesheet = Rc::new(Image::from_file(std::path::Path::new(
        "game/content/ts03.png",
    )));
    let solid = (0..96)
        .map(|x| Tile { solid: x != 0 })
        .collect::<Vec<Tile>>();
    let tileset = Rc::new(Tileset::new(
        solid,
        tilesheet,
    ));
    Tilemap::from_csv(
        Vec2i { x: PPOS.x - MOVE_SZ * START.x, y: PPOS.y - MOVE_SZ * START.y },
        (56, 54),
        tileset,
        Path::new("game/content/tm03.csv"),
        2,
        vec![0, 1, 2, 3, 34, 35, 36, 37, 70, 71, 72, 73, 74, 75, 104, 105, 106, 107, 108, 109],
    )
}

pub fn npcs03() -> NPCSet {
    let raw = read_to_string(Path::new("game/content/dlg03.json")).unwrap();
    let dlg: HashMap<String, String> = serde_json::from_str::<HashMap<String, String>>(&raw).unwrap();

    let npcs = vec![
        NPC::new(0, 3, Vec2i { x: 16, y: 16 }, dlg["BOY1"].to_string()),
        NPC::new(1, 0, Vec2i { x: 15,  y: 6  }, dlg["GIRL"].to_string()),
        NPC::new(2, 2, Vec2i { x: 6, y: 14 }, dlg["BIRCH"].to_string()),
        NPC::new(3, 2, Vec2i { x: 18, y: 18 }, dlg["BOY2"].to_string()),
        NPC::new(4, 0, Vec2i { x: 14,  y: 19 }, dlg["LSIGN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 12, y: 11 }, dlg["HSIGN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 18, y: 15 }, dlg["MSIGN"].to_string()),
        NPC::new(4, 0, Vec2i { x: 16, y: 11 }, dlg["FSIGN"].to_string()),
    ];

    NPCSet::new(
        "game/content/npcs03.png",
        npcs,
        Vec2i { x: 16, y: 20 },
        String::from("Looks like you've talked to everyone here in PALLET TOWN! Why don't you try walking around a bit?")
    )
}