use std::{collections::HashMap, fs};

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

fn part1_score(outcome: &Outcome, my_move: &Move) -> i32 {
    // The score for a single round is the score for ..
    // 1. the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus
    (match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
        // 2. the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
    } + match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    })
}

fn part1_compute_turn_outcome(opponent_move: &Move, my_move: &Move) -> Outcome {
    // Wins
    if matches!(my_move, Move::Scissors) && matches!(opponent_move, Move::Paper) {
        return Outcome::Win;
    } else if matches!(my_move, Move::Paper) && matches!(opponent_move, Move::Rock) {
        return Outcome::Win;
    } else if matches!(my_move, Move::Rock) && matches!(opponent_move, Move::Scissors) {
        return Outcome::Win;

    // Draws
    } else if matches!(my_move, Move::Rock) && matches!(opponent_move, Move::Rock) {
        return Outcome::Draw;
    } else if matches!(my_move, Move::Paper) && matches!(opponent_move, Move::Paper) {
        return Outcome::Draw;
    } else if matches!(my_move, Move::Scissors) && matches!(opponent_move, Move::Scissors) {
        return Outcome::Draw;
    }

    Outcome::Loss
}

fn part1_process(input: &String) -> i32 {
    let letter_to_move = HashMap::from([
        ("A", Move::Rock),
        ("B", Move::Paper),
        ("C", Move::Scissors),
        ("X", Move::Rock),
        ("Y", Move::Paper),
        ("Z", Move::Scissors),
    ]);

    // Split by lines to make "turns"
    let turns = input.split("\n");

    let mut score = 0;

    for turn in turns {
        // Split each line by space to make my/opponent moves
        let split_turn: Vec<&str> = turn.split(" ").collect();
        let opponent_move = letter_to_move.get(split_turn[0]).unwrap();
        let my_move = letter_to_move.get(split_turn[1]).unwrap();

        // Compute outcome
        let outcome = part1_compute_turn_outcome(&opponent_move, &my_move);

        // Based on outcome and move used, calculate score
        score = score + part1_score(&outcome, &my_move);
    }

    score
}

fn part2_determine_outcome_move(outcome: &Outcome, opponent_move: &Move) -> Move {
    match outcome {
        Outcome::Win => {
            return match opponent_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            }
        }
        Outcome::Loss => match opponent_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        Outcome::Draw => match opponent_move {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissors => Move::Scissors,
        },
    }
}

fn part2_process(input: &String) -> i32 {
    let letter_to_move =
        HashMap::from([("A", Move::Rock), ("B", Move::Paper), ("C", Move::Scissors)]);
    let letter_to_outcome = HashMap::from([
        ("X", Outcome::Loss),
        ("Y", Outcome::Draw),
        ("Z", Outcome::Win),
    ]);

    // Split by lines to make "turns"
    let turns = input.split("\n");

    let mut score = 0;

    for turn in turns {
        // Split each line by space to make my/opponent moves
        let split_turn: Vec<&str> = turn.split(" ").collect();
        let opponent_move = letter_to_move.get(split_turn[0]).unwrap();
        let my_outcome = letter_to_outcome.get(split_turn[1]).unwrap();
        let my_move = part2_determine_outcome_move(&my_outcome, &opponent_move);

        // Compute outcome
        let outcome = part1_compute_turn_outcome(&opponent_move, &my_move);

        // Based on outcome and move used, calculate score
        score = score + part1_score(&outcome, &my_move);
    }

    score
}

fn main() {
    let input = String::from(
        fs::read_to_string("input.txt")
            .expect("example.txt to exist")
            .trim(),
    );
    println!("Part 1: {}", part1_process(&input));
    println!("Part 2: {}", part2_process(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_work() {
        let input = String::from(
            fs::read_to_string("example.txt")
                .expect("example.txt to exist")
                .trim(),
        );
        assert_eq!(part1_process(&input), 15);
        assert_eq!(part2_process(&input), 12);
    }
}
