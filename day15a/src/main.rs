use std::collections::{HashMap, HashSet};
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

type Point = [i64; 2];

fn main() {
    let file_path = "./input.txt";
    let template = "Sensor at x={}, y={}: closest beacon is at x={}, y={}";
    let mut grid: HashMap<Point, Point> = HashMap::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let (sx, sy, bx, by) = scan!(line, template, i64, i64, i64, i64);
        for variable in [sx, sy, bx, by] {
            if variable.is_none() {
                panic!("Failed to parse line: {}", line);
            }
        }
        grid.insert([sx.unwrap(), sy.unwrap()], [bx.unwrap(), by.unwrap()]);
    }
    println!("Number of sensors dispatched: {}", grid.len());

    let target: i64 = 2000000;
    let mut coverage: HashMap<i64, Vec<[i64; 2]>> = HashMap::new();
    for (sensor, beacon) in grid.iter() {
        let l1 = (beacon[0] - sensor[0]).abs() + (beacon[1] - sensor[1]).abs();
        let horizontal = l1 - (target - sensor[1]).abs();
        if horizontal < 0 {
            continue;
        }
        coverage
            .entry(target)
            .and_modify(|v| {
                *v = merge_intervals(v, [sensor[0] - horizontal, sensor[0] + horizontal])
            })
            .or_insert(vec![[sensor[0] - horizontal, sensor[0] + horizontal]]);
    }
    let beacons: HashSet<&Point> = HashSet::from_iter(grid.values().filter(|p| p[1] == target));

    println!(
        "A total of {} positions cannot contain a beacon at row y={}",
        coverage
            .get(&target)
            .unwrap()
            .iter()
            .map(|[a, b]| *b - *a)
            .sum::<i64>()
            - beacons.len() as i64,
        target
    );
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
        if candidate[0] <= interval[1] && candidate[1] >= interval[0] {
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
