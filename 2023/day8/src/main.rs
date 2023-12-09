use std::{collections::HashMap, fs};

use num::{integer::lcm, FromPrimitive, Integer};
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

    let starting_islands: Vec<&str> = islands
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect();

    let answers_for_islands: Vec<u64> = starting_islands
        .iter()
        .map(|starting_island_name| {
            let mut steps = 0;
            let mut current_island = islands.get(starting_island_name).unwrap();
            for direction in directions.iter().cycle() {
                steps += 1;
                let next_island_name = match direction {
                    'L' => current_island.0,
                    _ => current_island.1,
                };
                if next_island_name.ends_with('Z') {
                    println!(
                        "Ending island found.. {} with {} steps",
                        next_island_name, steps
                    );
                    break;
                }
                current_island = islands.get(&next_island_name).unwrap();
            }
            steps
        })
        .collect();

    let mut steps = *answers_for_islands.first().unwrap();
    for x in 1..answers_for_islands.len() {
        // I had the idea of using a lowest common multiple, but wasn't sure about it--
        // then I peeked at someone else's answer after mine took too long and realized I was on the right track
        steps = lcm(steps, *answers_for_islands.get(x).unwrap());
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
