use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for games in reader.lines(){
        match games {
            Ok(game) => {
                let (game_id, is_possible) = calculate_game(&game);
                if is_possible {
                    sum = sum + game_id;
                }
            }
            Err(e) => {
                eprintln!("Cannot read line, {}", e);
            }
        }
    }
     println!("The sum of possible game ids is: {}", sum);
}

fn calculate_game (line: &str) -> (u32, bool){
    let sets:Vec<&str> = line.split(|c| c == ';' || c == ':').collect();
    let game_id: u32 = sets[0].split_whitespace().into_iter().last().unwrap().parse().expect("Can't parse string to number");
    for set in sets.into_iter().skip(1){
        if check_set(set.trim()) == false {
            return (game_id, false)
        }
    }
    (game_id, true)
}

fn check_set (set: &str) -> bool {
    let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let cubes: Vec<&str> = set.split(", ").collect();
    for cube in cubes.into_iter() {
        let split: Vec<&str> = cube.split_whitespace().collect();
        let value = split[0].parse::<u32>().unwrap();
        let color = split[1];
        match color {
            "green" => if value.gt(limits.get(color).unwrap()) {
                return false
            }
            "blue" => if value.gt(limits.get(color).unwrap()) {
                return false
            }
            "red" => if value.gt(limits.get(color).unwrap()) {
                return false
            }
            _ => panic!("Color can only be 'red', 'green' or 'blue'")
        }
    }
    true
}