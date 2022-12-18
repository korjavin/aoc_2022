use std::io::{self, BufRead};
use die::Die;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Beacon,
    Sensor,
    Closed,
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Beacon => 'B',
            Cell::Sensor => 'S',
            Cell::Closed => '#',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pair {
    Sensor: Coord,
    Beacon: Coord,
}
impl Pair {
    fn new(sensor: Coord, beacon: Coord) -> Self {
        Self {
            Sensor: sensor,
            Beacon: beacon,
        }
    }
    fn manhattan_distance(&self) -> i32 {
        (self.Sensor.x - self.Beacon.x).abs() + (self.Sensor.y - self.Beacon.y).abs()
    }
    fn move_to_board(&self, direction: Coord) -> Self {
        Self {
            Sensor: Coord::new(self.Sensor.x + direction.x, self.Sensor.y + direction.y),
            Beacon: Coord::new(self.Beacon.x + direction.x, self.Beacon.y + direction.y),
        }
    }
}

struct Board {
    x: usize,
    cells: Vec<Cell>,
}
impl Board {
    fn new(x: usize) -> Board {
        Board {
            x,
            cells: vec![Cell::Empty; x],
        }
    }
    fn print(&self) {
        for x in 0..self.x {
            print!("{}", self.cells[x].to_char());
        }
        println!();
    }
    fn clear(&mut self) {
        for x in 0..self.x {
            self.cells[x] = Cell::Empty;
        }
    }
}

const SIZE: usize = 4000000;
// const SIZE: usize = 20;
fn main() {
    let stdin = io::stdin();
    let mut sensors = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let words = line.split(|c| ",: =".contains(c)).collect::<Vec<&str>>();

        let x = words[3].parse::<i32>().unwrap();
        let y = words[6].parse::<i32>().unwrap();
        let x2 = words[13].parse::<i32>().unwrap();
        let y2 = words[16].parse::<i32>().unwrap();
        let pair = Pair {
            Sensor: Coord { x, y },
            Beacon: Coord { x: x2, y: y2 },
        };
        sensors.push(pair);
    }
    dbg!(SIZE);
    dbg!(sensors.len());
    //
    // for pair in &sensors {
    //     dbg!(pair.manhattan_distance());
    // }

    let mut board = Board::new(SIZE);
    for y in 0..SIZE{
        // mark all closed
        // print!("{}", y);
        board.clear();
        for x in 0..SIZE{
            for pair in &sensors {
                let dist = pair.manhattan_distance();
                if (x as i32 - pair.Sensor.x).abs() + (y as i32 - pair.Sensor.y).abs() <= dist
                    && board.cells[x] == Cell::Empty
                {
                    board.cells[x] = Cell::Closed;
                }
            }
            if board.cells[x] == Cell::Empty {
                println!("{} {}", x as i32, y as i32);
                die::die!();
            }
            // print!("{}", board.cells[x].to_char());
        }
        // println!("");
    }
}
