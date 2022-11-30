use std::path::Path;
use std::rc::Rc;

use engine::animations::AnimationSet;
use engine::sprite::{Action, Sprite};
use engine::tiles::*;
use engine::types::*;

struct Assets {
    spritesheet: Rc<Image>,
    numsheet: Rc<Image>,
    textsheet: Rc<Image>,
}

struct State {
    map: Tilemap,

    p_anims: AnimationSet,
    p_sprite: Sprite,

    movec: u8,
    cur_dir: usize,
    next_dir: Option<usize>,

    pos: Pos,
}

impl State {
    pub fn new(map: Tilemap) -> Self {
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
            movec: 0,
            cur_dir: DOWN,
            next_dir: None,
            pos: START,
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
    // RELEASED -> clear next_dir
    if !now_keys[DOWN] && prev_keys[DOWN] && s.next_dir == Some(DOWN) {
        s.next_dir = None;
    }
    if !now_keys[UP] && prev_keys[UP] && s.next_dir == Some(UP) {
        s.next_dir = None;
    }
    if !now_keys[LEFT] && prev_keys[LEFT] && s.next_dir == Some(LEFT) {
        s.next_dir = None;
    }
    if !now_keys[RIGHT] && prev_keys[RIGHT] && s.next_dir == Some(RIGHT) {
        s.next_dir = None;
    }

    // PRESSED -> set next_dir
    if s.next_dir == None {
        if now_keys[DOWN] { s.next_dir = Some(DOWN) }
        if now_keys[UP] { s.next_dir = Some(UP) }
        if now_keys[LEFT] { s.next_dir = Some(LEFT) }
        if now_keys[RIGHT] { s.next_dir = Some(RIGHT) }
    }

    // MOVEMENT DONE
    if s.movec == 0 {
        if s.next_dir == None { // NO HELD KEY
            // stand in current direction
            match s.cur_dir {
                DOWN => s.anim(Action::StandD),
                UP => s.anim(Action::StandU),
                LEFT => s.anim(Action::StandL),
                RIGHT => s.anim(Action::StandR),
                _ => ()
            }
        } else {
            // if same dir, do nothing
            if s.cur_dir != s.next_dir.unwrap() || s.p_sprite.animation_state.action.is_standing() { 
                s.cur_dir = s.next_dir.unwrap();
                
                match s.cur_dir {
                    DOWN => s.anim(Action::WalkD),
                    UP => s.anim(Action::WalkU),
                    LEFT => s.anim(Action::WalkL),
                    RIGHT => s.anim(Action::WalkR),
                    _ => ()
                }
            }

            if s.map.can_move(s.pos, s.cur_dir) {
                s.pos.walk(s.cur_dir);
                s.movec = 32;
            }
        }
    }
    
    if s.movec > 0 {
        s.movec -= 1;

        if s.movec % 2 == 1 {
            match s.cur_dir {
                DOWN => s.map.translate_y(-1),
                UP => s.map.translate_y(1),
                LEFT => s.map.translate_x(1),
                RIGHT => s.map.translate_x(-1),
                _ => ()
            }
        }
    }
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
            "game/content/ts01.png",
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

        let solid01 = (0..96)
            .map(|x| Tile { solid: !(x == 0 || x == 3 || x == 44 || x == 57) })
            .collect::<Vec<Tile>>();

        let tileset = Rc::new(Tileset::new(
            solid01,
            tilesheet,
        ));

        let map = Tilemap::from_csv(
            Vec2i { x: PPOS.x - MOVE_SZ * START.x, y: PPOS.y - MOVE_SZ * START.y },
            (56, 54),
            tileset,
            Path::new("game/content/tm01.csv"),
            2,
            vec![0, 3, 44, 57],
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
        update_state(state, now_keys, prev_keys);
    }

    fn render(state: &mut State, assets: &mut Assets, fb2d: &mut Image) {
        state.map.draw(fb2d);
        render_player(state, assets, fb2d);
    }
}
