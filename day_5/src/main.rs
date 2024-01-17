use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, vec};

#[derive(Debug)]
struct AlmanacMap{
    destination: u64,
    source: u64,
    range: u64
}

#[derive(Debug)]
struct Seed{
    number: u64
}

impl AlmanacMap {
    fn new(destination: u64, source: u64, range: u64) -> Self {
        Self { destination, source, range }
    }
    fn is_in_range(&self, number: u64) -> bool {
        if number >= self.source && number <= self.source + self.range{
            return true;
        }
        false
    }
}

impl Seed{
    fn new(number: u64) -> Self{
        Self{number}
    }
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut locations: Vec<u64> = vec![];
    let (seeds, maps) = parse_file(reader);

    for seed in seeds {
        let location = get_seed_location(seed.number, &maps);
        locations.push(location);
    }

    locations.sort_by(|a, b| a.cmp(b));
    println!("The lowest locaton is {}", locations[0]);
}

fn init_map() -> HashMap<String, Vec<AlmanacMap>> {
    HashMap::from([
        (String::from("seed-to-soil"), vec![]),
        (String::from("soil-to-fertilizer"), vec![]),
        (String::from("fertilizer-to-water"), vec![]),
        (String::from("water-to-light"), vec![]),
        (String::from("light-to-temperature"), vec![]),
        (String::from("temperature-to-humidity"), vec![]),
        (String::from("humidity-to-location"), vec![])
    ])
}

fn parse_file(reader: BufReader<File>) -> (Vec<Seed>, HashMap<String, Vec<AlmanacMap>>) {
    let mut lines = reader.lines().filter_map(|l| match l {
        Ok(line) => {
            if line.is_empty(){
                return  None;
            }
            Some(line)
        },
        Err(_) => None
    });
    
    //We get all seeds
    let seeds: Vec<Seed> = lines.next()
    .unwrap()
    .split(':')
    .skip(1)
    .next()
    .unwrap()
    .trim()
    .split_ascii_whitespace()
    .map(|num| Seed::new(num.parse::<u64>().unwrap()))
    .collect();

    //Next we get all maps
    let maps = parse_map(lines);

    (seeds, maps)
}

fn parse_map<I>(lines: I) -> HashMap<String, Vec<AlmanacMap>> where I : Iterator<Item = String> {
    let mut maps = init_map();
    let mut current_key = String::new();
    for line in lines {
        if line.ends_with("map:"){
            //Shave off the 'map:'
            current_key = line.split(" ").next().unwrap().to_owned();
            continue;
        }

        if line.is_empty(){
            continue;
        }

        let values: Vec<u64> = line.split(" ").map(|n| n.parse::<u64>().unwrap()).collect();
        let almanac_map = AlmanacMap::new(values[0], values[1], values[2]);
        maps.get_mut(&current_key).unwrap().push(almanac_map);
    }

    maps
}

fn get_seed_location(seed: u64, maps: &HashMap<String, Vec<AlmanacMap>>) -> u64 {

    let keys_in_order = vec!["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];
    let mut current_value: u64 = seed;
    for key in keys_in_order {
        current_value = get_mapped_value(key, maps, current_value);
    }
    current_value
}

fn get_mapped_value(key: &str, maps: &HashMap<String, Vec<AlmanacMap>>, current_value: u64) -> u64 {
    let almanacs = maps.get(key).unwrap();
    let mut mapped_value: u64 = 0;
    let mut value_found: bool = false;
    for almanac in almanacs {
        if value_found {
            break;
        }
        if almanac.is_in_range(current_value){
            mapped_value  = almanac.destination + (current_value - almanac.source);
            value_found = true;
            break;
        }
    }
    if value_found {
        return mapped_value;
    }
    current_value
}