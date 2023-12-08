use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    println!("Part 2: {}", part2_solve(&input));
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

fn part2_solve(input: &str) -> u64 {
    let (directions, islands) = parse_input(&input);

    println!("{:?}", islands);

    let mut steps = 0;
    let mut current_islands: Vec<String> = islands
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.clone())
        .collect();
    println!("STARTING ISLANDS: {:?}", current_islands);
    for direction in directions.iter().cycle() {
        steps += 1;
        let mut next_islands: Vec<String> = Vec::new();
        for current_island_key in &current_islands {
            let current_island = islands.get(current_island_key).unwrap();

            let next_island_name = match direction {
                'L' => current_island.0.clone(),
                _ => current_island.1.clone(),
            };

            next_islands.push(next_island_name);
        }

        let non_ending_islands = next_islands
            .iter()
            .filter(|k| !k.ends_with('Z'))
            .collect::<Vec<&String>>();
        let non_ending_island_count = non_ending_islands.len();
        // println!(
        //     "Ghosts on non-Z islands: {} .. {:?}",
        //     non_ending_island_count, non_ending_islands
        // );

        if non_ending_island_count == 0 {
            break;
        }
        current_islands = next_islands;

        if steps % 1_000_000 == 0 {
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
