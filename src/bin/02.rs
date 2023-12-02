advent_of_code::solution!(2);

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    red_count: Option<u32>,
    blue_count: Option<u32>,
    green_count: Option<u32>,
}

fn parse_input_to_games(input: &str) -> Vec<Game> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            let (game_info, rounds_info) = line.split_at(line.find(": ").unwrap());

            let game_id = game_info
                .replace("Game ", "")
                .parse::<u32>()
                .unwrap_or_default();

            let rounds: Vec<Round> = rounds_info.split("; ").map(parse_round).collect();

            Game {
                id: game_id,
                rounds,
            }
        })
        .collect();

    games
}

fn parse_round(input: &str) -> Round {
    let mut round = Round {
        red_count: None,
        blue_count: None,
        green_count: None,
    };

    input.split(", ").for_each(|cube_info| {
        let amount = cube_info
            .split(' ')
            .next()
            .and_then(|number| number.parse::<u32>().ok())
            .unwrap_or_default();

        if cube_info.contains("red") {
            round.red_count = Some(amount);
        } else if cube_info.contains("blue") {
            round.blue_count = Some(amount);
        } else if cube_info.contains("green") {
            round.green_count = Some(amount);
        };
    });

    round
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;

    let games = parse_input_to_games(input);

    let possible_games = games.iter().filter(|&game| {
        let impossible_rounds = game
            .rounds
            .iter()
            .filter(|&round| {
                round
                    .red_count
                    .map(|red| red > max_red_cubes)
                    .unwrap_or_default()
                    || round
                        .green_count
                        .map(|green| green > max_green_cubes)
                        .unwrap_or_default()
                    || round
                        .blue_count
                        .map(|blue| blue > max_blue_cubes)
                        .unwrap_or_default()
            })
            .count();

        impossible_rounds == 0
    });

    Some(possible_games.map(|game| game.id).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_input_to_games(input);

    let result = games
        .iter()
        .map(|game| {
            let max_blue = game
                .rounds
                .iter()
                .filter_map(|round| round.blue_count)
                .max()
                .unwrap_or(1);

            let max_red = game
                .rounds
                .iter()
                .filter_map(|round| round.red_count)
                .max()
                .unwrap_or(1);

            let max_green = game
                .rounds
                .iter()
                .filter_map(|round| round.green_count)
                .max()
                .unwrap_or(1);

            max_blue * max_red * max_green
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
