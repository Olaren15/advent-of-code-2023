use std::cmp::{max, min};
use std::ops::Add;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first = line
                    .chars()
                    .find(char::is_ascii_digit)
                    .map(|char: char| char.to_string())
                    .unwrap_or_default();

                let last = line
                    .chars()
                    .rev()
                    .find(char::is_ascii_digit)
                    .map(|char: char| char.to_string())
                    .unwrap_or_default();

                first.add(&last).parse::<u32>().unwrap_or_default()
            })
            .sum(),
    )
}

struct Number <'data> {
    letters_value: &'data str,
    numeric_value: &'data str,
    int_value: i32,
}

struct NumberMatches {
    first_match_index: Option<usize>,
    last_match_index: Option<usize>,
    value: i32
}

fn get_preffered_match(letter_match: Option<(usize, &str)>, numeric_match: Option<(usize, &str)>, comparator: fn(usize, usize) -> usize) -> Option<usize> {
    match (letter_match, numeric_match) {
        (Some(letter_match), Some(numeric_match)) => Some(comparator(letter_match.0, numeric_match.0)),
        (Some(letter_match), None) => Some(letter_match.0),
        (None, Some(numeric_match)) => Some(numeric_match.0),
        (None, None) => None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let numbers_to_find = vec![
                    Number { letters_value: "zero", numeric_value: "0", int_value: 0},
                    Number { letters_value: "one", numeric_value: "1", int_value: 1},
                    Number { letters_value: "two", numeric_value: "2", int_value: 2},
                    Number { letters_value: "three", numeric_value: "3", int_value: 3},
                    Number { letters_value: "four", numeric_value: "4", int_value: 4},
                    Number { letters_value: "five", numeric_value: "5", int_value: 5},
                    Number { letters_value: "six", numeric_value: "6", int_value: 6},
                    Number { letters_value: "seven", numeric_value: "7", int_value: 7},
                    Number { letters_value: "eight", numeric_value: "8", int_value: 8},
                    Number { letters_value: "nine", numeric_value: "9", int_value: 9},
                ];

                let matches: Vec<NumberMatches> = numbers_to_find.iter()
                    .map(|number_to_find| {
                        let mut forward_letters_matches = line.match_indices(number_to_find.letters_value);
                        let mut forward_numeric_matches = line.match_indices(number_to_find.numeric_value);
                        let mut backward_letters_matches = line.rmatch_indices(number_to_find.letters_value);
                        let mut backward_numeric_matches = line.rmatch_indices(number_to_find.numeric_value);

                        let first_match = get_preffered_match(forward_letters_matches.next(), forward_numeric_matches.next(), min);
                        let last_match = get_preffered_match(backward_letters_matches.next(), backward_numeric_matches.next(), max);

                        NumberMatches {
                            first_match_index: first_match,
                            last_match_index: last_match,
                            value: number_to_find.int_value
                        }
                    })
                    .collect();

                    let first = matches.iter()
                        .filter(|num_match| num_match.first_match_index.is_some())
                        .min_by_key(|num_match| num_match.first_match_index.unwrap())
                        .map(|max| max.value.to_string())
                        .unwrap_or_default();

                    let last = matches.iter()
                        .filter(|num_match| num_match.last_match_index.is_some())
                        .max_by_key(|num_match| num_match.last_match_index.unwrap())
                        .map(|max| max.value.to_string())
                        .unwrap_or_default();

                first.add(&last).parse::<u32>().unwrap_or_default()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
