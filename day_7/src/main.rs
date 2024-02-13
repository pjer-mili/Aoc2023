use std::{cmp::Ordering, collections::HashMap, fs::File, io::{BufRead, BufReader}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl From<HashMap<char, i32>> for Type {
    fn from(card_map: HashMap<char, i32>) -> Self {
        let mut vals: Vec<i32> = card_map.values().map(|count| *count).collect();

        vals.sort();
        vals.reverse();
  
        return match &vals[0] {
          5 => Type::FiveOfAKind,
          4 => Type::FourOfAKind,
          3 => {
              match &vals[1] {
                  2 => Type::FullHouse,
                  1 => Type::ThreeOfAKind,
                  _ => panic!("Found 3 but next was not 2 or 1")
              }
          },
          2 => {
              match &vals[1] {
                  2 => Type::TwoPair,
                  1 => Type::OnePair,
                  _ => panic!("Found 2 but next was not 2 or 1")
              }
          },
          1 => Type::HighCard,
          _ => panic!("Key is not betwwen 1 and 5")
        }
      }    
}

#[derive(Debug, PartialEq, Eq)]
struct Hand{
    cards: Vec<char>,
    bid: u64,
    hand_type: Type
}

impl Hand {
    fn new(cards: Vec<char>, bid: u64, hand_type: Type) -> Self {
        Self{cards, bid, hand_type}
    }

    fn card_cmp(&self, other: &Self) -> Ordering {
    let numbers_map = HashMap::from([('A', 15), ('K', 14), ('Q', 13), ('J', 12), ('T', 10), ('9', 9), ('8', 8), ('7', 7), ('6', 6), ('5', 5), ('4', 4), ('3', 3), ('2', 2), ('1', 1)]);
    let mut ordering: Ordering = Ordering::Equal;
        for i in 0..5 {
            let my_value = numbers_map.get(&self.cards[i]);
            let other_value = numbers_map.get(&other.cards[i]);

            if my_value == other_value {
                continue;
            }
            if my_value > other_value {
                ordering = Ordering::Greater;
                break;
            }
            if my_value < other_value {
                ordering = Ordering::Less;
                break;
            }
        }

        ordering
    }

}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return match self.hand_type {
            Type::FiveOfAKind => match other.hand_type {
                Type::FiveOfAKind => self.card_cmp(other),
                _ => Ordering::Greater
            },
            Type::FourOfAKind => match other.hand_type {
                Type::FiveOfAKind => Ordering::Less,
                Type::FourOfAKind => self.card_cmp(other),
                _ => Ordering::Greater
            },
            Type::FullHouse => match other.hand_type {
                Type::FiveOfAKind | Type::FourOfAKind => Ordering::Less,
                Type::FullHouse => self.card_cmp(other),
                _ => Ordering::Greater
            },
            Type::ThreeOfAKind => match other.hand_type {
                Type::FiveOfAKind | Type::FourOfAKind | Type::FullHouse => Ordering::Less,
                Type::ThreeOfAKind => self.card_cmp(other),
                _ => Ordering::Greater
            },
            Type::TwoPair => match other.hand_type {
                Type::FiveOfAKind | Type::FourOfAKind | Type::FullHouse | Type::ThreeOfAKind => Ordering::Less,
                Type::TwoPair => self.card_cmp(other),
                _ => Ordering::Greater
            },
            Type::OnePair => match other.hand_type {
                Type::FiveOfAKind | Type::FourOfAKind | Type::FullHouse | Type::ThreeOfAKind | Type::TwoPair => Ordering::Less,
                Type::OnePair => self.card_cmp(other),
                _ => Ordering::Greater
            },
            Type::HighCard => match  other.hand_type {
                Type::HighCard => self.card_cmp(other),
                _ => Ordering::Less
            },
        };
    }
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let mut hands: Vec<Hand> = parse_file(reader);      
    hands.sort();

    let mut result: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        let parsed: u64 = i.try_into().expect("Could not convert to u64");
        result += hand.bid * (parsed + 1);
    }

    println!("Result: {}", result);
}

fn parse_file(reader: BufReader<File>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    for line in reader.lines().into_iter() {
       let mut cards_map: HashMap<char, i32> = Default::default();

       let mut cards: Vec<char> = vec![];
       let split_line = line.as_ref().unwrap().split_ascii_whitespace().collect::<Vec<&str>>();
       for c in split_line[0].chars(){
            cards.push(c.to_owned());

            if let Some(value) = cards_map.get(&c) {
                cards_map.insert(c, value + 1);
            } else {
                cards_map.insert(c, 1);
            }
       }
      
        let bet = split_line[1].parse::<u64>().expect("Could not parse bet");
        let hand_type = Type::from(cards_map);
        hands.push(Hand::new(cards, bet, hand_type));
    }
    hands
}
