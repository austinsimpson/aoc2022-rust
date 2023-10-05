use std::cmp::Ordering;

use crate::coordinate::Coordinate;
use crate::rectangle::Rectangle;
use crate::interval::Interval;

pub struct SensorField {
    sensors: Vec<Coordinate>,
    beacons: Vec<Coordinate>
}


impl SensorField {
    pub fn new(sensors: Vec<Coordinate>, beacons: Vec<Coordinate>) -> Self {
        let result = SensorField {
            sensors,
            beacons
        };

        result
    }

    pub fn get_intervals_for_y(&self, y: i64) -> Vec<Interval> {
        let sensors_and_beacons_iter = self.sensors.iter().zip(self.beacons.iter());
        let pairs_in_range = sensors_and_beacons_iter.filter(|(s, b)| {
            let distance = Coordinate::distance_manhattan(*s, *b);
            let lower = s.y - distance;
            let upper = s.y + distance;

            lower <= y && y <= upper
        });

        let mut intervals: Vec<Interval> = pairs_in_range.map(|(s, b)| {
            let distance = Coordinate::distance_manhattan(s, b);
            let x_width = distance - match s.y.cmp(&y) {
                Ordering::Less => y - s.y,
                Ordering::Greater => s.y - y,
                Ordering::Equal => 0 
            };

            (s.x - x_width, s.x + x_width).into()
        }).collect::<Vec<_>>();
        intervals.sort();

        intervals
    }

    pub fn get_taken_spaces(&self, y: i64) -> usize {
        let intervals = self.get_intervals_for_y(y);
        let merged_intervals = Interval::merge(intervals);
        merged_intervals.iter().map(|i| i.width()).sum() 
    }

    pub fn get_open_space(&self, y_search: &Interval) -> Option<Coordinate> {
        for y in y_search.start..y_search.end {
            let intervals = Interval::merge(self.get_intervals_for_y(y));
            if intervals.len() > 1 {
                let x = intervals.first().unwrap().end + 1;
                return Some(Coordinate { x, y })
            }
        }

        None
    }
}
