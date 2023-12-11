use std::{collections::HashSet, fs};

type Coord = (i32, i32);

fn determine_shortest_distance((g1, g2): &(Coord, Coord)) -> u64 {
    let width = (g1.0 - g2.0).abs();
    let height = (g1.1 - g2.1).abs();

    width as u64 + height as u64
}

const EMPTY_SPACE: char = '.';

fn expand_map(input: &Vec<Vec<char>>, size: i32) -> Vec<Vec<char>> {
    // For every row that is empty, double the number of them in that spot
    let mut new_map: Vec<Vec<char>> = input
        .iter()
        .flat_map(|old_line| {
            // If every entry in this old line is a period.. put in two
            if old_line.iter().all(|c| *c == EMPTY_SPACE) {
                (0..size).map(|_x| old_line).collect()
            } else {
                vec![old_line]
            }
        })
        .cloned()
        .collect();

    // For every column that is empty, double the number of them in that spot.. including the other rows around it
    let mut empty_columns: HashSet<usize> = HashSet::new();
    for i in 0..input[0].len() {
        if input.iter().all(|line| line[i] == EMPTY_SPACE) {
            empty_columns.insert(i);
        }
    }
    new_map = new_map
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .flat_map(|(col_index, symbol)| {
                    if empty_columns.contains(&col_index) {
                        (0..size).map(|_x| EMPTY_SPACE).collect()
                    } else {
                        vec![*symbol]
                    }
                })
                .collect()
        })
        .collect();

    new_map
}

fn get_empty_space_values(map: &Vec<Vec<char>>) -> (HashSet<usize>, HashSet<usize>) {
    // Collect all empty rows
    let mut empty_rows: HashSet<usize> = HashSet::new();
    for (i, line) in map.iter().enumerate() {
        if line.iter().all(|symbol| *symbol == EMPTY_SPACE) {
            empty_rows.insert(i);
        }
    }

    // Collect all empty cols
    let mut empty_cols: HashSet<usize> = HashSet::new();
    for i in 0..map[0].len() {
        if map.iter().all(|line| line[i] == EMPTY_SPACE) {
            empty_cols.insert(i);
        }
    }

    (empty_cols, empty_rows)
}

// fn print_map(map: &Vec<Vec<char>>) {
//     for line in map.iter() {
//         for char in line {
//             print!("{char}");
//         }
//         println!();
//     }
// }

fn part1_solve(input: &str, size: i32) -> u64 {
    // Load the map into a 2d vec
    let map: Vec<Vec<char>> = input
        .trim()
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let map = expand_map(&map, size);

    // Extract all the galaxy coords into a list
    let mut galaxies: Vec<Coord> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            if *symbol == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }

    // Pair all the galaxies into the possible combos
    let mut galaxy_pairs: Vec<(Coord, Coord)> = Vec::new();
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies[i + 1..].iter() {
            galaxy_pairs.push((*galaxy1, *galaxy2));
        }
    }

    // Determine the shortest path between all galaxies.. (you can go through things)
    let galaxy_distances: Vec<u64> = galaxy_pairs
        .iter()
        .map(determine_shortest_distance)
        .collect();

    // galaxy_distances
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i, distance)| {
    //         let coords = galaxy_pairs[i];
    //         println!("{:?}: {:?}", coords, distance);
    //     });

    // - This can probably just be something simple like .. find the bigger X/Y diff, and counter it per step
    galaxy_distances.iter().sum()
}

fn part2_determine_shortest_distance(
    (g1, g2): &(Coord, Coord),
    (empty_cols, empty_rows): &(HashSet<usize>, HashSet<usize>),
    size: i32,
) -> u64 {
    let smaller_x = i32::min(g1.0, g2.0);
    let larger_x = i32::max(g1.0, g2.0);

    let smaller_y = i32::min(g1.1, g2.1);
    let larger_y = i32::max(g1.1, g2.1);

    let width: u64 = (smaller_x..larger_x)
        .map(|x| {
            if empty_cols.contains(&(x as usize)) {
                size as u64
            } else {
                1
            }
        })
        .sum();
    let height: u64 = (smaller_y..larger_y)
        .map(|x| {
            if empty_rows.contains(&(x as usize)) {
                size as u64
            } else {
                1
            }
        })
        .sum();

    width + height
}

fn part2_solve(input: &str, size: i32) -> u64 {
    // For part two.. rather than expand the universe, identify the boundaries and then determine X/Y distance by adding the size

    // Load the map into a 2d vec
    let map: Vec<Vec<char>> = input
        .trim()
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    // Extract the empty space columns vs rows
    let empty_spaces = get_empty_space_values(&map);

    // Extract all the galaxy coords into a list
    let mut galaxies: Vec<Coord> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            if *symbol == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }

    // Pair all the galaxies into the possible combos
    let mut galaxy_pairs: Vec<(Coord, Coord)> = Vec::new();
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies[i + 1..].iter() {
            galaxy_pairs.push((*galaxy1, *galaxy2));
        }
    }

    // Determine the shortest path between all galaxies.. (you can go through things)
    let galaxy_distances: Vec<u64> = galaxy_pairs
        .iter()
        .map(|pair| part2_determine_shortest_distance(pair, &empty_spaces, size))
        .collect();

    // galaxy_distances
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i, distance)| {
    //         let coords = galaxy_pairs[i];
    //         println!("{:?}: {:?}", coords, distance);
    //     });

    // - This can probably just be something simple like .. find the bigger X/Y diff, and counter it per step
    galaxy_distances.iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input, 2));
    println!("Part 2: {}", part2_solve(&input, 1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(374, part1_solve(&input, 2));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(1030, part2_solve(&input, 10));
        assert_eq!(8410, part2_solve(&input, 100));
    }
}
