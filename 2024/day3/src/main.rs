use regex::Regex;
use std::fs;

fn part1_solve(input: &str) -> u64 {
    let re = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();

    let mut sum: u64 = 0;

    for x in 0..input.len() {
        for (_, [left, right]) in re.captures_iter(&input[x..]).map(|c| c.extract()) {
            if left.len() > 3 || right.len() > 3 {
                continue;
            }
            let left = left.parse::<u64>().unwrap();
            let right = right.parse::<u64>().unwrap();

            sum += left * right;
        }
    }

    sum
}

fn part2_solve(input: &str) -> u64 {
    let do_re = Regex::new(r"^do\(\)").unwrap();
    let dont_re = Regex::new(r"^don't\(\)").unwrap();
    let mul_re = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();

    let mut enabled = true;
    let mut sum: u64 = 0;

    for x in 0..input.len() {
        let current_slice = &input[x..];

        if do_re.find(current_slice).is_some() {
            enabled = true;
            continue;
        } else if dont_re.find(current_slice).is_some() {
            enabled = false;
            continue;
        }

        for (_, [left, right]) in mul_re.captures_iter(current_slice).map(|c| c.extract()) {
            if enabled == false || left.len() > 3 || right.len() > 3 {
                continue;
            }
            let left = left.parse::<u64>().unwrap();
            let right = right.parse::<u64>().unwrap();

            sum += left * right;
        }
    }

    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    println!("Part 2: {}", part2_solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(161, part1_solve(&input));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example2.txt").unwrap();
        assert_eq!(48, part2_solve(&input));
    }
}
