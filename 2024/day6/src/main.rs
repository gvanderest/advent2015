use std::{fs, collections::HashSet};

struct Part1Solver<'a> {
    input: &'a str,
}

fn part1_solve(input: &str) -> u64 {
    let UP_MODIFIER = (0, -1);
    let DOWN_MODIFIER = (0, 1);
    let LEFT_MODIFIER = (-1, 0);
    let RIGHT_MODIFIER = (1, 0);

    #[derive(Debug, Clone)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let mut guard_direction = Direction::Up;
    let mut guard_coords: (i32, i32) = (0, 0);
    let mut coords_modifier: (i32, i32) = UP_MODIFIER;

    let mut visited: HashSet<(i32, i32)> = HashSet::from([guard_coords]);
    println!("{}", input);
    let map: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    'map_done: for (y, row) in map.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            if symbol == &'^' {
                guard_coords = (x as i32, y as i32);
                break 'map_done;
            }
        }
    }

    let max_y = map.len() as i32 - 1;
    let max_x = map.get(0).unwrap().len() as i32 - 1;

    let coords_are_in_map =
        |(x, y): (i32, i32)| -> bool { x >= 0 && y >= 0 && x <= max_x && y <= max_y };

    loop {
        println!("Current Coords: {guard_coords:?}");
        let next_coords = (
            guard_coords.0 + coords_modifier.0,
            guard_coords.1 + coords_modifier.1,
        );
        println!("Next Coords: {next_coords:?}");

        // Going off map, end
        if !coords_are_in_map(next_coords) {
            break;
        }
        println!("max_x={max_x}, max_y={max_y}");

        let next_symbol = map
            .get(next_coords.1 as usize)
            .unwrap()
            .get(next_coords.0 as usize)
            .unwrap();

        println!("Next symbol: {}", next_symbol);

        if next_symbol == &'#' {
            let old_direction = guard_direction.clone();
            (guard_direction, coords_modifier) = match guard_direction {
                Direction::Up => (Direction::Right, RIGHT_MODIFIER),
                Direction::Right => (Direction::Down, DOWN_MODIFIER),
                Direction::Down => (Direction::Left, LEFT_MODIFIER),
                Direction::Left => (Direction::Up, UP_MODIFIER),
            };
            println!("TURNING FROM {:?} TO {:?}", old_direction, guard_direction);
            continue;
        }

        // Advance and track position
        visited.insert(guard_coords);
        guard_coords = next_coords;
    }

    visited.len() as u64
}

fn part2_solve(input: &str) -> u64 {
    0
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input.trim()));
    println!("Part 2: {}", part2_solve(&input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(41, part1_solve(&input.trim()));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(1, part2_solve(&input.trim()));
    }
}
