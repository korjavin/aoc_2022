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

fn common3(s1: &String, s2: &String, s3: &String) -> char {
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

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;
    let mut counter =0;
    let mut lines : [String; 3] =["".to_string(),"".to_string(),"".to_string()];
    for line in stdin.lock().lines() {
        let val = line.unwrap();
        lines[counter]=val;
        counter+=1;
        if counter>2 {
        let common = common3(&lines[0], &lines[1], &lines[2]);
        let ord = priority(common);
        print!("{}-{}-", common, ord);
        sum += ord;
        counter=0;
        }
    }
    println!("score: {}", sum)
}
