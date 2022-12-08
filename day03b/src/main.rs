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
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let (rucksack_a, rucksack_b) = line.split_at(line.len() / 2);
        let rucksack_a: HashSet<char> = HashSet::from_iter(rucksack_a.chars());
        let rucksack_b: HashSet<char> = HashSet::from_iter(rucksack_b.chars());

        let intersection = rucksack_a
            .intersection(&rucksack_b)
            .next()
            .expect("Rucksacks should contain exactly one intersecting item");
        priority_total += priorities
            .get(intersection)
            .expect("Rucksack items should only be a-z or A-Z");
    }

    println!("Sum of all priorities: {priority_total}");
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
