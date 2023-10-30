use crate::shape::SHAPES;

mod board;
mod shape;

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(()),
        }
    }
}

fn parse_jet_streams(input: &str) -> Vec<Move> {
    input.chars().map(|c| Move::try_from(c).unwrap()).collect()
}

fn main() {
    let input = include_str!("test_input.txt");
    let jets = parse_jet_streams(input);

    let mut board = Board::new(jets);
    for _ in 0..2022 {
        board.perform_next_move()
    }

    println!("The height after the 2022'nd rock is: {}", board.height())
}
