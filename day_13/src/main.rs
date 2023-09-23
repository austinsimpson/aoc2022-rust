use std::str::{FromStr, Chars};

struct Packet<'a> {
    underlying: &'a str
}

impl<'a> Packet<'a> {
    fn new(str: &'a str) -> Packet<'a> {
        Packet {
            underlying: str
        }
    }
    
    fn iter(&self) -> PacketIter {
        PacketIter {
            packet: &self,
            offset: 0
        }
    }
}

impl<'a> Ord for Packet<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {

    }
}

impl<'a> PartialOrd for Packet<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Packet<'a> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl<'a> Eq for Packet<'a> {}

enum PacketField {
    Number(u32),
    List(usize)
}

struct PacketIter<'a> {
    packet: &'a Packet<'a>,
    offset: usize
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = PacketField;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.offset;
        let mut length = 0;
        loop {

            length += 1;
            self.offset += 1;
        }

        match length {
            0 => None,
            i => {

            }
        }
    }
}

fn main() {
    let input = include_str!("sample_input.txt").lines();
    let mut input_iter = input.into_iter();

    let valid_indices = vec![0; 0];
    let mut current_index = 0;
    loop {
        let left = Packet::new(input_iter.next().unwrap());
        let right = Packet::new(input_iter.next().unwrap());



        let possible_new_line = input_iter.next();
        if possible_new_line.is_none() {
            break;
        }
        current_index += 1;
    }
}

