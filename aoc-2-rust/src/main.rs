use std::io::{self, BufRead};

#[derive(Debug)]
enum FightResult {
    Win,
    Loss,
    Draw
}

impl FightResult {
    fn score (&self) -> i32 {
        match self {
            FightResult::Win => 6,
            FightResult::Draw=> 3,
            FightResult::Loss=> 0
        }
    }
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn new(x:String) -> Result<Move,String>{
        match x.as_str() {
            "A"| "X" => Ok(Move::Rock),
            "B"| "Y" => Ok(Move::Paper),
            "C"| "Z" => Ok(Move::Scissors),
            &_ => Err(x),
        }
    }
    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

}

#[derive(Debug)]
struct Battle {
    my : Move,
    his: Move,
}

impl Battle {
    fn new (s :String) -> Battle {
        let mut split =s.split_whitespace();

        let his= Move::new(split.next().unwrap().to_string());
        let my = Move::new(split.next().unwrap().to_string());

        Battle{
            my: my.unwrap(),
            his: his.unwrap()
        }
    }
    fn result(&self) -> FightResult {
        match self.my {
            Move::Rock => match self.his {
                Move::Rock => FightResult::Draw,
                Move::Paper => FightResult::Loss,
                Move::Scissors => FightResult::Win
            },
            Move::Paper => match self.his {
                Move::Rock => FightResult::Win,
                Move::Paper => FightResult::Draw,
                Move::Scissors => FightResult::Loss
            }
            Move::Scissors=> match self.his {
                Move::Rock => FightResult::Loss,
                Move::Paper => FightResult::Win,
                Move::Scissors => FightResult::Draw
            }
        }
    }
    fn score(&self) -> i32 {
        return self.my.score()+self.result().score() ;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut score = 0;
    for line in stdin.lock().lines() {
        let line_val = line.unwrap();
        score+=Battle::new(line_val).score()
    }
    println!("score: {}", score)
}
