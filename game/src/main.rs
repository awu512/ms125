use std::ops::RangeBounds;
use std::rc::Rc;
use winit;

use engine;
use engine::animations::AnimationSet;
use engine::sprite::{Action, Character, Sprite};
use engine::tiles::*;
use engine::types::*;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 320;

const PSZ: Vec2i = Vec2i { x: 20, y: 32 };
const PPOS: Vec2i = Vec2i { x: (WIDTH / 2) as i32 + (PSZ.x / 2), y: (HEIGHT / 2) as i32 + (PSZ.y / 2) };

const TILE_SZ: i32 = 16;

struct Assets {
    spritesheet: Rc<Image>,
    numsheet: Rc<Image>,
    textsheet: Rc<Image>,
}

struct State {
    map: Tilemap,
    facing: Vec<bool>, // up/down, right/left
    pos: Pos,
}

impl State {
    pub fn new(map: Tilemap) -> Self {
        let facing = vec![true, true];
        let pos = Pos { x: 10., y: 10. }; // REPLACE

        Self {
            map,
            facing: vec![true, true],
            pos
        }
    }
}

struct Game {}

fn main() {
    engine::eng::go::<Game>();
}

// [Up, Left, Right, Down]
fn update_state(s: &mut State, now_keys: &[bool], prev_keys: &[bool]) {

    if now_keys[0] // UP
    {
        s.map.translate_y(-1);
        s.facing[0] = true;
    }

    if now_keys[1] // LEFT
    {
        s.map.translate_x(-1);
        s.facing[1] = false;
    }

    if now_keys[2] // RIGHT
    {
        s.map.translate_x(1);
        s.facing[1] = true;
    }

    if now_keys[3] // DOWN
    {
        s.map.translate_y(1);
        s.facing[0] = false;
    }
}

fn render_player(state: &mut State, assets: &mut Assets, fb2d: &mut Image) {
    let temp = Rect {
        pos: Vec2i { x: 10, y: 10 },
        sz: PSZ,
    };

    fb2d.bitblt(
        &assets.spritesheet,
        temp,
        Vec2i { x: 800, y: 800 },
        false,
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
            "game/content/spritesheet.png",
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
            Vec2i { x: -160, y: -160 },
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
        use winit::event::VirtualKeyCode;

        let now_keys = vec![
            now_keys[VirtualKeyCode::Up as usize],
            now_keys[VirtualKeyCode::Left as usize],
            now_keys[VirtualKeyCode::Right as usize],
            now_keys[VirtualKeyCode::Down as usize],
        ];

        let prev_keys = vec![
            prev_keys[VirtualKeyCode::Up as usize],
            prev_keys[VirtualKeyCode::Left as usize],
            prev_keys[VirtualKeyCode::Right as usize],
            prev_keys[VirtualKeyCode::Down as usize],
        ];

        update_state(state, &now_keys, &prev_keys);
    }

    fn render(state: &mut State, assets: &mut Assets, fb2d: &mut Image) {
        render_player(state, assets, fb2d);
        state.map.draw(fb2d);
        render_player(state, assets, fb2d);
    }
}
