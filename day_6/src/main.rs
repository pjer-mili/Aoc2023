use std::{fs::File, io::{BufRead, BufReader, Lines}};

#[derive(Debug)]
struct Race{
    time: i32,
    distance: i32,
    number_of_ways: Vec<i32>
}

impl Race{
    fn new(time: i32, distance: i32, number_of_ways: Vec<i32>) -> Self {
        Self {time, distance, number_of_ways}
    }

    fn beats_record(&self, competing_dist: i32) -> bool {
        if competing_dist > self.distance {
            return true;
        }
        false
    }

    fn add_way(&mut self, time: i32) {
        self.number_of_ways.push(time);
    }

    fn get_number_of_ways(&self) -> usize {
        self.number_of_ways.len()
    }    
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let races = parse_file(reader);

    let mut result: usize = 1;

    for mut race in races {
        for i in 1..race.time {
            let new_distance = (race.time - i) * i;
            if race.beats_record(new_distance){
                race.add_way(i);
            }
        }
        result *= race.get_number_of_ways();
    }

    println!("The number of ways: {}", result);
}

fn parse_file(reader: BufReader<File>) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let mut lines = reader.lines();

    let times = parse_line(&mut lines);
    let distances = parse_line(&mut lines);

    if times.len() != distances.len() {
        panic!("The time and distance arrays are not of the same length!");
    }

    for i in 0..times.len() {
        races.push(Race::new(times[i], distances[i], vec![]));
    }

    races
}

fn parse_line(lines: &mut Lines<BufReader<File>>) -> Vec<i32> {
    let mut values: Vec<i32> = vec![];

    let next_line = lines.next().unwrap().expect("This is not a line!");

    let temp: Vec<&str> = next_line.split_whitespace().collect();

    for i in 1..temp.len() {
        values.push(temp[i].parse::<i32>().expect("Could not parse"));
    }
    values
} 
