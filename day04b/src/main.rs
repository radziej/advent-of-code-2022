use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut num_overlapping = 0;
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");

        let assignments: Vec<[u32; 2]> = line
            .split(",")
            .map(|range| {
                range
                    .split("-")
                    .map(|b| {
                        b.parse::<u32>()
                            .expect("Range boundaries should be small integers")
                    })
                    .collect::<Vec<u32>>()
                    .try_into()
                    .expect("Range boundaries should be pairs")
            })
            .collect();

        if !(assignments[0][0] > assignments[1][1] || assignments[0][1] < assignments[1][0]) {
            num_overlapping += 1;
        }
    }

    println!(
        "Number of assigned pairs where one range overlaps with the other: {}",
        num_overlapping
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
