use crate::{coordinate::Coordinate, rectangle::Rectangle};

pub enum KDTreeNodeType {
    Leaf(Coordinate),
    Edge(Vec<KDTreeNode>),
}

pub struct KDTreeNode {
    node_type: KDTreeNodeType,
    bounds: Option<Rectangle>,
}

impl KDTreeNode {
    pub fn new() -> Self {
        KDTreeNode {
            node_type: KDTreeNodeType::Edge(vec![]),
            bounds: None,
        }
    }

    pub fn insert(&mut self, data: Coordinate, bounds: Rectangle) {
        assert!(bounds.contains(&data));

        match self.node_type {
            KDTreeNodeType::Edge(ref mut children) => {
                for child in children {
                    if let Some(ref child_bounds) = child.bounds {
                        if child_bounds.insersects() {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn query(&mut self, coord: &Coordinate) -> Option<Coordinate> {
        None
    }
}
