use std::{collections::HashSet, fs};

// add up all the part numbers
// any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum
/** Example layout:
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
 */
const EMPTY_SPOT_SYMBOL: char = '.';

fn part1_solve(input: &str) -> i32 {
    let lines = input.trim().split('\n');

    let mut sum = 0;

    // Make an x/y grid of coordinates
    // x increasing = right
    // y increasing = down
    // because we're starting in the top left in terms of reading file
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (y, line) in lines.enumerate() {
        for symbol in line.chars() {
            match grid.get_mut(y) {
                Some(row) => {
                    row.push(symbol);
                }
                None => {
                    grid.push(vec![symbol]);
                }
            };
        }
    }

    let max_y = grid.len() as i32;
    let max_x = grid.first().unwrap().len() as i32;

    for y in 0..max_y {
        let mut current_number = 0;
        let mut is_part_number = false;

        for x in 0..max_x {
            let symbol = grid.get(y as usize).unwrap().get(x as usize).unwrap();

            // For each number you're parsing, determine if there's a symbol nearby
            if symbol.is_numeric() {
                'symbol_search: for y_modifier in -1..=1 {
                    for x_modifier in -1..=1 {
                        let modified_y = y + y_modifier;
                        if modified_y < 0 {
                            continue;
                        }
                        let search_row = grid.get(modified_y as usize);
                        match search_row {
                            None => continue,
                            Some(row) => {
                                let modified_x = x + x_modifier;
                                if modified_x < 0 {
                                    continue;
                                }
                                let search_col = row.get(modified_x as usize);
                                match search_col {
                                    None => continue,
                                    Some(nearby_symbol) => {
                                        if nearby_symbol.is_numeric()
                                            || *nearby_symbol == EMPTY_SPOT_SYMBOL
                                        {
                                            continue;
                                        } else {
                                            is_part_number = true;
                                            break 'symbol_search;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if symbol.is_numeric() {
                println!("digit={}, part_number={}", symbol, is_part_number);
            }

            if symbol.is_numeric() {
                // Parse and append any digits found
                let parsed_digit = symbol.to_string().parse::<i32>().unwrap();
                current_number *= 10;
                current_number += parsed_digit;
            } else {
                if is_part_number {
                    // Otherwise, if we had a part number.. add it to the sum and reset state
                    sum += current_number;
                    println!("part_number={}", current_number);
                }
                current_number = 0;
                is_part_number = false;
            }

            // If you hit a not-number and are mid-number.. sum it if you were in a happy spot
            // Handle newlines accordingly
        }

        if is_part_number {
            println!("endline part_number={}", current_number);
            sum += current_number;
        }
    }

    sum
}

const GEAR_SYMBOL: char = '*';

fn part2_solve(input: &String) -> i32 {
    let lines = input.trim().split('\n');

    let mut sum = 0;

    // Find all the asterisk coordinates as we go..
    let mut gear_coords: Vec<(usize, usize)> = Vec::new();

    // Make an x/y grid of coordinates.. same as part 1
    // x increasing = right
    // y increasing = down
    // because we're starting in the top left in terms of reading file
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (y, line) in lines.enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol == GEAR_SYMBOL {
                gear_coords.push((x, y));
            }

            match grid.get_mut(y) {
                Some(row) => {
                    row.push(symbol);
                }
                None => {
                    grid.push(vec![symbol]);
                }
            };
        }
    }

    for (x, y) in gear_coords {
        // Location where numbers start around the gear
        let mut gear_number_roots: HashSet<(usize, usize)> = HashSet::new();

        // For each asterisk, look around it to find numbers
        for x_modifier in -1..=1 {
            for y_modifier in -1..=1 {
                // Skip if it's the (0,0) modifiers, because that's the gear itself
                if x_modifier == 0 && y_modifier == 0 {
                    continue;
                }

                let modified_x = (x as i32) + x_modifier;
                let modified_y = (y as i32) + y_modifier;

                if modified_x < 0 || modified_y < 0 {
                    continue;
                }

                let found_symbol = *grid
                    .get(modified_y as usize)
                    .unwrap()
                    .get(modified_x as usize)
                    .unwrap();

                println!(
                    "gear_coord=({}, {}), offset=({}, {}), found_symbol={}",
                    x, y, x_modifier, y_modifier, found_symbol
                );

                if found_symbol.is_numeric() {
                    println!("it's a number!");
                    // If numbers are found, try to find the root coordinates (left-most) for that number
                    let mut next_x = modified_x;
                    loop {
                        let next_symbol = *grid
                            .get(modified_y as usize)
                            .unwrap()
                            .get(next_x as usize)
                            .unwrap();
                        println!("next_symbol={}", next_symbol);

                        if !next_symbol.is_numeric() {
                            gear_number_roots.insert((next_x as usize + 1, modified_y as usize));
                            break;
                        } else if next_x == 0 {
                            // Collect the roots until we find all the unique entries
                            gear_number_roots.insert((next_x as usize, modified_y as usize));
                            break;
                        }

                        // Move over and try again
                        next_x -= 1;
                    }
                }
            }
        }

        // If count of unique roots is not two, skip
        println!("gear_number_roots count={}", gear_number_roots.len());
        if gear_number_roots.len() != 2 {
            continue;
        }

        let mut gear_numbers: Vec<i32> = Vec::new();
        for (mut x, y) in gear_number_roots {
            // For those roots, go find the real numbers
            let mut value = 0;
            loop {
                let symbol = grid.get(y).unwrap().get(x);
                match symbol {
                    None => {
                        break;
                    }
                    Some(symbol) => {
                        if symbol.is_numeric() {
                            value *= 10;
                            value += symbol.to_string().parse::<i32>().unwrap();
                        } else {
                            break;
                        }

                        x += 1;
                    }
                }
            }
            gear_numbers.push(value);
        }
        println!("GEAR NUMBERS {:?}", gear_numbers);

        // Multiply those numbers
        let first = gear_numbers.pop().unwrap();
        let second = gear_numbers.pop().unwrap();
        let gear_ratio = first * second;

        // Add the gear ratios up
        sum += gear_ratio;
    }

    sum
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
    fn part1_test() {
        let input = fs::read_to_string("part1_example.txt").unwrap();
        assert_eq!(4361, part1_solve(&input));
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("part1_example.txt").unwrap();
        assert_eq!(467835, part2_solve(&input));
    }
}
