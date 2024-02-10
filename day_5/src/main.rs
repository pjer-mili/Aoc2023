use std::{collections::HashMap, fmt::Debug, fs::File, io::{BufRead, BufReader}, vec};

#[derive(Debug)]
struct MapRange{
    destination: u64,
    source: u64,
    range: u64
}

#[derive(Debug)]
struct Almanac{
    from: String,
    to: String,
    maps: Vec<MapRange>
}

#[derive(Debug)]
struct SeedRange{
    start: u64,
    range: u64
}

impl MapRange {
    fn new(destination: u64, source: u64, range: u64) -> Self {
        Self { destination, source, range }
    }
}

impl Almanac {
    fn new(from: String, to: String, maps: Vec<MapRange>) -> Self{
        Self {from, to, maps}
    }
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let (seed_ranges, almanacs) = parse_file(file);
    let mut lowest = u64::MAX;
    
    for seed_range in seed_ranges {
        let mut remaining  = seed_range.range;
        let mut start = seed_range.start;

        while remaining > 0 {
            let (start_location, consumed) = walk(start, remaining, "seed", &almanacs);

            remaining -= consumed;
            start += consumed;

            if start_location < lowest {
                lowest = start_location;
            }
        }
    }

    println!("Lowest location is: {}", lowest);
}

fn walk(value: u64, range: u64, name: &str, almanacs: &HashMap<String, Almanac>) -> (u64, u64) {
    if let Some(item) = almanacs.get(name) {
        if let Some(range_item) = item.maps.iter().find(|x| x.source <= value && value < x.source + x.range) {
            let diff = value - range_item.source;
            let new_value = range_item.destination + diff;
            let new_range = range.min(range_item.range - diff);
            return walk(new_value, new_range, &item.to, almanacs);
        }
        return walk(value, 1, &item.to, almanacs);
    }
    (value, range)
}

fn parse_file(file: File) -> (Vec<SeedRange>, HashMap<String, Almanac>) {
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(|l| match l {
        Ok(line) => {
            if line.is_empty() {
                return None;
            }
            Some(line)
        }
        Err(_) => None,
    });
    
    let seed_values: Vec<u64> = lines.nth(0).unwrap().split(": ").nth(1).unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let seed_ranges: Vec<SeedRange> = parse_seed_input(seed_values);

    let maps = parse_almanac_input(lines);

    (seed_ranges, maps)
}

fn parse_seed_input(seed_values: Vec<u64>) -> Vec<SeedRange> {
    let seed_ranges: Vec<SeedRange> = (0..seed_values.len()).step_by(2).map(|i| SeedRange {
        start: seed_values[i],
        range: seed_values[i + 1],
    }).collect();
    seed_ranges
}

fn parse_almanac_input<I>(lines: I) -> HashMap<String, Almanac>
where
    I: Iterator<Item = String>,
{
    let mut almanacs: HashMap<String, Almanac> = HashMap::new();
    let mut current_almanac: Option<Almanac> = None;
    let mut current_key: Option<String> = None;

    for line in lines {
        if line.contains("map:") {
            if let Some(almanac) = current_almanac.take() {
                if let Some(key) = current_key.take() {
                    almanacs.insert(key, almanac);
                }
            }

            let splited_line: Vec<&str> = line
                .split_ascii_whitespace()
                .next()
                .and_then(|s| Some(s.split('-').collect::<Vec<&str>>().into_iter().collect()))
                .unwrap();

            current_key = Some(splited_line[0].to_owned());
            current_almanac = Some(Almanac::new(splited_line[0].to_owned(), splited_line[2].to_owned(), vec![]));
        } else {
            let (destination, source, range) = {
                let mut parts = line.split_ascii_whitespace();
                (
                    parts.next().unwrap().parse::<u64>().unwrap(),
                    parts.next().unwrap().parse::<u64>().unwrap(),
                    parts.next().unwrap().parse::<u64>().unwrap(),
                )
            };

            if let Some(almanac) = &mut current_almanac {
                almanac.maps.push(MapRange::new(destination, source, range));
            }
        }
    }

    if let Some(almanac) = current_almanac {
        if let Some(key) = current_key {
            almanacs.insert(key, almanac);
        }
    }

    let mut almanacs_with_negative: HashMap<String, Almanac> = HashMap::new();

    for (key, alamanac) in almanacs {
        let from = alamanac.from;
        let to = alamanac.to;
        let new_ranges = create_negative_ranges(alamanac.maps);
        almanacs_with_negative.insert(key, Almanac::new(from, to, new_ranges));
    }
    
    almanacs_with_negative
}

fn create_negative_ranges(ranges: Vec<MapRange>) -> Vec<MapRange> {
    let mut result: Vec<MapRange> = Vec::new();

    for range in ranges {
        result.push(range);
    }

    result.sort_by(|a, b| a.source.cmp(&b.source));

    let mut start = 0;
    let mut i = 0;

    while i < result.len() {
        if result[i].source > start {
            let new_range = MapRange::new(start, start, result[i].source - start);
            result.insert(i, new_range);
            i += 1;
        }
        start = result[i].source + result[i].range;
        i += 1;
    }

    result
}