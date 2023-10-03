mod coordinate;
mod kd_tree;
mod rectangle;
mod sensor_field;

use coordinate::Coordinate;
use sensor_field::SensorField;

use regex::{self, Regex};

fn parse_input() -> (Vec<Coordinate>, Vec<Coordinate>) {
    let input = include_str!("sample_input.txt");
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
}
