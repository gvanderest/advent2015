use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    println!("Part 2: {}", part2_solve(&input));
}

fn determine_number_of_ways_to_win((time, distance_record): &(u64, u64)) -> u64 {
    let mut count = 0;
    for x in 0..*time {
        let remainder = time - x;
        if x * remainder > *distance_record {
            count += 1;
        }
    }
    count
}

fn get_times_and_distances_from_lines(lines: &Vec<&str>) -> Vec<(u64, u64)> {
    let times: Vec<u64> = lines
        .get(0)
        .unwrap()
        .replace("Time:", "")
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .get(1)
        .unwrap()
        .replace("Distance:", "")
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    times
        .iter()
        .zip(distances.iter())
        .map(|(a, b)| (*a, *b))
        .collect()
}

fn get_times_and_distances_from_lines_without_spaces(lines: &Vec<&str>) -> Vec<(u64, u64)> {
    let times: Vec<u64> = lines
        .get(0)
        .unwrap()
        .replace("Time:", "")
        .replace(" ", "")
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .get(1)
        .unwrap()
        .replace("Distance:", "")
        .replace(" ", "")
        .trim()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect();

    times
        .iter()
        .zip(distances.iter())
        .map(|(a, b)| (*a, *b))
        .collect()
}

fn part1_solve(input: &str) -> u64 {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let times_and_distances = get_times_and_distances_from_lines(&lines);

    times_and_distances
        .iter()
        .map(determine_number_of_ways_to_win)
        .product::<u64>()
}

fn part2_solve(input: &str) -> u64 {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let times_and_distances = get_times_and_distances_from_lines_without_spaces(&lines);

    // TODO: Could move from the left to find min, could move from right to find max.. then the answer is max - min
    times_and_distances
        .iter()
        .map(determine_number_of_ways_to_win)
        .product::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solve() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(288, part1_solve(&input));
    }

    #[test]
    fn test_part2_solve() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(71503, part2_solve(&input));
    }
}
