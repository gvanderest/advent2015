use std::fs;

fn is_nice_string(input: String) -> bool {
    // TODO: How to make a constant list of vowels?
    let vowels = vec!["a", "e", "i", "o", "u"];
    let invalid_patterns = vec!["ab", "cd", "pq", "xy"];

    // It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
    for invalid_pattern in invalid_patterns {
        if input.contains(invalid_pattern) {
            return false;
        }
    }

    let mut vowel_count: usize = 0;

    let mut previous_char: char = ' ';
    let mut double_char_found: bool = false;

    for letter_char in input.chars() {
        // It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
        let letter_string = String::from(letter_char.to_ascii_lowercase());
        let letter = letter_string.as_str();
        if vowels.contains(&letter) {
            vowel_count += 1;
        }

        // It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
        if previous_char == letter_char {
            double_char_found = true;
        }
        previous_char = letter_char;
    }

    return vowel_count >= 3 && double_char_found;
}

const DOUBLE_PATTERN_LENGTH: usize = 2;
const ALTERNATING_PATTERN_LENGTH: usize = 3;

fn is_nice_string2(input: String) -> bool {
    // It contains a pair of any two letters that appears at least twice in the
    // string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not
    // like aaa (aa, but it overlaps).
    let mut double_pattern_found = false;
    'outer: for index in 0..=input.len() - DOUBLE_PATTERN_LENGTH {
        let slice = &input[index..=index + DOUBLE_PATTERN_LENGTH - 1];
        for future_index in index + DOUBLE_PATTERN_LENGTH..=input.len() - DOUBLE_PATTERN_LENGTH {
            let future_slice = &input[future_index..=future_index + DOUBLE_PATTERN_LENGTH - 1];
            let matches = slice == future_slice;
            if matches {
                double_pattern_found = true;
                break 'outer;
            }
        }
    }

    let mut alternating_pattern_found = false;
    // It contains at least one letter which repeats with exactly one letter
    // between them, like xyx, abcdefeghi (efe), or even aaa.
    for index in 0..=input.len() - ALTERNATING_PATTERN_LENGTH {
        let current_letter = &input[index..index + 1];
        let future_letter = &input[index + 2..index + 3];
        if current_letter == future_letter {
            alternating_pattern_found = true;
            break;
        }
    }

    double_pattern_found && alternating_pattern_found
}

fn part1_compute(lines: &Vec<&str>) -> usize {
    let mut count: usize = 0;
    for line in lines {
        let wrapped = String::from(*line);
        if is_nice_string(wrapped) {
            count += 1;
        }
    }

    count
}

fn part2_compute(lines: &Vec<&str>) -> usize {
    let mut count: usize = 0;
    for line in lines {
        let wrapped = String::from(*line);
        if is_nice_string2(wrapped) {
            count += 1;
        }
    }

    count
}

fn main() {
    let input = String::from(fs::read_to_string("input.txt").expect("Unable to read input file."));
    let lines: Vec<&str> = input.trim().split("\n").collect();
    println!("Part 1: {}", part1_compute(&lines));
    println!("Part 2: {}", part2_compute(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_happy_strings() {
        assert_eq!(is_nice_string(String::from("ugknbfddgicrmopn")), true);
        assert_eq!(is_nice_string(String::from("aaa")), true);
        assert_eq!(is_nice_string(String::from("jchzalrnumimnmhp")), false);
        assert_eq!(is_nice_string(String::from("haegwjzuvuyypxyu")), false);
        assert_eq!(is_nice_string(String::from("dvszwmarrgswjxmb")), false);
    }

    #[test]
    fn test_part_2_example_happy_strings() {
        assert_eq!(is_nice_string2(String::from("qjhvhtzxzqqjkmpb")), true);
        assert_eq!(is_nice_string2(String::from("xxyxx")), true);
        assert_eq!(is_nice_string2(String::from("uurcxstgmygtbstg")), false);
        assert_eq!(is_nice_string2(String::from("ieodomkazucvgmuy")), false);
        assert_eq!(is_nice_string2(String::from("abcdefef")), true);
        assert_eq!(is_nice_string2(String::from("xxx")), false);
    }
}
