advent_of_code::solution!(3);

struct Engine {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
    dimensions: Point,
}

impl Engine {
    fn from_input(input: &str) -> Engine {
        let mut found_numbers: Vec<Number> = vec![];
        let mut found_symbols: Vec<Symbol> = vec![];

        for (y, line) in input.lines().enumerate() {
            let mut numbers_buffer: Vec<char> = vec![];
            let mut start_index = None;
            let mut end_index = None;

            for (x, char) in line.chars().enumerate() {
                if char.is_ascii_digit() {
                    numbers_buffer.push(char);

                    if start_index.is_none() {
                        start_index = Some(x);
                    }

                    end_index = Some(x);
                } else if let (Some(start), Some(end)) = (start_index, end_index) {
                    found_numbers.push(Number::new(&numbers_buffer, start, end, y));

                    numbers_buffer.clear();
                    start_index = None;
                    end_index = None;
                }

                if !char.is_ascii_digit() && char != '.' {
                    found_symbols.push(Symbol {
                        value: char,
                        location: Point { x, y },
                    });
                }
            }

            if let (Some(start), Some(end)) = (start_index, end_index) {
                found_numbers.push(Number::new(&numbers_buffer, start, end, y));
            }
        }

        let dimensions = Point {
            x: input.lines().count(),
            y: input
                .lines()
                .next()
                .map(|line| line.chars().count())
                .unwrap_or_default(),
        };

        Engine {
            numbers: found_numbers,
            symbols: found_symbols,
            dimensions,
        }
    }
}

struct Number {
    value: u32,
    start: Point,
    end: Point,
}

impl Number {
    fn new(char_buffer: &[char], x_start: usize, x_end: usize, y: usize) -> Number {
        let int_value = char_buffer
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or_default();

        Number {
            value: int_value,
            start: Point { x: x_start, y },
            end: Point { x: x_end, y },
        }
    }
}

struct Symbol {
    location: Point,
    value: char,
}

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct Gear {
    location: Point,
    adjacent_numbers: Vec<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let engine = Engine::from_input(input);

    let result = engine
        .numbers
        .iter()
        .filter(|&number| {
            let can_go_before = number.start.x > 0;
            let can_go_after = number.end.x < engine.dimensions.x - 1;
            let can_go_over = number.start.y > 0;
            let can_go_under = number.start.y < engine.dimensions.y - 1;

            let symbol_before = can_go_before
                && engine.symbols.iter().any(|symbol| {
                    symbol.location.y == number.start.y && symbol.location.x == number.start.x - 1
                });

            let symbol_after = can_go_after
                && engine.symbols.iter().any(|symbol| {
                    symbol.location.y == number.end.y && symbol.location.x == number.end.x + 1
                });

            let start = if can_go_before {
                number.start.x - 1
            } else {
                number.start.x
            };

            let end = if can_go_after {
                number.end.x + 1
            } else {
                number.end.x
            };

            let symbol_over = can_go_over
                && (start..=end).any(|x| {
                    engine.symbols.iter().any(|symbol| {
                        symbol.location.x == x && symbol.location.y == number.start.y - 1
                    })
                });

            let symbol_under = can_go_under
                && (start..=end).any(|x| {
                    engine.symbols.iter().any(|symbol| {
                        symbol.location.x == x && symbol.location.y == number.start.y + 1
                    })
                });

            symbol_before || symbol_after || symbol_over || symbol_under
        })
        .map(|number| number.value)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let engine = Engine::from_input(input);
    let mut gears: Vec<Gear> = engine
        .symbols
        .iter()
        .filter(|&symbol| symbol.value == '*')
        .map(|gear| Gear {
            location: gear.location,
            adjacent_numbers: vec![],
        })
        .collect();

    engine.numbers.iter().for_each(|number| {
        let can_go_before = number.start.x > 0;
        let can_go_after = number.end.x < engine.dimensions.x - 1;
        let can_go_over = number.start.y > 0;
        let can_go_under = number.start.y < engine.dimensions.y - 1;

        if can_go_before {
            if let Some(gear) = gears.iter_mut().find(|gear| {
                gear.location.y == number.start.y && gear.location.x == number.start.x - 1
            }) {
                gear.adjacent_numbers.push(number.value);
            }
        }

        if can_go_after {
            if let Some(gear) = gears.iter_mut().find(|gear| {
                gear.location.y == number.start.y && gear.location.x == number.end.x + 1
            }) {
                gear.adjacent_numbers.push(number.value);
            }
        }

        let start = if can_go_before {
            number.start.x - 1
        } else {
            number.start.x
        };

        let end = if can_go_after {
            number.end.x + 1
        } else {
            number.end.x
        };

        if can_go_over {
            (start..=end).for_each(|x| {
                if let Some(gear) = gears
                    .iter_mut()
                    .find(|gear| gear.location.x == x && gear.location.y == number.start.y - 1)
                {
                    gear.adjacent_numbers.push(number.value);
                }
            });
        }

        if can_go_under {
            (start..=end).for_each(|x| {
                if let Some(gear) = gears
                    .iter_mut()
                    .find(|gear| gear.location.x == x && gear.location.y == number.start.y + 1)
                {
                    gear.adjacent_numbers.push(number.value);
                }
            });
        }
    });

    let result = gears
        .iter()
        .filter_map(|gear| {
            if gear.adjacent_numbers.len() == 2 {
                Some(gear.adjacent_numbers.iter().take(2).product::<u32>())
            } else {
                None
            }
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
