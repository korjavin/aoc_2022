use std::io::{self, BufRead};

fn max(numbers: &Vec<usize>) -> usize {
    let mut max = numbers[0];
    for number in numbers {
        if number > &max {
            max = *number;
        }
    }
    max
}


fn is_bigger_than_any(number: usize, numbers: &Vec<usize>) -> bool {
    return number > max(numbers);
}

fn is_visible(x: usize, y: usize, map: &Vec<Vec<usize>>) -> bool {
    if x <= 0 || y <= 0 || x == map[0].len()-1 || y == map.len()-1 {
        return true;
    }
    let val = map[x][y];
    let mut vec = Vec::new();
    for i in 0..x  {
        vec.push(map[i][y]);
    }
    let top = is_bigger_than_any(val, &vec) ;
    vec = Vec::new();
    for i in x + 1..map[0].len() {
        vec.push(map[i][y]);
    }
    let bottom  = is_bigger_than_any(val, &vec) ;
    vec = Vec::new();
    for i in y + 1..map.len() {
        vec.push(map[x][i]);
    }
    let left = is_bigger_than_any(val, &vec) ;
    vec = Vec::new();
    for i in 0..y  {
        vec.push(map[x][i]);
    }
    let right = is_bigger_than_any(val, &vec) ;

    return top || bottom || left || right;
}

fn main() {
    // read stdin by lines and put it into a vector of strings
    let mut lines = Vec::new();
    for line in io::stdin().lock().lines() {
        let mut numbers = Vec::new();
        for digit in line.unwrap().chars() {
            numbers.push(digit.to_digit(10).unwrap() as usize);
        }
        lines.push(numbers);
    }

    let mut counter =0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if is_visible(j, i, &lines) {
                print!("X");
                counter+=1;
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("sum visible = {}", counter);
}
