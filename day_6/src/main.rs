use std::{fs::File, io::{BufRead, BufReader, Lines}};

#[derive(Debug)]
struct Race{
    time: u64,
    distance: u64,
    number_of_ways: Vec<u64>
}

impl Race{
    fn new(time: u64, distance: u64, number_of_ways: Vec<u64>) -> Self {
        Self {time, distance, number_of_ways}
    }

    fn beats_record(&self, competing_dist: u64) -> bool {
        if competing_dist > self.distance {
            return true;
        }
        false
    }

    fn add_way(&mut self, time: u64) {
        self.number_of_ways.push(time);
    }

    fn get_number_of_ways(&self) -> usize {
        self.number_of_ways.len()
    }    
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut race = parse_file(reader);

    let mut result: usize = 1;

    //TODO: optimize code, maybe we dont need an for loop

    println!("Race: {:#?}", race);

    for i in 1..race.time {
        let new_distance = (race.time - i) * i;
        if race.beats_record(new_distance){
            race.add_way(i);
        }
    }

    result *= race.get_number_of_ways();

    println!("The number of ways: {}", result);
}

fn parse_file(reader: BufReader<File>) -> Race {
    let mut lines = reader.lines();

    let time_parts = parse_line(&mut lines);
    let distance_parts = parse_line(&mut lines);

    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    let base: u64 = 10;
    for i in 0..time_parts.len() {
        let raise: u32 = (time_parts.len() - 1 - i).try_into().unwrap();
        time += base.pow(raise) * time_parts[i];
    }

    for i in 0..distance_parts.len() {
        let raise: u32 = (distance_parts.len() - 1 - i).try_into().unwrap();
        distance += base.pow(raise) * distance_parts[i];
    }

    Race::new(time, distance, vec![])
}

fn parse_line(lines: &mut Lines<BufReader<File>>) -> Vec<u64> {
    let mut values: Vec<u64> = vec![];

    let next_line = lines.next().unwrap().expect("This is not a line!");

    let temp: Vec<&str> = next_line.split_whitespace().collect();

    for i in 1..temp.len() {
        let x = temp[i];
        for c in x.chars() {
            values.push(c.to_digit(10).expect("Cannot parse char").into());
        }
    }
    values
} 
