use std::io::{self, BufRead};

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

fn update_min(a: &mut i32, b: i32) {
    if b < *a {
        *a = b;
    }
}
fn update_max(a: &mut i32, b: i32) {
    if b > *a {
        *a = b;
    }
}

struct Board {
    x: usize,
    y: usize,
    query_y: usize,
    cells: Vec<Cell>,
}
impl Board {
    fn new(x: usize, y: usize, query_y: usize) -> Board {
        Board {
            x,
            y,
            query_y,
            cells: vec![Cell::Empty; x ] ,
        }
    }
    fn print(&self) {
            print!("{}", self.query_y);
            for x in 0..self.x {
                print!("{}", self.cells[x].to_char());
            }
            println!();
    }
    fn mark_closed(&mut self, p1: Pair, min_x: i32, min_y: i32) {
        let dist = p1.manhattan_distance();

        let p = p1.move_to_board(Coord::new(-min_x, -min_y));

        if (self.query_y as i32) > p.Sensor.y +dist && ( self.query_y as i32) < p.Sensor.y - dist
            && (self.query_y as i32) > p.Beacon.y +dist && (self.query_y as i32) < p.Beacon.y - dist {
            return;
        }

        for x in p.Sensor.x - dist..=p.Sensor.x + dist {
            for y in p.Sensor.y - dist..=p.Sensor.y + dist {
                if y as usize !=self.query_y {
                    continue;
                }
                if (x - p.Sensor.x).abs() + (y - p.Sensor.y).abs() <= dist {
                    if self.cells[x as usize] != Cell::Beacon {
                        self.cells[x as usize] = Cell::Closed;
                    }
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_dist = 0;
    let mut sensors = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        // Parse the i32 from the  line
        // example:
        // Sensor at x=489739, y=1144461: closest beacon is at x=-46516, y=554951
        // into a struct Pair
        let words = line.split(|c| ",: =".contains(c)).collect::<Vec<&str>>();
        // take 4th and 7th word as x and y
        // take 14th and 17th word as x and y
        // and create a Pair

        let x = words[3].parse::<i32>().unwrap();
        let y = words[6].parse::<i32>().unwrap();
        let x2 = words[13].parse::<i32>().unwrap();
        let y2 = words[16].parse::<i32>().unwrap();
        let pair = Pair {
            Sensor: Coord { x, y },
            Beacon: Coord { x: x2, y: y2 },
        };
        sensors.push(pair);
        update_min(&mut min_x, pair.Sensor.x);
        update_min(&mut min_y, pair.Sensor.y);
        update_max(&mut max_x, pair.Sensor.x);
        update_max(&mut max_y, pair.Sensor.y);

        update_min(&mut min_x, pair.Beacon.x);
        update_min(&mut min_y, pair.Beacon.y);
        update_max(&mut max_x, pair.Beacon.x);
        update_max(&mut max_y, pair.Beacon.y);
        update_max(&mut max_dist, pair.manhattan_distance());
    }
    min_x -= max_dist;
    min_y -= max_dist;
    max_x += max_dist;
    max_y += max_dist;
    let query_y: usize = (2000000- min_y) as usize;
    // let query_y: usize = (10 - min_y) as usize;
    println!(
        "min_x: {}, min_y: {}, max_x: {}, max_y: {}, max_dist: {}, query_y: {}",
        min_x, min_y, max_x, max_y, max_dist, query_y
    );
    let mut board = Board::new((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize, query_y);
    for pair in &sensors {
        if pair.Sensor.y - min_y == board.query_y as i32 {
            board.cells[(pair.Sensor.x - min_x) as usize] = Cell::Sensor;
        }
        if pair.Beacon.y - min_y == board.query_y as i32 {
            board.cells[(pair.Beacon.x - min_x) as usize] = Cell::Beacon;
        }
        // board.cells[(pair.Sensor.x - min_x) as usize][(pair.Sensor.y - min_y) as usize] =
        //     Cell::Sensor;
        // board.cells[(pair.Beacon.x - min_x) as usize][(pair.Beacon.y - min_y) as usize] =
        //     Cell::Beacon;
    //     board.mark_closed(pair, min_x, min_y);
    }
    dbg!(sensors.len());

    for x in 0..board.x {
        let coord = Coord::new(x as i32 + min_x, board.query_y as i32 + min_y);
        for pair in &sensors {
            let dist = pair.manhattan_distance();
            if (coord.x - pair.Sensor.x).abs() + (coord.y - pair.Sensor.y).abs() <= dist && board.cells[x] == Cell::Empty {
                board.cells[x] = Cell::Closed;
            }
        }
    }

    // board.print();

    let mut count = 0;
    for x in 0..board.x {
        // print!("{}", board.cells[x].to_char());

        if board.cells[x] == Cell::Closed {
            count += 1;
        }
    }
    println!("count: {}", count);
}
