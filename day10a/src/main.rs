use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";
    let signal_cycles: HashSet<usize> = (20..=220).step_by(40).collect();
    let mut signal_strengths: Vec<i32> = Vec::with_capacity(signal_cycles.len());

    let mut register = 1;
    let mut cycle: usize = 0;
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");

        match &line[0..4] {
            // No-Operations take 1 cycle to complete
            "noop" => {
                cycle += 1;
                if signal_cycles.contains(&cycle) {
                    signal_strengths.push(register * cycle as i32);
                }
            }
            "addx" => {
                // Additions take two cycles to complete
                for _ in 0..2 {
                    cycle += 1;
                    if signal_cycles.contains(&cycle) {
                        signal_strengths.push(register * cycle as i32);
                    }
                }

                let value = line
                    .rsplit_once(" ")
                    .expect("'addx' instruction should be followed by value")
                    .1
                    .parse::<i32>()
                    .expect("Instruction argument should be small integer");
                register += value;
            }
            _ => unreachable!(),
        }
    }

    assert!(
        cycle >= *signal_cycles.iter().max().unwrap(),
        "Instructions not long enough to reach final signal strentgh cycle"
    );

    println!(
        "Sum of six signal strengths: {}",
        signal_strengths.iter().sum::<i32>()
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
