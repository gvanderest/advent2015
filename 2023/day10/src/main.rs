use std::{
    collections::{HashMap, HashSet},
    fs,
    mem::replace,
};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
struct PipeSetting {
    openings: HashSet<Direction>,
}

/**
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
 */
const START_SYMBOL: char = 'S';
const GROUND_SYMBOL: char = '.';

// x = grow east
// y = grow south
type Coord = (usize, usize);
type CoordOffset = (i32, i32);
type Map = Vec<Vec<char>>;

fn load_map_from_input(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn find_map_start_location(map: &Map) -> Coord {
    // FIXME: Could figure coordinates out while loading map instead
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == START_SYMBOL {
                return (x, y);
            }
        }
    }
    panic!("Did not find starting location in map");
}

fn part1_solve(input: &str) -> u64 {
    // For each symbol, explain the openings
    let symbol_to_pipe = HashMap::from([
        (
            '|',
            PipeSetting {
                openings: HashSet::from([Direction::North, Direction::South]),
            },
        ),
        (
            '-',
            PipeSetting {
                openings: HashSet::from([Direction::West, Direction::East]),
            },
        ),
        (
            'L',
            PipeSetting {
                openings: HashSet::from([Direction::North, Direction::East]),
            },
        ),
        (
            'J',
            PipeSetting {
                openings: HashSet::from([Direction::West, Direction::North]),
            },
        ),
        (
            '7',
            PipeSetting {
                openings: HashSet::from([Direction::West, Direction::South]),
            },
        ),
        (
            'F',
            PipeSetting {
                openings: HashSet::from([Direction::East, Direction::South]),
            },
        ),
        (
            START_SYMBOL,
            PipeSetting {
                openings: HashSet::from([
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]),
            },
        ),
        (
            GROUND_SYMBOL,
            PipeSetting {
                openings: HashSet::from([]),
            },
        ),
    ]);

    let direction_opposites: HashMap<Direction, Direction> = HashMap::from([
        (Direction::North, Direction::South),
        (Direction::East, Direction::West),
        (Direction::South, Direction::North),
        (Direction::West, Direction::East),
    ]);

    let direction_to_offsets: HashMap<Direction, CoordOffset> = HashMap::from([
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
    ]);

    // Load entire map into a two dimension vec..
    let map: Vec<Vec<char>> = load_map_from_input(input);

    // Collect limits for guarding later
    let max_y = (map.len() - 1) as i32;
    let max_x = (map.first().unwrap().len() - 1) as i32;

    // From the starting symbol.. Determine by scanning around to find which legal pipe is next based on symbol openings
    let start_coords = find_map_start_location(&map);

    // Current position..
    let mut current_coords = start_coords;
    let mut entered_from: Option<Direction> = None;

    // Create a hash set for storing coordinates we've visited
    let mut coords_visited: HashSet<Coord> = HashSet::from([]);

    // Scan based on available openings to find the next pipe
    'next_coords: loop {
        coords_visited.insert(current_coords);

        let current_symbol = map
            .get(current_coords.1)
            .unwrap()
            .get(current_coords.0)
            .unwrap();
        let current_pipe = symbol_to_pipe.get(current_symbol).unwrap();
        let directions_to_check = &current_pipe.openings;

        'next_direction: for direction in directions_to_check {
            // Filter out the way you just came from
            if let Some(entered_from_direction) = &entered_from {
                // println!(
                //     "Comparing entered_from_direction={:?}",
                //     entered_from_direction
                // );
                if entered_from_direction == direction {
                    // println!("Skipping direction because we just came from that way");
                    continue 'next_direction;
                }
            }

            let offset = direction_to_offsets.get(direction).unwrap();
            // println!(
            //     "From {:?} ({}), checking {:?} offset {:?}..",
            //     current_coords, current_symbol, direction, offset
            // );
            let opposite_direction = direction_opposites.get(direction).unwrap();

            let (current_x, current_y) = current_coords;
            let (x_offset, y_offset) = offset;
            let new_coords = (current_x as i32 + x_offset, current_y as i32 + y_offset);

            // Off the map in the negatives or positives
            if new_coords.0 < 0 || new_coords.0 > max_x || new_coords.1 < 0 || new_coords.1 > max_y
            {
                // println!("{:?} would go off the map", new_coords);
                continue;
            }

            let next_coords: Coord = (new_coords.0 as usize, new_coords.1 as usize);
            let next_symbol = map.get(next_coords.1).unwrap().get(next_coords.0).unwrap();
            let next_pipe = symbol_to_pipe.get(next_symbol).unwrap();
            // println!("New coords would be.. {:?} ({})", next_coords, next_symbol);

            // If this is the start again, end it all
            if *next_symbol == START_SYMBOL {
                // TODO: Figure out what we do here
                // println!("BACK TO START!");
                break 'next_coords;
            }

            // If we've already visited this coordinate, skip it
            if coords_visited.contains(&next_coords) {
                // println!("ALREADY VISITED");
                continue 'next_direction;
            }

            // Verify the next symbol supports the opposite direction (the way we came from) or skip it..
            if !next_pipe.openings.contains(opposite_direction) {
                // println!(
                //     "NOT A VALID PIPE, {} does not support opposite direction {:?}",
                //     next_symbol, opposite_direction
                // );
                continue 'next_direction;
            }

            // If all matches up, store the next coords and keep going
            // println!(
            //     "MOVING {:?} TO {:?} ({})",
            //     direction, next_coords, next_symbol,
            // );
            current_coords = next_coords;
            entered_from = Some(*opposite_direction);
            continue 'next_coords;
        }
    }

    coords_visited.len() as u64 / 2
}

fn part2_solve(input: &str) -> u64 {
    // For each symbol, explain the openings
    let symbol_to_pipe = HashMap::from([
        (
            '|',
            PipeSetting {
                openings: HashSet::from([Direction::North, Direction::South]),
            },
        ),
        (
            '-',
            PipeSetting {
                openings: HashSet::from([Direction::West, Direction::East]),
            },
        ),
        (
            'L',
            PipeSetting {
                openings: HashSet::from([Direction::North, Direction::East]),
            },
        ),
        (
            'J',
            PipeSetting {
                openings: HashSet::from([Direction::West, Direction::North]),
            },
        ),
        (
            '7',
            PipeSetting {
                openings: HashSet::from([Direction::West, Direction::South]),
            },
        ),
        (
            'F',
            PipeSetting {
                openings: HashSet::from([Direction::East, Direction::South]),
            },
        ),
        (
            START_SYMBOL,
            PipeSetting {
                openings: HashSet::from([
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]),
            },
        ),
        (
            GROUND_SYMBOL,
            PipeSetting {
                openings: HashSet::from([]),
            },
        ),
    ]);

    let direction_opposites: HashMap<Direction, Direction> = HashMap::from([
        (Direction::North, Direction::South),
        (Direction::East, Direction::West),
        (Direction::South, Direction::North),
        (Direction::West, Direction::East),
    ]);

    let direction_to_offsets: HashMap<Direction, CoordOffset> = HashMap::from([
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
    ]);

    // Load entire map into a two dimension vec..
    let mut map: Vec<Vec<char>> = load_map_from_input(input);

    // Collect limits for guarding later
    let max_y = (map.len() - 1) as i32;
    let max_x = (map.first().unwrap().len() - 1) as i32;

    // From the starting symbol.. Determine by scanning around to find which legal pipe is next based on symbol openings
    let start_coords = find_map_start_location(&map);

    // Current position..
    let mut current_coords = start_coords;
    let mut entered_from: Option<Direction> = None;

    // Create a hash set for storing coordinates we've visited
    let mut coords_visited: HashSet<Coord> = HashSet::from([]);

    // Scan based on available openings to find the next pipe
    'next_coords: loop {
        coords_visited.insert(current_coords);

        let current_symbol = map
            .get(current_coords.1)
            .unwrap()
            .get(current_coords.0)
            .unwrap();
        let current_pipe = symbol_to_pipe.get(current_symbol).unwrap();
        let directions_to_check = &current_pipe.openings;

        'next_direction: for direction in directions_to_check {
            // Filter out the way you just came from
            if let Some(entered_from_direction) = &entered_from {
                // println!(
                //     "Comparing entered_from_direction={:?}",
                //     entered_from_direction
                // );
                if entered_from_direction == direction {
                    // println!("Skipping direction because we just came from that way");
                    continue 'next_direction;
                }
            }

            let offset = direction_to_offsets.get(direction).unwrap();
            // println!(
            //     "From {:?} ({}), checking {:?} offset {:?}..",
            //     current_coords, current_symbol, direction, offset
            // );
            let opposite_direction = direction_opposites.get(direction).unwrap();

            let (current_x, current_y) = current_coords;
            let (x_offset, y_offset) = offset;
            let new_coords = (current_x as i32 + x_offset, current_y as i32 + y_offset);

            // Off the map in the negatives or positives
            if new_coords.0 < 0 || new_coords.0 > max_x || new_coords.1 < 0 || new_coords.1 > max_y
            {
                // println!("{:?} would go off the map", new_coords);
                continue;
            }

            let next_coords: Coord = (new_coords.0 as usize, new_coords.1 as usize);
            let next_symbol = map.get(next_coords.1).unwrap().get(next_coords.0).unwrap();
            let next_pipe = symbol_to_pipe.get(next_symbol).unwrap();
            // println!("New coords would be.. {:?} ({})", next_coords, next_symbol);

            // If this is the start again, end it all
            if *next_symbol == START_SYMBOL {
                // TODO: Figure out what we do here
                // println!("BACK TO START!");
                break 'next_coords;
            }

            // If we've already visited this coordinate, skip it
            if coords_visited.contains(&next_coords) {
                // println!("ALREADY VISITED");
                continue 'next_direction;
            }

            // Verify the next symbol supports the opposite direction (the way we came from) or skip it..
            if !next_pipe.openings.contains(opposite_direction) {
                // println!(
                //     "NOT A VALID PIPE, {} does not support opposite direction {:?}",
                //     next_symbol, opposite_direction
                // );
                continue 'next_direction;
            }

            // If all matches up, store the next coords and keep going
            // println!(
            //     "MOVING {:?} TO {:?} ({})",
            //     direction, next_coords, next_symbol,
            // );
            current_coords = next_coords;
            entered_from = Some(*opposite_direction);
            continue 'next_coords;
        }
    }

    // Gather all coords we don't know the state of..
    let mut unknown_coords: HashSet<Coord> = HashSet::new();
    for (y, line) in map.iter().enumerate() {
        for (x, _symbol) in line.iter().enumerate() {
            let coord = (x, y);
            if !coords_visited.contains(&coord) {
                unknown_coords.insert(coord);
            }
        }
    }

    let all_directions = HashSet::from([
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]);

    // Replace the start symbol with a valid pipe shape
    // println!("DETERMINING START SYMBOL");
    let mut valid_openings = HashSet::new();
    for direction in &all_directions {
        let direction_offset = direction_to_offsets.get(direction).unwrap();
        let new_coord = (
            start_coords.0 as i32 + direction_offset.0,
            start_coords.1 as i32 + direction_offset.1,
        );
        if new_coord.0 < 0 || new_coord.0 > max_x || new_coord.1 < 0 || new_coord.1 > max_y {
            continue;
        }
        let next_coord = (new_coord.0 as usize, new_coord.1 as usize);
        let opposite_direction = direction_opposites.get(direction).unwrap();
        let next_symbol = map.get(next_coord.1).unwrap().get(next_coord.0).unwrap();
        let next_pipe = symbol_to_pipe.get(next_symbol).unwrap();

        // println!(
        //     "Checking {:?}.. comparing {:?} to {:?}",
        //     direction, opposite_direction, next_pipe.openings
        // );
        if next_pipe.openings.contains(opposite_direction) {
            // println!(
            //     "This next pipe {:?} ({}) supports {:?} entry",
            //     next_coord, next_symbol, opposite_direction
            // );
            valid_openings.insert(*direction);
        }
    }
    // println!("Valid openings.. {:?}", valid_openings);

    let matching_symbol = *symbol_to_pipe
        .iter()
        .find(|(_symbol, pipe)| pipe.openings.eq(&valid_openings))
        .unwrap()
        .0;

    let start_symbol = matching_symbol;
    let start_line = map.get_mut(start_coords.1).unwrap();
    let _got = replace(&mut start_line[start_coords.0], start_symbol);
    // println!("Start symbol is a {:?}", start_symbol);

    // 3x3 grids of booleans.. true = pipe, false = ground
    let symbol_to_grid: HashMap<char, [[bool; 3]; 3]> = HashMap::from([
        (
            '|',
            [
                [false, true, false],
                [false, true, false],
                [false, true, false],
            ],
        ),
        (
            '-',
            [
                [false, false, false],
                [true, true, true],
                [false, false, false],
            ],
        ),
        (
            'L',
            [
                [false, true, false],
                [false, true, true],
                [false, false, false],
            ],
        ),
        (
            'J',
            [
                [false, true, false],
                [true, true, false],
                [false, false, false],
            ],
        ),
        (
            '7',
            [
                [false, false, false],
                [true, true, false],
                [false, true, false],
            ],
        ),
        (
            'F',
            [
                [false, false, false],
                [false, true, true],
                [false, true, false],
            ],
        ),
        (
            '.',
            [
                [false, false, false],
                [false, false, false],
                [false, false, false],
            ],
        ),
    ]);

    // Grow the map by a factor of 3, giving 3x3 grids to fill
    const GRID_FACTOR: usize = 3;
    let mut big_map: Vec<Vec<char>> = Vec::new();
    for _i in 0..map.len() * GRID_FACTOR {
        big_map.push(Vec::new());
    }

    for (y, line) in map.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            let cell_was_visited = coords_visited.contains(&(x, y));
            let grid = symbol_to_grid.get(symbol).unwrap();
            for (grid_y, grid_line) in grid.iter().enumerate() {
                let big_map_y = (y * GRID_FACTOR) + grid_y;
                let big_map_line = &mut big_map[big_map_y];
                for is_wall in grid_line.iter() {
                    big_map_line.push(match cell_was_visited {
                        false => ' ',
                        true => match is_wall {
                            true => 'X',
                            false => ' ',
                        },
                    });
                }
            }
        }
    }

    // TODO: Starting from 0,0 (which is guaranteed to be ground now.. start filling all ground that can be touched in adjacent squares
    let offsets: Vec<(i32, i32)> = Vec::from([(0, -1), (1, 0), (0, 1), (-1, 0)]);
    let mut next_coords: Vec<Coord> = vec![(0, 0)];

    let big_map_max_y = big_map.len() - 1;
    let big_map_max_x = big_map[0].len() - 1;

    while let Some(current_coord) = next_coords.pop() {
        for offset in &offsets {
            let new_coord = (
                current_coord.0 as i32 + offset.0,
                current_coord.1 as i32 + offset.1,
            );
            if new_coord.0 < 0
                || new_coord.0 as usize > big_map_max_x
                || new_coord.1 < 0
                || new_coord.1 as usize > big_map_max_y
            {
                continue;
            }
            let next_coord = (new_coord.0 as usize, new_coord.1 as usize);
            let next_symbol = big_map[next_coord.1][next_coord.0];
            if next_symbol == ' ' {
                let _got = replace(&mut big_map[next_coord.1][next_coord.0], 'O');
                next_coords.push(next_coord);
            }
        }
    }

    // Debug the map and show it off
    // for line in &big_map {
    //     for col in line {
    //         print!("{}", col);
    //     }
    //     println!();
    // }

    // TODO: Iterate over every 3x3 grid and mark it as outside if it's completely marked.. and inside if it's completely unmarked
    // .. anything in between is a "pipe" but allowed for transporting "outside goo"
    let mut insides = 0;
    for y in 0..map.len() {
        'next_cell: for x in 0..map[0].len() {
            for grid_y in 0..GRID_FACTOR {
                for grid_x in 0..GRID_FACTOR {
                    let big_map_y = (y * GRID_FACTOR) + grid_y;
                    let big_map_x = (x * GRID_FACTOR) + grid_x;
                    if big_map[big_map_y][big_map_x] != ' ' {
                        continue 'next_cell;
                    }
                }
            }
            insides += 1;
        }
    }

    insides
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
        assert_eq!(4, part1_solve(&input));
        let input = fs::read_to_string("example2.txt").unwrap();
        assert_eq!(8, part1_solve(&input));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example3.txt").unwrap();
        assert_eq!(8, part2_solve(&input));
        let input = fs::read_to_string("example4.txt").unwrap();
        assert_eq!(10, part2_solve(&input));
    }
}
