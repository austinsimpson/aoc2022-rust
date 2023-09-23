use std::fs::File;
use std::io::prelude::Read;

use itertools::{self, Itertools};

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read content from input.txt");

    let lines: Vec<&str> = content.lines().collect();
    let groups = lines.split(|line| line.is_empty());
    let numbers = groups.map(|group| group.iter().map(|item| item.parse::<u32>().unwrap()).sum::<u32>()).sorted_by(|a, b| Ord::cmp(b, a)).collect::<Vec<_>>();
    let maximum = numbers.iter().max();

    match maximum {
        Some(value) => println!("The most calories is {}", value),
        None => println!("There is no elf carrying a maximum amount of calories")
    }

    let sum = numbers.iter().take(3).sum::<u32>();
    println!("The top 3 elves have {} calories", sum)
    
}
