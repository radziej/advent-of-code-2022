use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut priorities: HashMap<char, i32> = HashMap::with_capacity(52);
    for (c, i) in ('a'..='z').zip(1..=26) {
        priorities.insert(c, i);
    }
    for (c, i) in ('A'..='Z').zip(27..=52) {
        priorities.insert(c, i);
    }

    let mut priority_total = 0;
    let mut num_elves = 0;
    let mut intersection: HashSet<char> = HashSet::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");

        if intersection.is_empty() {
            intersection = HashSet::from_iter(line.chars());
        } else {
            intersection = intersection
                .intersection(&HashSet::from_iter(line.chars()))
                .copied()
                .collect();
        }
        num_elves += 1;

        if num_elves % 3 == 0 {
            priority_total += priorities
                .get(
                    intersection
                        .iter()
                        .next()
                        .expect("Intersection between group rucksacks should be the badge item"),
                )
                .expect("Rucksack items should only be a-z or A-Z");
            intersection.clear();
        }
    }

    println!("Sum of all badge priorities: {priority_total}");
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
