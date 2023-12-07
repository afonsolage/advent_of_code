#[derive(Debug, Clone, Copy)]
struct Range {
    destination: u32,
    source: u32,
    length: u32,
}

impl Range {
    fn map(&self, target: u32) -> Option<u32> {
        if target >= self.source && (target - self.source) < self.length {
            let offset = target - self.source;
            Some(self.destination + offset)
        } else {
            None
        }
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let values = value
            .split_whitespace()
            .filter_map(|c| c.parse::<u32>().ok())
            .collect::<Vec<_>>();
        Self {
            destination: values[0],
            source: values[1],
            length: values[2],
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Alamanac {
    seeds: Vec<u32>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

impl Alamanac {
    fn new(seeds: Vec<u32>) -> Self {
        Self {
            seeds,
            ..Default::default()
        }
    }

    fn find_lowest_location(&self) -> u32 {
        self.seeds
            .iter()
            .map(|&seed| self.get_seed_location(seed))
            .min()
            .unwrap()
    }

    fn find_mapping(&self, target: u32, ranges: &[Range]) -> u32 {
        ranges
            .iter()
            .find_map(|range| range.map(target))
            .unwrap_or(target)
    }

    fn get_seed_location(&self, seed: u32) -> u32 {
        let soil = self.find_mapping(seed, &self.seed_to_soil);
        let fertilizer = self.find_mapping(soil, &self.soil_to_fertilizer);
        let water = self.find_mapping(fertilizer, &self.fertilizer_to_water);
        let light = self.find_mapping(water, &self.water_to_light);
        let temperature = self.find_mapping(light, &self.light_to_temperature);
        let humidity = self.find_mapping(temperature, &self.temperature_to_humidity);
        let location = self.find_mapping(humidity, &self.humidity_to_location);

        location
    }
}

fn part02(input: &str) -> u32 {
    let mut split = input.split("\n\n");

    let values = split
        .next()
        .unwrap()
        .trim_start_matches("seed: ")
        .split_whitespace()
        .filter_map(|seed| seed.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let seeds = values
        .chunks(2)
        .flat_map(|pair| {
            let begin = pair[0];
            let len = pair[1];
            begin..(begin + len)
        })
        .collect();
    let almanac = Alamanac::new(seeds);

    let almanac = split.fold(almanac, |mut almanac, raw| {
        let (tp, ranges) = raw.trim().split_once(':').unwrap();
        let tp = tp.split_whitespace().next().unwrap();
        let ranges = ranges
            .lines()
            .filter_map(|raw| {
                if !raw.is_empty() {
                    Some::<Range>(raw.into())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        match tp {
            "seed-to-soil" => almanac.seed_to_soil = ranges,
            "soil-to-fertilizer" => almanac.soil_to_fertilizer = ranges,
            "fertilizer-to-water" => almanac.fertilizer_to_water = ranges,
            "water-to-light" => almanac.water_to_light = ranges,
            "light-to-temperature" => almanac.light_to_temperature = ranges,
            "temperature-to-humidity" => almanac.temperature_to_humidity = ranges,
            "humidity-to-location" => almanac.humidity_to_location = ranges,
            _ => unreachable!(),
        }

        almanac
    });

    almanac.find_lowest_location()
}

fn part01(input: &str) -> u32 {
    let mut split = input.split("\n\n");
    let seeds = split
        .next()
        .unwrap()
        .trim_start_matches("seed: ")
        .split_whitespace()
        .filter_map(|seed| seed.parse::<u32>().ok())
        .collect();

    let almanac = Alamanac::new(seeds);

    let almanac = split.fold(almanac, |mut almanac, raw| {
        let (tp, ranges) = raw.trim().split_once(':').unwrap();
        let tp = tp.split_whitespace().next().unwrap();
        let ranges = ranges
            .lines()
            .filter_map(|raw| {
                if !raw.is_empty() {
                    Some::<Range>(raw.into())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        match tp {
            "seed-to-soil" => almanac.seed_to_soil = ranges,
            "soil-to-fertilizer" => almanac.soil_to_fertilizer = ranges,
            "fertilizer-to-water" => almanac.fertilizer_to_water = ranges,
            "water-to-light" => almanac.water_to_light = ranges,
            "light-to-temperature" => almanac.light_to_temperature = ranges,
            "temperature-to-humidity" => almanac.temperature_to_humidity = ranges,
            "humidity-to-location" => almanac.humidity_to_location = ranges,
            _ => unreachable!(),
        }

        almanac
    });

    almanac.find_lowest_location()
}

fn main() {
    let input = include_str!("../input/day05.input");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn part01() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(super::part01(input), 35);
    }

    #[test]
    fn part02() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(super::part02(input), 46);
    }
}
