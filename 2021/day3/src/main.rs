use std::collections::HashSet;
use std::fs;

fn convert_binary_to_decimal(binary: String) -> usize {
    return usize::from_str_radix(&binary, 2).unwrap();
}

fn compute_part1_raw_binary_strings(lines: &Vec<&str>) -> (String, String) {
    let mut counts: Vec<isize> = vec![];
    for line in lines.iter() {
        for _ in counts.len()..line.len() {
            counts.push(0)
        }
        for (char_index, c) in line.chars().enumerate() {
            if let Some(value) = counts.get_mut(char_index) {
                *value += match c {
                    '0' => -1,
                    '1' => 1,
                    _ => 0,
                };
            }
        }
    }
    let gamma: String = counts
        .iter()
        .map(|x| {
            if *x >= 0 {
                return 1;
            }
            return 0;
        })
        .map(|x| x.to_string())
        .collect();
    let epsilon: String = counts
        .iter()
        .map(|x| {
            if *x >= 0 {
                return 0;
            }
            return 1;
        })
        .map(|x| x.to_string())
        .collect();
    (gamma, epsilon)
}

fn part1_compute(lines: &Vec<&str>) -> usize {
    let (raw_gamma, raw_epsilon) = compute_part1_raw_binary_strings(&lines);
    let gamma = convert_binary_to_decimal(raw_gamma);
    let epsilon = convert_binary_to_decimal(raw_epsilon);

    gamma * epsilon
}

fn compute_part2_raw_binary_strings(lines: &Vec<&str>) -> (String, String) {
    let num_chars = lines.get(0).unwrap().len();
    let mut high_remainders: HashSet<&str> = HashSet::new();
    let mut low_remainders: HashSet<&str> = HashSet::new();

    for line in lines {
        high_remainders.insert(line);
        low_remainders.insert(line);
    }

    // Get high number..
    'high_outer: for position in 0..num_chars {
        let mut sum = 1;
        for high_num in high_remainders.iter() {
            let char_at_position = high_num.chars().nth(position).unwrap();
            sum += match char_at_position {
                '0' => -1,
                _ => 1,
            }
        }
        let expected_char_at_position = match sum > 0 {
            true => '1',
            false => '0',
        };

        for high_num in high_remainders.clone() {
            let char_at_position = high_num.chars().nth(position).unwrap();
            if char_at_position != expected_char_at_position {
                high_remainders.remove(high_num);
            }
            if high_remainders.len() == 1 {
                break 'high_outer;
            }
        }
    }

    // Get low number..
    'low_outer: for position in 0..num_chars {
        let mut sum = 1;
        for low_num in low_remainders.iter() {
            let char_at_position = low_num.chars().nth(position).unwrap();
            sum += match char_at_position {
                '0' => -1,
                _ => 1,
            }
        }
        let expected_char_at_position = match sum > 0 {
            true => '0',
            false => '1',
        };

        for low_num in low_remainders.clone() {
            let char_at_position = low_num.chars().nth(position).unwrap();
            if char_at_position != expected_char_at_position {
                low_remainders.remove(low_num);
            }
            if low_remainders.len() == 1 {
                break 'low_outer;
            }
        }
    }

    (
        String::from(*high_remainders.iter().next().unwrap()),
        String::from(*low_remainders.iter().next().unwrap()),
    )
}

fn part2_compute(lines: &Vec<&str>) -> usize {
    let (raw_high, raw_low) = compute_part2_raw_binary_strings(&lines);
    let high = convert_binary_to_decimal(raw_high);
    let low = convert_binary_to_decimal(raw_low);

    high * low
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input file.");
    let lines = input.trim().split("\n").collect::<Vec<&str>>();
    println!("Part one: {}", part1_compute(&lines));
    println!("Part two: {}", part2_compute(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_gamma_and_epsilon_values() {
        let input = fs::read_to_string("example-1.txt").unwrap();
        let lines = input.trim().split("\n").collect::<Vec<&str>>();
        let (raw_gamma, raw_epsilon) = compute_part1_raw_binary_strings(&lines);
        assert_eq!(raw_gamma, String::from("10110"));
        assert_eq!(raw_epsilon, String::from("01001"));
    }

    #[test]
    fn test_part1_strings_are_correct_decimals() {
        assert_eq!(convert_binary_to_decimal(String::from("10110")), 22);
        assert_eq!(convert_binary_to_decimal(String::from("01001")), 9);
    }

    #[test]
    fn test_part2_binary_strings() {
        let input = fs::read_to_string("example-1.txt").unwrap();
        let lines = input.trim().split("\n").collect::<Vec<&str>>();
        let (one, two) = compute_part2_raw_binary_strings(&lines);
        assert_eq!(one, String::from("10111"));
        assert_eq!(two, String::from("01010"));
    }
}
