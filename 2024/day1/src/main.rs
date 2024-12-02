use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Split;

fn part1(lines: Split<&str>) -> i32 {
    let mut sum = 0;
    let mut num_lists: [Vec<i32>; 2] = [vec![], vec![]];
    for line in lines.into_iter() {
        let mut parts = line.split_whitespace();
        let num1: i32 = str::parse::<i32>(parts.next().unwrap()).unwrap();
        let num2: i32 = str::parse::<i32>(parts.next().unwrap()).unwrap();
        num_lists[0].push(num1);
        num_lists[1].push(num2);
    }

    num_lists[0].sort();
    num_lists[1].sort();

    for (index, num1) in num_lists[0].iter().enumerate() {
        let num2 = num_lists[1].get(index).unwrap();

        let diff = i32::abs(num1 - num2);
        sum += diff;
    }

    sum
}

fn part2(lines: Split<&str>) -> i32 {
    let mut sum = 0;
    let mut num1_list: Vec<i32> = vec![];
    let mut num2_count: HashMap<i32, i32> = HashMap::new();

    for line in lines.into_iter() {
        let mut parts = line.split_whitespace();

        let num1: i32 = str::parse::<i32>(parts.next().unwrap()).unwrap();
        num1_list.push(num1);

        let num2: i32 = str::parse::<i32>(parts.next().unwrap()).unwrap();
        let existing = num2_count.get(&num2).unwrap_or(&0);
        num2_count.insert(num2, existing + 1);
    }

    for num1 in num1_list.iter() {
        let count = num2_count.get(&num1).unwrap_or(&0);

        sum += num1 * count;
    }

    sum
}

fn main() {
    let contents = read_to_string("./input.txt").unwrap();
    let lines = contents.trim_end().split("\n");
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = read_to_string("./example.txt").unwrap();
        let lines = contents.trim_end().split("\n");
        assert_eq!(part1(lines.clone()), 11);
    }

    #[test]
    fn test_part2() {
        let contents = read_to_string("./example.txt").unwrap();
        let lines = contents.trim_end().split("\n");
        assert_eq!(part2(lines.clone()), 31);
    }
}
