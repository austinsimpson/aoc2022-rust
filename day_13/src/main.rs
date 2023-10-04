use std::str::{FromStr, Chars};

mod packet;
use packet::{Packet, PacketIter, PacketIterResult};

fn print_packet<'a>(fields_iter: PacketIter<'a>) {
    
    for field in fields_iter {
        //print!("|*");
        match field {
            PacketIterResult::Number(n) => print!("{n},"),
            PacketIterResult::List(new_iter) => {
                print!("[");
                print_packet(new_iter);
                print!("],")
            }
        }
    } 
}

fn main() {
    let input = include_str!("input.txt").lines();
    let mut input_iter = input.into_iter();

    let mut result = 0;
    let mut index = 1;
    loop {
        let left = Packet::new(input_iter.next().unwrap());
        let right = Packet::new(input_iter.next().unwrap());

        let ordering = left.partial_cmp(&right);
        if ordering == Some(std::cmp::Ordering::Less) || ordering == Some(std::cmp::Ordering::Equal) {
            result += index;
        }
        index += 1;

        let possible_new_line = input_iter.next();
        if possible_new_line.is_none() {
            break;
        }
    }
    println!("The sum of the valid pair indices is: {}", result);

    let mut lines_for_second_part: Vec<_> = include_str!("input.txt").lines().filter(|x| !x.is_empty()).collect();
    let start_marker = "[[2]]";
    let end_marker = "[[6]]";
    lines_for_second_part.push(start_marker);
    lines_for_second_part.push(end_marker);

    lines_for_second_part.sort_by(|a, b| {
        let left = Packet::new(*a);
        let right = Packet::new(*b);

        left.partial_cmp(&right).unwrap()
    });

    let start_index = lines_for_second_part.iter().position((|x| *x == start_marker)).unwrap() + 1;
    let end_index = lines_for_second_part.iter().position((|x| *x == end_marker)).unwrap() + 1;

    println!("Decryption Key: {}", start_index * end_index)   
}

