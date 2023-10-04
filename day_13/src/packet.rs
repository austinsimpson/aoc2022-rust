use std::{str::{Chars, FromStr}, iter::Enumerate, rc::Rc, cell::RefCell};

pub struct Packet<'a> {
    underlying: &'a str
}

impl<'a> Packet<'a> {
    pub fn new(str: &'a str) -> Packet<'a> {
        Packet {
            underlying: str
        }
    }
    
    pub fn iter(&self) -> PacketIter {
        PacketIter::new(self)
    }
}

//Added this to allow for PartialOrd without error
impl<'a> PartialEq for Packet<'a> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl<'a> PartialOrd for Packet<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left_iter = self.iter();
        let right_iter = other.iter();

        Some(PacketIter::consuming_compare(left_iter, right_iter))
    }
}

pub struct PacketIter<'a> {
    chars: Rc<RefCell<Chars<'a>>>,
    at_end: bool
}

pub enum PacketIterResult<'a> {
    Number(u32),
    List(PacketIter<'a>)
}

impl<'a> PacketIterResult<'a> {
    fn compare(first: PacketIterResult<'a>, second: PacketIterResult<'a>) -> std::cmp::Ordering {
        match (first, second) {
            (PacketIterResult::Number(n), PacketIterResult::Number(m)) => {
                return n.cmp(&m)
            },
            (PacketIterResult::Number(n), PacketIterResult::List(mut m)) => {
                let next = m.next();
                match next {
                    None => std::cmp::Ordering::Greater,
                    Some(r) => {
                        let compare_result = Self::compare(PacketIterResult::Number(n), r);
                        let list_has_more = m.next().is_some();

                        if compare_result == std::cmp::Ordering::Equal && list_has_more {
                            return std::cmp::Ordering::Less
                        }
                        else {
                            return compare_result
                        }
                    }
                }
            },
            (PacketIterResult::List(mut n), PacketIterResult::Number(m)) => {
                let next = n.next();
                match next {
                    None => std::cmp::Ordering::Less,
                    Some(r) => {
                        let compare_result = Self::compare(r, PacketIterResult::Number(m));
                        let list_has_more = n.next().is_some();

                        if compare_result == std::cmp::Ordering::Equal && list_has_more {
                            return std::cmp::Ordering::Greater
                        } else {
                            return compare_result;
                        }
                    }
                }
            },
            (PacketIterResult::List(n), PacketIterResult::List(m)) => PacketIter::consuming_compare(n, m)
        }
    }
}

impl<'a> PacketIter<'a> {
    fn new(packet: &'a Packet) -> PacketIter<'a> {
        PacketIter { chars: Rc::new(RefCell::from(packet.underlying.chars())), at_end: false }
    }

    fn consuming_compare(mut first: PacketIter<'a>, mut second: PacketIter<'a>) -> std::cmp::Ordering {
        loop {
            let first_val = first.next();
            let second_val = second.next();
            match (first_val, second_val) {
                (None, None) => return std::cmp::Ordering::Equal,
                (None, Some(_)) => return std::cmp::Ordering::Less,
                (Some(_), None) => return std::cmp::Ordering::Greater,
                (Some(first_field), Some(second_field)) => {
                    let field_compare = PacketIterResult::compare(first_field, second_field);
                    if field_compare == std::cmp::Ordering::Equal {
                        continue;
                    }
                    return field_compare
                }
            }
        }
    }
}



impl<'a> Iterator for PacketIter<'a> {
    type Item = PacketIterResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.chars.borrow_mut();
        let mut current_num_chars: String = String::new();

        if self.at_end {
            return None;
        }

        loop {
            let next_char = chars.next()?;
            match next_char {
                '[' => {
                    return Some(PacketIterResult::List(PacketIter{ chars: self.chars.clone(), at_end: false }))
                },
                ']' => {
                    self.at_end = true;
                    break;
                },
                ',' => {
                    if current_num_chars.len() == 0 {
                        continue;   
                    }
                    else {
                        break;
                    }
                },
                _ => {
                    current_num_chars.push(next_char)
                }
            }
        }

        if current_num_chars.len() == 0 {
            None
        } else {
            Some(PacketIterResult::Number(current_num_chars.parse().unwrap()))
        }
    }
}
