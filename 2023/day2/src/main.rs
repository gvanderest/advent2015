use std::{collections::HashMap, fs};

fn part1_solve(content: &str) -> i32 {
    let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    // Split the content into lines and iterate
    let lines = content.trim().split('\n');

    let mut sum = 0;

    'line: for line in lines {
        // Split the lines up into their parts
        let mut game_id_and_cubes = line.split(": ");

        // Determine the ID as an i32
        let game_id = game_id_and_cubes
            .next()
            .unwrap()
            .replace("Game ", "")
            .parse::<i32>()
            .unwrap();

        let raw_cubes = game_id_and_cubes.next().unwrap();
        let handfuls = raw_cubes.split("; ");

        for handful in handfuls {
            // Determine the maxes for each colour
            let mut maxes: HashMap<&str, i32> = HashMap::new();
            let cube_counts = handful.split(", ");
            for cube_count in cube_counts {
                let mut count_and_colour = cube_count.split(' ');
                let count = count_and_colour.next().unwrap().parse::<i32>().unwrap();
                let colour = count_and_colour.next().unwrap();
                maxes.insert(colour, count);
            }

            // For each of the limits, determine if the maxes are lower or non-existent
            for (colour_name, colour_limit) in &limits {
                let max_entry = *maxes.get(colour_name).unwrap_or(&0);

                // If failed: Skip to next line
                if max_entry > *colour_limit {
                    continue 'line;
                }
            }
        }

        // If passed: Add the ID's value to the sum if the maxes are less than the limits
        sum += game_id;
    }

    sum
}

fn part2_solve(content: &str) -> i32 {
    // Split the content into lines and iterate
    let lines = content.trim().split('\n');

    let mut sum = 0;

    for line in lines {
        // Split the lines up into their parts
        let mut game_id_and_cubes = line.split(": ");

        // Determine the ID as an i32
        let _game_id = game_id_and_cubes.next().unwrap();

        let raw_cubes = game_id_and_cubes.next().unwrap();
        let handfuls = raw_cubes.split("; ");

        // Determine the mins for each colour
        let mut mins: HashMap<&str, i32> = HashMap::new();

        for handful in handfuls {
            let cube_counts = handful.split(", ");
            for cube_count in cube_counts {
                let mut count_and_colour = cube_count.split(' ');
                let count = count_and_colour.next().unwrap().parse::<i32>().unwrap();
                let colour = count_and_colour.next().unwrap();

                // If there is no value or the count is less than existing for colour, set it
                match mins.get(colour) {
                    None => {
                        mins.insert(colour, count);
                    }
                    Some(colour_min) => {
                        if count > *colour_min {
                            mins.insert(colour, count);
                        }
                    }
                }
            }
        }

        // Sum up the power, aka the sum of minimum cube counts
        let power = mins.values().product::<i32>();
        sum += power;
    }

    sum
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&content));
    println!("Part 2: {}", part2_solve(&content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let content = fs::read_to_string("part1_example.txt").unwrap();
        assert_eq!(8, part1_solve(&content));
    }

    #[test]
    fn test_part_2_example() {
        let content = fs::read_to_string("part1_example.txt").unwrap();
        assert_eq!(2286, part2_solve(&content));
    }
}
