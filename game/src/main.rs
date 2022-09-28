use std::ops::RangeBounds;
use std::rc::Rc;
use winit;

use engine;
use engine::animations::AnimationSet;
use engine::sprite::{Action, Character, Sprite};
use engine::tiles::*;
use engine::types::*;

const PLAYER_WIDTH: i32 = 20;
const PLAYER_HEIGHT: i32 = 32;
pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 320;
const TILE_SZ: i32 = 16;

struct Assets {
    spritesheet: Rc<Image>,
    numsheet: Rc<Image>,
    textsheet: Rc<Image>,
    tilemap: Tilemap,
}

struct State {
    p: PlayerState,
}

impl State {
    pub fn new() -> Self {
        let p = PlayerState::new();

        Self {
            p,
        }
    }
}

struct PlayerState {
    pos: Pos,
    vel: f32
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            pos: Pos { x: 0., y: 0. },
            vel: 0.05
        }
    }
}

struct Game {}

fn main() {
    engine::eng::go::<Game>();
}

// [Up, Left, Right, Down]
fn update_player(ps: &mut PlayerState, now_keys: &[bool], prev_keys: &[bool]) {

    if now_keys[0]
    {
        ps.pos.y -= ps.vel
    }

    if now_keys[1]
    {
        ps.pos.x -= ps.vel
    }

    if now_keys[2]
    {
        ps.pos.x += ps.vel
    }

    if now_keys[3]
    {
        ps.pos.y += ps.vel
    }
}

fn render_player(state: &mut PlayerState, assets: &mut Assets, fb2d: &mut Image) {
    let mut temp = Rect {
        pos: Vec2i { x: 0, y: 0 },
        sz: Vec2i { x: PLAYER_WIDTH, y: PLAYER_HEIGHT },
    };

    fb2d.bitblt(
        &assets.spritesheet,
        temp,
        state.pos.get(),
        false,
    );
}

impl engine::eng::Game for Game {
    type Assets = Assets;
    type State = State;
    fn new() -> (State, Assets) {
        let tilesheet = Rc::new(Image::from_file(std::path::Path::new(
            "content/tilesheet.png",
        )));
        let spritesheet = Rc::new(Image::from_file(std::path::Path::new(
            "content/spritesheet.png",
        )));
        let numsheet = Rc::new(Image::from_file(std::path::Path::new(
            "content/numsheet.png",
        )));
        let textsheet = Rc::new(Image::from_file(std::path::Path::new(
            "content/textsheet.png",
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
            tilesheet.clone(),
        ));
        let map = Tilemap::new(
            Vec2i { x: 0, y: 0 },
            (20, 20),
            tileset.clone(),
            vec![
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 0, 0, 0, 0, 0, 0, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0,
                6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 6, 6, 6, 6, 6, 6,
                6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 6, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 8, 5, 5, 5, 5, 5, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2,
                2, 2, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                4, 4, 4, 4, 4, 4, 4, 4,
            ],
        );

        let assets = Assets {
            spritesheet,
            numsheet,
            textsheet,
            tilemap: map,
        };
        let state = State::new();
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

        update_player(&mut state.p, &now_keys, &prev_keys);
    }

    fn render(state: &mut State, assets: &mut Assets, fb2d: &mut Image) {
        assets.tilemap.draw(fb2d);
        render_player(&mut state.p, assets, fb2d);
    }
}
