use crate::types::{HEIGHT, TILE_SZ, TSPEED, WIPENUM};
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
    rows: Vec<[usize; 20]>,
    rptr: usize,
    pub cptr: usize
}

impl Textbox {
    pub fn new(textset: Rc<Textset>) -> Self {
        let base = (0usize..=9).map(|x| textset.get_rect(x)).collect::<Vec<Rect>>();
        Self {
            position: Vec2i { x: 0, y: HEIGHT as i32 - 48 },
            dims: (22, 6),
            textset,
            base,
            rows: vec![],
            rptr: 1,
            cptr: 0
        }
    }

    pub fn set_base(&mut self, level: usize) {
        self.base = (10*level..=(10*level+9)).map(|x| self.textset.get_rect(x)).collect::<Vec<Rect>>();
    }

    pub fn set_text(&mut self, text: &str) {
        self.cptr = 0;
        self.rows = Textbox::parse(text);
        self.rptr = 1;
    }

    pub fn scroll(&mut self) -> bool {
        self.cptr = 0;
        self.rptr += 2;
        self.rptr < self.rows.len()
    }

    pub fn draw(&self, screen: &mut Image) {
        const W: usize = 21;
        const H: usize = 5;

        let is_last = self.rptr >= self.rows.len()-1;
        for y in 0..self.dims.1 {
            let ypx = (y * TILE_SZ as usize) as i32 + self.position.y;
            for x in 0..self.dims.0 {
                let xpx = (x * TILE_SZ as usize) as i32 + self.position.x;

                let frame = match (x,y) {
                    (0,0)  => self.base[1], // top left
                    (W,0)  => self.base[2], // top right
                    (W,H)  => self.base[3], // bottom right
                    (0,H)  => self.base[4], // bottom left
                    (_,0)  => self.base[5], // top horizontal
                    (_,H)  => self.base[6], // bottom horizontal
                    (0,_)  => self.base[7], // left vertical
                    (W,_)  => self.base[8], // right vertical
                    (20,4) => if !is_last { self.base[9] } else { self.base[0] },
                    (_,2)  => if TSPEED*(x - 1) <= self.cptr { // top row
                                        self.textset.get_rect(self.rows.get(self.rptr-1).unwrap()[x-1])
                                    } else { 
                                        self.base[0] 
                                    },
                    (_,4)  => if TSPEED*(x + 19) <= self.cptr { // btm row
                                        self.textset.get_rect(self.rows.get(self.rptr).unwrap()[x-1])
                                    } else { 
                                        self.base[0] 
                                    },
                    _      => self.base[0], // space
                };

                screen.bitblt(&self.textset.image, frame, Vec2i { x: xpx, y: ypx });
            }
        }
    }

    fn parse(s: &str) -> Vec<[usize; 20]> {
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
        let mut nl = false;
        let mut li = 0;
        let mut line = [0; 20];
        let mut r = Vec::new();
        for w in words {
            if nl {
                r.push(line);
                if r.len()%2 == 1 { r.push([0;20]) }
                li = 0;
                line = [0; 20];
            } else if li + w.len() + 1 >= 20 {
                r.push(line);
                li = 0;
                line = [0; 20];
            }
             
            for (wi,c) in w.iter().enumerate() {
                line[li + wi] = *c;
            }
            li += w.len() + 1;
            nl = matches!(w[w.len()-1], 33 | 63 | 46 | 42);
        }
        r.push(line);
        if r.len() % 2 > 0 {
            r.push([0; 20]);
        }
        r
    }
}

pub struct Textscreen {
    pub position: Vec2i,
    textset: Rc<Textset>,
    rows: Vec<[usize; 20]>,
    rptr: usize,
    pub cptr: usize,
    pub animc: i32
}

impl Textscreen {
    pub fn new(textset: Rc<Textset>, text: &str) -> Self {
        let rows = Textbox::parse(text);
        Self {
            position: Vec2i { x: 0, y: HEIGHT as i32 - 48 },
            textset,
            rows,
            rptr: 1,
            cptr: 0,
            animc: 242
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.cptr = 0;
        self.rows = Textbox::parse(text);
        self.rptr = 1;
    }

    pub fn scroll(&mut self) -> bool {
        self.cptr = 0;
        self.rptr += 2;
        self.rptr < self.rows.len()
    }

    pub fn anim(&mut self, screen: &mut Image) {
        for x in 0..22 {
            for y in 0..22 {
                if (WIPENUM - (22 * y + x)) < self.animc {
                    screen.draw_rect(
                        &Rect {
                            pos: Vec2i { x: 8 * x, y: 8 * y },
                            sz: Vec2i { x: 8, y: 8 }
                        }, 
                        (0,0,0,0)
                    );
                }
            }
        }
    }

    pub fn draw(&self, screen: &mut Image) {
        screen.clear((0,0,0,0));
        for x in 1..=20 {
            let xpx = (x * TILE_SZ as usize) as i32 + self.position.x;

            if TSPEED*(x - 1) <= self.cptr { // top row
                screen.bitblt(
                    &self.textset.image, 
                    self.textset.get_rect(self.rows.get(self.rptr-1).unwrap()[x-1]), 
                    Vec2i { x: xpx, y: 80 }
                );
            }

            if TSPEED*(x + 19) <= self.cptr { // btm row
                screen.bitblt(
                    &self.textset.image, 
                    self.textset.get_rect(self.rows.get(self.rptr).unwrap()[x-1]), 
                    Vec2i { x: xpx, y: 88 }
                );
            }
        }
    }
}