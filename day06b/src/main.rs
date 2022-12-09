use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

const WINDOW_SIZE: usize = 14;

fn main() {
    let file_path = "./input.txt";

    let input = read_to_string(file_path).expect("Should be able to read input file");
    // let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();  // Marker after 29 characters
    let buffer: Vec<char> = input.chars().collect();

    let mut window: VecDeque<char> =
        VecDeque::from_iter(buffer.iter().copied().take(WINDOW_SIZE - 1));
    let mut seen: HashSet<char> = HashSet::with_capacity(WINDOW_SIZE);
    let iterator = buffer.iter().enumerate().skip(WINDOW_SIZE - 1);
    for (index, character) in iterator {
        window.push_back(*character);
        for element in window.iter() {
            if !seen.insert(*element) {
                seen.clear();
                break;
            }
        }
        if seen.len() == WINDOW_SIZE {
            println!("Marker present after {} characters", index + 1);
            break;
        }
        window.pop_front();
    }
}
