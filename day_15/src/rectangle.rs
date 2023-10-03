use crate::coordinate::Coordinate;

#[derive(Debug)]
pub struct Rectangle {
    top_left: Coordinate,
    bottom_right: Coordinate,
}

impl Rectangle {
    pub fn new(center: Coordinate, radius: i64) -> Self {
        let radius_vec = Coordinate {
            x: radius,
            y: radius,
        };

        let top_left = center - radius_vec;
        let bottom_right = center + radius_vec;

        Rectangle {
            top_left,
            bottom_right,
        }
    }

    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        self.top_left <= *coordinate && *coordinate <= self.bottom_right
    }

    pub fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        None
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.intersection(other).is_some()
    }
}
