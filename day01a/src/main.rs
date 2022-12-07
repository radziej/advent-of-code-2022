use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut current_elf: u32 = 0;
    let mut calories: Vec<u32> = Vec::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        if line.is_empty() {
            calories.push(current_elf);
            current_elf = 0;
            continue;
        }

        current_elf += line
            .parse::<u32>()
            .expect("Calories should only be medium sized integers");
    }
    println!(
        "Elf carrying the highest amount of calories carries {} calories",
        calories
            .into_iter()
            .max()
            .expect("Calorie list should contain at least one elf")
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
