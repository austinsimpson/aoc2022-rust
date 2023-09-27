mod cave;
mod coordinate;

use std::str::FromStr;

use cave::Cave;

use crate::{cave::StoneLine, coordinate::Coordinate};

fn main() {
    let input = include_str!("input.txt");
    let mut cave = Cave::from_str(input).unwrap();

    let mut count = 0;
    while cave.place_new_sand().is_some() {
        count += 1;
    }
    println!("Grains placed: {count}");

    cave.reset();
    cave.add_floor();
    count = 0;

    while !cave.is_blocked() {
        cave.place_new_sand();
        count += 1;
    }

    println!("Grains placed: {count}");
}
