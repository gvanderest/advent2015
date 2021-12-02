use std::fs;

fn part1_compute(lines: &Vec<&str>) -> isize {
    let mut depth: isize = 0;
    let mut distance: isize = 0;

    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();

        let command = parts.get(0).expect("Unable to find command.");
        let amount = parts
            .get(1)
            .expect("Unable to find amount.")
            .parse::<isize>()
            .unwrap();

        match command.as_ref() {
            "up" => depth -= amount,
            "down" => depth += amount,
            "forward" => distance += amount,
            _ => panic!("Unexpected command: {}", command),
        }
    }

    depth * distance
}

fn part2_compute(lines: &Vec<&str>) -> isize {
    let mut depth: isize = 0;
    let mut distance: isize = 0;
    let mut aim: isize = 0;

    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();

        let command = parts.get(0).expect("Unable to find command.");
        let amount = parts
            .get(1)
            .expect("Unable to find amount.")
            .parse::<isize>()
            .unwrap();

        match command.as_ref() {
            // down X increases your aim by X units.
            "down" => aim += amount,
            // up X decreases your aim by X units.
            "up" => aim -= amount,
            // forward X does two things:
            //     It increases your horizontal position by X units.
            //     It increases your depth by your aim multiplied by X.
            "forward" => {
                distance += amount;
                depth += aim * amount;
            }
            _ => panic!("Unexpected command: {}", command),
        }
    }

    depth * distance
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
    fn test_example_depths() {
        let inputs = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(part1_compute(&inputs), 150);
        assert_eq!(part2_compute(&inputs), 900);
    }
}
