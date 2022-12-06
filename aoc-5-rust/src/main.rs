use std::io::{self, BufRead};
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

// Function parses stdin up to empty line, and returns
// a vector of strings.
fn parse_first_part() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    lines
}

// Function takes a vector of strings and return a vector of strings by columns.
fn parse_first_lines(lines: Vec<String>) -> Vec<String> {
    let mut columns = Vec::new();
    for _ in 0..lines[0].len() {
        columns.push(String::new());
    }
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            columns[i].push(c);
        }
    }
    columns
}

// Function takes a vector of string and filter out all who don't ends with a number.
fn filter_first_input(lines: Vec<String>) -> Vec<String> {
    let mut filtered = Vec::new();
    for line in lines {
        if line.chars().last().unwrap().is_numeric() {
            // drop last character of line and drop spaces
            filtered.push(line[..line.len() - 1].trim().to_string());
        }
    }
    filtered
}


// Function takes a vector of string and a tuple of three integers A,B, and C,
// and moves first A characters from B to C.
fn move_first_chars( mut lines: Vec<String>, (a, b, c): (i32,i32,i32)) -> Vec<String> {
    let mut chars_to_move = Vec::new();

// a times push first char of lines[b] to chars_to_move
    for _ in 0..a {
        chars_to_move.push((lines[b as usize-1]).remove(0));
    }
    // println!("chars_to_move: {:?}", chars_to_move);

    // convert chars_to_move to string
    let string_to_move = chars_to_move.into_iter().rev().collect::<String>();
    
    lines[c as usize-1]= format!("{}{}", string_to_move, lines[c as usize-1]);

    lines
}

// // test for move_first_chars
#[test]
fn test_move_first_chars() {
    let lines = vec!["abcde".to_string(), "fghij".to_string()];
    let new_lines = move_first_chars(lines, (2, 1, 2));
    assert_eq!(new_lines, vec!["cde".to_string(), "bafghij".to_string()]);
}

// Function takes vector of Sting and returna a string concatenated from the last char of every string.
fn concat_last_chars(lines: Vec<String>) -> String {
    let mut result = String::new();
    for line in lines {
        result.push(line.chars().last().unwrap());
    }
    result
}

// Function takes vector of Sting and returna a string concatenated from the first char of every string.
fn concat_first_chars(lines: Vec<String>) -> String {
    let mut result = String::new();
    for line in lines {
        result.push(line.chars().next().unwrap());
    }
    result
}
// test for concat_first_chars
#[test]
fn test_concat_first_chars() {
    let lines = vec!["abcde".to_string(), "fghij".to_string()];
    let result = concat_first_chars(lines);
    assert_eq!(result, "af");
}

// Function parses stdin and slpit line by whitespaces and convert parts in position 1, 3, 5 to i32.
fn parse_second_part() -> Vec<(i32, i32, i32)> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" ");
        let first = parts.nth(1).unwrap().parse::<i32>().unwrap();
        let second = parts.nth(1).unwrap().parse::<i32>().unwrap();
        let third = parts.nth(1).unwrap().parse::<i32>().unwrap();
        lines.push((first, second, third));
    }
    lines
}

fn main() {
    // print first part of stdin
    let lines = parse_first_part();
    let mut input = filter_first_input(parse_first_lines(lines));
    println!("{:?}", input);

    let moves = parse_second_part();
    println!("{:?}", moves);

    for m in moves {
        input=move_first_chars(input, m);
        println!("{:?}",input );
    }
    println!("{:?}",input );
    println!("{:?}", concat_first_chars(input) );
}
