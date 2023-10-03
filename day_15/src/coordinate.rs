use std::{ops, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_iter = s.split(",");
        let x_str = split_iter.next().unwrap();
        let y_str = split_iter.next().unwrap();

        Ok(Coordinate {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        })
    }
}

impl ops::Sub for Coordinate {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Coordinate {
    pub fn length(&self) -> i64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt() as i64
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn distance_manhattan(first: &Coordinate, second: &Coordinate) -> i64 {
        (first.x - second.x).abs() + (first.y - second.y).abs()
    }
}
