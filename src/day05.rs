#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u64,
    lenght: u64,
}

impl Range {
    fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..(self.start + self.lenght).into()
    }

    fn end(&self) -> u64 {
        self.start + self.lenght - 1
    }
}

impl From<&[u64]> for Range {
    fn from(value: &[u64]) -> Self {
        Range {
            start: value[0],
            lenght: value[1],
        }
    }
}

impl From<u64> for Range {
    fn from(value: u64) -> Self {
        Range {
            start: value,
            lenght: 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MapRange {
    destination: u64,
    source: u64,
    length: u64,
}

impl MapRange {
    fn source_end(&self) -> u64 {
        self.source + self.length - 1
    }

    fn map_single(&self, target: u64) -> Option<u64> {
        if target >= self.source && (target - self.source) < self.length {
            let offset = target - self.source;
            Some(self.destination + offset)
        } else {
            None
        }
    }

    fn map_range(&self, target: Range) -> (Option<Range>, Vec<Range>) {
        if let Some((inside, outside)) = self.split(target) {
            let offset = inside.start - self.source;
            let mapped = Range {
                start: self.destination + offset,
                ..inside
            };
            (Some(mapped), outside)
        } else {
            (None, vec![target])
        }
    }

    fn split(&self, target: Range) -> Option<(Range, Vec<Range>)> {
        if target.end() < self.source || target.start > self.source_end() {
            return None;
        }

        let mut outside = vec![];
        let left_split = target.start.max(self.source);
        let right_split = target.end().min(self.source_end());

        let inside = Range {
            start: left_split,
            lenght: right_split - left_split + 1,
        };

        if left_split > target.start {
            outside.push(Range {
                start: target.start,
                lenght: left_split - target.start,
            })
        };

        if right_split < target.end() {
            outside.push(Range {
                start: right_split + 1,
                lenght: target.end() - right_split,
            })
        }

        Some((inside, outside))
    }
}

impl From<&str> for MapRange {
    fn from(value: &str) -> Self {
        let values = value
            .split_whitespace()
            .filter_map(|c| c.parse::<u64>().ok())
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
    seeds: Vec<Range>,
    seed_to_soil: Vec<MapRange>,
    soil_to_fertilizer: Vec<MapRange>,
    fertilizer_to_water: Vec<MapRange>,
    water_to_light: Vec<MapRange>,
    light_to_temperature: Vec<MapRange>,
    temperature_to_humidity: Vec<MapRange>,
    humidity_to_location: Vec<MapRange>,
}

impl Alamanac {
    fn new(seeds: Vec<Range>) -> Self {
        Self {
            seeds,
            ..Default::default()
        }
    }

    fn find_lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .flat_map(|seed| seed.iter())
            .into_iter()
            .map(|seed| self.get_seed_location(seed))
            .min()
            .unwrap()
    }

    fn find_lowest_ranged_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|&seed| self.get_seed_range_location(seed))
            .flat_map(|ranges| ranges.into_iter().map(|range| range.start))
            .min()
            .unwrap()
    }

    fn find_range_mapping(&self, targets: &[Range], ranges: &[MapRange]) -> Vec<Range> {
        let (mapped_targets, unmapped) = ranges.iter().fold(
            (vec![], targets.to_vec()),
            |(mut mapped_targets, unmapped_targets), &map_range| {
                let (mapped, unmapped): (Vec<_>, Vec<_>) = unmapped_targets
                    .iter()
                    .map(|&target| map_range.map_range(target))
                    .unzip();

                let mut mapped = mapped.into_iter().filter_map(|r| r).collect::<Vec<_>>();
                let unmapped_targets = unmapped.into_iter().flatten().collect::<Vec<_>>();

                mapped_targets.append(&mut mapped);
                (mapped_targets, unmapped_targets)
            },
        );

        mapped_targets
            .into_iter()
            .chain(unmapped.into_iter())
            .collect()
    }

    fn find_mapping(&self, target: u64, ranges: &[MapRange]) -> u64 {
        ranges
            .iter()
            .find_map(|range| range.map_single(target))
            .unwrap_or(target)
    }

    fn get_seed_range_location(&self, seed: Range) -> Vec<Range> {
        let soil = self.find_range_mapping(&vec![seed], &self.seed_to_soil);
        let fertilizer = self.find_range_mapping(&soil, &self.soil_to_fertilizer);
        let water = self.find_range_mapping(&fertilizer, &self.fertilizer_to_water);
        let light = self.find_range_mapping(&water, &self.water_to_light);
        let temperature = self.find_range_mapping(&light, &self.light_to_temperature);
        let humidity = self.find_range_mapping(&temperature, &self.temperature_to_humidity);
        let location = self.find_range_mapping(&humidity, &self.humidity_to_location);

        location
    }

    fn get_seed_location(&self, seed: u64) -> u64 {
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

fn part02(input: &str) -> u64 {
    let mut split = input.split("\n\n");

    let values = split
        .next()
        .unwrap()
        .trim_start_matches("seed: ")
        .split_whitespace()
        .filter_map(|seed| seed.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let seeds = values
        .chunks(2)
        .map(|pair| pair.into())
        .collect::<Vec<Range>>();

    let almanac = Alamanac::new(seeds);

    let almanac = split.fold(almanac, |mut almanac, raw| {
        let (tp, ranges) = raw.trim().split_once(':').unwrap();
        let tp = tp.split_whitespace().next().unwrap();
        let ranges = ranges
            .lines()
            .filter_map(|raw| {
                if !raw.is_empty() {
                    Some::<MapRange>(raw.into())
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

    almanac.find_lowest_ranged_location()
}

fn part01(input: &str) -> u64 {
    let mut split = input.split("\n\n");
    let seeds = split
        .next()
        .unwrap()
        .trim_start_matches("seed: ")
        .split_whitespace()
        .filter_map(|seed| seed.parse::<u64>().ok().and_then(|s| Some(s.into())))
        .collect();

    let almanac = Alamanac::new(seeds);

    let almanac = split.fold(almanac, |mut almanac, raw| {
        let (tp, ranges) = raw.trim().split_once(':').unwrap();
        let tp = tp.split_whitespace().next().unwrap();
        let ranges = ranges
            .lines()
            .filter_map(|raw| {
                if !raw.is_empty() {
                    Some::<MapRange>(raw.into())
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
    use super::*;

    #[test]
    fn split_outside() {
        let map_range = MapRange {
            destination: 30,
            source: 8,
            length: 7,
        };

        let after_range = Range {
            start: 15,
            lenght: 3,
        };
        let before_range = Range {
            start: 5,
            lenght: 3,
        };
        assert_eq!(map_range.split(after_range), None);
        assert_eq!(map_range.split(before_range), None);
    }

    #[test]
    fn split_part_inside() {
        let map_range = MapRange {
            destination: 30,
            source: 8,
            length: 7,
        };
        let part_inside_range = Range {
            start: 13,
            lenght: 7,
        };
        assert_eq!(
            map_range.split(part_inside_range),
            Some((
                Range {
                    start: 13,
                    lenght: 2,
                },
                vec![Range {
                    start: 15,
                    lenght: 5,
                },]
            ))
        );
    }

    #[test]
    fn split_full_inside() {
        let map_range = MapRange {
            destination: 30,
            source: 8,
            length: 7,
        };

        let full_inside_range = Range {
            start: 10,
            lenght: 3,
        };

        assert_eq!(
            map_range.split(full_inside_range),
            Some((
                Range {
                    start: 10,
                    lenght: 3,
                },
                vec![],
            ))
        );
    }

    #[test]
    fn map_range_outside() {
        let map_range = MapRange {
            destination: 30,
            source: 8,
            length: 7,
        };

        let after_range = Range {
            start: 15,
            lenght: 3,
        };
        let before_range = Range {
            start: 5,
            lenght: 3,
        };
        assert_eq!(map_range.map_range(after_range), (None, vec![after_range]));
        assert_eq!(
            map_range.map_range(before_range),
            (None, vec![before_range])
        );
    }

    #[test]
    fn map_range_part_inside() {
        let map_range = MapRange {
            destination: 30,
            source: 8,
            length: 7,
        };
        let part_inside_range = Range {
            start: 13,
            lenght: 7,
        };
        assert_eq!(
            map_range.map_range(part_inside_range),
            (
                Some(Range {
                    start: 35,
                    lenght: 2,
                }),
                vec![Range {
                    start: 15,
                    lenght: 5,
                },]
            )
        );
    }

    #[test]
    fn map_range_full_inside() {
        let map_range = MapRange {
            destination: 30,
            source: 8,
            length: 7,
        };

        let full_inside_range = Range {
            start: 10,
            lenght: 3,
        };

        assert_eq!(
            map_range.map_range(full_inside_range),
            (
                Some(Range {
                    start: 32,
                    lenght: 3,
                }),
                vec![],
            )
        );
    }

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
