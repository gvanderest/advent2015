use std::{fs, collections::HashMap};

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
enum Direction {
    Increasing,
    Decreasing,
    Unchanged,
}

fn part1_solve(input: &str) -> u64 {
    let mut count = 0;

    let lines = input.split("\n");
    'line: for line in lines.into_iter() {
        println!("Line: {line}");

        let levels = line.split_whitespace().map(|val| {
            str::parse::<i32>(val).unwrap()
        }).collect::<Vec<i32>>();

        let mut overall_direction: Option<Direction> = None;

        for index in 1..levels.len() {
            let previous = levels.get(index-1).unwrap();
            let current = levels.get(index).unwrap();

            // TODO: Could use a cmp or similar instead of making my own enum
            let direction = if current > previous {
                Direction::Increasing
            } else if current < previous {
                Direction::Decreasing
            } else {
                // Rule 1: The levels are either all increasing or all decreasing.
                println!("Skipped because values didn't continue increasing/decreasing, previous={previous}, current={current}");
                continue 'line;
            };


            // Rule 1: The levels are either all increasing or all decreasing.
            if index == 1 {
                overall_direction = Some(direction);
            } else if overall_direction.clone().unwrap() != direction {
                println!("Skipped because values didn't continue increasing/decreasing, overall_direction={overall_direction:?}, direction={direction:?}, previous={previous}, current={current}");
                continue 'line;
            }

            // Rule 2: Any two adjacent levels differ by at least one and at most three.
            let diff = i32::abs(current - previous);

            if diff < 1 || diff > 3 {
                println!("Skipped because values changed by too much, previous={previous}, current={current}");
                continue 'line;
            }
        }

        println!("YAY");
        count += 1;
    }

    count
}

fn part2_solve(input: &str) -> u64 {
    let mut count = 0;

    let lines = input.split("\n");
    'line: for line in lines {
        let levels = line.split_whitespace().collect::<Vec<&str>>();

        for pos in 0..levels.len() {
            // TODO: For each list, iterate through and remove one item.. then use part1 solve to see if it works
            let mut filtered_levels = levels.clone();
            filtered_levels.remove(pos);
            let filtered_levels_string = filtered_levels.join(" ");
            if part1_solve(filtered_levels_string.as_str()) == 1 {
                count += 1;
                continue 'line;
            }
        }
    }

    count
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input.trim_end()));
    println!("Part 2: {}", part2_solve(&input.trim_end()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(2, part1_solve(&input.trim_end()));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(4, part2_solve(&input.trim_end()));
    }
}
