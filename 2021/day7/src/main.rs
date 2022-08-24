use std::fs;

fn part2_cost_for_distance(dist: usize) -> usize {
    let mut total = 0;
    for x in 1..=dist {
        total += x;
    }
    total
}

fn process(input: &String, part: usize) -> usize {
    let positions: Vec<usize> = input
        .trim()
        .split(",")
        .map(|n| -> usize { n.parse::<usize>().unwrap() })
        .collect();

    let mut min = *positions.get(0).unwrap();
    let mut max = *positions.get(0).unwrap();

    for pos in &positions[..] {
        if *pos < min {
            min = *pos;
        }
        if *pos > max {
            max = *pos;
        }
    }

    let mut smallest_gas = std::usize::MAX;

    for potential_position in min..=max {
        let mut total_gas_for_position = 0;
        for pos in &positions[..] {
            total_gas_for_position += match part {
                1 => (*pos).abs_diff(potential_position),
                2 => part2_cost_for_distance((*pos).abs_diff(potential_position)),
                _ => panic!("Invalid part"),
            }
        }
        if total_gas_for_position < smallest_gas {
            smallest_gas = total_gas_for_position;
        }
    }

    smallest_gas
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open example text.");
    println!("Day 1: {}", process(&input, 1));
    println!("Day 2: {}", process(&input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts_work() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(process(&input, 1), 37);

        assert_eq!(part2_cost_for_distance(1), 1);
        assert_eq!(part2_cost_for_distance(2), 3);
        assert_eq!(part2_cost_for_distance(3), 6);

        assert_eq!(process(&input, 2), 168);
    }
}
