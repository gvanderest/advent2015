use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    println!("Part 2: {}", part2_solve(&input));
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.trim().split('\n');
    let directions = lines.next().unwrap().to_string().chars().collect();

    // Empty line
    lines.next().unwrap();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut islands = HashMap::new();

    for line in lines {
        // Parse island name vs left/right
        // Format: "AAA = (BBB, CCC)"
        let caps = re.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();

        islands.insert(name, (left, right));
    }

    (directions, islands)
}

fn part1_solve(input: &str) -> u64 {
    let (directions, islands) = parse_input(input);

    println!("{:?}", islands);

    let mut steps = 0;
    let mut current_island = islands.get("AAA").unwrap();
    for direction in directions.iter().cycle() {
        steps += 1;
        let next_island_name = match direction {
            'L' => current_island.0,
            _ => current_island.1,
        };
        if next_island_name == "ZZZ" {
            break;
        }
        current_island = islands.get(&next_island_name).unwrap();
    }
    steps
}

fn part2_solve(input: &str) -> u64 {
    let (directions, islands) = parse_input(input);

    println!("{:?}", islands);

    let mut steps = 0;
    let mut current_islands: Vec<&str> = islands
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect();
    println!("STARTING ISLANDS: {:?}", current_islands);
    for direction in directions.iter().cycle() {
        steps += 1;

        // All step to next island
        current_islands = current_islands
            .iter()
            .map(|current_island_key| {
                let current_island = islands.get(current_island_key).unwrap();

                match direction {
                    'L' => current_island.0,
                    _ => current_island.1,
                }
            })
            .collect();

        // Check if we're done
        let all_ending_islands = current_islands
            .iter()
            .all(|island_name| island_name.ends_with('Z'));
        if all_ending_islands {
            break;
        }

        if steps % 10_000_000 == 0 {
            println!("{}..", steps);
        }
        // println!("NEXT ISLANDS: {:?}", current_islands);
    }
    steps
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_examples() {
        let example1 = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(2, part1_solve(&example1));

        let example2 = fs::read_to_string("example2.txt").unwrap();
        assert_eq!(6, part1_solve(&example2));
    }
    #[test]
    fn part2_examples() {
        let example1 = fs::read_to_string("example3.txt").unwrap();
        assert_eq!(6, part2_solve(&example1));
    }
}
