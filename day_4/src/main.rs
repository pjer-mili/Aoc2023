use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug)]
struct Card<'a>{
    title: &'a str,
    winning_numbers: &'a str,
    numbers: &'a str
}
fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let cards: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut sum = 0;

    for card_line in cards{
        sum = sum + handle_game(&card_line);
    }

    println!("The total sum of points is: {}", sum);
}

fn handle_game(card_line: &String) -> u32 {

    let mut game_sum: u32 = 0;

    let card = create_card(card_line);
    
    let winning_numbers_array: Vec<&str> = card.winning_numbers.split_whitespace().collect();
    let numbers_array = card.numbers.split_whitespace();

    for number in numbers_array {
        if winning_numbers_array.contains(&number){
            if game_sum == 0{
                game_sum = 1
            }
            else {
                game_sum = game_sum * 2;
            }
        }
    }

    game_sum
}

fn create_card(game_line: &String) -> Card {

    let input: Vec<&str> = game_line.split('|').collect();
    let title_winning_numbers: Vec<&str> =  input[0].trim().split(':').collect();
    let title = title_winning_numbers[0].trim();
    let winning_numbers = title_winning_numbers[1].trim();
    let numbers = input[1].trim();

    let game = Card {
        numbers: numbers,
        title: title,
        winning_numbers: winning_numbers
    };

    game

}