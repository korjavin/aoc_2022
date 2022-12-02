use log::warn;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Top3 {
    data: [i32; 3],
}

impl Top3 {
    fn sum(&self) -> i32 {
        self.data[0] + self.data[1] + self.data[2]
    }
    fn add(&mut self, n: i32) {
        for idx in 0..3 {
            if self.data[idx] < n {
                for j in (idx..3).rev() {
                    if j > 0 {
                        self.data[j] = self.data[j - 1];
                    }
                }
                self.data[idx] = n;
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top3_sum() -> Result<(), String> {
        let x = Top3 { data: [0, 1, 2] };
        assert_eq!(x.sum(), 3);
        Ok(())
    }

    #[test]
    fn test_top3_add() -> Result<(), String> {
        let mut x = Top3 { data: [0, 0, 0] };

        x.add(6);
        println!("{:?}", x.data);
        assert_eq!(x.sum(), 6);

        x.add(4);
        println!("{:?}", x.data);
        assert_eq!(x.sum(), 10);

        x.add(11);
        println!("{:?}", x.data);
        assert_eq!(x.sum(), 21);

        x.add(24);
        println!("{:?}", x.data);
        assert_eq!(x.sum(), 41);

        x.add(10);
        println!("{:?}", x.data);
        assert_eq!(x.sum(), 45);

        Ok(())
    }
}

fn main() {
    let stdin = io::stdin();
    let mut current = 0;
    let mut t3 = Top3 { data: [0, 0, 0] };
    for line in stdin.lock().lines() {
        let line_val = line.unwrap();
        if line_val == "" {
            t3.add(current);

            //   println!("current: {}",current);
            current = 0;
        }
        match line_val.parse::<i32>() {
            Ok(n) => current += n,
            Err(e) => warn!("Can't parse : {}", e),
        }
    }
    println!("maximum: {}", t3.sum())
}
