mod world;

use std::path::Path;
use std::rc::Rc;

use engine::animations::AnimationSet;
use engine::npc::NPCSet;
use engine::sprite::{Action, Sprite};
use engine::text::{Textbox, Textset};
use engine::tiles::*;
use engine::types::*;

struct Assets {
    spritesheet: Rc<Image>,
}

struct State {
    map: Tilemap,

    anims: AnimationSet,
    sprite: Sprite,

    npcs: NPCSet,

    movec: u8,
    cur_dir: usize,
    next_dir: Option<usize>,

    is_text: bool,
    textbox: Textbox
}

impl State {
    pub fn new(map: Tilemap) -> Self {
        let anims = AnimationSet::new(
            "game/content/sp01ash.png", 
            world::anims01()
        );
        let sprite = Sprite {
            animation_state: anims.play_animation(Action::StandD),
            pos: START,
        };
        let npcs = world::npcs01();

        let textset = Textset::new("game/content/text01.png", world::coords01);
        let textbox = Textbox::new(Rc::new(textset), "Hello world! This is a test. Plz work");

        Self {
            map,
            anims,
            sprite,
            npcs,
            movec: 0,
            cur_dir: DOWN,
            next_dir: None,
            is_text: false,
            textbox
        }
    }

    fn anim(&mut self, act: Action) {
        self.sprite.animation_state = self.anims.play_animation(act);
    }

    fn translate(&mut self) {
        match self.cur_dir {
            DOWN => self.map.translate_y(-1),
            UP => self.map.translate_y(1),
            LEFT => self.map.translate_x(1),
            RIGHT => self.map.translate_x(-1),
            _ => ()
        }
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

    let next_pos = match s.next_dir {
        Some(DOWN) => Vec2i { x: s.sprite.pos.x, y: s.sprite.pos.y + 1 },
        Some(UP) => Vec2i { x: s.sprite.pos.x, y: s.sprite.pos.y - 1 },
        Some(LEFT) => Vec2i { x: s.sprite.pos.x - 1, y: s.sprite.pos.y },
        Some(RIGHT) => Vec2i { x: s.sprite.pos.x + 1, y: s.sprite.pos.y },
        None => match s.cur_dir {
            DOWN => Vec2i { x: s.sprite.pos.x, y: s.sprite.pos.y + 1 },
            UP => Vec2i { x: s.sprite.pos.x, y: s.sprite.pos.y - 1 },
            LEFT => Vec2i { x: s.sprite.pos.x - 1, y: s.sprite.pos.y },
            RIGHT => Vec2i { x: s.sprite.pos.x + 1, y: s.sprite.pos.y },
            _ => panic!("Invalid direction")
        }
        _ => panic!("Invalid direction")
    };

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
            if s.cur_dir != s.next_dir.unwrap() || s.sprite.animation_state.action.is_standing() { 
                s.cur_dir = s.next_dir.unwrap();
                
                match s.cur_dir {
                    DOWN => s.anim(Action::WalkD),
                    UP => s.anim(Action::WalkU),
                    LEFT => s.anim(Action::WalkL),
                    RIGHT => s.anim(Action::WalkR),
                    _ => ()
                }
            };
    
            if s.map.can_move_to(next_pos) && matches!(s.npcs.at(next_pos), None) {
                s.sprite.pos.walk(s.cur_dir);
                s.movec = 32;
            }

        }

        // INTERACT KEY (SPACE)
        if now_keys[SPACE] && !prev_keys[SPACE] {
            if let Some(npc) = s.npcs.at(next_pos) {
                if s.is_text {
                    s.is_text = s.textbox.scroll();
                } else {
                    npc.turn_to_face(s.cur_dir);
                    s.textbox.set_text(&npc.text);
                    s.is_text = !s.is_text;
                }
            }
        }
    }

    // TEXT REVEAL
    if s.textbox.cptr < 40 * TSPEED {
        s.textbox.cptr += 1;
    }

    // HANDLE MOVEMENT
    if s.movec > 0 {
        s.movec -= 1;

        if s.movec % 2 == 1 {
            s.translate();
        }
    }
}

fn render_player(state: &mut State, assets: &mut Assets, fb2d: &mut Image) {
    fb2d.bitblt(
        &assets.spritesheet,
        state.sprite.play_animation(&20), // TODO: investigate speedup
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
        };

        let state = State::new(map);
        (state, assets)
    }

    fn update(s: &mut State, _assets: &mut Assets, now_keys: &[bool], prev_keys: &[bool]) {
        update_state(s, now_keys, prev_keys);
    }

    fn render(s: &mut State, assets: &mut Assets, fb2d: &mut Image) {
        s.map.draw(fb2d);
        s.npcs.draw(fb2d, s.sprite.pos, s.movec, s.cur_dir);
        render_player(s, assets, fb2d);

        if s.is_text {
            s.textbox.draw(fb2d);
        }
    }
}
