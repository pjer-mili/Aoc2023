use std::{fs::File, io::{BufReader, BufRead}, usize};

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut sum = 0;

    let symbols = vec!['*', '=', '+', '/', '#', '-', '&', '$', '@', '%'];
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for (i, _) in lines.iter().enumerate() {
        sum = sum + get_line_sum(&i, &lines, &symbols);
    }

    println!("The sum of numbers adjecent to symbols is: {}", sum);
}

fn get_line_sum(i: &usize, lines: &Vec<String>, symbols: &Vec<char>) -> u32 {
    let line = &lines[*i];

    let mut partial_sum = 0;
    let mut current_number = String::new();
    let mut start_index: Option<usize> = None;
    let mut end_index: Option<usize> = None;

    for (j, c) in line.char_indices() {
        if c.is_digit(10){
            update_indices(&mut start_index, &mut end_index, j);
            current_number.push(c);

            if j == line.len() - 1 {
                    match (start_index, end_index) {
                        (Some(start), Some(end)) => {
                            let has_adjecent = check_adjacent(*i, start, end, lines, symbols);
                            if has_adjecent {
                                partial_sum = partial_sum + current_number.parse::<u32>().unwrap();
                            }
                            clear_params(&mut start_index, &mut end_index, &mut current_number);
                        },
                        _ => ()
                    }
            }
        } else {
                if current_number.len() > 0 {
                    match (start_index, end_index) {
                        (Some(start), Some(end)) => {
                            let has_adjecent = check_adjacent(*i, start, end, lines, symbols);
                            if has_adjecent {
                                partial_sum = partial_sum + current_number.parse::<u32>().unwrap();
                            }
                            clear_params(&mut start_index, &mut end_index, &mut current_number);
                        },
                        _ => ()
                    }
                }    
        }
    }
    partial_sum
}


fn update_indices(start: &mut Option<usize>, end: &mut Option<usize>, index: usize){
    if let None = start{
        *start = Some(index);
    }
    *end = Some(index);
}

fn clear_params(start: &mut Option<usize>, end: &mut Option<usize>, current_number: &mut String) {
    *start = None;
    *end = None;
    current_number.clear();
}

fn check_adjacent(i: usize, start_index: usize, end_index: usize, lines: &Vec<String>, symbols: &Vec<char>) -> bool {

    let width = lines[i].len();
    let rows_length = lines.len();

    let (left, right) = clamp_char_bounds(
        width,
        (start_index as isize) - 1,
        end_index + 1
    );
    let indices = generate_iterable_indices(i, rows_length);
    for line_index in indices {
            let current_line = &lines[line_index];
            let line_chars = &current_line[left..=right];
            if has_symbol(line_chars, symbols) {
                return  true;
            }
    }

    false
}

fn clamp_char_bounds(width: usize, left: isize, right: usize) -> (usize, usize) {
    (left.clamp(0, width as isize) as usize, right.clamp(0, width - 1))
}


fn generate_iterable_indices(i: usize, rows_length: usize) -> Vec<usize> {
    let indices = match i {
        0 => vec![i, i + 1],
        n if n == rows_length - 1 => vec![i - 1, i],
        _ => (i - 1..=i + 1).collect(),
    };
    
    indices
}

fn has_symbol(line: &str, symbols: &Vec<char>) -> bool{
    for c in line.chars(){
        if symbols.contains(&c) {
            return  true;
        }
    }
    false
}
