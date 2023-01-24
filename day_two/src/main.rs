fn main() {
    let lines = include_str!("input.txt").lines();
    let rounds = lines.map(|line| parse_round(line));
    let scores = rounds.map(|round| score_round(round));
    let score: u32 = scores.sum();
    println!("Score is {}", score);
}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Tie,
    Loss
}

fn score_round(round: (Move, Move)) -> u32 {
    let move_score = match round.1 {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    };

    let outcome = get_round_outcome(round);
    let outcome_score = match outcome {
        Outcome::Loss => 0,
        Outcome::Tie => 3,
        Outcome::Win => 6
    };
    move_score + outcome_score
}

fn get_round_outcome(round: (Move, Move)) -> Outcome { 
    match round {
        (Move::Rock, Move::Paper) => Outcome::Win,
        (Move::Rock, Move::Scissors) => Outcome::Loss,
        (Move::Paper, Move::Rock) => Outcome::Loss,
        (Move::Paper, Move::Scissors) => Outcome::Win,
        (Move::Scissors, Move::Rock) => Outcome::Win,
        (Move::Scissors, Move::Paper) => Outcome::Loss,
        _ => Outcome::Tie
    }
}

fn parse_round(input: &str) -> (Move, Move) {
    let mut move_iter = input.splitn(2, " ");
    let opponent_move = parse_opponent_move(move_iter.next().expect("Failed to parse opponent move.")).unwrap();

    let outcome = parse_outcome(move_iter.next().expect("Failed to parse my move.")).unwrap();
    let my_move = parse_my_move(&opponent_move, &outcome);
    (opponent_move, my_move)
}

fn parse_opponent_move(input: &str) -> Option<Move> {
    match input.to_lowercase().as_str() {
        "a" => Some(Move::Rock),
        "b" => Some(Move::Paper),
        "c" => Some(Move::Scissors),
        _ => None
    }
}

fn parse_outcome(input: &str) -> Option<Outcome> {
    match input.to_lowercase().as_str() {
        "x" => Some(Outcome::Loss),
        "y" => Some(Outcome::Tie),
        "z" => Some(Outcome::Win),
        _ => None
    }
}

fn parse_my_move(opponent_move: &Move, outcome: &Outcome) -> Move {
    match (opponent_move, outcome) {
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Paper, Outcome::Loss) => Move::Rock,
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Loss) => Move::Scissors,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (Move::Scissors, Outcome::Loss) => Move::Paper,
        _ => opponent_move.clone()          
    }
}

