use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Point = [i32; 2];

fn main() {
    let file_path = "./input.txt";

    let mut grid: HashSet<Point> = HashSet::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|s| {
                s.splitn(2, ",")
                    .map(|ss| {
                        ss.parse::<i32>()
                            .expect("Points should be specified by two medium sized integers")
                    })
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap()
            })
            .collect();
        for (a, b) in points.iter().zip(points.iter().skip(1)) {
            if a[0] == b[0] {
                for y in std::cmp::min(a[1], b[1])..=std::cmp::max(a[1], b[1]) {
                    grid.insert([a[0], y]);
                }
            } else if a[1] == b[1] {
                for x in std::cmp::min(a[0], b[0])..=std::cmp::max(a[0], b[0]) {
                    grid.insert([x, a[1]]);
                }
            } else {
                panic!("Lines should either be horizontal or vertical");
            }
        }
    }

    // Vertical abyss threshold constitutes stopping criterion
    let abyss_threshold = grid.iter().max_by_key(|p| p[1]).unwrap()[1];
    println!("Abyss threshold: {abyss_threshold}");

    // Fill sand until sand starts falling into abyss
    let mut sand_counter = 0;
    loop {
        let mut sand: Point = [500, 0];
        while sand[1] <= abyss_threshold {
            if !grid.contains(&[sand[0], sand[1] + 1]) {
                sand[1] += 1;
            } else if !grid.contains(&[sand[0] - 1, sand[1] + 1]) {
                sand[0] -= 1;
                sand[1] += 1;
            } else if !grid.contains(&[sand[0] + 1, sand[1] + 1]) {
                sand[0] += 1;
                sand[1] += 1;
            } else {
                break;
            }
        }

        if sand[1] <= abyss_threshold {
            grid.insert(sand);
            sand_counter += 1;
        } else {
            break;
        }
    }

    println!(
        "Number of sand units that have come to rest: {}",
        sand_counter
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
