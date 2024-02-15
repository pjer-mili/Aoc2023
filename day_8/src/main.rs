use std::{collections::BTreeMap, fs::File, io::{BufRead, BufReader, Lines}};

#[derive(Debug)]
struct Directions {
    array: Vec<char>,
    current_index: usize
}

impl Directions {
    fn get_direction(&self) -> char {
        self.array[self.current_index]
    }

    fn traverse(&mut self) {
        if self.current_index < self.array.len() - 1 {
            self.current_index += 1; 
        }
        else {
            self.current_index = 0;
        }
    }
}

impl From<String> for Directions {
    fn from(input: String) -> Self {
        let array: Vec<char> = input.chars().collect();
        let current_index: usize = 0;
        Directions {array, current_index}
    }
}

#[derive(Debug)]
struct Node{
    value: String,
    left: String,
    right: String,
}

impl Node {
    fn new(value: String, left: String, right: String) -> Self {
        Self { value, left, right }
    }

    fn is_destination(&self) -> bool {
        self.value == "ZZZ"
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let (input, nodes) = parse_file(reader);
    let mut directions = Directions::from(input);

    let mut current_node = nodes.iter().next().map(|(_, v)| v).unwrap();
    let mut steps = 0;

    while !current_node.is_destination() {
        let direction = directions.get_direction();

        if direction == 'R' {
            current_node = nodes.get(&current_node.right).unwrap();
        } else {
            current_node = nodes.get(&current_node.left).unwrap();
        }

        directions.traverse();
        steps += 1;        
    }

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
    let split: Vec<&str> = line.split(&[' ', '=', '(' , ')', ',']).filter(|part| part.ne(&"")).collect();
    Node::new(String::from(split[0]), String::from(split[1]), String::from(split[2]))
}

fn get_directions(lines: &mut Lines<BufReader<File>>) -> String {
    lines.next().unwrap().expect("Could not get input")
}
