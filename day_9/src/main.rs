use std::{fs::File, io::{BufRead, BufReader}, iter::successors};

#[derive(Debug)]
struct History{
    inputs: Vec<i32>
}

impl History {
    fn new(inputs: Vec<i32>) -> Self {
        Self { inputs }
    }

    fn is_zero(&self) -> bool {
        self.inputs.iter().all(|&value| value == 0)
    }

    fn first(&self) -> i32 {
        *self.inputs.first().unwrap()
    }

    fn last(&self) -> i32 {
        *self.inputs.last().unwrap()
    }

    fn reduce(&self) -> History {
        History::new(self.inputs.iter().zip(self.inputs.iter().skip(1)).map(|(a, b)| b - a).collect())
    }

    fn reduced_histories(&self) -> Vec<History> {
        successors(Some(self.reduce()), |history| {
            (!history.is_zero()).then(|| history.reduce())
        }).collect()
    }

    fn sum_next(&self) -> i32 {
        self.last() + self.reduced_histories().iter().rfold(0, |value, history| history.last() + value)
    }

    fn sum_prev(&self) -> i32 {
        self.first() - self.reduced_histories().iter().rfold(0, |value, history| history.first() - value)
    }
}


struct Report {
    histories: Vec<History>
}

impl Report {
    fn sum_next(&self) -> i32 {
        self.histories.iter().map(|history| history.sum_next()).sum()
    }

    fn sum_prev(&self) -> i32 {
        self.histories.iter().map(|history| history.sum_prev()).sum()
    }
}

impl Report {
    fn new(histories: Vec<History>) -> Self {
        Self { histories }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not opet file");
    let reader = BufReader::new(file);
    let report = parse_file(reader);
    println!("PART 1: {}", report.sum_next());
    println!("PART 2: {}", report.sum_prev());
}

fn parse_file(reader: BufReader<File>) -> Report {
    let mut result: Vec<History> = vec![];
    for line in reader.lines() {
        let parsed_history = parse_line(&line.unwrap());
        result.push(parsed_history);
    }
    Report::new(result)
}

fn parse_line(line: &String) -> History {
    let inputs: Vec<i32> = line.split_ascii_whitespace().map(|part| part.parse::<i32>().unwrap()).collect();
    History::new(inputs)
}