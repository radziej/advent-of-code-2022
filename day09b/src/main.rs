use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    // Mathematical coordinates [x, y]
    let mut knots: [[i32; 2]; 10] = [[0, 0]; 10];
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
                "U" => knots[0][1] += 1,
                "D" => knots[0][1] -= 1,
                "L" => knots[0][0] -= 1,
                "R" => knots[0][0] += 1,
                _ => unreachable!(),
            }

            // Knots must be within +-1 on both axes
            for current in 1..knots.len() {
                let previous = current - 1;
                let x_distance = knots[previous][0] - knots[current][0];
                let y_distance = knots[previous][1] - knots[current][1];
                if x_distance.abs() > 1 || y_distance.abs() > 1 {
                    // Movement is either within a column/row or diagonally
                    knots[current][0] += x_distance.signum();
                    knots[current][1] += y_distance.signum();
                }
            }

            if !tail_positions.contains(&knots[knots.len() - 1]) {
                tail_positions.insert(knots[knots.len() - 1].clone());
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
