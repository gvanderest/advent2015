use itertools::Itertools;
use pad::{Alignment, PadStr};
use rayon::prelude::*;
use std::{
    fs, iter,
    sync::{Arc, Mutex},
};

type Pattern<'a> = &'a str;
type Counts = Vec<i32>;
type PatternAndCounts<'a> = (Pattern<'a>, Counts);

fn parse_patterns_and_counts<'a>(lines: &'a Vec<&str>) -> Vec<PatternAndCounts<'a>> {
    lines
        .iter()
        .map(|line| {
            let mut pattern_vs_count = line.split(' ');
            let pattern = pattern_vs_count.next().unwrap();
            let count = pattern_vs_count.next().unwrap();

            let counts = count
                .split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect();

            (pattern, counts)
        })
        .collect()
}

const UNKNOWN_CHAR: char = '?';

fn count_groupings(pattern: &str) -> Vec<i32> {
    pattern
        .split('.')
        .filter(|x| !x.is_empty())
        .map(|s| s.len() as i32)
        .collect()
}

fn count_pattern_possibilities((pattern, expected_counts): &PatternAndCounts) -> i32 {
    // Identify the regions of the number that are `?` to determine how many bits we need to walk
    let unknown_count = pattern.chars().filter(|c| *c == UNKNOWN_CHAR).count();

    // Starting at zero, walk to the max of.. 2^BITS
    let max_number = usize::pow(2, unknown_count as u32);

    let mut count = 0;

    for x in 0..max_number {
        let generated_bits = format!("{x:b}").pad(unknown_count, '0', Alignment::Right, false);
        let bits_as_symbols = generated_bits.replace('0', ".").replace('1', "#");

        // Each step, replace the `?` in the string to be the bit states of the walked number
        let mut bits_to_ingest = bits_as_symbols.chars();
        let filled_pattern = pattern
            .chars()
            .map(|c| {
                if c == '?' {
                    return bits_to_ingest.next().unwrap();
                }
                c
            })
            .collect::<String>();

        // Count the grouped bits and compare against expected counts.. increment if matched
        if count_groupings(&filled_pattern).eq(expected_counts) {
            count += 1;
        }
    }

    count
}

fn part1_solve(input: &str) -> i32 {
    // Let's try something with.. bitmasks or similar
    // Read the lines to determine the patterns and expected counts
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let patterns_and_counts = parse_patterns_and_counts(&lines);

    patterns_and_counts
        .iter()
        .map(count_pattern_possibilities)
        .sum()
}

fn unfold_pattern((pattern, counts): &(&str, Vec<i32>), amount: usize) -> (String, Vec<i32>) {
    let new_pattern: String = (0..amount)
        .map(|_x| *pattern)
        .collect::<Vec<&str>>()
        .join("?");
    let new_counts: Vec<i32> = counts
        .iter()
        .cycle()
        .take(counts.len() * amount)
        .cloned()
        .collect();

    (new_pattern, new_counts)
}

const PART_2_REPEATS: usize = 5;

fn part2_count_pattern_possibilities(pattern_and_count: &PatternAndCounts) -> i32 {
    println!("{:?}", pattern_and_count);
    let mut values: Vec<i32> = Vec::new();

    for x in 1..=2 {
        let unfolded = unfold_pattern(pattern_and_count, x);
        let count = count_pattern_possibilities(&(unfolded.0.as_str(), unfolded.1));
        println!("{x}: count={count}, unfolded={}", unfolded.0);
        values.push(count);
    }

    for x in 2..PART_2_REPEATS {
        let back_one = values[x - 1];
        let back_two = values[x - 2];

        values.push(back_one * (back_one / back_two));
    }

    *values.last().unwrap()
}

fn part2_solve(input: &str) -> i32 {
    // By figuring out iteration 1 and 2, we can extrapolate answers from them
    let completed_count = Arc::new(Mutex::new(0));
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let patterns_and_counts = parse_patterns_and_counts(&lines);
    patterns_and_counts
        .par_iter()
        .enumerate()
        .map(|(index, (pattern, counts))| {
            let count = part2_count_pattern_possibilities(&(pattern, counts.clone()));

            let completed_count = Arc::clone(&completed_count);
            let mut completed_count_lock = completed_count.lock().unwrap();
            *completed_count_lock += 1;
            println!("{}/{}", completed_count_lock, lines.len());

            count
        })
        .sum()
}

fn generate_pattern(counts: &Vec<i32>, gaps: &Vec<usize>) -> String {
    let mut output = String::new();
    for (gap_index, gap_size) in gaps.iter().enumerate() {
        output.push_str(".".repeat(*gap_size).as_str());
        if gap_index < counts.len() {
            let pool_size = counts[gap_index];
            output.push_str("#".repeat(pool_size as usize).as_str());
        }
    }
    output
}

fn pattern_matches(attempt: &str, pattern: &str) -> bool {
    if attempt.len() != pattern.len() {
        return false;
    }
    let mut attempt_chars = attempt.chars();
    for pattern_char in pattern.chars() {
        let attempt_char = attempt_chars.next().unwrap();

        // Auto-pass if it's a '?'
        if pattern_char == '?' {
            continue;
        }

        if pattern_char != attempt_char {
            return false;
        }
    }
    true
}

fn part2_count_pattern_possibilities_b((pattern, expected_counts): &PatternAndCounts) -> i32 {
    // For example.. (3,3,3) would be something like "###.###.###" or "###..###..###" until all the spots are filled, with varying space between

    // Total pool size will be sum of all pools
    let total_pool_size: usize = expected_counts.iter().map(|c| *c as usize).sum();
    // Gap room will be total pattern size minus total pool size
    let total_gap_size: usize = pattern.len() - total_pool_size;

    // This approach will use two vecs.. one for pool sizes, the other for gaps between them.. (gap will be pool.len()-1 in size)
    // Pool sizes are the expected counts above
    // let mut gaps: Vec<usize> = (0..(expected_counts.len() + 1)).map(|_x| 0).collect();

    println!("pattern={pattern:?}");
    println!("pattern_length={:?}", pattern.len());
    println!("total_pool_size={total_pool_size:?}");
    println!("total_gap_size={total_gap_size:?}");
    println!("expected_counts={expected_counts:?}");
    println!("expected_counts_len={:?}", expected_counts.len());

    let mut count = 0;
    println!("HEY");

    for gaps in (0..total_gap_size).combinations_with_replacement(expected_counts.len() + 1) {
        if gaps.iter().sum::<usize>() > total_gap_size {
            continue;
        }

        // And then compare against the pattern to see if it can even apply
        let generated_pattern = generate_pattern(expected_counts, &gaps);
        if pattern_matches(generated_pattern.as_str(), pattern) {
            count += 1;
        }
    }

    count
}

fn part2_solve_b(input: &str) -> i32 {
    // Hmm.. Insteaed of starting from empties and trying to fill, let's start from answers and see if they fit
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let patterns_and_counts: Vec<(String, Vec<i32>)> = parse_patterns_and_counts(&lines)
        .iter()
        .map(|pc| unfold_pattern(pc, PART_2_REPEATS))
        .collect();

    patterns_and_counts
        .iter()
        .map(|(pattern, counts)| part2_count_pattern_possibilities_b(&(pattern, counts.clone())))
        .sum()
}

const UNKNOWN_SYMBOL: char = '?';
const OPERATIONAL_SYMBOL: char = '.';
const DAMAGED_SYMBOL: char = '#';

fn generate_pattern_c(pattern: &str, replacements: &[char]) -> String {
    let mut replaces = replacements.iter();
    pattern
        .chars()
        .map(|c| {
            if c == UNKNOWN_SYMBOL {
                *replaces.next().unwrap()
            } else {
                c
            }
        })
        .collect::<String>()
}

fn part2_count_pattern_possibilities_c((pattern, expected_counts): &PatternAndCounts) -> i32 {
    // Let's try using combinations?
    // Fill a bag with all the combinations possible for remaining working/broken springs based on known information
    let unknowns_count = pattern.chars().filter(|c| *c == UNKNOWN_SYMBOL).count();
    let known_damaged_count = pattern.chars().filter(|c| *c == DAMAGED_SYMBOL).count();
    let expected_damaged_count: usize = expected_counts.iter().map(|c| *c as usize).sum();

    println!("pattern={pattern}, expected_counts={expected_counts:?}, expected_damaged_count={expected_damaged_count}, known_damaged_count={known_damaged_count}");

    let bag_damaged_count = expected_damaged_count - known_damaged_count;
    let bag_operational_count = unknowns_count - bag_damaged_count;

    assert_eq!(
        bag_damaged_count + bag_operational_count,
        unknowns_count,
        "Counts must be equal!"
    );

    println!("unknowns_count={unknowns_count}, bag_damaged_count={bag_damaged_count}, bag_operational_count={bag_operational_count}");
    let bag = iter::repeat(DAMAGED_SYMBOL)
        .take(bag_damaged_count)
        .chain(iter::repeat(OPERATIONAL_SYMBOL).take(bag_operational_count));

    bag.permutations(unknowns_count)
        .par_bridge()
        .map(|combo| {
            // And then compare against the pattern to see if it can even apply
            let generated_pattern = generate_pattern_c(pattern, &combo);
            // println!("generated: {generated_pattern}");
            if pattern_matches(generated_pattern.as_str(), pattern) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2_solve_c(input: &str) -> i32 {
    // Hmm.. Insteaed of starting from empties and trying to fill, let's start from answers and see if they fit
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let patterns_and_counts: Vec<(String, Vec<i32>)> = parse_patterns_and_counts(&lines)
        .par_iter()
        .map(|pc| unfold_pattern(pc, PART_2_REPEATS))
        .collect();

    patterns_and_counts
        .par_iter()
        .map(|(pattern, counts)| part2_count_pattern_possibilities_c(&(pattern, counts.clone())))
        .sum()
}

fn part2_count_pattern_possibilities_d((pattern, provided_counts): &PatternAndCounts) -> u64 {
    let mut pattern_chars: Vec<char> = pattern.chars().collect();
    let counts: Vec<usize> = provided_counts.par_iter().map(|c| *c as usize).collect();

    pattern_chars.push(OPERATIONAL_SYMBOL);
    let mut cache = vec![vec![None; pattern_chars.len()]; counts.len()];
    part2_count_pattern_possibilities_d_inner(&pattern_chars, &counts, &mut cache)
}

fn part2_count_pattern_possibilities_d_inner(
    springs: &[char],
    counts: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    if counts.is_empty() {
        return if springs.contains(&DAMAGED_SYMBOL) {
            // Too many previous unknowns were counted as damaged
            0
        } else {
            // All remaining unknowns are operational
            1
        };
    }
    if springs.len() < counts.iter().sum::<usize>() + counts.len() {
        // Not enough space for remaining numbers
        return 0;
    }
    if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
        return cached;
    }
    let mut arangements = 0;
    if springs[0] != DAMAGED_SYMBOL {
        // Assume operational
        arangements += part2_count_pattern_possibilities_d_inner(&springs[1..], counts, cache);
    }
    let next_group_size = counts[0];
    if !springs[..next_group_size].contains(&OPERATIONAL_SYMBOL)
        && springs[next_group_size] != DAMAGED_SYMBOL
    {
        // Assume damaged
        arangements += part2_count_pattern_possibilities_d_inner(
            &springs[next_group_size + 1..],
            &counts[1..],
            cache,
        );
    }
    cache[counts.len() - 1][springs.len() - 1] = Some(arangements);
    arangements
}

fn part2_solve_d(input: &str) -> u64 {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let patterns_and_counts: Vec<(String, Vec<i32>)> = parse_patterns_and_counts(&lines)
        .par_iter()
        .map(|pc| unfold_pattern(pc, PART_2_REPEATS))
        .collect();

    patterns_and_counts
        .par_iter()
        .map(|(pattern, counts)| part2_count_pattern_possibilities_d(&(pattern, counts.clone())))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    println!("Part 2: {}", part2_solve_d(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Vec::from([1, 2, 1]), count_groupings("..#....##.....#...."));
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(10, part1_solve(&input));
        let input = fs::read_to_string("example2.txt").unwrap();
        assert_eq!(21, part1_solve(&input));
    }

    #[test]
    fn test_part2_unfold() {
        let result = unfold_pattern(&(".#", vec![1]), PART_2_REPEATS);
        assert_eq!(
            (".#?.#?.#?.#?.#", vec![1, 1, 1, 1, 1]),
            (result.0.as_str(), result.1)
        );

        let result = unfold_pattern(&("???.###", vec![1, 1, 3]), PART_2_REPEATS);
        assert_eq!(
            (
                "???.###????.###????.###????.###????.###",
                vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
            ),
            (result.0.as_str(), result.1)
        );
    }

    #[test]
    fn test_part2_match() {
        assert!(pattern_matches("", ""));
        assert!(pattern_matches("#.#.###", "???.###"));
        assert!(pattern_matches("###.###", "???.###"));
    }

    #[test]
    fn test_generate_pattern_c() {
        assert_eq!(
            generate_pattern_c("?.?.#.?.?", &['#', '#', '.', '.']),
            "#.#.#...."
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2_solve_d("???.### 1,1,3"));
        assert_eq!(16384, part2_solve_d(".??..??...?##. 1,1,3"));
        assert_eq!(1, part2_solve_d("?#?#?#?#?#?#?#? 1,3,1,6"));
        assert_eq!(16, part2_solve_d("????.#...#... 4,1,1"));
        assert_eq!(2500, part2_solve_d("????.######..#####. 1,6,5"));
        assert_eq!(506250, part2_solve_d("?###???????? 3,2,1"));

        // let lines: Vec<&str> = "????.######..#####. 1,6,5".split('\n').collect();
        // // let lines: Vec<&str> = input.trim().split('\n').collect();
        // let patterns_and_counts = parse_patterns_and_counts(&lines);
        // for x in 1..=3 {
        //     let unfolded = unfold_pattern(&patterns_and_counts[0], x);
        //     println!(
        //         "{}: {}",
        //         x,
        //         count_pattern_possibilities(&(unfolded.0.as_str(), unfolded.1))
        //     );
        //     // For "?###???????? 3,2,1" expect 506250
        //     // 1: 10
        //     // 2: 150
        //     // 3: 2250
        //     // --- .. val_at_x*(val_at_x_minus_1/val_at_x_minus_2) ??
        //     // 4: 33750
        //     // 5: 506250

        //     // For "????.######..#####. 1,6,5" expect 2500
        //     // 1: 4
        //     // 2: 20
        //     // 3: 100
        //     // --
        //     // 4: 500
        //     // 5: 2500
        // }
        // assert_eq!(1, 2);
    }
}
