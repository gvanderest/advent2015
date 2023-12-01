use std::{collections::HashMap, fs};

fn step1_solve(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        let mut digits: Vec<i32> = vec![];

        for char in line.chars() {
            if char.is_digit(10) {
                digits.push(char.to_string().parse::<i32>().unwrap());
            }
        }

        if digits.len() < 1 {
            continue;
        }

        sum += (digits.first().unwrap() * 10) + digits.last().unwrap();
    }
    sum
}

fn step2_solve(lines: &Vec<String>) -> i32 {
    let word_numbers_mapping = HashMap::from([
        (String::from("zero"), 0),
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ]);

    let mut sum = 0;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();

        let mut digits: Vec<i32> = vec![];

        let mut i = 0;
        while i < line.len() {
            // println!("line={}", line);
            // If this is a digit character, parse and append
            let char = chars.get(i).unwrap();
            // println!("char={}", char);
            if char.is_digit(10) {
                // println!("char={} is a digit, appending and moving on", char);
                digits.push(char.to_string().parse::<i32>().unwrap());
                i += 1;
                continue;
            }

            // Otherwise, if this is a word that matches, parse and advance by length of word
            for (word, word_number) in &word_numbers_mapping {
                // Can't possibly be a word if we're too far into the string
                if line.len() - i < word.len() {
                    continue;
                }

                let line_slice = String::from(&line[i..i + word.len()]);
                if line_slice.eq(word) {
                    digits.push(word_number.clone());
                    break;
                }
            }
            i += 1;
        }

        println!("line {}, digits {:?}", line, digits);

        sum += (digits.first().unwrap() * 10) + digits.last().unwrap();
    }
    sum
}

fn main() {
    let lines = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| String::from(s))
        .collect();
    println!("Part 1: {}", step1_solve(&lines));
    println!("Part 2: {}", step2_solve(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let lines = fs::read_to_string("part1_test.txt")
            .unwrap()
            .trim()
            .split("\n")
            .map(|s| String::from(s))
            .collect();
        assert_eq!(142, step1_solve(&lines));
    }

    #[test]
    fn part2_test() {
        let lines = fs::read_to_string("part2_test.txt")
            .unwrap()
            .trim()
            .split("\n")
            .map(|s| String::from(s))
            .collect();
        assert_eq!(281, step2_solve(&lines));
    }
}
