use std::collections::HashMap;

use crate::types::{DOWN, UP, LEFT, RIGHT};
use crate::types::{Image, Rect, Vec2i};

pub struct NPC {
    pub id: i32,
    cur_dir: i32,
    def_dir: i32,
    pos: Vec2i,
    pub text: String,
    pub talked: bool
}

impl NPC {
    pub fn new(id: i32, dir: i32, pos: Vec2i, text: String) -> Self {
        Self { 
            id,
            cur_dir: dir,
            def_dir: dir,
            pos,
            text,
            talked: false
        }
    }

    pub fn turn_to_face(&mut self, dir: usize) {
        self.cur_dir = match dir {
            DOWN => UP,
            UP => DOWN,
            LEFT => RIGHT,
            RIGHT => LEFT,
            _ => panic!("Invalid direction")
        } as i32;
    }

    pub fn reset_dir(&mut self) {
        self.cur_dir = self.def_dir;
    }
}

pub struct NPCSet{
    image: Image,
    dict: HashMap<Vec2i, NPC>,
    pub npc_sz: Vec2i,
    pub fin: bool,
    pub fin_text: String
}

impl NPCSet {
    pub fn new(path: &str, npcs: Vec<NPC>, npc_sz: Vec2i, fin_text: String) -> Self {
        let mut dict: HashMap<Vec2i, NPC> = HashMap::new();
        for npc in npcs {
            dict.insert(npc.pos, npc);
        }
        Self {
            image: Image::from_file(std::path::Path::new(path)),
            dict,
            npc_sz,
            fin: false,
            fin_text
        }
    }

    pub fn draw(&self, fb2d: &mut Image, ppos: Vec2i, movec: u8, dir: usize) {
        let sub_pos = match dir {
            DOWN => Vec2i { x: 0, y: (movec as i32 / 2) },
            UP => Vec2i { x: 0, y: -(movec as i32 / 2) },
            LEFT => Vec2i { x: -(movec as i32 / 2), y: 0 },
            RIGHT => Vec2i { x: (movec as i32 / 2), y: 0 },
            _ => panic!("Invalid direction")
        };

        let adj = if self.npc_sz.y > 16 { 2 } else { 0 };

        for npc in self.dict.values() {
            fb2d.bitblt(
                &self.image, 
                Rect { 
                    pos: Vec2i { x: self.npc_sz.x * npc.cur_dir, y: self.npc_sz.y * npc.id }, sz: self.npc_sz 
                }, 
                sub_pos + Vec2i { 
                    x: 16 * (5 + npc.pos.x - ppos.x), 
                    y: 16 * (5 + npc.pos.y - ppos.y) + adj
                }
            );
        }
    }

    pub fn at(&mut self, pos: Vec2i) -> Option<&mut NPC> {
        self.dict.get_mut(&pos)
    }
}