// First symbol must be on.
const IMG: &str = "
██░░░██░░░░░░░░██░░░░░░░░██████░░░░░░░░░░████░░░░░░██░░░░░░░░░░░░░░░░░░░░░░░░░░░
██░░░░██░░░░░░██░░░░░░░░██░░░░██░░░░░░░░░██░██░░░░░██░░░░░░░░░░░░░░██░░░░░░░░░░░
██░░░░██░░░░░░██░░░░░░░██░░░░░░██░░░░░░░░██░░██░░░░██░░░░░░░░░░░░██░░██░░░░░░░░░
██░░░░░██░░░░██░░░░░░░░██████████░░░░░░░░██░░░██░░░██░░░░░░░░░░░░░░██░░░░░░██░░░
██░░░░░░██░░██░░░░░░░░██░░░░░░░░██░░░░░░░██░░░░██░░██░░░░░░░░░░░░░░░░░░░░██░░██░
██░░░░░░░░██░░░░░░░░░██░░░░░░░░░░██░░░░░░██░░░░░██░██░░░░░░░░░░░░░░░░░░░░░░██░░░
";

fn main() {
    // parset IMG by line matching by two chars in to a vector of bools
    let img: Vec<Vec<bool>> = IMG
        .lines()
        .skip(1)
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|line| line.chunks(2).map(|chunk| chunk[0] == '░').collect())
        .collect();

    let mut register: i32 = 1;
    let mut tick = 1;
    for line in img {
        for pixel in line {
            let x_coord = (tick) % 40 ;
            if pixel {
                //pixel is on
                if (x_coord - register).abs() < 2 {
                    // We have it on already
                    println!("noop");
                } else {
                    // we have to move register to x_coord
                    println!("addx {}", register - x_coord);
                    register = x_coord;
                }
            } else {
                // pixel is off
                if (x_coord - register).abs() > 2 {
                    // Fine, we have it off already
                    println!("noop");
                } else {
                    let mut add = 3;
                    if x_coord == 40 {
                        add = -3;
                    }
                    println!("addx {}", add);
                    register += add;
                }
            }
            tick += 1;
        }
    }
}
