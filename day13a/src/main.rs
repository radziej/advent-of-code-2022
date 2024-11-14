use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

enum PacketData {
    Integer(u32),
    List(Vec<PacketData>),
}

impl std::fmt::Debug for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PacketData::Integer(i) => write!(f, "{i}"),
            PacketData::List(l) => write!(
                f,
                "[{}]",
                l.iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

fn main() {
    let file_path = "./input.txt";

    let mut correctly_ordered: usize = 0;
    let mut left: Vec<PacketData> = Vec::new();
    let mut right: Vec<PacketData> = Vec::new();
    for (index, line) in read_lines(file_path)
        .expect("Should be able to read input file")
        .enumerate()
    {
        let line = line.expect("Should be able to parse input line");
        match index % 3 {
            0 => {
                left = parse_packet(&mut line.chars());
            }
            1 => {
                right = parse_packet(&mut line.chars());

                if compare_packets(&left, &right) == 1 {
                    correctly_ordered += index / 3 + 1;
                    // println!("--- Pair {} ---", index / 3 + 1);
                    // println!(" Left: {:?}", left);
                    // println!("Right: {:?}", right);
                    // println!("Correct order!");
                } else {
                    // println!("Incorrect order.");
                }
            }
            2 => {
                left.clear();
                right.clear();
            }
            _ => unreachable!(),
        }
    }

    println!(
        "Sum of indices of correctly ordered pairs: {}",
        correctly_ordered
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_packet(characters: &mut Chars) -> Vec<PacketData> {
    let first = characters.next();
    assert!(first.is_some() && first.unwrap() == '[');
    parse_list(characters)
}

fn parse_list(characters: &mut Chars) -> Vec<PacketData> {
    let mut data = Vec::new();
    let mut buffer: Vec<char> = Vec::new();
    while let Some(c) = characters.next() {
        match c {
            '[' => data.push(PacketData::List(parse_list(characters))),
            ']' => {
                if !buffer.is_empty() {
                    data.push(PacketData::Integer(
                        buffer
                            .iter()
                            .collect::<String>()
                            .parse()
                            .expect("Data that is not a list should be a small integer"),
                    ));
                    buffer.clear();
                }
                return data;
            }
            '0'..='9' => buffer.push(c),
            ',' => {
                if !buffer.is_empty() {
                    data.push(PacketData::Integer(
                        buffer
                            .iter()
                            .collect::<String>()
                            .parse()
                            .expect("Data that is not a list should be a small integer"),
                    ));
                    buffer.clear();
                }
            }
            _ => unreachable!(),
        }
    }
    data
}

fn compare_packets(lhs: &Vec<PacketData>, rhs: &Vec<PacketData>) -> i32 {
    for index in 0..std::cmp::max(lhs.len(), rhs.len()) {
        // Account for packet length criterion: left side must run out first
        match (index >= lhs.len(), index >= rhs.len()) {
            (true, true) => return 0,
            (true, false) => return 1,
            (false, true) => return -1,
            (false, false) => (),
        }
        let left = lhs.get(index).unwrap();
        let right = rhs.get(index).unwrap();
        let result: i32;
        match (left, right) {
            (PacketData::Integer(l), PacketData::Integer(r)) => match l.cmp(r) {
                Ordering::Less => result = 1,
                Ordering::Equal => result = 0,
                Ordering::Greater => result = -1,
            },
            (PacketData::List(l), PacketData::List(r)) => {
                result = compare_packets(l, r);
            }
            (PacketData::Integer(l), PacketData::List(r)) => {
                result = compare_packets(&Vec::<PacketData>::from([PacketData::Integer(*l)]), r);
            }
            (PacketData::List(l), PacketData::Integer(r)) => {
                result = compare_packets(l, &Vec::<PacketData>::from([PacketData::Integer(*r)]));
            }
        }
        if result != 0 {
            return result;
        }
    }
    0
}
