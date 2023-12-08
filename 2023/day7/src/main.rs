use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1_solve(&input));
    println!("Part 2: {}", part2_solve(&input));
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    JOKER,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type Hand<'a> = Vec<&'a Card>;
type Bid = u64;

fn determine_hand_type(hand: &Hand) -> HandType {
    let mut card_counts: HashMap<&Card, u64> = HashMap::new();
    for card in hand {
        let count = card_counts.get(card).unwrap_or(&0);
        card_counts.insert(card, count + 1);
    }

    let max_count = card_counts.values().max().unwrap();
    let min_count = card_counts.values().min().unwrap();
    match max_count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => match min_count {
            2 => HandType::FullHouse,
            _ => HandType::ThreeOfAKind,
        },
        2 => match card_counts.values().filter(|v| **v == 2).count() {
            2 => HandType::TwoPair,
            _ => HandType::OnePair,
        },
        _ => HandType::HighCard,
    }
}

fn parse_cards(raw_hand: String, cards_map: &HashMap<char, Card>) -> Hand {
    raw_hand
        .chars()
        .map(|c| cards_map.get(&c).unwrap())
        .collect()
}

fn part1_solve(input: &str) -> u64 {
    let raw_card_to_card: HashMap<char, Card> = HashMap::from([
        ('2', Card::TWO),
        ('3', Card::THREE),
        ('4', Card::FOUR),
        ('5', Card::FIVE),
        ('6', Card::SIX),
        ('7', Card::SEVEN),
        ('8', Card::EIGHT),
        ('9', Card::NINE),
        ('T', Card::TEN),
        ('J', Card::JACK),
        ('Q', Card::QUEEN),
        ('K', Card::KING),
        ('A', Card::ACE),
    ]);

    let hands_with_bids: Vec<(Hand, Bid)> = input
        .trim()
        .split('\n')
        .map(|line| {
            let mut raw_parts = line.split(' ');
            let hand = parse_cards(String::from(raw_parts.next().unwrap()), &raw_card_to_card);
            let bid = raw_parts.next().unwrap().parse::<u64>().unwrap();
            (hand, bid)
        })
        .collect();

    let mut scored: Vec<(HandType, &Hand, Bid)> = hands_with_bids
        .iter()
        .map(|(hand, bid)| {
            let hand_type = determine_hand_type(&hand);
            (hand_type, hand, *bid)
        })
        .collect();

    scored.sort_by(|a, b| {
        // Return the higher value hand
        if a.0.eq(&b.0) {
            // Equal hands, so compare card values from left to right
            for i in 0..a.1.len() {
                let a_card = a.1.get(i).unwrap();
                let b_card = b.1.get(i).unwrap();
                if a_card.eq(b_card) {
                    continue;
                }
                match a_card.cmp(b_card) {
                    Ordering::Equal => {
                        continue;
                    }
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                }
            }
            return Ordering::Equal;
        }
        // Otherwise, compare them and return better hand
        match a.0.gt(&b.0) {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    });

    scored
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u64 + 1) * hand.2)
        .sum()
}

fn part2_solve(input: &str) -> u64 {
    let raw_card_to_card: HashMap<char, Card> = HashMap::from([
        ('J', Card::JOKER),
        ('2', Card::TWO),
        ('3', Card::THREE),
        ('4', Card::FOUR),
        ('5', Card::FIVE),
        ('6', Card::SIX),
        ('7', Card::SEVEN),
        ('8', Card::EIGHT),
        ('9', Card::NINE),
        ('T', Card::TEN),
        ('Q', Card::QUEEN),
        ('K', Card::KING),
        ('A', Card::ACE),
    ]);

    let mut best_cards: Vec<&Card> = raw_card_to_card.values().collect();
    best_cards.sort();

    let hands_with_bids: Vec<(Hand, Bid)> = input
        .trim()
        .split('\n')
        .map(|line| {
            let mut raw_parts = line.split(' ');
            let hand = parse_cards(String::from(raw_parts.next().unwrap()), &raw_card_to_card);
            let bid = raw_parts.next().unwrap().parse::<u64>().unwrap();
            (hand, bid)
        })
        .collect();

    let mut scored: Vec<(HandType, Hand, Hand, Bid)> = hands_with_bids
        .iter()
        .map(|(original_hand, bid)| {
            let mut possible_hands: Vec<(Hand, Hand, HandType)> = vec![(
                original_hand.clone(),
                original_hand.clone(),
                determine_hand_type(&original_hand),
            )];
            println!("===");
            println!(
                "For hand {:?} which is {:?}..",
                original_hand,
                determine_hand_type(&original_hand)
            );

            // For each non-Joker, determine if replacing jokers makes things better
            // FIXME: Handle two-pair?  I think these should always be won-over by three of a kind?
            if original_hand.contains(&&Card::JOKER) {
                for card_to_replace_with in &best_cards {
                    if card_to_replace_with.eq(&&Card::JOKER) {
                        continue;
                    }
                    let replaced_hand: Vec<&Card> = original_hand
                        .clone()
                        .iter()
                        .map(|v| match v {
                            Card::JOKER => card_to_replace_with,
                            _ => *v,
                        })
                        .collect();

                    let replaced_hand_type = determine_hand_type(&replaced_hand);
                    possible_hands.push((original_hand.clone(), replaced_hand, replaced_hand_type));
                }

                possible_hands.sort_by(
                    |(a_original_cards, a_replaced_cards, a_hand_type),
                     (b_original_cards, b_replaced_cards, b_hand_type)| {
                        if a_hand_type.eq(&b_hand_type) {
                            // Equal hands, so compare card values from left to right
                            for i in 0..a_replaced_cards.len() {
                                let a_card = a_replaced_cards.get(i).unwrap();
                                let b_card = b_replaced_cards.get(i).unwrap();
                                if a_card.eq(b_card) {
                                    continue;
                                }
                                match a_card.cmp(b_card) {
                                    Ordering::Equal => {
                                        continue;
                                    }
                                    Ordering::Less => {
                                        return Ordering::Less;
                                    }
                                    Ordering::Greater => {
                                        return Ordering::Greater;
                                    }
                                }
                            }
                            return Ordering::Equal;
                        }
                        // Otherwise, compare them and return better hand
                        match a_hand_type.gt(&b_hand_type) {
                            true => Ordering::Greater,
                            false => Ordering::Less,
                        }
                    },
                );
                possible_hands.reverse();

                possible_hands.iter().for_each(
                    |(original_hand, replaced_hand, replaced_hand_type)| {
                        println!(
                            "Generating hand.. {:?} which is {:?}",
                            replaced_hand, replaced_hand_type,
                        );
                    },
                );
            }

            // Get the "best hand" which bubbled to the top
            let best_hand = possible_hands.get(0).unwrap();

            (best_hand.2, best_hand.0.clone(), best_hand.1.clone(), *bid)
        })
        .collect();

    scored.sort_by(|a, b| {
        // Return the higher value hand
        if a.0.eq(&b.0) {
            // Equal hands, so compare card values from left to right
            for i in 0..a.1.len() {
                let a_card = a.1.get(i).unwrap();
                let b_card = b.1.get(i).unwrap();
                if a_card.eq(b_card) {
                    continue;
                }
                match a_card.cmp(b_card) {
                    Ordering::Equal => {
                        continue;
                    }
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                }
            }
            return Ordering::Equal;
        }
        // Otherwise, compare them and return better hand
        match a.0.gt(&b.0) {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    });

    println!("======");
    println!("FINAL SCOREBOARD..");
    scored.iter().enumerate().for_each(
        |(index, (hand_type, original_hand, replaced_hand, bid))| {
            println!(
                "{}: {:?} {:?} -> {:?} with bid {}",
                index + 1,
                hand_type,
                original_hand,
                replaced_hand,
                bid
            );
        },
    );

    // Final rankings..
    scored
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u64 + 1) * hand.3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(6440, part1_solve(&input));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("example1.txt").unwrap();
        assert_eq!(5905, part2_solve(&input));
    }
}
