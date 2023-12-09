use std::fs;

fn part1_solve(input: &str) -> u64 {
    0
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
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(0, part1_solve(&input));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example2.txt").unwrap();
        assert_eq!(0, part2_solve(&input));
    }
}
