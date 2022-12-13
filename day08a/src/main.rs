use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut heights: Vec<Vec<i32>> = Vec::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        heights.push(
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("Tree heights should be small integers") as i32
                })
                .collect(),
        );
    }

    let shape = [heights.len(), heights[0].len()];
    let mut visibility: Vec<Vec<bool>> =
        Vec::from_iter((0..shape[0]).map(|_| vec![false; shape[1]]));

    for row in 0..shape[0] {
        // Left to right
        let mut max_height = -1;
        for column in 0..shape[1] {
            if heights[row][column] > max_height {
                visibility[row][column] = true;
                max_height = heights[row][column];
            }
        }

        // Right to left
        max_height = -1;
        for column in (0..shape[1]).rev() {
            if heights[row][column] > max_height {
                visibility[row][column] = true;
                max_height = heights[row][column];
            }
        }
    }

    for column in 0..shape[1] {
        // Top to bottom
        let mut max_height = -1;
        for row in 0..shape[0] {
            if heights[row][column] > max_height {
                visibility[row][column] = true;
                max_height = heights[row][column];
            }
        }

        // Bottom to top
        max_height = -1;
        for row in (0..shape[0]).rev() {
            if heights[row][column] > max_height {
                visibility[row][column] = true;
                max_height = heights[row][column];
            }
        }
    }

    println!(
        "Number of trees visible from the outside: {}",
        visibility
            .iter()
            .map(|v| v.iter().map(|b| if *b { 1 } else { 0 }).sum::<i32>())
            .sum::<i32>()
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
