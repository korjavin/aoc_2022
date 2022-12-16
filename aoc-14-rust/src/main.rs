use std::io::{self, BufRead};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Rock,
    Air,
    Sand,
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Rock => '#',
            Cell::Air => '.',
            Cell::Sand => 'o',
        }
    }
}
const SIZE_X: usize = 1200;
const SIZE_Y: usize = 159;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Board {
    cells: [[Cell; SIZE_Y]; SIZE_X],
}
impl Board {
    fn new() -> Board {
        let mut board = [[Cell::Air; SIZE_Y]; SIZE_X];
        Board{cells: board}
    }
    fn print(&self) {
        for y in 0..SIZE_Y {
            for x in 0..SIZE_X {
                print!("{}", self.cells[x][y].to_char());
            }
            println!();
        }
    }
    // put rocks at given path
    fn put_rocks(&mut self, path: &Vec<Coord>) {
        let mut prev = Option::<Coord>::None;
        for p in path {
            match prev {
                Some(point) => {
                    let mut x = point.x;
                    let mut y = point.y;
                    self.cells[p.x][p.y] = Cell::Rock;
                    while x != p.x || y != p.y {
                        self.cells[x][y] = Cell::Rock;
                        if x < p.x {
                            x += 1;
                        } else if x > p.x {
                            x -= 1;
                        }
                        if y < p.y {
                            y += 1;
                        } else if y > p.y {
                            y -= 1;
                        }
                    }
                    prev = Some(*p);
                },
                None => {
                    prev = Some(*p);
                },
            }
        }
    }
    // move sand coord -> coord 
    fn move_sand(&mut self, from: Coord) -> Coord {
        if self.cells[from.x][ from.y+1]== Cell::Air {
            return Coord{x: from.x, y: from.y+1};
        } else if  self.cells[from.x-1][from.y+1] == Cell::Air {
            return Coord{x: from.x-1, y: from.y+1};
        } else if   self.cells[from.x+1][from.y+1] == Cell::Air {
            return Coord{x: from.x+1, y: from.y+1};
        } else {
            return from;
        }
    }

    // put sand at given coord
    fn put_sand(&mut self, coord: Coord) -> bool {
        let mut sand = coord;
        for _ in 0..SIZE_Y-2 {
            let new_sand = self.move_sand(sand);
            if sand == new_sand {
                self.cells[sand.x][sand.y] = Cell::Sand;
                return true;
            } else {
                sand = new_sand;
            }
        }
        return false; // can't hold
    }
}

    #[test]
    fn test_move_sand() {
        let mut board = Board::new();
        assert_eq!(board.move_sand(Coord{x: 5, y: 5}), Coord{x: 5, y: 6});
        board.cells[5][6] = Cell::Rock;
        assert_eq!(board.move_sand(Coord{x: 5, y: 5}), Coord{x: 4, y: 6});
        board.cells[4][6] = Cell::Rock;
        assert_eq!(board.move_sand(Coord{x: 5, y: 5}), Coord{x: 6, y: 6});
        assert_eq!(board.move_sand(Coord{x: 7, y: 0}), Coord{x: 7, y: 1});
    }

fn main() {

    let mut board = Board::new();

    // read stdin line by line parse every line as a vec of coords
    let stdin = io::stdin();
    let mut max_y = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut path = Vec::<Coord>::new();
        for coord in line.split(" -> ") {
            let mut coord = coord.split(',');
            let x = coord.next().unwrap().parse::<usize>().unwrap();
            let y = coord.next().unwrap().parse::<usize>().unwrap();
            path.push(Coord{x: x, y: y});
            if y > max_y {
                max_y = y;
            }
        }
        board.put_rocks(&path);
    }
    dbg!(max_y);
    board.put_rocks(&vec![Coord{x: 0, y: max_y+2}, Coord{x: SIZE_X-1, y: max_y+2}]);

    let mut counter = 0;
    let mut  more = true;
    while more {
        if board.put_sand(Coord{x: 500, y: 0}) {
            counter += 1;
            if board.cells[500][0] == Cell::Sand {
                more = false;
            }
        } else {
            more = false;
        }
    }
    // board.print();

    println!("counter: {}", counter);
}
