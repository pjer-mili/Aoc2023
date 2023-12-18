use std::{
    fs::File,
    io::{BufRead, BufReader}, cell::RefCell,
};

#[derive(Debug)]
struct Card {
    card_number: usize,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
    instances: RefCell<u32>
}

impl Card{
    fn new(card_number: usize, numbers: Vec<u32>, winning_numbers: Vec<u32>) -> Self {
        Self { card_number, numbers, winning_numbers, instances: RefCell::new(1) }
    }

    fn increase_instances(&self) {
        *self.instances.borrow_mut() += 1;
    }

}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let cards = parse_lines(lines);

    let mut sum: u32 = 0;
    for (index, _) in cards.iter().enumerate(){
        handle_card_recursive(&cards, index);
    }
   
    for card in cards{
        println!("Card: {} {:?} {:?} {:?}", card.card_number, card.numbers, card.winning_numbers, card.instances);  
        sum += *card.instances.borrow();
    }

    println!("Total instances of cards: {}", sum);
}

fn handle_card_recursive(
    all_cards: &Vec<Card>,
    parent_card_index: usize,
) {
    let parent_card = &all_cards[parent_card_index];

    let mut counter: usize = 0;

    for number in &parent_card.numbers {
        if parent_card.winning_numbers.contains(number) {
            counter = counter + 1;
        }
    }

    if counter == 0 {
        return;
    }
    
    for index in 1..=counter {
        let child_card_index = parent_card_index + index;
        let child_card = &all_cards[child_card_index];
        child_card.increase_instances();
        handle_card_recursive(all_cards, child_card_index);
    }
}

fn parse_lines(lines: Vec<String>) -> Vec<Card> {

    let mut cards: Vec<Card> = vec![];

    for line in lines {
        cards.push(get_card(&line))
    }

    cards
}

fn get_card(game_line: &String) -> Card {
    let input: Vec<&str> = game_line.split('|').collect();

    let title_winning_numbers: Vec<&str> = input[0].trim().split(':').collect();
    let winning_numbers = title_winning_numbers[1].trim().split_ascii_whitespace().map(|num| num.parse::<u32>().unwrap()).collect();
    let card_number = title_winning_numbers[0].trim_start_matches(|c: char| !c.is_digit(10)).parse::<usize>().unwrap();
    let numbers = input[1].trim().split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect();

    let new_card = Card::new(card_number, numbers, winning_numbers);
    new_card
}