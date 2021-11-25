use std::fs;
use std::str::Lines;

fn part_to_int(part: &str) -> usize {
    return part.to_string().parse::<usize>().unwrap();
}

fn part1_answer(lines: Lines) -> usize {
    let mut total: usize = 0;

    for line in lines {
        let parts: Vec<&str> = line.split("x").collect();
        let length = part_to_int(parts.get(0).expect("Unable to read length"));
        let width = part_to_int(parts.get(1).expect("Unable to read width"));
        let height = part_to_int(parts.get(2).expect("Unable to read height"));

        let sides = [length * width, width * height, height * length];
        let smallest_side = sides
            .iter()
            .min()
            .expect("Unable to compute smallest side.");

        let area =
            (2 * length * width) + (2 * width * height) + (2 * height * length) + smallest_side;
        total += area;
    }

    total
}

fn part2_answer(lines: Lines) -> usize {
    let mut total: usize = 0;

    for line in lines {
        let parts: Vec<&str> = line.split("x").collect();
        let length = part_to_int(parts.get(0).expect("Unable to read length"));
        let width = part_to_int(parts.get(1).expect("Unable to read width"));
        let height = part_to_int(parts.get(2).expect("Unable to read height"));

        let mut sides = vec![length, width, height];
        sides.sort();

        let smallest_side = sides.get(0).expect("Unable to get first side.");
        let second_smallest_side = sides.get(1).expect("Unable to get second side.");

        let perimeter_length = (smallest_side * 2) + (second_smallest_side * 2);
        let bow_length = length * width * height;

        total += perimeter_length + bow_length;
    }

    total
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open input file.");
    let lines = input.trim().lines();

    println!("Part 1 answer: {}", part1_answer(lines.clone()));

    println!("Part 2 answer: {}", part2_answer(lines.clone()));
}
