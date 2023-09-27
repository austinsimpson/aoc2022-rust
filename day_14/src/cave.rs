use std::{collections::HashMap, str::FromStr, thread::current};

use crate::coordinate::Coordinate;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TileType {
    Stone,
    Sand,
}

#[derive(Debug)]
pub struct StoneLine {
    coords: Vec<Coordinate>,
}

impl FromStr for StoneLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords_iter = s.split("->").map(|s| s.trim());

        Ok(Self {
            coords: coords_iter
                .map(|coord| Coordinate::from_str(coord).unwrap())
                .collect(),
        })
    }
}

pub struct Cave {
    tiles: HashMap<Coordinate, TileType>,
    source: Coordinate,
    max_height: i32,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave = Cave::new(Coordinate { x: 500, y: 0 });

        let lines = s.lines();
        let stone_lines = lines.map(|l| StoneLine::from_str(l).unwrap());

        for stone_line in stone_lines {
            cave.place_stone_line(&stone_line);
        }

        Ok(cave)
    }
}

impl Cave {
    fn new(source: Coordinate) -> Self {
        Cave {
            tiles: HashMap::new(),
            source,
            max_height: 0,
        }
    }

    fn place_stone_line(&mut self, stone_line: &StoneLine) {
        for window in stone_line.coords.windows(2) {
            if let [start, end] = window {
                let direction = (*end - *start).normalized();
                let mut current_pos = start.clone();
                loop {
                    self.tiles.insert(current_pos, TileType::Stone);
                    self.update_bounds(&current_pos);
                    if current_pos == *end {
                        break;
                    }
                    current_pos += direction;
                }
            }
        }
    }

    fn update_bounds(&mut self, coord: &Coordinate) {
        if coord.y > self.max_height {
            self.max_height = coord.y;
        }
    }

    pub fn place_new_sand(&mut self) -> Option<Coordinate> {
        let mut coord = self.source.clone();
        let gravity = Coordinate { x: 0, y: 1 };
        loop {
            let below = coord + gravity; //Higher number is lower
            if below.y > self.max_height {
                return None;
            }

            if self.tiles.contains_key(&below) {
                let left = below - Coordinate { x: 1, y: 0 };
                if !self.tiles.contains_key(&left) {
                    coord = coord - Coordinate { x: 1, y: 0 };
                    continue;
                }

                let right = below + Coordinate { x: 1, y: 0 };
                if !self.tiles.contains_key(&right) {
                    coord = coord + Coordinate { x: 1, y: 0 };
                    continue;
                }

                self.tiles.insert(coord, TileType::Sand);
                return Some(coord);
            } else {
                coord += gravity
            }
        }
    }

    pub fn add_floor(&mut self) {
        let half_width = 2 * self.max_height + 1;
        let start_x = self.source.x - half_width;
        let end_x = self.source.x + half_width;

        let y = self.max_height + 2;
        for x in start_x..end_x {
            self.tiles.insert(Coordinate { x, y }, TileType::Stone);
        }

        self.max_height = self.max_height + 3
    }

    pub fn is_blocked(&self) -> bool {
        self.tiles.contains_key(&self.source)
    }

    pub fn reset(&mut self) {
        self.tiles.retain(|_, v| *v == TileType::Stone);
    }
}
