use std::ops::Index;
use std::rc::Rc;
use engine::eng::{WIDTH, HEIGHT};
use winit::event::VirtualKeyCode;

use engine::animations::{AnimationSet, PSZ};
use engine::sprite::{Action, Sprite};
use engine::tiles::*;
use engine::types::*;

const SSZ: Vec2i = Vec2i { x: WIDTH as i32, y: HEIGHT as i32}; // SCREEN SIZE
const TSZ: i32 = 16; // TILE SIZE

const PPOS: Vec2i = Vec2i { x: (SSZ.x / 2) - (PSZ.x / 2), y: (SSZ.y / 2) - (PSZ.y / 2) };

const DOWN: usize = 0;
const UP: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;
const SPACE: usize = 4;

struct Assets {
    spritesheet: Rc<Image>,
    numsheet: Rc<Image>,
    textsheet: Rc<Image>,
}

struct State {
    map: Tilemap,

    p_anims: AnimationSet,
    p_sprite: Sprite,

    tapc: [u8; 4],
    movec: u8,
    move_lock: bool,

    dir: usize,
    pos: Pos,
}

impl State {
    pub fn new(map: Tilemap) -> Self {
        let pos = Pos { x: 10., y: 10. }; // TODO

        let p_anims = AnimationSet::new(1);
        let p_sprite = Sprite {
            animation_state: p_anims.play_animation(Action::StandD),
            shape: Rect {
                pos: Vec2i { x: 20, y: 20 },
                sz: PSZ,
            },
        };

        Self {
            map,
            p_anims,
            p_sprite,
            tapc: [0; 4],
            movec: 0,
            move_lock: false,
            dir: 0,
            pos
        }
    }

    fn anim(&mut self, act: Action) {
        self.p_sprite.animation_state = self.p_anims.play_animation(act);
    }
}

struct Game {}

fn main() {
    engine::eng::go::<Game>();
}

// [Down, Up, Left, Right, Space]
fn update_state(s: &mut State, now_keys: &[bool], prev_keys: &[bool]) {

    // GESTURE: RELEASE
    if !now_keys[DOWN] && prev_keys[DOWN] && s.dir == DOWN {
        s.move_lock = false;
    }
    if !now_keys[UP] && prev_keys[UP] && s.dir == UP {
        s.move_lock = false;
    }
    if !now_keys[LEFT] && prev_keys[LEFT] && s.dir == LEFT {
        s.move_lock = false;
    }
    if !now_keys[RIGHT] && prev_keys[RIGHT] && s.dir == RIGHT {
        s.move_lock = false;
    }

    if s.movec == 0 {

        // GESTURE: TAP
        if !now_keys[DOWN] && s.tapc[DOWN] > 0 {
            s.anim(Action::StandD);
        }
        if !now_keys[UP] && s.tapc[UP] > 0 {
            s.anim(Action::StandU);
        }
        if !now_keys[LEFT] && s.tapc[LEFT] > 0 {
            s.anim(Action::StandL);
        }
        if !now_keys[RIGHT] && s.tapc[RIGHT] > 0 {
            s.anim(Action::StandR);
        }

        // GESTURE: START
        if now_keys[DOWN] && !prev_keys[DOWN] {
            s.dir = DOWN;
            s.tapc[DOWN] = 8;
        }
        if now_keys[UP] && !prev_keys[UP] {
            s.dir = UP;
            s.tapc[UP] = 8;
        }
        if now_keys[LEFT] && !prev_keys[LEFT] {
            s.dir = LEFT;
            s.tapc[LEFT] = 8;
        }
        if now_keys[RIGHT] && !prev_keys[RIGHT] {
            s.dir = RIGHT;
            s.tapc[RIGHT] = 8;
        }

        // GESTURE: NEW HOLD
        if !s.move_lock {
            if now_keys[DOWN] && s.tapc[DOWN] == 0 {
                s.anim(Action::WalkD);
                if s.movec == 0 { s.movec = 32; }
            }
            if now_keys[UP] && s.tapc[UP] == 0 {
                s.anim(Action::WalkU);
                if s.movec == 0 { s.movec = 32; }
            }
            if now_keys[LEFT] && s.tapc[LEFT] == 0 {
                s.anim(Action::WalkL);
                if s.movec == 0 { s.movec = 32; }
            }
            if now_keys[RIGHT] && s.tapc[RIGHT] == 0 {
                s.anim(Action::WalkR);
                if s.movec == 0 { s.movec = 32; }
            }
        }
    }

    // SPACE START


    // HANDLE MOVEMENT
    if s.movec > 0 { 
        s.movec -= 1; 

        if s.movec % 2 == 1 {
            match s.dir {
                DOWN => s.map.translate_y(-1),
                UP => s.map.translate_y(1),
                LEFT => s.map.translate_x(1),
                RIGHT => s.map.translate_x(-1),
                _ => ()
            }
        }

        if s.movec == 0 {
            if s.move_lock {
                s.movec = 32;
            } else {
                match s.dir {
                    DOWN => s.anim(Action::StandD),
                    UP => s.anim(Action::StandU),
                    LEFT => s.anim(Action::StandL),
                    RIGHT => s.anim(Action::StandR),
                    _ => ()
                }
            }
        }
    }

    // DECREASE TAP COUNTERS
    for i in 0..4 {
        if s.tapc[i] > 0 { s.tapc[i] -= 1; }
    }

    // if now_keys[0] // DOWN
    // {
    //     s.map.translate_y(-1);
    //     s.anim(Action::WalkD);
    // }

    // if now_keys[1] // UP
    // {
    //     s.map.translate_y(1);
    // }

    // if now_keys[2] // LEFT
    // {
    //     s.map.translate_x(1);
    // }

    // if now_keys[3] // RIGHT
    // {
    //     s.map.translate_x(-1);
    // }
}

fn render_player(state: &mut State, assets: &mut Assets, fb2d: &mut Image) {
    fb2d.bitblt(
        &assets.spritesheet,
        state.p_sprite.play_animation(&20), // TODO: investigate speedup
        PPOS,
    );
}

impl engine::eng::Game for Game {
    type Assets = Assets;
    type State = State;
    fn new() -> (State, Assets) {
        let tilesheet = Rc::new(Image::from_file(std::path::Path::new(
            "game/content/tilesheet.png",
        )));
        let spritesheet = Rc::new(Image::from_file(std::path::Path::new(
            "game/content/sp01ash.png",
        )));
        let numsheet = Rc::new(Image::from_file(std::path::Path::new(
            "game/content/numsheet.png",
        )));
        let textsheet = Rc::new(Image::from_file(std::path::Path::new(
            "game/content/textsheet.png",
        ))); 
        let tileset = Rc::new(Tileset::new(
            vec![
                Tile { solid: true },
                Tile { solid: true },
                Tile { solid: true },
                Tile { solid: true },
                Tile { solid: true },
                Tile { solid: false },
                Tile { solid: false },
                Tile { solid: false },
                Tile { solid: false },
                Tile { solid: false },
            ],
            tilesheet,
        ));

        let map = Tilemap::new(
            Vec2i { x: -160, y: -160 }, // TODO: by map/screen width
            (40, 40),
            tileset,
            (0_usize..1600).map(|x| (x + ((x / 40 + 1) % 2)) % 2).collect::<Vec<usize>>(),
        );

        let assets = Assets {
            spritesheet,
            numsheet,
            textsheet,
        };

        let state = State::new(map);
        (state, assets)
    }

    fn update(state: &mut State, _assets: &mut Assets, now_keys: &[bool], prev_keys: &[bool]) {

        // let now_keys = vec![
        //     now_keys[VirtualKeyCode::Down as usize],
        //     now_keys[VirtualKeyCode::Up as usize],
        //     now_keys[VirtualKeyCode::Left as usize],
        //     now_keys[VirtualKeyCode::Right as usize],
        // ];

        // let prev_keys = vec![
        //     prev_keys[VirtualKeyCode::Down as usize],
        //     prev_keys[VirtualKeyCode::Up as usize],
        //     prev_keys[VirtualKeyCode::Left as usize],
        //     prev_keys[VirtualKeyCode::Right as usize],
        // ];

        // let prev2_keys = vec![
        //     prev2_keys[VirtualKeyCode::Down as usize],
        //     prev2_keys[VirtualKeyCode::Up as usize],
        //     prev2_keys[VirtualKeyCode::Left as usize],
        //     prev2_keys[VirtualKeyCode::Right as usize],
        // ];

        update_state(state, now_keys, prev_keys);
    }

    fn render(state: &mut State, assets: &mut Assets, fb2d: &mut Image) {
        state.map.draw(fb2d);
        render_player(state, assets, fb2d);
    }
}
