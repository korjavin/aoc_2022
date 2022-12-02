use std::io::{self, BufRead};

#[derive(Debug)]
enum FightResult {
    Win,
    Loss,
    Draw
}

impl FightResult {
    fn new(x:String) -> Result<FightResult,String>{
        match x.as_str() {
             "X" => Ok(FightResult::Loss),
             "Y" => Ok(FightResult::Draw),
             "Z" => Ok(FightResult::Win),
            &_ => Err(x),
        }
    }
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
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
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
    result: FightResult,
}

impl Battle {
    fn new (s :String) -> Battle {
        let mut split =s.split_whitespace();

        let his= Move::new(split.next().unwrap().to_string());
        let result= FightResult::new(split.next().unwrap().to_string());
        let my= match result.as_ref().unwrap() {
            FightResult::Win=>match his.as_ref().unwrap(){
                Move::Paper=> Move::Scissors,
                Move::Scissors=> Move::Rock,
                Move::Rock=>Move::Paper,
            }
            FightResult::Loss=>match his.as_ref().unwrap() {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors=>Move::Paper,
            },
            FightResult::Draw=>match his.as_ref().unwrap() { // borrow checker
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors=>Move::Scissors,
            },
        };

        Battle{
            my: my,
            his: his.unwrap(),
            result: result.unwrap()
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
        let b=Battle::new(line_val);
        println!("{:?}",b);
        score+=b.score()
    }
    println!("score: {}", score)
}
