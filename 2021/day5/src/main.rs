use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Coords {
    from: Coord,
    to: Coord,
}

fn part1_compute(all_coords: &Vec<Coords>) -> usize {
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();

    all_coords
        .iter()
        // Remove non-vertical/-horizontal entries.
        .filter(|coords| coords.from.x == coords.to.x || coords.from.y == coords.to.y)
        .for_each(|coords| {
            let min_x = min(coords.from.x, coords.to.x);
            let max_x = max(coords.from.x, coords.to.x);
            let min_y = min(coords.from.y, coords.to.y);
            let max_y = max(coords.from.y, coords.to.y);

            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let coord = (x, y);

                    if !grid.contains_key(&coord) {
                        grid.insert(coord, 0);
                    }

                    let coord_count = grid.get(&(x, y)).unwrap();
                    let updated_count = coord_count + 1;

                    grid.insert(coord, updated_count);
                }
            }
        });

    grid.values()
        .filter(|overlaps| **overlaps >= 2)
        .collect::<Vec<&usize>>()
        .len()
}

fn part2_increase_grid_count(grid: &mut HashMap<(usize, usize), usize>, coord: &(usize, usize)) {
    if !grid.contains_key(coord) {
        grid.insert(*coord, 0);
    }

    let coord_count = grid.get(&(coord.0, coord.1)).unwrap();
    let updated_count = coord_count + 1;

    grid.insert(*coord, updated_count);
}

fn part2_compute(all_coords: &Vec<Coords>) -> usize {
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();

    all_coords
        .iter()
        // Remove non-vertical/-horizontal entries.
        .for_each(|coords| {
            let min_x = min(coords.from.x, coords.to.x);
            let max_x = max(coords.from.x, coords.to.x);
            let min_y = min(coords.from.y, coords.to.y);
            let max_y = max(coords.from.y, coords.to.y);

            let diagonal = coords.from.x != coords.to.x && coords.from.y != coords.to.y;

            if diagonal {
                let y_step: i32 = match coords.to.y > coords.from.y {
                    true => 1,
                    false => -1,
                };
                let x_step: i32 = match coords.to.x > coords.from.x {
                    true => 1,
                    false => -1,
                };
                let steps = max_x - min_x;
                for step in 0..=steps {
                    let x = (coords.from.x as i32 + (step as i32 * x_step)) as usize;
                    let y = (coords.from.y as i32 + (step as i32 * y_step)) as usize;
                    let coord = (x, y);
                    part2_increase_grid_count(&mut grid, &coord);
                }
            } else {
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        let coord = (x, y);
                        part2_increase_grid_count(&mut grid, &coord);
                    }
                }
            };
        });

    grid.values()
        .filter(|overlaps| **overlaps >= 2)
        .collect::<Vec<&usize>>()
        .len()
}

/**
 * Convert the raw input text into a list of coordinates.
 */
fn input_to_coords(input: &String) -> Vec<Coords> {
    let lines = input.trim().split("\n").collect::<Vec<&str>>();

    // For each line..
    lines
        .iter()
        .map(|line| {
            // Split by arrow..
            line.split(" -> ")
                // Convert each raw coordinate pair to a vector of usizes.
                .map(|raw_coord_pair| {
                    raw_coord_pair
                        .split(",")
                        .map(|raw_coord| raw_coord.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                // Convert to Coord.
                .map(|coords| Coord {
                    x: coords.get(0).unwrap().clone(),
                    y: coords.get(1).unwrap().clone(),
                })
                .collect()
        })
        // Convert to a Coord object using the two pairs of coordinates..
        .map(|coord_pairs: Vec<Coord>| Coords {
            from: *coord_pairs.get(0).unwrap(),
            to: *coord_pairs.get(1).unwrap(),
        })
        .collect::<Vec<Coords>>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input file.");
    let coords = input_to_coords(&input);
    println!("Part one: {}", part1_compute(&coords));
    println!("Part two: {}", part2_compute(&coords));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_computes_properly() {
        let input = fs::read_to_string("example.txt").expect("Unable to read example file.");
        let coords = input_to_coords(&input);

        assert_eq!(5, part1_compute(&coords));
        assert_eq!(12, part2_compute(&coords));
    }
}
