use std::fs;
#[derive(Debug, Copy, Clone)]
pub enum Move {
    Rock(i32),
    Paper(i32),
    Scissors(i32),
}
#[derive(Debug)]
pub struct Round(Move, Move);

pub fn judge(round: &Round) -> (i32, i32) {
    match round {
        Round(Move::Rock(x), Move::Rock(y)) => (x + 3, y + 3),
        Round(Move::Rock(x), Move::Paper(y)) => (*x, y + 6),
        Round(Move::Rock(x), Move::Scissors(y)) => (x + 6, *y),

        Round(Move::Paper(x), Move::Paper(y)) => (x + 3, y + 3),
        Round(Move::Paper(x), Move::Rock(y)) => (x + 6, *y),
        Round(Move::Paper(x), Move::Scissors(y)) => (*x, y + 6),

        Round(Move::Scissors(x), Move::Scissors(y)) => (x + 3, y + 3),
        Round(Move::Scissors(x), Move::Rock(y)) => (*x, y + 6),
        Round(Move::Scissors(x), Move::Paper(y)) => (x + 6, *y),
    }
}

pub fn to_move(a: &str) -> Move {
    match a {
        "A" | "X" => Move::Rock(1),
        "B" | "Y" => Move::Paper(2),
        "C" | "Z" => Move::Scissors(3),
        &_ => Move::Scissors(0),
    }
}

// Y = Draw
// X = Lose
// Z = Win

pub fn decide_move((o_move, instruction): (Move, &&str)) -> Round {
    match (o_move, instruction) {
        (Move::Rock(_), &"Y") => Round(o_move, Move::Rock(1)),
        (Move::Rock(_), &"X") => Round(o_move, Move::Scissors(3)),
        (Move::Rock(_), &"Z") => Round(o_move, Move::Paper(2)),

        (Move::Paper(_), &"Y") => Round(o_move, Move::Paper(2)),
        (Move::Paper(_), &"X") => Round(o_move, Move::Rock(1)),
        (Move::Paper(_), &"Z") => Round(o_move, Move::Scissors(3)),

        (Move::Scissors(_), &"Y") => Round(o_move, Move::Scissors(3)),
        (Move::Scissors(_), &"X") => Round(o_move, Move::Paper(2)),
        (Move::Scissors(_), &"Z") => Round(o_move, Move::Rock(1)),
        _ => Round(o_move, o_move),
    }
}

pub fn to_round(moves_vec: Vec<&str>) -> Round {
    let round = Round(
        to_move(moves_vec.get(0).unwrap()),
        to_move(moves_vec.get(1).unwrap()),
    );

    round
}

pub fn make() {
    let raw_input = fs::read_to_string("input/day-02.txt").expect("Failed to read input");
    let parsed_input = raw_input.split("\n").map(|x: &str| x.split(" ").collect());

    let rounds: Vec<Round> = parsed_input.map(to_round).collect();

    println!("Day 2 results");
    let part_1_result: i32 = rounds.iter().map(judge).map(|x| x.1).sum::<i32>();
    println!("pt 1: {:?}", part_1_result);
}

pub fn make_2() {
    let raw_input = fs::read_to_string("input/day-02.txt").expect("Failed to read input");
    let parsed_input: Vec<Vec<&str>> = raw_input
        .split("\n")
        .map(|x: &str| x.split(" ").collect())
        .collect();

    let rounds: Vec<Round> = parsed_input
        .iter()
        .map(|x| (to_move(x.get(0).unwrap()), x.get(1).unwrap()))
        .map(decide_move)
        .collect();

    let part_1_result: i32 = rounds.iter().map(judge).map(|x| x.1).sum::<i32>();
    println!("pt 2: {:?}", part_1_result);
}
