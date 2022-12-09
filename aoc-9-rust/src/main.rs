use std::collections::HashMap;
use std::io::{self, BufRead};
// Enum to keep four directions
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
// Construct a new Direction from string U,D,L,R
impl Direction {
    fn from_str(s: &str) -> Option<Direction> {
        match s {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }
}

//  Struct to keep a point in 2D space
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
// Calculate distance between points
impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn Move(p: &mut Point, t: &mut Point, d: &Direction, n: i32, visited: &mut HashMap<Point, i32>) {
    for i in 0..n {
        MoveOne(p, d);
        MoveTail(t, p);
        visited.insert(*t, 0);
    }
}

fn MoveOne(p: &mut Point, d: &Direction) {
    match d {
        Direction::Up => p.y += 1,
        Direction::Down => p.y -= 1,
        Direction::Left => p.x -= 1,
        Direction::Right => p.x += 1,
    }
}

fn MoveTail(p: &mut Point, head: &Point ) {
    //move tail to head
    let mut dirs = Vec::new();

    if p.x < head.x-1 {
        dirs.push(Direction::Right);
        if p.y < head.y {
            dirs.push(Direction::Up);
        } else if p.y > head.y {
            dirs.push(Direction::Down);
        }
    } else if p.x > head.x+1 {
        dirs.push(Direction::Left);
        if p.y < head.y {
            dirs.push(Direction::Up);
        } else if p.y > head.y {
            dirs.push(Direction::Down);
        }
    } else if p.y < head.y-1 {
        dirs.push(Direction::Up);
        if p.x < head.x {
            dirs.push(Direction::Right);
        } else if p.x > head.x {
            dirs.push(Direction::Left);
        }
    } else if p.y > head.y+1 {
        dirs.push(Direction::Down);
        if p.x < head.x {
            dirs.push(Direction::Right);
        } else if  p.x > head.x  {
            dirs.push(Direction::Left);
        }
    }
    for d in dirs {
        MoveOne(p, &d);
    }
}
#[test]
fn test_move_tail() {
    let testcases = vec![
        (Point { x: 0, y: 0 }, Point { x: 0, y: 0 }, Point { x: 0, y: 0 }),
        (Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 1, y: 0 }),
        (Point { x: 0, y: 0 }, Point { x: 0, y: 1 }, Point { x: 0, y: 1 }),
        (Point { x: 0, y: 0 }, Point { x: 1, y: 1 }, Point { x: 1, y: 1 }),
        (Point { x: 2, y: 0 }, Point { x: 0, y: 0 }, Point { x: 1, y: 0 }),
        (Point { x: 0, y: 2 }, Point { x: 0, y: 0 }, Point { x: 0, y: 1 }),
        (Point { x: 1, y: 2 }, Point { x: 0, y: 0 }, Point { x: 1, y: 1 }),

    ];
    for (head, tail, expected) in testcases {
        let mut t = tail;
        MoveTail(&mut t, &head);
        assert_eq!(t, expected, "head: {:?}, tail: {:?}, expected: {:?}", head, tail, expected);
    }
}

// a hashmap of point for visited points

fn read_input() -> Vec<(Direction, i32)> {
    // read stdin line by line
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    // Parse directions and steps from every line
    let mut result = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let (dir, mut steps) = line.split_at(1);
        steps = steps.trim();
        let dir = Direction::from_str(dir).unwrap();
        let steps = steps.parse::<i32>().unwrap();
        result.push((dir, steps));
    }
    result
}

fn print_visited(visited: &HashMap<Point, i32>, size: i32    ) {
    println!("=======================");
    for i in 0..size {
        for j in 0..size {
            let p = Point { x: j, y: i };
            if visited.contains_key(&p) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}


fn main() {
    let input = read_input();

    let mut PointHead = Point { x: 0, y: 0 };
    let mut PointTail = Point { x: 0, y: 0 };

    let mut visited = HashMap::new();

    for m in input {
        Move(&mut PointHead, &mut PointTail, &m.0, m.1, & mut visited);
        print_visited(&visited, 10);
    }
    println!("{:?}", visited.len());
}
