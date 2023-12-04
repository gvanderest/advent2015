use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    println!("Part 2: {}", part2_solve(&input));
}

fn convert_numbers_to_sets(input: &str) -> HashSet<i32> {
    HashSet::from_iter(
        input
            .split_whitespace()
            .map(|raw_num| raw_num.parse::<i32>().unwrap()),
    )
}

fn part1_solve(input: &str) -> i32 {
    input
        .trim()
        .split('\n')
        .map(|line| {
            // Read each line and parse the obtained and winning numbers
            let mut label_vs_numbers = line.split(": ");
            let card_label = label_vs_numbers.next().unwrap();
            let numbers_sections = label_vs_numbers.next().unwrap();

            (card_label, numbers_sections)
        })
        .map(|(_card_label, numbers_sections)| {
            // Store the numbers in HashSets
            let mut number_sets = numbers_sections.split(" | ").map(convert_numbers_to_sets);
            (number_sets.next().unwrap(), number_sets.next().unwrap())
                as (HashSet<i32>, HashSet<i32>)
        })
        .map(|(winning_numbers, my_numbers)| {
            // Perform intersection to determine how many of mine are winning
            let my_winning_numbers: Vec<&i32> = my_numbers.intersection(&winning_numbers).collect();
            my_winning_numbers.len() as i32
        })
        .map(|winning_number_count| {
            println!("Winning number count: {}", winning_number_count);
            // Do a 2^(winners-1) and add to sum
            if winning_number_count == 0 {
                0
            } else {
                i32::pow(2, winning_number_count as u32 - 1)
            }
        })
        .map(|score| {
            println!("Scoooore: {}", score);
            score
        })
        .sum()
}

type Card = (i32, i32);

fn part2_solve(input: &str) -> i32 {
    // Load all the cards into a Vector for easy lookup by index
    let cards_list: Vec<Card> = input
        .trim()
        .split('\n')
        .map(|line| {
            // Read each line and parse the obtained and winning numbers
            let mut label_vs_numbers = line.split(": ");
            let card_label = label_vs_numbers.next().unwrap();
            let numbers_sections = label_vs_numbers.next().unwrap();

            (card_label, numbers_sections)
        })
        .map(|(card_label, numbers_section)| {
            println!("BAD LABEL? {}", card_label);
            (
                card_label
                    .replace("Card ", "")
                    .trim()
                    .parse::<i32>()
                    .unwrap(),
                numbers_section,
            )
        })
        .map(|(card_id, numbers_sections)| {
            // Store the numbers in HashSets
            let mut number_sets = numbers_sections.split(" | ").map(convert_numbers_to_sets);
            (
                card_id,
                number_sets.next().unwrap(),
                number_sets.next().unwrap(),
            ) as (i32, HashSet<i32>, HashSet<i32>)
        })
        .map(|(card_id, winning_numbers, my_numbers)| {
            // Perform intersection to determine how many of mine are winning
            let my_winning_numbers: Vec<&i32> = my_numbers.intersection(&winning_numbers).collect();
            let winner_count = my_winning_numbers.len() as i32;
            (card_id, winner_count)
        })
        .collect();

    let mut card_counts: Vec<i32> = cards_list.iter().map(|_| 1).collect();
    for (card_id, winning_number_count) in &cards_list {
        // For each card..
        // based on how many winning numbers it has, increase the counts of the subsequent cards in the list
        if *winning_number_count > 0 {
            let card_start_index = *card_id as usize;
            let card_end_index = card_start_index + *winning_number_count as usize;

            let quantity_of_current_card = card_counts.get(*card_id as usize - 1).unwrap().clone();
            println!(
                "card_id={} wins={} so indices={}..{} to increment by {}",
                card_id,
                winning_number_count,
                card_start_index,
                card_end_index,
                quantity_of_current_card
            );
            for bonus_card_index in card_start_index..card_end_index {
                let bonus_card_existing_count = card_counts.get(bonus_card_index).unwrap();
                let new_value = *bonus_card_existing_count + quantity_of_current_card;
                card_counts[bonus_card_index] = new_value;
            }
        }
    }

    println!("Final results..");
    for (card_index, card_count) in card_counts.iter().enumerate() {
        println!("card_id={}, card_count={}", card_index + 1, card_count);
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("part1_example.txt").unwrap();
        assert_eq!(13, part1_solve(&input));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("part1_example.txt").unwrap();
        assert_eq!(30, part2_solve(&input));
    }
}
