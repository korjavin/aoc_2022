use std::io::{self, BufRead};
//Type counter that holds a value and increments it
struct Counter {
    tick: i32,
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0, tick: 1 }
    }
    fn tick(&mut self, x: i32) {
        self.tick += 1;
        if (self.tick - 20) % 40 == 0 {
            // dbg!(x);
            let add = x * self.tick;
            self.count += add;
            // dbg!(self.tick,self.count,add);
        }
    visualize(self.tick-1, x);
    }
}
#[test]
// Test to see if the counter tick increments the count
fn test_counter_tick() {
    let mut counter = Counter::new();
    counter.tick(1);
    assert_eq!(counter.tick, 1);
    for i in 0..20 {
        counter.tick(1);
    }
    assert_eq!(counter.tick, 21);
    assert_eq!(counter.count, 20);
    for i in 0..20 {
        counter.tick(2);
    }
    assert_eq!(counter.tick, 41);
    assert_eq!(counter.count, 20 + 40 * 2);
}

const size_x: usize = 40;
const size_y: usize = 6;

fn visualize(tick: i32, x: i32) {

    let x_coord = (tick) % size_x as i32;

    if x_coord == 0 {
        println!();
    }

    if (x_coord-x).abs() < 2 {
        print!("██");
    } else {
        print!("░░");
    }

}

fn main() {
    let mut x: i32 = 1;
    let mut counter = Counter::new();
    // read the stdin line by line and match based on the first word
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        match words.next() {
            Some("noop") => {
                counter.tick(x);
            }
            Some("addx") => {
                let (_, mut steps) = line.split_at(5);
                steps = steps.trim();
                let steps = steps.parse::<i32>().unwrap();
                // dbg!(steps);
                counter.tick(x);
                x += steps;
                counter.tick(x);
            }
            _ => {
                println!("unknown command");
            }
        }
    }
    println!("Final count: {}", counter.count);
}
