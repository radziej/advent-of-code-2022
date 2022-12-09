use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";
    let mut lines = read_lines(file_path).expect("Should be able to read input file");

    let mut initial_state: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.expect("Should be able to read line from input file");
        if line.is_empty() {
            break;
        }
        initial_state.push(line.chars().collect());
    }

    let mut stacks: HashMap<char, Vec<char>> = HashMap::new();
    for (index, key) in initial_state
        .pop()
        .expect("Stacks should have an index line")
        .iter()
        .enumerate()
    {
        if !key.is_ascii_digit() {
            continue;
        }

        stacks.insert(
            *key,
            initial_state
                .iter()
                .rev()
                .map(|l| l.get(index).unwrap())
                .copied()
                .filter(|c| c.is_ascii_alphabetic())
                .collect(),
        );
    }
    println!("Stacks: {:?}", stacks);

    for line in lines {
        let line = line.expect("Should be able to read line from input file");
        let parts: Vec<&str> = line.split(" ").collect();
        assert_eq!(parts.len(), 6, "Move instruction should consist of 6 parts");

        let amount: usize = parts[1].parse().expect("Amount must be numeric");
        let source: char = parts[3].chars().next().unwrap();
        let target: char = parts[5].chars().next().unwrap();

        let length = stacks.get_mut(&source).unwrap().len();
        let containers: Vec<char> = stacks
            .get_mut(&source)
            .unwrap()
            .drain(length - amount..length)
            .rev()
            .collect();
        for container in containers {
            stacks.entry(target).and_modify(|v| v.push(container));
        }
    }

    println!(
        "Top of each stack: {}",
        ('1'..='9')
            .map(|c| stacks.get(&c).unwrap().last().unwrap())
            .collect::<String>()
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
