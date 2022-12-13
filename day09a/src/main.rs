use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    // Mathematical coordinates [x, y]
    let mut head: [i32; 2] = [0, 0];
    let mut tail: [i32; 2] = [-1, -1];
    let mut tail_positions: HashSet<[i32; 2]> = HashSet::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let (direction, steps) = line
            .split_once(" ")
            .expect("Step count and direction should be separated by a space");
        for _ in 0..steps
            .parse::<usize>()
            .expect("Step count should be small integer")
        {
            // Move head according to state changes
            match direction {
                "U" => head[1] += 1,
                "D" => head[1] -= 1,
                "L" => head[0] -= 1,
                "R" => head[0] += 1,
                _ => unreachable!(),
            }

            // Tail must be within +-1 on both axes
            let x_distance = head[0] - tail[0];
            let y_distance = head[1] - tail[1];
            if x_distance.abs() > 1 || y_distance.abs() > 1 {
                // Movement is either within a column/row or diagonally
                tail[0] += x_distance.signum();
                tail[1] += y_distance.signum();
            }

            if !tail_positions.contains(&tail) {
                tail_positions.insert(tail.clone());
            }
        }
    }

    println!(
        "Number of unique positions the tail has visited: {}",
        tail_positions.len()
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
