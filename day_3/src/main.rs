use std::{str::FromStr, fmt};

fn main() {
    let rucksacks = include_str!("input.txt")
        .lines()
        .map(|line| Rucksack::from_str(line))
        .filter_map(|res| match res {
            Ok(rucksack) => Some(rucksack),
            Err(_) => None
        })
        .collect::<Vec<_>>();

    let sum = rucksacks
        .to_vec()
        .into_iter()
        .map(|r| r.find_common() as u64)
        .sum::<u64>();

    println!("Part 1: {}", sum);

    let badge_sum = rucksacks.chunks(3)
        .map(|r| Rucksack::find_badge(r.to_vec()) as u64)
        .sum::<u64>();

    println!("Part 2: {}", badge_sum);

}

#[derive(Debug, Clone)]
struct Rucksack {
    compartments: [u64; 2]
}

#[derive(Debug, Clone)]
struct RucksackError;

impl Rucksack {
    fn get_priority(c: char) -> u8 {
        let offset: u8 = if c.is_uppercase() { 26 } else { 0 };
        let lowercase_priority = c.to_ascii_lowercase() as u8;
        let ascii_a_digit = 'a' as u8;
        lowercase_priority + 1 + offset - ascii_a_digit
    }

    fn find_badge(rucksacks: Vec<Rucksack>) -> u8 {
        let common = rucksacks.iter()
            .map(|r| r.compartments.into_iter().fold(0u64, |a, b| a | b))
            .fold(0xFFFFFFFFFFFFFFFF, |a, b| a & b);

        Rucksack::find_set_index(common)
    }

    fn find_common(&self) -> u8 {
        let common = self.compartments[0] & self.compartments[1];

        Rucksack::find_set_index(common)
    }

    fn find_set_index(input: u64) -> u8 {
        let mut check: u8 = 1;
        while (1 << check) != input {
            check = check + 1;
        }
        check
    }

    fn parse_compartment(compartment_string: String) -> u64 {
        let mut compartment: u64 = 0;
        for c in compartment_string.chars() {
            compartment |= 1 << Rucksack::get_priority(c)
        }
        compartment
    }
}

impl FromStr for Rucksack {
    type Err = RucksackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.len();
        if length % 2 != 0 {
            RucksackError;
        }

        Ok(Rucksack {
            compartments: [Rucksack::parse_compartment(s[0..length/2].to_string()), Rucksack::parse_compartment(s[(length/2)..length].to_string())]  
        })
    }
}