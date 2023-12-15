use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

#[derive(PartialEq,Eq, Hash, Debug)]
struct Point(usize, usize);
fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut gear_map: HashMap<Point, Vec<String>> = HashMap::new();

    let gear_symbols = vec!['*'];
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for (i, _) in lines.iter().enumerate() {
        handle_line(i, &lines, &gear_symbols, &mut gear_map);
    }

    let sum = gear_map.values()
    .filter(|array| array.len() == 2)
    .map(|array| array.iter().fold(1, |acc, g|{
        acc * g.parse::<u32>().unwrap()
    }))
    .fold(0, |acc, v| {
        acc + v
    });

    println!("The sum of gears is: {}", sum);
}

fn handle_line(
    i: usize,
    lines: &Vec<String>,
    gear_symbols: &Vec<char>,
    gear_map: &mut HashMap<Point, Vec<String>>,
) {
    let line = &lines[i];

    let mut current_number = String::new();
    let mut start_index: Option<usize> = None;
    let mut end_index: Option<usize> = None;

    for (j, c) in line.char_indices() {
        if c.is_digit(10) {
            update_indices(&mut start_index, &mut end_index, j);
            current_number.push(c);

            if j == line.len() - 1 {
                match (start_index, end_index) {
                    (Some(start), Some(end)) => {
                        fill_adjacent_gears(i,start,end,lines,gear_symbols,&current_number, gear_map);
                        clear_params(&mut start_index, &mut end_index, &mut current_number);
                    }
                    _ => (),
                }
            }
        } else {
            if !current_number.is_empty() {
                match (start_index, end_index) {
                    (Some(start), Some(end)) => {
                        fill_adjacent_gears(i ,start,end,lines, gear_symbols, &current_number, gear_map);
                        clear_params(&mut start_index, &mut end_index, &mut current_number);
                    }
                    _ => (),
                }
            }
        }
    }
}

fn update_indices(start: &mut Option<usize>, end: &mut Option<usize>, index: usize) {
    if start.is_none() {
        *start = Some(index);
    }
    *end = Some(index);
}

fn clear_params(start: &mut Option<usize>, end: &mut Option<usize>, current_number: &mut String) {
    *start = None;
    *end = None;
    current_number.clear();
}

fn fill_adjacent_gears(i: usize, start_index: usize, end_index: usize,lines: &Vec<String>, gear_symbols: &Vec<char>,current_number: &String, gear_map: &mut HashMap<Point, Vec<String>>,
) {
    let width = lines[i].len();

    let (left, right) = clamp_char_bounds(width, (start_index as isize) - 1, end_index + 1);

    let current_line_chars = &lines[i][left..=right];
    fill_gears(current_line_chars, i, left, gear_map, current_number, gear_symbols);
    
    if i != 0 {
        let top_chars  = &lines[i - 1][left..=right];
        fill_gears(top_chars, i - 1, left, gear_map, current_number, gear_symbols);
    }

    if i != lines.len() - 1 {
        let bot_chars = &lines[i + 1][left..=right];
        fill_gears(bot_chars, i + 1, left, gear_map, current_number, gear_symbols);
    }
}

fn clamp_char_bounds(width: usize, left: isize, right: usize) -> (usize, usize) {
    (left.clamp(0, width as isize) as usize, right.clamp(0, width - 1))
}

fn fill_gears(line: &str, line_index: usize, start_index: usize, gear_map: &mut HashMap<Point, Vec<String>>, num: &String, gear_symbols: &Vec<char>) -> Option<Point> {
    for (i, char) in line.chars().enumerate() {
        if gear_symbols.contains(&char) {
            let column_idx = start_index + i;
            let gear_point = Point(line_index, column_idx);
            let gear_nums = gear_map.get_mut(&gear_point);
            match gear_nums {
                Some(nums) => {
                    nums.push(num.clone())
                },
                None => {
                    gear_map.insert(gear_point, vec![num.clone()]);
                }
            }
        }
    }
    None
}