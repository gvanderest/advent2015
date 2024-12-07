#![feature(map_try_insert)]
use std::{collections::HashMap, fs};

fn part1_solve(input: &str) -> i32 {
    let mut sum = 0;

    enum InputMode {
        Rules,
        Lists,
    }

    let lines = input.split("\n");
    let mut mode = InputMode::Rules;

    let mut rules: Vec<(i32, i32)> = vec![];
    let mut lists: Vec<Vec<i32>> = vec![];

    for line in lines {
        if line.is_empty() {
            mode = InputMode::Lists;
            continue;
        }

        match mode {
            InputMode::Rules => {
                let mut rule = line.split("|");
                rules.push((
                    rule.next().unwrap().parse::<i32>().unwrap(),
                    rule.next().unwrap().parse::<i32>().unwrap(),
                ));
            }
            InputMode::Lists => {
                println!("{}", line);
                let list_of_numbers = line
                    .split(",")
                    .map(|raw_num| raw_num.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                assert!(list_of_numbers.len() % 2 != 0);
                lists.push(list_of_numbers);
            }
        }
    }

    // For each number, store the rules into two hashmaps
    let mut required_befores: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut required_afters: HashMap<i32, Vec<i32>> = HashMap::new();
    for (before, after) in rules {
        match required_befores.try_insert(after, vec![before]) {
            Ok(_) => {}
            Err(mut entry) => {
                entry.value.push(before);
            }
        }
        match required_afters.try_insert(before, vec![after]) {
            Ok(_) => {}
            Err(mut entry) => {
                entry.value.push(after);
            }
        }
    }

    // For each list, go through the numbers and test all the surrounding numbers for matching a rule or not
    'next_list: for list in lists {
        println!("list: {:?}", list);
        for (current_index, current_number) in list.iter().enumerate() {
            'next_number_in_list: for (check_index, check_number) in list.iter().enumerate() {
                let (earlier_number, later_number) = if check_index < current_index {
                    (check_number, current_number)
                } else if check_index > current_index {
                    (current_number, check_number)
                } else {
                    // same number in two lists, skip
                    continue 'next_number_in_list;
                };

                // ensure no required_after rules are broken
                if let Some(required_afters) = required_afters.get(later_number) {
                    if required_afters.contains(earlier_number) {
                        println!(
                            "failed because {} is required to be after {}",
                            earlier_number, later_number
                        );
                        continue 'next_list;
                    }
                }

                // ensure no required_befores rules are broken
                if let Some(required_befores) = required_befores.get(earlier_number) {
                    if required_befores.contains(later_number) {
                        println!(
                            "failed because {} is required to be before {}",
                            later_number, earlier_number
                        );
                        continue 'next_list;
                    }
                }
            }
        }
        sum += list.get(list.len() / 2).unwrap();
    }

    sum
}

fn part2_solve(input: &str) -> u64 {
    0
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
        assert_eq!(143, part1_solve(&input.trim()));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(1, part2_solve(&input.trim()));
    }
}
