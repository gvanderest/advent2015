use std::fs;

fn parse_floor_part_1(input: String) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        floor = floor
            + match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
    }
    floor
}

fn parse_floor_part_2(input: String) -> usize {
    let mut floor: i32 = 0;
    for (index, c) in input.chars().enumerate() {
        floor = floor
            + match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
        if floor < 0 {
            return index + 1;
        }
    }
    panic!("Didn't find a floor.")
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read the input.");

    println!("Part 1: {}", parse_floor_part_1(input.clone()));

    println!("Part 2: {}", parse_floor_part_2(input.clone()));
}
