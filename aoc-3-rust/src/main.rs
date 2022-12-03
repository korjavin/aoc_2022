use std::io::{self, BufRead};

fn split_string(mut s: String, byte_index: usize) -> (String, String) {
    let tail = s[byte_index..].into();
    s.truncate(byte_index);
    (s, tail)
}

fn priority(c: char) -> u32 {
    let minus: u32 = 'a'.into(); //TODO const
    let MINUS: u32 = 'A'.into(); //TODO const
    let b: u32 = c.into();
    let mut ord: u32 = 0;
    if b > 96 {
        ord = b - minus + 1;
    } else {
        ord = b - MINUS + 27;
    }
    return ord;
}

fn common(s1: &String, s2: &String) -> char {
    let mut common = 'a';
    for i in s1.chars() {
        if s2.contains(i) {
            common = i;
            break;
        }
    }
    return common;
}

fn common3(s1: &str, s2: &str, s3: &str) -> char {
    let mut common = 'a';
    for i in s1.chars() {
        if s2.contains(i) {
            if s3.contains(i) {
                common = i;
                break;
            }
        }
    }
    return common;
}

fn bit_encode(a: char) -> u64 {
    let code = a as u32;
    let out = if code > 'Z' as u32 {
        1 + code - 'a' as u32
    } else {
        27 + code - 'A' as u32
    };

    1u64 << out
}

fn string_to_one_hot(input: &str) -> u64 {
    input.chars().map(bit_encode).reduce(|i, j| i | j).unwrap()
}

fn one_hot_to_num(input: u64) -> i32 {
    f64::log2(input as f64) as i32
}

fn common3_hot(s1: &str, s2: &str, s3: &str) -> char {
    let common_bit= string_to_one_hot(s1) & string_to_one_hot(s2) & string_to_one_hot(s3);
    std::char::from_u32(one_hot_to_num(common_bit)  as u32).unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;
    
    for line in stdin.lock().lines().collect::<Vec<_>>().chunks(3) {
        let common = common3_hot(
            line[0].as_ref().unwrap(),
            line[1].as_ref().unwrap(),
            line[2].as_ref().unwrap()
        );
        let ord = priority(common);
        print!("{}-{}-", common, ord);
        sum += ord;
    }
    println!("score: {}", sum)
}
