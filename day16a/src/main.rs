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

fn main() {
    // let file_path = "./test.txt";
    let file_path = "./input.txt";
    let template = "Valve {} has flow rate={}; tunnel";
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let (node, flow) = scan!(line, template, String, u32);
        let node = node.unwrap();
        let flow = flow.unwrap();
        let edges: Vec<String> = line
            .rsplit_once("valve")
            .unwrap()
            .1
            .trim_start_matches(&['s', ' '])
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        println!("{}, {}, {:?}", node, flow, edges);
    }
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
