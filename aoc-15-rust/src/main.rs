use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash )]
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
    // Fn returns vector of coords just on the border of the pair
    // where border defined by manhattan distance between sensor and beacon must be equal to
    // manhattan distance between sensor and point
    fn get_border(&self) -> Vec<Coord> {
        let mut result = Vec::new();
        let distance = self.manhattan_distance();
        const diff: i32 =1;
        for x in self.Sensor.x-distance-diff..=self.Sensor.x+distance+diff {
            let yd = self.Sensor.y + distance - (x - self.Sensor.x).abs()+diff;
            result.push(Coord::new(x, self.Sensor.y+yd));
            if yd != 0 {
                result.push(Coord::new(x, self.Sensor.y-yd));
            }
        }
        result
    }
}

#[test]
// get_border test
fn test_get_border() {
    let mut pair = Pair::new(Coord::new(0, 0), Coord::new(1, 1));
    let mut result = pair.get_border();
    assert_eq!(
        result,
        vec!  [Coord { x: -3, y: 0 }, Coord { x: -2, y: 1 }, Coord { x: -2, y: -1 }, Coord { x: -1, y: 2 }, Coord { x: -1, y: -2 }, Coord { x: 0, y: 3 }, Coord { x: 0, y: -3 }, Coord { x: 1, y: 2 }, Coord { x: 1, y: -2 }, Coord { x: 2, y: 1 }, Coord { x: 2, y: -1 }, Coord { x: 3, y: 0 }],
    );

    pair = Pair::new(Coord::new(0, 0), Coord::new(2, 2));
    result = pair.get_border();
    assert_eq!(
        result,
        vec!  [Coord { x: -5, y: 0 }, Coord { x: -4, y: 1 }, Coord { x: -4, y: -1 }, Coord { x: -3, y: 2 }, Coord { x: -3, y: -2 }, Coord { x: -2, y: 3 }, Coord { x: -2, y: -3 }, Coord { x: -1, y: 4 }, Coord { x: -1, y: -4 }, Coord { x: 0, y: 5 }, Coord { x: 0, y: -5 }, Coord { x: 1, y: 4 }, Coord { x: 1, y: -4 }, Coord { x: 2, y: 3 }, Coord { x: 2, y: -3 }, Coord { x: 3, y: 2 }, Coord { x: 3, y: -2 }, Coord { x: 4, y: 1 }, Coord { x: 4, y: -1 }, Coord { x: 5, y: 0 }]
    );

    pair = Pair::new(Coord::new(0, 0), Coord::new(1, 0));
    result = pair.get_border();
    assert_eq!(
        result,
        vec!  [Coord { x: -2, y: 0 }, Coord { x: -1, y: 1 }, Coord { x: -1, y: -1 }, Coord { x: 0, y: 2 }, Coord { x: 0, y: -2 }, Coord { x: 1, y: 1 }, Coord { x: 1, y: -1 }, Coord { x: 2, y: 0 }],
    );
    pair = Pair::new(Coord::new(0, 0), Coord::new(-1, 0));
    result = pair.get_border();
    assert_eq!(
        result,
        vec!  [Coord { x: -2, y: 0 }, Coord { x: -1, y: 1 }, Coord { x: -1, y: -1 }, Coord { x: 0, y: 2 }, Coord { x: 0, y: -2 }, Coord { x: 1, y: 1 }, Coord { x: 1, y: -1 }, Coord { x: 2, y: 0 }],
    );
    pair = Pair::new(Coord::new(0, 0), Coord::new(0, 1));
    result = pair.get_border();
    assert_eq!(
        result,
        vec!  [Coord { x: -2, y: 0 }, Coord { x: -1, y: 1 }, Coord { x: -1, y: -1 }, Coord { x: 0, y: 2 }, Coord { x: 0, y: -2 }, Coord { x: 1, y: 1 }, Coord { x: 1, y: -1 }, Coord { x: 2, y: 0 }],
    );

    pair = Pair::new(Coord::new(5, 0), Coord::new(5, 1));
    result = pair.get_border();
    assert_eq!(
        result,
        vec!  [Coord { x: 3, y: 0 }, Coord { x: 4, y: 1 }, Coord { x: 4, y: -1 }, Coord { x: 5, y: 2 }, Coord { x: 5, y: -2 }, Coord { x: 6, y: 1 }, Coord { x: 6, y: -1 }, Coord { x: 7, y: 0 }],
    );
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


    let mut counters = HashMap::new();

    for pair in &sensors {
        for c in pair.get_border() {
            let counter = counters.entry(c).or_insert(0);
            *counter += 1;
        }
    }
    dbg!(counters.len());

    for (c,v)  in counters {
        if c.x <= 0 || c.y <= 0 || c.x >= SIZE as i32 || c.y >= SIZE as i32 {
            continue;
        }
        if v> 1 {
            dbg!(c,v);
        }
    }

}
