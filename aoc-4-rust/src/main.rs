use log::warn;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Range {
    begin : i32,
    end : i32
}
impl Range {
    fn Contains (&self, other: &Range) -> bool {
        return self.begin<= other.begin && self.end>=other.end 
    }
    fn Overlaps (&self, other: &Range) -> bool {
        return
         self.end >= other.begin && self.begin<=other.begin 


    }
    fn new(s: &str) -> Range {
        let mut sp = s.split("-");
        let r1 = sp.next().unwrap();
        let r2 = sp.next().unwrap();

        let mut r = Range{begin:0,end:0};

        match r1.parse::<i32>() {
            Ok(n) => r.begin=n,
            Err(e) => warn!("Can't parse : {}", e),
        }
        match r2.parse::<i32>() {
            Ok(n) => r.end=n,
            Err(e) => warn!("Can't parse : {}", e),
        }
        return r;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Contains() -> Result<(), String> {
        let r1 = Range{begin:1, end: 5};
        let r2 = Range{begin:4, end: 6};
        let r3 = Range{begin:1, end: 4};

        let b1=r1.Contains(&r2);
        assert_eq!(b1, false);
        
        let b2=r1.Contains(&r3);
        assert_eq!(b2, true);

        Ok(())
    }

    #[test]
    fn test_Overlap() -> Result<(), String> {
        let r1 = Range{begin:2, end: 4};
        let r2 = Range{begin:6, end: 8};

        let b1=r1.Overlaps(&r2);
        assert_eq!(b1, false);

        assert_eq!(
            Range{begin: 7, end: 7}
            .Overlaps(
                &Range{begin:7, end: 12}),
                true
        );

        assert_eq!(
            Range{begin: 1, end: 7}
            .Overlaps(
                &Range{begin:7, end: 7}),
                true
        );
        

        Ok(())
    }
}

fn main() {
    let stdin = io::stdin();
    let mut score = 0;
    for line in stdin.lock().lines() {
        let line_val = line.unwrap();
        //2-4,6-8 
        let mut sp = line_val.split(",");
        let r1 = Range::new(sp.next().unwrap());
        let r2 = Range::new(sp.next().unwrap());
        if r1.Overlaps(&r2) || r2.Overlaps(&r1) {
            score +=1 ;
        }
    }
    println!("score: {}", score)
}