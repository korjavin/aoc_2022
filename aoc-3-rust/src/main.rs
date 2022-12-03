use std::io::{self, BufRead};

fn split_string(mut s: String, byte_index: usize) -> (String, String)
{
    let tail = s[byte_index..].into();
    s.truncate(byte_index);
    (s, tail)
}


fn main() {
    let stdin = io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let val=line.unwrap();
        let size=val.len();
        let (part1,part2) = split_string(val,size/2 );
        let mut common = 'a'  ;
        let minus: u32 = 'a'.into();
        let MINUS: u32 = 'A'.into();
        for i in part1.chars() {
            if part2.contains(i) {
                common=i;
                break;
            }
        }
            let b: u32 = common.into();
            let mut ord :u32 = 0;
            if b>96 {
                ord=b-minus+1;
            } else {
                ord=b-MINUS+27;
            }
            print!("{}-{}-",common,ord);
            sum+=ord;
        println!("{},{}",part1, part2);
    }
    println!("score: {}", sum)
}
