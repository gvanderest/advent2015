use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    // println!("Part 2: {}", part2_solve(&input));
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut lines = input.trim().split('\n');
    let directions = lines.next().unwrap().to_string().chars().collect();

    // Empty line
    lines.next().unwrap();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut islands: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        // Parse island name vs left/right
        // Format: "AAA = (BBB, CCC)"
        let caps = re.captures(line).unwrap();
        // TODO: Can we use a tuple deconstruction here?
        let name = caps.get(1).unwrap().as_str().to_string();
        let left = caps.get(2).unwrap().as_str().to_string();
        let right = caps.get(3).unwrap().as_str().to_string();

        islands.insert(name, (left, right));
    }

    (directions, islands)
}

fn part1_solve(input: &str) -> u64 {
    let (directions, islands) = parse_input(&input);

    println!("{:?}", islands);

    let mut steps = 0;
    let mut current_island = islands.get("AAA").unwrap();
    for direction in directions.iter().cycle() {
        steps += 1;
        let next_island_name = match direction {
            'L' => current_island.0.clone(),
            _ => current_island.1.clone(),
        };
        if next_island_name == "ZZZ" {
            break;
        }
        current_island = islands.get(&next_island_name).unwrap();
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
}
