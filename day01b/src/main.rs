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

    let mut total = 0;
    for _ in 0..3 {
        let (index, maximum) = calories
            .iter()
            .enumerate()
            .max_by_key(|(_, b)| **b)
            .expect("Should be more elves stashes left");
        total += maximum;
        calories.swap_remove(index);
    }
    println!("Sum of top 3 calorie amounts carried by elves {total}");
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
