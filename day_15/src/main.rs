mod coordinate;
mod rectangle;
mod interval;
mod sensor_field;

use coordinate::Coordinate;
use sensor_field::SensorField;

use regex::{self, Regex};

use crate::interval::Interval;

fn parse_input() -> (Vec<Coordinate>, Vec<Coordinate>) {
    let input = include_str!("input.txt");
    let parse_regex = Regex::new(
        "Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )
    .unwrap();

    let mut sensors = vec![];
    let mut beacons = vec![];

    for line in input.lines() {
        let regex_result = parse_regex.captures(line).unwrap();
        let sensor_x: i64 = regex_result.get(1).unwrap().as_str().parse().unwrap();
        let sensor_y: i64 = regex_result.get(2).unwrap().as_str().parse().unwrap();
        let beacon_x: i64 = regex_result.get(3).unwrap().as_str().parse().unwrap();
        let beacon_y: i64 = regex_result.get(4).unwrap().as_str().parse().unwrap();

        sensors.push(Coordinate {
            x: sensor_x,
            y: sensor_y,
        });

        beacons.push(Coordinate {
            x: beacon_x,
            y: beacon_y,
        });
    }

    (sensors, beacons)
}

fn main() {
    let (sensors, beacons) = parse_input();

    let sensor_field = SensorField::new(sensors, beacons);

    //let taken_spaces = sensor_field.get_taken_spaces(10);
    let taken_spaces = sensor_field.get_taken_spaces(2000000);
    println!("The number of taken spaces is: {taken_spaces}");

    let search_interval: Interval = (0, 4000000).into();
    let open_spot = sensor_field.get_open_space(&search_interval).unwrap();

    let tuning_frequency = open_spot.x * 4000000 + open_spot.y;
    println!("The tuning frequency is: {tuning_frequency}");
}
