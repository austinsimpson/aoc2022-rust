use std::{str::FromStr, collections::HashSet, hash::Hash};

#[derive(Clone, Copy, Debug)]
struct GenericParseError; 

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash,)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate { x: 0, y: 0 }
    }
}

#[derive(Clone, Copy)]
enum Move {
    X(i32),
    Y(i32)
}

impl FromStr for Move {
    type Err = GenericParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_iter = s.split(" ");
        let direction_char = split_iter.next().unwrap();
        let number =  i32::from_str(split_iter.next().unwrap()).unwrap();

        match direction_char {
            "L" => Ok(Move::X(-number)),
            "R" => Ok(Move::X(number)),
            "U" => Ok(Move::Y(number)),
            "D" => Ok(Move::Y(-number)),
            _ => Err(GenericParseError)
        }
    }
}

#[derive(Debug)]
struct Rope {
    coordinates: Vec<Coordinate>,
    visited_positions: HashSet<Coordinate>
}

impl Default for Rope {
    fn default() -> Self {
        Rope {
            coordinates: vec![Coordinate::default(); 2],
            visited_positions: HashSet::new()
        }
    }
}

impl Rope {
    fn new(len: usize) -> Self {
        Rope {
            coordinates: vec![Coordinate::default(); len],
            visited_positions: HashSet::new()
        }
    }

    fn step(&mut self, direction: Move) {
        match direction {
            Move::X(step) => {
                self.coordinates[0].x += step;
            },
            Move::Y(step) => {
                self.coordinates[0].y += step;
            }
        }

        for coord_indx in 1..self.coordinates.len() {
            let (x_diff, y_diff) = (self.coordinates[coord_indx - 1].x - self.coordinates[coord_indx].x, self.coordinates[coord_indx - 1].y - self.coordinates[coord_indx].y);
            if i32::abs(x_diff) > 1 || i32::abs(y_diff) > 1 {
                self.coordinates[coord_indx].x += i32::clamp(x_diff, -1, 1);
                self.coordinates[coord_indx].y += i32::clamp(y_diff, -1, 1);
            }
        }

        self.visited_positions.insert(self.coordinates.last().unwrap().clone());
    } 

    fn make_move(&mut self, m: Move) {
        match m {
            Move::X(total) => {
                for unused in 0..i32::abs(total) {
                    self.step(Move::X(1 * i32::signum(total)))
                }
            },
            Move::Y(total) => {
                for unused in 0..i32::abs(total) {
                    self.step(Move::Y(1 * i32::signum(total)))
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt").lines().map(|x| Move::from_str(x).unwrap()).collect::<Vec<_>>();

    println!("For head and tail");
    let mut short_rope = Rope::default();
    for rope_move in input.iter() {
        short_rope.make_move(rope_move.clone());

        println!("{:?} {:?}", short_rope.coordinates[0], short_rope.coordinates[1])
    }
    println!("Short rope visited {} distinct spaces.", short_rope.visited_positions.len());

    let mut long_rope = Rope::new(10);
    for rope_move in input.iter() {
        long_rope.make_move(rope_move.clone());
    }
    println!("Long rope visiited {} distinct spaces", long_rope.visited_positions.len())
}
