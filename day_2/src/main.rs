use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for games in reader.lines(){
        match games {
            Ok(game) => {
                let max_hash = calculate_game(&game);
                sum = sum + (max_hash.get("red").unwrap() * max_hash.get("blue").unwrap() * max_hash.get("green").unwrap());
            }
            Err(e) => {
                eprintln!("Cannot read line, {}", e);
            }
        }
    }
     println!("The sum of power of sets is: {}", sum);
}

fn calculate_game (game: &str) -> HashMap<&str, u32>{

    let mut maxes = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);

    let game_split:Vec<&str> = game.split(": ").collect();
    for cube in game_split.into_iter().skip(1){
        let cube_strings:Vec<&str> = cube.split(|c|  c == ',' || c == ';').collect();
        for cube_string in cube_strings{
            let die: Vec<&str> = cube_string.split_whitespace().collect();
            let value = die[0].parse::<u32>().expect("Could not parse string into number");
            let color = die[1];
            let current_value = maxes.get(color).expect("Should get a number");
            if value > *current_value {maxes.insert(color, value);}
        }
    }
    maxes
}