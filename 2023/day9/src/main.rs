use std::fs;

fn part1_solve_problem(problem: &Vec<Vec<i64>>) -> i64 {
    println!("PROBLEM: {:?}", problem);
    // Collector of all the "pyramid" of values
    let mut sets = problem.clone();

    let mut current_set = sets.first().unwrap().clone();

    // While we don't have all zeroes in the current set..
    while !current_set.iter().all(|n| *n == 0) {
        // Calculate the diffs between numbers to form a new set
        let mut new_set = Vec::new();
        for x in 0..current_set.len() - 1 {
            // Look at pairs to determine diffs
            let a = current_set.get(x).unwrap();
            let b = current_set.get(x + 1).unwrap();
            let diff = b - a;
            new_set.push(diff);
        }
        // Append and replace the current set
        println!("Set.. {:?}", new_set);
        sets.push(new_set.clone());
        current_set = new_set;
    }

    // Make a function that goes through the outer vec backwards to add the numbers to the ends
    sets.last_mut().unwrap().push(0);
    for x in (0..sets.len() - 1).rev() {
        let current_set = sets.get(x).unwrap();

        let start_value = current_set.last().unwrap();
        let add_value = sets.get(x + 1).unwrap().last().unwrap();
        let new_value = start_value + add_value;
        println!(
            "{}: start_value={}, add_value={}, new_value={}",
            x, start_value, add_value, new_value
        );

        sets.get_mut(x).unwrap().push(new_value);
    }
    println!("Almost done.. {:?}", sets);

    *sets.first().unwrap().last().unwrap() // Collect the end numbers from the first outer vec
}

fn part1_solve(input: &str) -> i64 {
    // Collect the numbers to start with
    let problems: Vec<Vec<Vec<i64>>> = input
        .trim()
        .split('\n')
        .map(|line| {
            vec![line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()]
        })
        .collect();

    // For each set of numbers, do the diffs between numbers until all zeroes.. Use a vec of vecs
    let answers: Vec<i64> = problems.iter().map(part1_solve_problem).collect();

    answers.iter().sum()
}

fn part2_solve_problem(problem: &Vec<Vec<i64>>) -> i64 {
    println!("PROBLEM: {:?}", problem);
    // Collector of all the "pyramid" of values
    let mut sets = problem.clone();

    let mut current_set = sets.first().unwrap().clone();

    // While we don't have all zeroes in the current set..
    while !current_set.iter().all(|n| *n == 0) {
        // Calculate the diffs between numbers to form a new set
        let mut new_set = Vec::new();
        for x in 0..current_set.len() - 1 {
            // Look at pairs to determine diffs
            let a = current_set.get(x).unwrap();
            let b = current_set.get(x + 1).unwrap();
            let diff = b - a;
            new_set.push(diff);
        }
        // Prepend and replace the current set
        println!("Set.. {:?}", new_set);
        sets.push(new_set.clone());
        current_set = new_set;
    }

    // Make a function that goes through the outer vec backwards to add the numbers to the ends
    sets.last_mut().unwrap().insert(0, 0);
    for x in (0..sets.len() - 1).rev() {
        let current_set = sets.get(x).unwrap();

        let start_value = current_set.first().unwrap();
        let sub_value = sets.get(x + 1).unwrap().first().unwrap();
        let new_value = start_value - sub_value;
        println!(
            "{}: start_value={}, sub_value={}, new_value={}",
            x, start_value, sub_value, new_value
        );

        sets.get_mut(x).unwrap().insert(0, new_value);
    }
    println!("Almost done.. {:?}", sets);

    *sets.first().unwrap().first().unwrap() // Collect the end numbers from the first outer vec
}

fn part2_solve(input: &str) -> i64 {
    // Collect the numbers to start with
    let problems: Vec<Vec<Vec<i64>>> = input
        .trim()
        .split('\n')
        .map(|line| {
            vec![line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()]
        })
        .collect();

    // For each set of numbers, do the diffs between numbers until all zeroes.. Use a vec of vecs
    let answers: Vec<i64> = problems.iter().map(part2_solve_problem).collect();

    answers.iter().sum()
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
        assert_eq!(114, part1_solve(&input));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(2, part2_solve(&input));
    }
}
