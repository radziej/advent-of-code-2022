use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

#[derive(Clone, Eq, PartialEq)]
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

    let mut packets: Vec<Vec<PacketData>> = Vec::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to parse input line");
        if line.is_empty() {
            continue;
        }
        packets.push(parse_packet(&mut line.chars()));
    }

    // Introduce predefined divider packets
    let first = vec![PacketData::List(vec![PacketData::Integer(2)])];
    let second = vec![PacketData::List(vec![PacketData::Integer(6)])];
    packets.push(first.clone());
    packets.push(second.clone());

    packets.sort_by(|a, b| compare_packets(a, b));
    let mut decoder_key = 1;
    for (i, p) in packets.iter().enumerate() {
        // println!("{}: {:?}", i + 1, p);
        if *p == first || *p == second {
            println!("Index entering decoder key as factor: {}", i + 1);
            decoder_key *= i + 1;
        }
    }

    println!("Decoder key is: {}", decoder_key);
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

fn compare_packets(lhs: &Vec<PacketData>, rhs: &Vec<PacketData>) -> Ordering {
    for index in 0..std::cmp::max(lhs.len(), rhs.len()) {
        // Account for packet length criterion: left side must run out first
        match (index >= lhs.len(), index >= rhs.len()) {
            (true, true) => return Ordering::Equal,
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            (false, false) => (),
        }
        let left = lhs.get(index).unwrap();
        let right = rhs.get(index).unwrap();
        let result: Ordering;
        match (left, right) {
            (PacketData::Integer(l), PacketData::Integer(r)) => result = l.cmp(r),
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
        if result != Ordering::Equal {
            return result;
        }
    }
    Ordering::Equal
}
