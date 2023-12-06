advent_of_code::solution!(5);

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

impl Almanac {
    fn from_input(input: &str) -> Almanac {
        let (seeds_info, rest) = input.split_once("seed-to-soil map:").unwrap();

        let seeds: Vec<u64> = seeds_info
            .replace("seeds:", "")
            .split_whitespace()
            .filter_map(|number| number.parse::<u64>().ok())
            .collect();

        let (seed_to_soil_info, rest) = rest.split_once("soil-to-fertilizer map:").unwrap();
        let (soil_to_fertilizer_info, rest) = rest.split_once("fertilizer-to-water map:").unwrap();
        let (fertilizer_to_water_info, rest) = rest.split_once("water-to-light map:").unwrap();
        let (water_to_light_info, rest) = rest.split_once("light-to-temperature map:").unwrap();
        let (light_to_temperature_info, rest) =
            rest.split_once("temperature-to-humidity map:").unwrap();
        let (temperature_to_humidity_info, humidity_to_location_info) =
            rest.split_once("humidity-to-location map:").unwrap();

        Almanac {
            seeds,
            seed_to_soil: seed_to_soil_info
                .lines()
                .filter_map(Range::from_line)
                .collect(),
            soil_to_fertilizer: soil_to_fertilizer_info
                .lines()
                .filter_map(Range::from_line)
                .collect(),
            fertilizer_to_water: fertilizer_to_water_info
                .lines()
                .filter_map(Range::from_line)
                .collect(),
            water_to_light: water_to_light_info
                .lines()
                .filter_map(Range::from_line)
                .collect(),
            light_to_temperature: light_to_temperature_info
                .lines()
                .filter_map(Range::from_line)
                .collect(),
            temperature_to_humidity: temperature_to_humidity_info
                .lines()
                .filter_map(Range::from_line)
                .collect(),
            humidity_to_location: humidity_to_location_info
                .lines()
                .filter_map(Range::from_line)
                .collect(),
        }
    }
}

struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Range {
    fn from_line(line: &str) -> Option<Range> {
        let mut parts = line
            .split(' ')
            .filter_map(|number| number.parse::<u64>().ok());

        Some(Range {
            destination_start: parts.next()?,
            source_start: parts.next()?,
            length: parts.next()?,
        })
    }
}

fn translate_numbers(numbers: &[u64], ranges: &[Range]) -> Vec<u64> {
    numbers
        .iter()
        .map(|&number| {
            if let Some(range) = ranges.iter().find(|&range| {
                number >= range.source_start && number < range.source_start + range.length
            }) {
                let difference = number - range.source_start;
                range.destination_start + difference
            } else {
                number
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = Almanac::from_input(input);

    let soils = translate_numbers(&almanac.seeds, &almanac.seed_to_soil);
    let fertilizer = translate_numbers(&soils, &almanac.soil_to_fertilizer);
    let water = translate_numbers(&fertilizer, &almanac.fertilizer_to_water);
    let light = translate_numbers(&water, &almanac.water_to_light);
    let temperature = translate_numbers(&light, &almanac.light_to_temperature);
    let humidity = translate_numbers(&temperature, &almanac.temperature_to_humidity);
    let location = translate_numbers(&humidity, &almanac.humidity_to_location);

    let closest_location = location.iter().min().unwrap();

    Some(*closest_location)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
