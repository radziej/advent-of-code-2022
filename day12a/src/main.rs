use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut start: [usize; 2] = [0, 0];
    let mut destination: [usize; 2] = [0, 0];
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let numeric_elevation: HashMap<char, i32> = HashMap::from_iter(('a'..='z').zip(1..=26));
    for (y, line) in read_lines(file_path)
        .expect("Should be able to read input file")
        .enumerate()
    {
        let line = line.expect("Should be able to read line of input file");
        let mut row = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = [y, x];
                    row.push(1);
                }
                'E' => {
                    destination = [y, x];
                    row.push(26);
                }
                _ => row.push(
                    *numeric_elevation
                        .get(&c)
                        .expect("Elevation should be specified using a-z"),
                ),
            }
        }
        grid.push(row);
    }
    // for row in grid.iter() {
    //     println!("{:?}", row);
    // }

    let shape = [grid.len(), grid[0].len()];
    let mut steps: Vec<Vec<u32>> = grid
        .iter()
        .map(|r| (0..r.len()).map(|_| u32::MAX).collect())
        .collect();
    steps[start[0]][start[1]] = 0;
    let mut visited: HashSet<[usize; 2]> = HashSet::new();
    let mut candidates: HashSet<[usize; 2]> = HashSet::from([start]);

    while !visited.contains(&destination) {
        let position = *candidates.iter().min_by_key(|c| steps[c[0]][c[1]]).unwrap();
        let current_step = steps[position[0]][position[1]];
        candidates.remove(&position);
        visited.insert(position.clone());
        let elevation = grid[position[0]][position[1]].clone();

        for next_position in adjacent(position, [0, shape[0]], [0, shape[1]]) {
            if !visited.contains(&next_position)
                && grid[next_position[0]][next_position[1]] <= elevation + 1
            {
                if steps[next_position[0]][next_position[1]] > current_step + 1 {
                    steps[next_position[0]][next_position[1]] = current_step + 1;
                }
                candidates.insert([next_position[0], next_position[1]]);
            }
        }
    }
    println!(
        "Number of unique positions the tail has visited: {}",
        steps[destination[0]][destination[1]]
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn adjacent(position: [usize; 2], xlimit: [usize; 2], ylimit: [usize; 2]) -> Vec<[usize; 2]> {
    let mut adjacents: Vec<[usize; 2]> = Vec::new();
    for [a, b] in [[1, 0], [0, 1], [0, -1], [-1, 0]] {
        if position[0] as i64 + a >= xlimit[0] as i64
            && position[0] as i64 + a < xlimit[1] as i64
            && position[1] as i64 + b >= ylimit[0] as i64
            && position[1] as i64 + b < ylimit[1] as i64
        {
            adjacents.push([
                (position[0] as i64 + a) as usize,
                (position[1] as i64 + b) as usize,
            ]);
        }
    }
    adjacents
}
