use crate::types::{DOWN, UP, LEFT, RIGHT, TILE_SZ};
use crate::types::{Image, Pos, Rect, Vec2i};

use std::fs;
use std::rc::Rc;

/// A graphical tile, we'll implement Copy since it's tiny
#[derive(Clone, Copy)]
pub struct Tile {
    pub solid: bool, // ... any extra data like collision flags or other properties
}

/// A set of tiles used in multiple Tilemaps
pub struct Tileset {
    pub tiles: Vec<Tile>,
    image: Rc<Image>,
}

/// Indices into a Tileset
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileID(usize);

/// Grab a tile with a given ID
impl std::ops::Index<TileID> for Tileset {
    type Output = Tile;
    fn index(&self, id: TileID) -> &Self::Output {
        &self.tiles[id.0]
    }
}
impl Tileset {
    /// Create a new tileset
    pub fn new(tiles: Vec<Tile>, texture: Rc<Image>) -> Self {
        Self {
            tiles,
            image: texture,
        }
    }
    /// Get the frame rect for a tile ID
    fn get_rect(&self, id: TileID) -> Rect {
        let idx = id.0;
        let (w, _h) = self.image.size();
        let tw = w as usize / TILE_SZ as usize;
        let row = idx / tw;
        let col = idx - (row * tw);
        Rect {
            pos: Vec2i {
                x: col as i32 * TILE_SZ,
                y: row as i32 * TILE_SZ,
            },
            sz: Vec2i {
                x: TILE_SZ,
                y: TILE_SZ,
            },
        }
    }
}

/// An actual tilemap
pub struct Tilemap {
    /// Where the tilemap is in space
    pub position: Vec2i,
    /// How big it is
    dims: (usize, usize),
    /// Which tileset is used for this tilemap
    tileset: Rc<Tileset>,
    /// A row-major grid of tile IDs in tileset
    map: Vec<TileID>,
    /// Scale factor
    sf: i32,
    /// Vector containing which tiles are solid
    movemap: Vec<bool>
}

impl Tilemap {
    pub fn new(
        position: Vec2i,
        dims: (usize, usize),
        tileset: Rc<Tileset>,
        map: Vec<usize>,
        sf: i32,
        moveables: Vec<usize>
    ) -> Self {
        assert_eq!(dims.0 * dims.1, map.len(), "Tilemap is the wrong size!");

        let movemap = Self::move_map(dims, &map, moveables, sf);

        Self {
            position,
            dims,
            tileset,
            map: map.into_iter().map(TileID).collect(),
            sf,
            movemap
        }
    }

    pub fn from_csv(
        position: Vec2i,
        dims: (usize, usize),
        tileset: Rc<Tileset>,
        path: &std::path::Path,
        sf: i32,
        moveables: Vec<usize>
    ) -> Self {
        let content = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(error) => panic!("Problem reading file: {:?}", error),
        };

        let map: Vec<usize> = content
            .split([',', '\n'])
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let movemap = Self::move_map(dims, &map, moveables, sf);

        assert_eq!(dims.0 * dims.1, map.len(), "Tilemap is the wrong size!");
        Self {
            position,
            dims,
            tileset,
            map: map.into_iter().map(TileID).collect(),
            sf,
            movemap
        }
    }

    pub fn contains_solid((w, _h): (usize, usize), map: &[usize], Rect { pos, sz }: Rect, moveables: Vec<usize>) -> bool {
        for row in pos.y..pos.y+sz.y {
            for col in pos.x..pos.x+sz.x {
                if !moveables.contains(&map[row as usize * w + col as usize]) {
                    return true;
                }
            }
        }
        false
    }

    pub fn move_map((w, h): (usize, usize), map: &[usize], moveables: Vec<usize>, sf: i32) -> Vec<bool> {
        let mut smap: Vec<bool> = Vec::with_capacity((w * h) / (sf*sf) as usize);

        for y in 0..h/sf as usize {
            for x in 0..w/sf as usize {
                let r = Rect {
                    pos: Vec2i { x: sf * x as i32, y: sf * y as i32},
                    sz: Vec2i { x: sf, y: sf }
                };
                smap.push(!Self::contains_solid((w,h), map, r, moveables.clone()));
            }
        }

        assert_eq!((w * h) / (sf*sf) as usize, smap.len(), "SolidMap is the wrong size!");
        smap
    }

    pub fn can_move(&self, pos: Pos, dir: usize) -> bool {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let next: usize = match dir {
            DOWN => x + (self.dims.0 / self.sf as usize) * (y + 1),
            UP => x + (self.dims.0 / self.sf as usize) * (y - 1),
            LEFT => (x - 1) + (self.dims.0 / self.sf as usize) * y,
            RIGHT => (x + 1) + (self.dims.0 / self.sf as usize) * y,
            _ => panic!("Invalid direction")
        };

        self.movemap[next]
    }

    pub fn tile_id_at(&self, Vec2i { x, y }: Vec2i) -> (Vec2i, TileID) {
        // Translate into map coordinates
        let x = (x - self.position.x) / TILE_SZ;
        let y = (y - self.position.y) / TILE_SZ;
        // return the tile corner and the tile ID
        (
            Vec2i {
                x: x * TILE_SZ + self.position.x,
                y: y * TILE_SZ + self.position.y,
            },
            self.map[y as usize * self.dims.0 + x as usize],
        )
    }

    pub fn size(&self) -> (usize, usize) {
        self.dims
    }

    pub fn tile_at(&self, posn: Vec2i) -> (Vec2i, Tile) {
        let (pos, tile_id) = self.tile_id_at(posn);
        (pos, self.tileset[tile_id])
    }

    pub fn translate(&mut self, delta: Vec2i) {
        self.position = self.position + delta;
    }

    pub fn translate_x(&mut self, delta: i32) {
        self.position.x += delta;
    }

    pub fn translate_y(&mut self, delta: i32) {
        self.position.y += delta;
    }

    pub fn draw(&self, screen: &mut Image) {
        for (y, row) in self.map.chunks_exact(self.dims.0).enumerate() {
            // We are in tile coordinates at this point so we'll need to translate back to pixel units and world coordinates to draw.
            let ypx = (y * TILE_SZ as usize) as i32 + self.position.y;
            // Here we can iterate through the column index and tiles in the row in parallel
            for (x, id) in row.iter().enumerate() {
                let xpx = (x * TILE_SZ as usize) as i32 + self.position.x;
                let frame = self.tileset.get_rect(*id);
                screen.bitblt(&self.tileset.image, frame, Vec2i { x: xpx, y: ypx });
            }
        }
    }
}