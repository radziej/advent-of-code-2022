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
    let mut scenic_scores: Vec<Vec<i32>> = Vec::from_iter((0..shape[0]).map(|_| vec![0; shape[1]]));

    for row in 0..shape[0] {
        for column in 0..shape[1] {
            let height = heights[row][column];
            let mut scores = [0, 0, 0, 0];
            // From tree to bottom
            for y in (row + 1)..shape[0] {
                scores[0] += 1;
                if heights[y][column] >= height {
                    break;
                }
            }

            // From tree to top
            for y in (0..row).rev() {
                scores[1] += 1;
                if heights[y][column] >= height {
                    break;
                }
            }

            // From tree to right
            for x in (column + 1)..shape[1] {
                scores[2] += 1;
                if heights[row][x] >= height {
                    break;
                }
            }

            // From tree to left
            for x in (0..column).rev() {
                scores[3] += 1;
                if heights[row][x] >= height {
                    break;
                }
            }

            scenic_scores[row][column] = scores.iter().product();
        }
    }

    println!(
        "Highest scenic score for any tree: {}",
        scenic_scores
            .iter()
            .map(|v| v.iter().max().unwrap())
            .max()
            .unwrap()
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
