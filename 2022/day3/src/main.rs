use std::{collections::HashMap, fs};

fn part1_convert_char_to_score(c: char) -> i32 {
    let val = c as i32;

    // Source: https://www.asciitable.com/
    // a-z
    if val >= 97 && val <= 122 {
        return val - 97 + 1;
    // A-Z
    } else if val >= 65 && val <= 132 {
        return val - 65 + 27;
    }
    0
}

fn part1_process_rucksack(input: &String) -> i32 {
    // Split the string in half for the two sides
    let len = input.len();
    if len % 2 != 0 {
        panic!("Uneven line: {}", input);
    }
    let half = len / 2;
    let halves = input.split_at(half);

    // Get left side counts collected
    let mut left_counts = HashMap::<char, i32>::new();
    for c in halves.0.chars() {
        *(left_counts.entry(c).or_insert(0)) += 1;
    }

    // If any item on the right side triggers a
    for c in halves.1.chars() {
        let left_count_value = *(left_counts.get(&c).unwrap_or(&0));
        if left_count_value > 0 {
            return part1_convert_char_to_score(c);
        }
    }

    0
}

fn part1_process(input: &String) -> i32 {
    let mut score = 0;

    // For each line, add score
    for line in input.split("\n") {
        score += part1_process_rucksack(&String::from(line));
    }

    // Return score
    score
}

fn part2_process_rucksacks(rucksacks: &Vec<String>) -> i32 {
    let mut letter_counts = HashMap::<char, usize>::new();

    for rucksack in rucksacks {
        let mut seen_chars = HashMap::<char, bool>::new();
        for c in rucksack.chars() {
            if seen_chars.contains_key(&c) {
                continue;
            }
            seen_chars.insert(c, true);
            *(letter_counts.entry(c).or_insert(0)) += 1;
        }
    }

    let rucksack_count = rucksacks.len();
    for (c, count) in letter_counts {
        if count == rucksack_count {
            return part1_convert_char_to_score(c);
        }
    }
    0
}

fn part2_process(input: &String) -> i32 {
    let mut score = 0;

    // For each line, add score
    let lines = input.split("\n").into_iter();
    let mut chunk = vec![];

    for line in lines {
        chunk.push(String::from(line));
        if chunk.len() == 3 {
            score += part2_process_rucksacks(&chunk);
            chunk.clear();
        }
    }

    // Return score
    score
}

fn main() {
    let input = String::from(
        fs::read_to_string("input.txt")
            .expect("error while opening input.txt")
            .trim(),
    );
    println!("Part 1: {}", part1_process(&input));
    println!("Part 2: {}", part2_process(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_char_conversion() {
        assert_eq!(part1_convert_char_to_score('a'), 1);
        assert_eq!(part1_convert_char_to_score('z'), 26);
        assert_eq!(part1_convert_char_to_score('A'), 27);
        assert_eq!(part1_convert_char_to_score('Z'), 52);
    }

    #[test]
    fn test_examples_work() {
        let input = String::from(
            fs::read_to_string("example.txt")
                .expect("error while opening example.txt")
                .trim(),
        );
        assert_eq!(part1_process(&input), 157);
        assert_eq!(part2_process(&input), 70);
    }
}
