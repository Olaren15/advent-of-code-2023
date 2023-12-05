use std::collections::HashMap;

advent_of_code::solution!(4);

struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

impl Card {
    fn count_matching_numbers(&self) -> u32 {
        self.drawn_numbers
            .iter()
            .filter(|&drawn_number| {
                self.winning_numbers
                    .iter()
                    .any(|winning| winning == drawn_number)
            })
            .count() as u32
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (card_info, numbers) = line.split_once(':').unwrap();

            let number = card_info
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            let (winning_numbers_info, drawn_numbers_info) = numbers.split_once('|').unwrap();

            Card {
                number,
                winning_numbers: extract_numbers(winning_numbers_info),
                drawn_numbers: extract_numbers(drawn_numbers_info),
            }
        })
        .collect()
}

fn extract_numbers(input: &str) -> Vec<u32> {
    input
        .split(' ')
        .filter_map(|number| number.parse().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_cards(input);

    let total_points = cards
        .iter()
        .map(|card| {
            let matching_number_count = card.count_matching_numbers();

            let mut points = 0;

            if matching_number_count >= 1 {
                points += 1;
            }

            if matching_number_count > 1 {
                let base: u32 = 2;
                points *= base.pow(matching_number_count - 1)
            }

            points
        })
        .sum();

    Some(total_points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_cards(input);

    let mut card_copies: HashMap<u32, u32> = HashMap::new();

    cards.iter().for_each(|card| {
        let matching_numbers_count = card.count_matching_numbers();

        let copies_of_this_card = if let Some(copies) = card_copies.get(&card.number) {
            *copies
        } else {
            0
        };

        if matching_numbers_count > 0 {
            (card.number + 1..=card.number + matching_numbers_count).for_each(|card_number| {
                if let Some(copies) = card_copies.get_mut(&card_number) {
                    *copies += copies_of_this_card + 1;
                } else {
                    card_copies.insert(card_number, copies_of_this_card + 1);
                }
            })
        }
    });

    let card_copies_count: u32 = card_copies.values().sum();

    Some(cards.len() as u32 + card_copies_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
