use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DIMENSIONS: [usize; 2] = [6, 40];

fn main() {
    let file_path = "./input.txt";
    let mut pixels: Vec<[char; DIMENSIONS[1]]> =
        Vec::from_iter((0..DIMENSIONS[0]).map(|_| ['.'; DIMENSIONS[1]]));

    let mut register: i32 = 1;
    let mut cycle: usize = 0;
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");

        match &line[0..4] {
            // No-Operations take 1 cycle to complete
            "noop" => {
                let current_pixel = [cycle / 40, cycle % DIMENSIONS[1]];
                if current_pixel[1] as i32 == register - 1
                    || current_pixel[1] as i32 == register
                    || current_pixel[1] as i32 == register + 1
                {
                    pixels[current_pixel[0]][current_pixel[1]] = '#';
                }
                cycle += 1;
            }
            "addx" => {
                // Additions take two cycles to complete
                for _ in 0..2 {
                    let current_pixel = [cycle / 40, cycle % DIMENSIONS[1]];
                    if current_pixel[1] as i32 == register - 1
                        || current_pixel[1] as i32 == register
                        || current_pixel[1] as i32 == register + 1
                    {
                        pixels[current_pixel[0]][current_pixel[1]] = '#';
                    }
                    cycle += 1;
                }

                let value = line
                    .rsplit_once(" ")
                    .expect("'addx' instruction should be followed by value")
                    .1
                    .parse::<i32>()
                    .expect("Instruction argument should be small integer");
                register += value;
            }
            _ => unreachable!(),
        }
    }

    println!("Final executed cycle: {}", cycle);
    // assert!(
    //     cycle >= *signal_cycles.iter().max().unwrap(),
    //     "Instructions not long enough to reach final signal strentgh cycle"
    // );

    for row in pixels.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
