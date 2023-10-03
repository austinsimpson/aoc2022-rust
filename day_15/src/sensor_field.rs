use crate::coordinate::Coordinate;
use crate::kd_tree::KDTreeNode;
use crate::rectangle::Rectangle;

pub struct SensorField {
    tree: KDTreeNode,
}

impl SensorField {
    pub fn new(sensors: Vec<Coordinate>, beacons: Vec<Coordinate>) -> Self {
        let result = SensorField {
            tree: Self::build_spatial_index(sensors, beacons),
        };

        result
    }

    fn build_spatial_index(sensors: Vec<Coordinate>, beacons: Vec<Coordinate>) -> KDTreeNode {
        let mut tree = KDTreeNode::new();

        for (sensor, beacon) in sensors.into_iter().zip(beacons.into_iter()) {
            let dist = Coordinate::distance_manhattan(&sensor, &beacon);
            let bounds = Rectangle::new(sensor.clone(), dist);
            tree.insert(sensor, bounds);
        }

        tree
    }
}
