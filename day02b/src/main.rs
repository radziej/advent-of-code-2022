use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut total_score: i32 = 0;
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let (opponent, outcome) = line
            .split_once(" ")
            .expect("Strategy guide should be formatted as opponent choice and outcome");
        let opponent = match opponent {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!(),
        };
        let outcome = match outcome {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => unreachable!(),
        };

        total_score += outcome;
        total_score += match outcome {
            0 => offset_modulo(opponent - 1, 3, 1),
            3 => opponent,
            6 => offset_modulo(opponent + 1, 3, 1),
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

fn offset_modulo(value: i32, modulus: i32, offset: i32) -> i32 {
    (value - offset).rem_euclid(modulus) + offset
}
