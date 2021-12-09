use std::{collections::HashMap, fs};

#[derive(Copy, Clone)]
struct BoardSlot {
    value: usize,
    called: bool,
}

struct Board {
    id: usize,
    grid: HashMap<(usize, usize), BoardSlot>,
    value_to_coords: HashMap<usize, (usize, usize)>,
}

fn parse_feed(input: String) -> Vec<usize> {
    input
        .split(",")
        .map(|val| val.parse::<usize>().unwrap())
        .collect()
}

fn parse_board(board_index: usize, raw_board: &str) -> Board {
    // Convert lines/numbers into BoardSlots.
    let row_slots = String::from(raw_board)
        .split("\n")
        .enumerate()
        .map(|(row_index, raw_line)| {
            (
                row_index,
                raw_line
                    .split_whitespace()
                    .map(|raw_number| raw_number.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(row_index, row_values)| {
            (
                row_index,
                row_values
                    .iter()
                    .map(|cell_value| BoardSlot {
                        value: *cell_value,
                        called: false,
                    })
                    .collect(),
            )
        })
        .collect::<Vec<(usize, Vec<BoardSlot>)>>();

    // Populate the Board grid.
    let mut grid = HashMap::new();
    let mut value_to_coords = HashMap::new();
    row_slots.iter().for_each(|(row_index, slots)| {
        slots.iter().enumerate().for_each(|(col_index, slot)| {
            grid.insert((*row_index, col_index), *slot);
            value_to_coords.insert(slot.value, (*row_index, col_index));
        })
    });

    Board {
        id: board_index,
        grid,
        value_to_coords,
    }
}

fn parse_boards(raw_boards: Vec<&str>) -> Vec<Board> {
    raw_boards
        .iter()
        .enumerate()
        .map(|(board_index, raw_board)| parse_board(board_index, raw_board))
        .collect()
}

fn split_input_into_feed_and_boards(input: String) -> (Vec<usize>, Vec<Board>) {
    let mut lines = input.split("\n\n").collect::<Vec<&str>>();
    let feed = parse_feed(String::from(lines.remove(0)));

    let boards = parse_boards(lines);

    (feed, boards)
}
const GRID_SIZE: usize = 5;

fn test_board_wins(board: &Board) -> bool {
    let mut row_matches: HashMap<usize, usize> = HashMap::new();
    let mut col_matches: HashMap<usize, usize> = HashMap::new();

    for ((row, col), cell) in board.grid.iter() {
        if !cell.called {
            continue;
        }
        *row_matches.entry(*row).or_insert(0) += 1;
        *col_matches.entry(*col).or_insert(0) += 1;
    }

    for count in row_matches.values() {
        if *count == GRID_SIZE {
            return true;
        }
    }

    for count in col_matches.values() {
        if *count == GRID_SIZE {
            return true;
        }
    }

    false
}

fn find_winning_boards(boards: &Vec<Board>) -> Vec<&Board> {
    boards
        .iter()
        .filter(|board| test_board_wins(board))
        .collect()
}

fn compute_board_total(board: &Board) -> usize {
    board
        .grid
        .values()
        .map(|slot| {
            slot.value
                * match slot.called {
                    false => 1,
                    true => 0,
                }
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

fn part1_compute(feed: &Vec<usize>, provided_boards: &mut Vec<Board>) -> usize {
    for called_number in feed {
        for board in provided_boards.into_iter() {
            let coords_lookup = board.value_to_coords.get(called_number);

            if coords_lookup.is_none() {
                // Number not in this board.
                continue;
            }
            let coords = coords_lookup.unwrap();
            let slot = board.grid.get_mut(coords).unwrap();
            slot.called = true;
        }

        // Find first winner.
        let winning_boards = find_winning_boards(provided_boards);
        if winning_boards.len() > 0 {
            let winning_board = winning_boards.get(0).unwrap();
            return called_number * compute_board_total(winning_board);
        }
    }

    0
}

fn part2_compute(feed: &Vec<usize>, provided_boards: &mut Vec<Board>) -> usize {
    // Find last winner.
    let mut winning_boards_in_order: Vec<usize> = vec![];

    for called_number in feed {
        for board in provided_boards.into_iter() {
            let coords_lookup = board.value_to_coords.get(called_number);

            if coords_lookup.is_none() {
                // Number not in this board.
                continue;
            }
            let coords = coords_lookup.unwrap();
            let slot = board.grid.get_mut(coords).unwrap();
            slot.called = true;
        }

        let winning_boards = find_winning_boards(provided_boards);
        for board in winning_boards.iter() {
            if !winning_boards_in_order.contains(&board.id) {
                winning_boards_in_order.push(board.id);
            }
        }
        if winning_boards.len() == provided_boards.len() {
            let winning_board_index = winning_boards_in_order.last().unwrap();
            let winning_board = provided_boards.get(*winning_board_index).unwrap();
            return called_number * compute_board_total(winning_board);
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open input text.");
    let (feed, mut boards) = split_input_into_feed_and_boards(input);

    println!("Part one: {}", part1_compute(&feed, &mut boards));
    println!("Part two: {}", part2_compute(&feed, &mut boards));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert_board_to_number_vec(board: &Board) -> Vec<usize> {
        let grid = &board.grid;

        let mut numbers: Vec<usize> = vec![];

        for row in 0..5 {
            for col in 0..5 {
                numbers.push(grid.get(&(row, col)).unwrap().value);
            }
        }

        numbers
    }

    #[test]
    fn feeds_and_boards_are_extracted() {
        let input = fs::read_to_string("example.txt").expect("Unable to open example text.");
        let (feed, boards) = split_input_into_feed_and_boards(input);

        assert_eq!(
            feed,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );

        // TODO: Test a board
        assert_eq!(
            convert_board_to_number_vec(boards.iter().nth(0).unwrap()),
            vec![
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19
            ]
        )
    }

    #[test]
    fn part_1_returns_proper_score() {
        let input = fs::read_to_string("example.txt").expect("Unable to open example text.");
        let (feed, mut boards) = split_input_into_feed_and_boards(input);

        assert_eq!(4512, part1_compute(&feed, &mut boards));
    }

    #[test]
    fn part_2_returns_proper_score() {
        let input = fs::read_to_string("example.txt").expect("Unable to open example text.");
        let (feed, mut boards) = split_input_into_feed_and_boards(input);

        assert_eq!(1924, part2_compute(&feed, &mut boards));
    }
}
