use std::collections::HashMap;
use std::fs;

fn part1_compute(directions: String) -> usize {
    let mut coords: (isize, isize) = (0, 0);

    // Collect coordinates to counts
    let mut coords_to_deliveries: HashMap<(isize, isize), usize> = HashMap::new();
    coords_to_deliveries.insert(coords, 1);

    for direction in directions.chars() {
        coords = match direction {
            '>' => (coords.0 + 1, coords.1),
            '<' => (coords.0 - 1, coords.1),
            '^' => (coords.0, coords.1 + 1),
            'v' => (coords.0, coords.1 - 1),
            _ => panic!("Unexpected symbol encountered: {}", direction),
        };

        let old_deliveries_count = coords_to_deliveries.get(&coords).unwrap_or(&0).clone();
        coords_to_deliveries.insert(coords, old_deliveries_count + 1);
    }

    let mut count = 0;

    // Iterate over deliveries to see how many are >= 2
    for deliveries in coords_to_deliveries.into_values() {
        if deliveries >= 1 {
            count += 1;
        }
    }

    // Return count
    count
}

// Same as before, but alternating whose turn it is.
fn part2_compute(directions: String) -> usize {
    let mut santa_coords: (isize, isize) = (0, 0);
    let mut robot_coords: (isize, isize) = (0, 0);

    // Collect coordinates to counts
    let mut coords_to_deliveries: HashMap<(isize, isize), usize> = HashMap::new();
    coords_to_deliveries.insert(santa_coords, 1);
    coords_to_deliveries.insert(robot_coords, 1);

    for (index, direction) in directions.chars().enumerate() {
        let mut coords = match index % 2 {
            0 => santa_coords,
            _ => robot_coords,
        };

        coords = match direction {
            '>' => (coords.0 + 1, coords.1),
            '<' => (coords.0 - 1, coords.1),
            '^' => (coords.0, coords.1 + 1),
            'v' => (coords.0, coords.1 - 1),
            _ => panic!("Unexpected symbol encountered: {}", direction),
        };

        let old_deliveries_count = coords_to_deliveries.get(&coords).unwrap_or(&0).clone();
        coords_to_deliveries.insert(coords, old_deliveries_count + 1);

        match index % 2 {
            0 => santa_coords = coords,
            _ => robot_coords = coords,
        }
    }

    let mut count = 0;

    // Iterate over deliveries to see how many are >= 2
    for deliveries in coords_to_deliveries.into_values() {
        if deliveries >= 1 {
            count += 1;
        }
    }

    // Return count
    count
}

fn main() {
    let input = String::from(
        fs::read_to_string("input.txt")
            .expect("Unable to read input file.")
            .trim(),
    );

    println!("Part 1: {}", part1_compute(input.clone()));
    println!("Part 2: {}", part2_compute(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_definition() {
        assert_eq!(part1_compute(String::from(">")), 0);
        assert_eq!(part1_compute(String::from("^>v<")), 1);
        assert_eq!(part1_compute(String::from("^v^v^v^v^v")), 2);
    }
}
