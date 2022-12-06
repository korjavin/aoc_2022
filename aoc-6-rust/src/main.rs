use std::io::{self, BufRead};
use die::die;

// function that checks that all the characters in a string are unique
fn is_unique(s: &str) -> bool {
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if chars.clone().any(|x| x == c) {
            return false;
        }
    }
    true
}

#[test]
fn test_is_unique() {
    assert!(is_unique("abcd"));
    assert!(!is_unique("abccd"));
}


fn main() {
    let stdin = io::stdin();
    let mut datastream = "".to_string();
    for line in stdin.lock().lines() {
        datastream= line.unwrap();
            break;
    };
    
    println!("datastream: {}", datastream.len());
    for i in 0..datastream.len()-4 {
        let sub = &datastream[i..i+4];
        if is_unique(sub) {
            println!("{}, {}",sub,i+4);
            die!();
        } 
    }
}
