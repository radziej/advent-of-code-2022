use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

macro_rules! scan {
    ( $string:expr, $template:expr, $( $r:ty ),+ ) => {{
        let parts: Vec<&str> = $template.split("{}").collect();
        let mut left: usize;
        let mut right: usize = 0;
        let mut indices: Vec<[usize; 2]> = Vec::new();
        for i in 1..parts.len() {
            left = right + parts[i - 1].len();
            right = if (&parts[i]).is_empty() {
                $string.len()
            } else {
                left + (&$string[left..]).find(&parts[i]).expect(&*format!(
                    "String should match template part: {}",
                    &parts[i]
                ))
            };
            indices.push([left, right]);
        }
        let mut index_pair = indices.iter();
        ($(index_pair.next().and_then(|[l, r]| (&$string[*l..*r]).parse::<$r>().ok()),)*)
    }}
}

const AREA: [i64; 2] = [0, 4000000];

type Point = [i64; 2];

fn main() {
    let file_path = "./input.txt";
    let template = "Sensor at x={}, y={}: closest beacon is at x={}, y={}";
    let mut grid: HashMap<Point, i64> = HashMap::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let (sx, sy, bx, by) = scan!(line, template, i64, i64, i64, i64);
        for variable in [sx, sy, bx, by] {
            if variable.is_none() {
                panic!("Failed to parse line: {}", line);
            }
        }
        grid.insert(
            [sx.unwrap(), sy.unwrap()],
            (bx.unwrap() - sx.unwrap()).abs() + (by.unwrap() - sy.unwrap()).abs(),
        );
    }
    println!("Number of sensors dispatched: {}", grid.len());

    for y in 0..=AREA[1] {
        // if y % 100000
        let mut coverage: Vec<[i64; 2]> = Vec::new();
        for (sensor, distance) in grid.iter() {
            let horizontal = distance - (sensor[1] - y).abs();
            if horizontal < 0 {
                continue;
            }
            // Clip to defined area
            let lower_bound = std::cmp::max(sensor[0] - horizontal, AREA[0]);
            let upper_bound = std::cmp::min(sensor[0] + horizontal, AREA[1]);
            if !coverage.is_empty() {
                coverage = merge_intervals(&coverage, [lower_bound, upper_bound]);
            } else {
                coverage = vec![[lower_bound, upper_bound]];
            }
        }

        coverage.sort_unstable();
        if coverage.len() == 2 {
            // After clipping, only row with exactly two intervals should have a gap
            let x = coverage[0][1] + 1;
            // println!("{}, {}: {:?}", x, y, coverage);
            println!(
                "Tuning frequency of distress beacon: {}",
                x as u64 * 4000000 + y as u64
            );
            break;
        }
    }
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn merge_intervals(distinct_intervals: &Vec<[i64; 2]>, interval: [i64; 2]) -> Vec<[i64; 2]> {
    let mut interval = interval;
    let mut result: Vec<[i64; 2]> = Vec::new();
    for candidate in distinct_intervals.iter() {
        if candidate[0] <= interval[1] + 1 && candidate[1] >= interval[0] - 1 {
            // Overlap
            interval = [
                std::cmp::min(candidate[0], interval[0]),
                std::cmp::max(candidate[1], interval[1]),
            ];
        } else {
            // No overlap
            result.push(candidate.clone());
        }
    }
    result.push(interval);
    result
}
