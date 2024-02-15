use std::{collections::BTreeMap, fs::File, io::{BufRead, BufReader, Lines}};

#[derive(Debug)]
struct Directions {
    array: Vec<char>,
    current_index: usize
}

impl Directions {

    fn reset(&mut self) {
        self.current_index = 0
    }

    fn get_direction(&self) -> char {
        self.array[self.current_index]
    }

    fn traverse(&mut self) {
        if self.current_index < self.array.len() - 1 {
            self.current_index += 1;
        } else {
            self.current_index = 0;
        }
    }
}

impl From<String> for Directions {
    fn from(input: String) -> Self {
        let array: Vec<char> = input.chars().collect();
        let current_index: usize = 0;
        Directions { array, current_index }
    }
}

#[derive(Debug)]
struct Node {
    value: String,
    left: String,
    right: String,
}

impl Node {
    fn new(value: String, left: String, right: String) -> Self {
        Self { value, left, right }
    }

    fn is_destination(&self) -> bool {
        if self.value.ends_with('Z') {
            return true;
        }
        false
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let (input, nodes) = parse_file(reader);
    let mut directions = Directions::from(input);

    let mut start_nodes = get_start_nodes(&nodes);
    let mut steps_array :Vec<u128> = vec![];
    
    for start in start_nodes.iter_mut() {
        let mut steps_to_z: u128 = 0;
        let mut current_node = start.clone();
        directions.reset();

        while !current_node.is_destination() {
            let direction = directions.get_direction();
            match direction {
                'L' => {
                    current_node = nodes.get(&current_node.left).unwrap();
                },
                'R' => {
                    current_node = nodes.get(&current_node.right).unwrap();
                }
                _ => panic!("Bad input!")
            }
            directions.traverse();
            steps_to_z += 1;
        }
        steps_array.push(steps_to_z);
    }

    let steps = lcm_array(steps_array);
    println!("STEPS: {}", steps);
}

fn parse_file(reader: BufReader<File>) -> (String, BTreeMap<String, Node>) {
    let mut nodes: BTreeMap<String, Node> = BTreeMap::new();
    let mut lines = reader.lines();
    let input = get_directions(&mut lines);
    for line in lines.skip(1) {
        let node = get_nodes(&line.unwrap());
        nodes.insert(node.value.clone(), node);
    }
    (input, nodes)
}

fn get_nodes(line: &String) -> Node {
    let split: Vec<&str> = line.split(&[' ', '=', '(', ')', ',']).filter(|part| part.ne(&"")).collect();
    Node::new(String::from(split[0]), String::from(split[1]), String::from(split[2]))
}

fn get_directions(lines: &mut Lines<BufReader<File>>) -> String {
    lines.next().unwrap().expect("Could not get input")
}

fn get_start_nodes(nodes: &BTreeMap<String, Node>) -> Vec<&Node> {
    let start_nodes: Vec<&Node> = nodes.values().filter(|&node| node.value.ends_with('A')).collect();
    start_nodes
}

fn lcm_array(array: Vec<u128>) -> u128 {
    array.iter().fold(1, |acc , &x| lcm(acc, x))
}

fn lcm(first: u128, second: u128) -> u128 {
    first * second / gcd(first, second)
}

fn gcd(first: u128, second: u128) -> u128 {
    let mut max = first;
    let mut min = second;

    if min > max {
        let temp = max;
        max = min;
        min = temp;
    }

    loop {
         let res = max % min;
         if res == 0 {
            return min;
         }
          
          max = min;
          min = res;
    }
}