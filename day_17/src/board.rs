use crate::{shape::SHAPES, Move};

pub struct Board {
    heights: Vec<i32>,
    jets: Vec<Move>,
    current_jet: usize,
    current_shape: usize,
}

const BOARD_WIDTH: usize = 7;
impl Board {
    fn new(jets: Vec<Move>) -> Self {
        Self {
            heights: vec![0; BOARD_WIDTH],
            jets,
            current_jet: 0,
            current_shape: 0,
        }
    }

    fn height(&self) -> i32 {
        *self.heights.iter().max().unwrap()
    }

    fn perform_next_move(&mut self) {
        let shape = &SHAPES[self.current_shape];
        let starting_height = self.height() + 3;

        println!("{}, {}", self.current_jet, self.current_shape);
        self.current_jet = (self.current_jet + 1) % self.jets.len();
        self.current_shape = (self.current_shape + 1) % SHAPES.len();
    }
}
