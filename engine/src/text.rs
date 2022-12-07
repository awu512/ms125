use crate::types::{HEIGHT, TILE_SZ};
use crate::types::{Image, Rect, Vec2i};

use std::rc::Rc;

pub struct Charcode(usize);

pub struct Textset {
    image: Rc<Image>,
    get_coord: fn(usize) -> Vec2i
}

impl Textset {
    pub fn new(path: &str, get_coord: fn(usize) -> Vec2i) -> Self {
        Self {
            image: Rc::new(Image::from_file(std::path::Path::new(path))),
            get_coord
        }
    }
    
    fn get_rect(&self, c: usize) -> Rect {
        Rect {
            pos: (self.get_coord)(c),
            sz: Vec2i {
                x: TILE_SZ,
                y: TILE_SZ,
            },
        }
    }
}

pub struct Textbox {
    pub position: Vec2i,
    dims: (usize, usize),
    textset: Rc<Textset>,
    base: Vec<Rect>,
    rows: Vec<[usize; 19]>,
    ptr: usize,
}

impl Textbox {
    pub fn new(textset: Rc<Textset>, text: &str) -> Self {
        let base = (0usize..7).map(|x| textset.get_rect(x)).collect::<Vec<Rect>>();
        let rows = Textbox::parse(text);
        Self {
            position: Vec2i { x: 0, y: HEIGHT as i32 - 48 },
            dims: (22, 6),
            textset,
            base,
            rows,
            ptr: 1,
        }
    }

    // pub fn update(&mut self) {
    //     for i in 1..self.dims.0-1 { // top row
    //         self.cur_text[2 * self.dims.0 + i] = if i <= self.rows[0].len() {
    //             self.rows[0][i-1]
    //         } else {
    //             0
    //         }
    //     }
    //     for i in 1..self.dims.0-1 { // bottom row
    //         self.cur_text[4 * self.dims.0 + i] = if i <= self.rows[1].len() {
    //             self.rows[1][i-1]
    //         } else {
    //             0
    //         }
    //     }
    // }

    pub fn draw(&self, screen: &mut Image) {
        const W: usize = 21;
        const H: usize = 5;
        const WX: usize = 20;
        for y in 0..self.dims.1 {
            let ypx = (y * TILE_SZ as usize) as i32 + self.position.y;
            for x in 0..self.dims.0 {
                let xpx = (x * TILE_SZ as usize) as i32 + self.position.x;

                let frame = match (x,y) {
                    (0,0)         => self.base[1], // top right
                    (W,0)         => self.base[2], // top left
                    (W,H)         => self.base[3], // bottom left
                    (0,H)         => self.base[4], // bottom right
                    (_,0) | (_,H) => self.base[5], // horizontal
                    (0,_) | (W,_) => self.base[6], // vertical
                    (WX,_)        => self.base[0], // space
                    (_,2)         => self.textset.get_rect(self.rows.get(self.ptr-1).unwrap()[x-1]), // top row
                    (_,4)         => self.textset.get_rect(self.rows.get(self.ptr).unwrap()[x-1]), // btm row
                    _             => self.base[0], // space
                };

                screen.bitblt(&self.textset.image, frame, Vec2i { x: xpx, y: ypx });
            }
        }
    }

    fn parse(s: &str) -> Vec<[usize; 19]> {
        let mut a = false;
        let mut q = false;
        let mut word = Vec::new();
        let mut words = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c.is_whitespace() {
                words.push(word.clone());
                word = Vec::new();
            } else if a {
                word.push(match c {
                    'd' => 256,
                    'l' => 257,
                    's' => 258,
                    't' => 259,
                    'v' => 260,
                    'r' => 261,
                    'm' => 262,
                    _ => panic!("Invalid char trailing an apostrophe")
                });
                a = false;
            } else if c == '\'' {
                a = true
            } else if c == '"' {
                if q { word.push(264); q = false }
                else { word.push(263); q = true }
            } else {
                word.push(c as usize);
            }
        }
        words.push(word.clone());

        let mut li = 0;
        let mut line = [0; 19];
        let mut r = Vec::new();
        for w in words {
            if li + w.len() + 1 >= 19 {
                r.push(line);
                li = 0;
                line = [0; 19];
            }
            for (wi, c) in w.iter().enumerate() {
                line[li + wi] = *c;
            }
            li += w.len() + 1;
        }
        r.push(line);
        if r.len() % 2 > 0 {
            r.push([0; 19]);
        }
        dbg!(&r);
        r
    }
}