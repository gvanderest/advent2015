use std::fs;

fn part1_solve(input: &str) -> u64 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let grid = lines
        .iter()
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let word_to_find = "XMAS";

    let row_count = grid.len() as i32;
    let col_count = grid.get(0).unwrap().len() as i32;

    let directions: Vec<(i32, i32)> = vec![
        // vertical
        (0, 1),  // go down
        (0, -1), // go up
        // horizontal
        (1, 0),  // go right
        (-1, 0), // go left
        // diagonals
        (1, 1),   // down right
        (-1, 1),  // down left
        (1, -1),  // up right
        (-1, -1), // up left
    ];

    let mut count = 0;

    for row_index in 0..row_count {
        for col_index in 0..col_count {
            let base_coords = (col_index, row_index);
            'direction: for direction in &directions {
                for letter_index in 0..(word_to_find.chars().count() as i32) {
                    let (x, y) = (
                        base_coords.0 + (direction.0 * letter_index),
                        base_coords.1 + (direction.1 * letter_index),
                    );

                    if x < 0 || y < 0 {
                        continue 'direction;
                    }

                    println!(
                        "({x}, {y}) .. max_x: {}, max_y: {}",
                        col_count - 1,
                        row_count - 1
                    );
                    let row = match grid.get(y as usize) {
                        Some(row) => row,
                        None => {
                            continue 'direction;
                        }
                    };

                    let letter = match row.get(x as usize) {
                        Some(letter) => *letter,
                        None => {
                            continue 'direction;
                        }
                    };

                    let letter_index = letter_index as usize;
                    if letter != &word_to_find[letter_index..letter_index + 1] {
                        continue 'direction;
                    }
                }
                count += 1;
            }
        }
    }

    count
}

fn part2_solve(input: &str) -> u64 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let grid = lines
        .iter()
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let word_to_find = "MAS";

    let row_count = grid.len() as i32;
    let col_count = grid.get(0).unwrap().len() as i32;

    let direction_sets: Vec<Vec<(i32, i32)>> = vec![
        vec![(-1, -1), (0, 0), (1, 1)],
        vec![(-1, 1), (0, 0), (1, -1)],
    ];

    let mut count = 0;

    for row_index in 0..row_count {
        'next_position: for col_index in 0..col_count {
            println!("--- START ---");

            let base_coords = (col_index, row_index);

            let mut found_words = vec![];

            for direction_set in &direction_sets {
                let mut found_word = String::new();

                for direction in direction_set {
                        let (x, y) = (base_coords.0 + direction.0, base_coords.1 + direction.1);

                        if x < 0 || y < 0 {
                            continue 'next_position;
                        }

                        let row = match grid.get(y as usize) {
                            Some(row) => row,
                            None => {
                                continue 'next_position;
                            }
                        };

                        let letter = match row.get(x as usize) {
                            Some(letter) => *letter,
                            None => {
                                continue 'next_position;
                            }
                        };
                        println!("({x}, {y}) -> {letter}");

                        found_word.push_str(letter);
                }
                println!("found_word: {}", found_word.clone());
                println!("--- FOUND ---");
                found_words.push(found_word);
            }

            for word in found_words {
                let forwards = word.as_str();

                let backwards = word.chars().rev().collect::<String>();
                let backwards = backwards.as_str();

                if forwards != word_to_find && backwards != word_to_find {
                    continue 'next_position;
                }
            }

            count += 1;
        }
    }

    count
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    println!("Part 2: {}", part2_solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("example.txt").unwrap();
        let input = input.trim();
        assert_eq!(18, part1_solve(&input));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example.txt").unwrap();
        let input = input.trim();
        assert_eq!(9, part2_solve(&input));
    }
}
