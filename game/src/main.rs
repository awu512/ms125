mod world;

use std::process::exit;
use std::rc::Rc;

use engine::animations::AnimationSet;
use engine::npc::NPCSet;
use engine::sprite::{Action, Sprite};
use engine::text::{Textbox, Textset, Textscreen};
use engine::tiles::*;
use engine::types::*;

struct Assets {
    citation: Rc<Image>,
}

struct State {
    maps: [Tilemap; 3],
    level: usize,

    talkc: u8,
    swapping: bool,

    anims: AnimationSet,
    sprite: Sprite,
    spritesheet: Rc<Image>,

    npcs: NPCSet,

    movec: u8,
    cur_dir: usize,
    next_dir: Option<usize>,

    is_text: bool,
    textbox: Textbox,

    textscreen: Textscreen,
    open: bool,
    end: bool,
    wipe_dir: i32,
    cit: i32
}

impl State {
    pub fn new() -> Self {
        let exe_path = std::env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();

        let maps = [world::map01(), world::map02(), world::map03()];
        let anims = AnimationSet::new(
            exe_dir.join("content/sp01ash.png").as_path(), 
            world::anims(Vec2i { x: 16, y: 16 })
        );
        let sprite = Sprite {
            animation_state: anims.play_animation(Action::StandD),
            pos: START,
            sz: Vec2i { x: 16, y: 16 }
        };
        let spritesheet = Rc::new(Image::from_file(
            exe_dir.join("content/sp01ash.png").as_path()
        ));
        let npcs = world::npcs01();

        let textset = Textset::new(
            exe_dir.join("content/textsheet.png").as_path(),
            world::text_coords
        );
        let textbox = Textbox::new(Rc::new(textset));

        let textset2 = Textset::new(
            exe_dir.join("content/textsheet2.png").as_path(),
            world::text_coords
        );
        let textscreen = Textscreen::new(Rc::new(textset2), &world::open_text());

        Self {
            maps,
            level: 0,
            talkc: 0,
            swapping: false,
            anims,
            sprite,
            spritesheet,
            npcs,
            movec: 0,
            cur_dir: DOWN,
            next_dir: None,
            is_text: false,
            textbox,
            textscreen,
            open: true,
            end: false,
            wipe_dir: -1,
            cit: -1,
        }
    }

    fn next_level(&mut self) {
        self.level += 1;
        self.swapping = false;
        self.talkc = 0;

        let exe_path = std::env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();

        if self.level == 1 {
            self.spritesheet = Rc::new(Image::from_file(
                exe_dir.join("content/sp02ash.png").as_path()
            ));
            self.anims = AnimationSet::new(
                exe_dir.join("content/sp02ash.png").as_path(), 
                world::anims(Vec2i { x: 16, y: 16 })
            );
        } else if self.level == 2 {
            self.spritesheet = Rc::new(Image::from_file(
                exe_dir.join("content/sp03ash.png").as_path(),
            ));
            self.anims = AnimationSet::new(
                exe_dir.join("content/sp03ash.png").as_path(), 
                world::anims(Vec2i { x: 16, y: 20 })
            );
            self.sprite.sz = Vec2i { x: 16, y: 20 };
        }

        match self.cur_dir {
            DOWN => self.anim(Action::StandD),
            UP => self.anim(Action::StandU),
            LEFT => self.anim(Action::StandL),
            RIGHT => self.anim(Action::StandR),
            _ => ()
        }

        self.textbox.set_base(self.level);
        self.npcs = world::npcs(self.level);
    }

    fn anim(&mut self, act: Action) {
        self.sprite.animation_state = self.anims.play_animation(act);
    }

    fn translate(&mut self) {
        for map in self.maps.iter_mut() {
            map.translate(self.cur_dir);
        }
    }

    fn circle_mask(&mut self) {
        let tix = 2 * self.sprite.pos.x as usize;
        let tiy = 2 * self.sprite.pos.y as usize;


        for y in 0..6 {
            match y {
                0 | 5 => for x in 0..2 {
                    self.maps[self.level].mask.unmask(tix + x, tiy - 2 + y);
                },
                1 | 4 => for x in 0..4 {
                    self.maps[self.level].mask.unmask(tix - 1 + x, tiy - 2 + y);
                },
                2 | 3 => for x in 0..6 {
                    self.maps[self.level].mask.unmask(tix - 2 + x, tiy - 2 + y);
                },
                _ => panic!("Something went wrong masking the map")
            }
        }
    }
}

struct Game {}

fn main() {
    engine::eng::go::<Game>();
}

// [Down, Up, Left, Right, Space]
fn update_state(s: &mut State, now_keys: &[bool], prev_keys: &[bool]) {
    // CITATIONS
    #[allow(clippy::collapsible_if)]
    if s.cit >= 0 {
        if now_keys[SPACE] && !prev_keys[SPACE] {
            if s.cit < 5 {
                s.cit += 1;
            } else {
                exit(0) 
            }
        } 
        return
    }

    // OPEN TEXT
    #[allow(clippy::collapsible_if)]
    if s.open {
        if now_keys[SPACE] && !prev_keys[SPACE] {
            if !s.textscreen.scroll() && s.open {
                s.open = false;
                s.textscreen.animc = WIPENUM - 1;
            }
        } 
        if s.textscreen.cptr < 40 * TSPEED {
            s.textscreen.cptr += 1;
        }
        return
    }

    // END TEXT
    #[allow(clippy::collapsible_if)]
    if s.end {
        if now_keys[SPACE] && !prev_keys[SPACE] {
            if !s.textscreen.scroll() && s.end {
                s.cit = 0;
                return
            }
        } 
        if s.textscreen.cptr < 40 * TSPEED {
            s.textscreen.cptr += 1;
        }
        return
    }

    if s.textscreen.animc == WIPENUM - 1 && s.level == 2 {
        s.end = true;
        s.textscreen.set_text(&world::end_text());
        s.is_text = true;
    }

    if s.textscreen.animc < WIPENUM && s.textscreen.animc > 0 {
        return
    }

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
    if s.next_dir == None && !s.is_text {
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
    
            if !s.is_text && (
               (s.maps[s.level].can_move_to(next_pos) && 
               matches!(s.npcs.at(next_pos), None)) ||
               ((Tilemap::swap_can_move_to(next_pos)) && 
               (s.swapping || !s.maps[s.level].can_move_to(s.sprite.pos))))
            {
                s.sprite.pos.walk(s.cur_dir);
                s.movec = 32;
            }

        }

        // INTERACT KEY (SPACE)
        if now_keys[SPACE] && !prev_keys[SPACE] && !s.swapping {
            if let Some(npc) = s.npcs.at(next_pos) {
                if s.is_text {
                    let more = s.textbox.scroll();
                    if !more {
                        if s.npcs.fin {
                            s.is_text = false;
                            if s.level == 2 {
                                s.textscreen.animc = 1;
                                s.wipe_dir = 1;
                                return
                            } else {
                                s.swapping = true;
                            }
                        } else if s.talkc >= 4 {
                            s.textbox.set_text(&s.npcs.fin_text);
                            s.npcs.fin = true;
                        } else {
                            s.is_text = false;
                        }
                    }
                } else {
                    npc.turn_to_face(s.cur_dir);
                    s.textbox.set_text(&npc.text);
                    s.is_text = !s.is_text;
                    if npc.id < 4 && !npc.talked {
                        s.talkc += 1;
                        npc.talked = true;
                    }
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
            if s.swapping {
                s.circle_mask();
            }
        }
    }

    // COMPLETE SWAP
    if s.swapping && s.maps[s.level].mask.swapc >= SWAPNUM {
        s.next_level();
    }
}

fn render_player(state: &mut State, assets: &mut Assets, fb2d: &mut Image) {
    fb2d.bitblt(
        &state.spritesheet,
        state.sprite.play_animation(&20),
        Vec2i {
            x: (WIDTH as i32 / 2) - (state.sprite.sz.x / 2),
            y: (HEIGHT as i32 / 2) - (state.sprite.sz.y / 2)
        }
    );
}

impl engine::eng::Game for Game {
    type Assets = Assets;
    type State = State;
    fn new() -> (State, Assets) {
        let exe_path = std::env::current_exe().unwrap();
        let exe_dir = exe_path.parent().unwrap();

        let citation = Rc::new(Image::from_file(
            exe_dir.join("content/citation.png").as_path(),
        ));

        let assets = Assets {
            citation,
        };

        let state = State::new();
        (state, assets)
    }

    fn update(s: &mut State, _assets: &mut Assets, now_keys: &[bool], prev_keys: &[bool]) {
        update_state(s, now_keys, prev_keys);
    }

    fn render(s: &mut State, assets: &mut Assets, fb2d: &mut Image) {
        if s.cit >= 0 {
            fb2d.bitblt(
                &assets.citation, 
                Rect { 
                    pos: Vec2i { x: 0, y: s.cit * 176 }, 
                    sz: Vec2i { x: 176, y: 176 }
                }, 
                Vec2i { x: 0, y: 0 }
            );
            return
        }

        if s.open || s.end {
            s.textscreen.draw(fb2d);
            return
        }

        if s.swapping {
            s.maps[s.level+1].draw(fb2d);
            s.maps[s.level].masked_draw(fb2d);
        } else {
            s.maps[s.level].draw(fb2d);
            s.npcs.draw(fb2d, s.sprite.pos, s.movec, s.cur_dir);
        }
        render_player(s, assets, fb2d);

        if s.is_text {
            s.textbox.draw(fb2d);
        }

        if s.textscreen.animc < WIPENUM && s.textscreen.animc > 0 {
            s.textscreen.anim(fb2d);
            s.textscreen.animc += s.wipe_dir;
        }
    }
}
