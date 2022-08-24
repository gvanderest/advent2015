use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;

fn step_lanternfish(
    current_fish: Vec<usize>,
    reset_fish_value: usize,
    new_fish_value: usize,
) -> Vec<usize> {
    current_fish
        .par_iter()
        .flat_map(|days| -> Vec<usize> {
            match days {
                0 => [reset_fish_value, new_fish_value].to_vec(),
                _ => [days - 1].to_vec(),
            }
        })
        .collect()
}

fn parse_initial_fish(input: &String) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|n| -> usize { n.parse::<usize>().unwrap() })
        .collect()
}

fn process(input: &String, days: usize) -> usize {
    let mut current_fish = parse_initial_fish(&input);
    for day in 0..days {
        println!("Processing day {}", day);
        current_fish = step_lanternfish(current_fish, 6, 8);
        println!("It was {} fish!", current_fish.len());
    }
    current_fish.len()
}

const FISH_RESET_VALUE: usize = 6;
const NEW_FISH_VALUE: usize = 8;

fn process2(input: &String, days: usize, reporting: bool) -> usize {
    let initial_fish = parse_initial_fish(input);

    // Setup container for fish counts and prime values to zero
    let mut fish_counts: HashMap<usize, usize> = HashMap::new();
    for x in 0..=NEW_FISH_VALUE {
        fish_counts.insert(x, 0);
    }

    // Count initial fish
    for days_until_birth in initial_fish {
        let existing_value = fish_counts.get(&days_until_birth).unwrap();
        let new_value = existing_value + 1;
        fish_counts.insert(days_until_birth, new_value);
    }

    // For each day, shift fish down.. taking those that are zero and re-injecting them in two spots
    for current_day in 0..days {
        if reporting {
            println!("Day {}", current_day);
        }

        let quantity_birthing = *fish_counts.get(&0).unwrap();

        for day_to_move_from in 1..=NEW_FISH_VALUE {
            let today_value = *fish_counts.get(&day_to_move_from).unwrap();
            let day_to_move_to = day_to_move_from - 1;
            fish_counts.insert(day_to_move_to, today_value);
        }
        fish_counts.insert(NEW_FISH_VALUE, 0);

        // One spot, for the initial fish resetting their timers
        let old_reset_count = *fish_counts.get(&FISH_RESET_VALUE).unwrap();
        let new_reset_count = old_reset_count + quantity_birthing;
        fish_counts.insert(FISH_RESET_VALUE, new_reset_count);

        // Second spot, for their children
        let old_created_count = *fish_counts.get(&NEW_FISH_VALUE).unwrap();
        let new_created_count = old_created_count + quantity_birthing;
        fish_counts.insert(NEW_FISH_VALUE, new_created_count);

        if reporting {
            let mut report = String::from("|");
            for day in 0..=NEW_FISH_VALUE {
                report.push_str(fish_counts.get(&day).unwrap().to_string().as_str());
                report.push_str("|");
            }

            println!("{}", report);
        }
    }

    // At end, sum up total counts
    let mut count = 0;
    for x in 0..=NEW_FISH_VALUE {
        count += *fish_counts.get(&x).unwrap_or(&0);
    }

    count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open example text.");
    //println!("Part 1: {}", process(&input, 80)); -- turns out process 2 is way faster
    println!("Part 1: {}", process2(&input, 80, false));
    println!("Part 2: {}", process2(&input, 256, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_computes_properly() {
        let input = fs::read_to_string("example.txt").expect("Unable to read example file.");

        assert_eq!(26, process(&input, 18));
        assert_eq!(5934, process(&input, 80));
    }

    #[test]
    fn part2_computes_properly() {
        let input = fs::read_to_string("example.txt").expect("Unable to read example file.");

        // Basic tests
        assert_eq!(1, process2(&String::from("1"), 1, true));
        assert_eq!(2, process2(&String::from("1"), 2, true));
        assert_eq!(3, process2(&String::from("1"), 9, true));

        // Real tests
        assert_eq!(26, process2(&input, 18, true));
        assert_eq!(5934, process2(&input, 80, true));
        assert_eq!(26984457539, process2(&input, 256, true));
    }
}
