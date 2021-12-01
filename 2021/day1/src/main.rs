use std::fs;

fn part1_compute(input: String) -> usize {
    let lines = input.trim().split("\n");

    let mut previous_depth: isize = -1;
    let mut increase_count: usize = 0;

    for line in lines {
        let new_depth = line.parse::<isize>().unwrap();
        if previous_depth == -1 {
            previous_depth = new_depth;
            continue;
        }

        if new_depth > previous_depth {
            increase_count += 1;
        }
        previous_depth = new_depth;
    }

    increase_count
}

fn part2_compute(input: String) -> usize {
    let raw_lines: Vec<&str> = input.trim().split("\n").collect();

    let mut summed_depths: Vec<isize> = vec![];
    let mut last_few_numbers: Vec<isize> = vec![];

    for line in raw_lines {
        let value = line.parse::<isize>().unwrap();

        last_few_numbers.push(value);

        if last_few_numbers.len() < 3 {
            continue;
        }

        if last_few_numbers.len() > 3 {
            last_few_numbers.remove(0);
        }

        summed_depths.push(last_few_numbers.iter().sum());
    }

    let mut previous_depth: isize = -1;
    let mut increase_count: usize = 0;

    for new_depth in summed_depths {
        if previous_depth == -1 {
            previous_depth = new_depth;
            continue;
        }

        if new_depth > previous_depth {
            increase_count += 1;
        }
        previous_depth = new_depth;
    }

    increase_count
}

fn main() {
    let input = String::from(fs::read_to_string("input.txt").expect("Unable to open input file."));

    println!("Part 1: {}", part1_compute(input.clone()));
    println!("Part 2: {}", part2_compute(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part_1_depths_work() {
        assert_eq!(
            part1_compute(String::from(
                "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
            )),
            7
        )
    }

    #[test]
    fn check_part_2_depths_work() {
        assert_eq!(
            part2_compute(String::from("607\n618\n618\n617\n647\n716\n769\n792")),
            5
        )
    }
}
