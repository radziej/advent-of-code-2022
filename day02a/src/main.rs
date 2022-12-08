use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut total_score: i32 = 0;
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let (opponent, player) = line
            .split_once(" ")
            .expect("Strategy guide should be formatted as opponent and player choice");
        let opponent = match opponent {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!(),
        };
        let player = match player {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!(),
        };

        total_score += player;
        total_score += match opponent - player {
            0 => 3,
            1 | -2 => 0,
            -1 | 2 => 6,
            _ => unreachable!(),
        };
    }

    println!("Total score when following the strategy guide {total_score}");
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
