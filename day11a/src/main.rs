use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Monkey {
    pub items: Vec<u64>,
    operation: fn(u64, u64) -> u64,
    operand: u64,
    test_args: (u64, u64, u64),
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        operation: fn(u64, u64) -> u64,
        operand: u64,
        test_args: (u64, u64, u64),
    ) -> Self {
        Monkey {
            items,
            operation,
            operand,
            test_args,
        }
    }

    fn test(&self, item: u64) -> u64 {
        if item % self.test_args.0 == 0 {
            self.test_args.1
        } else {
            self.test_args.2
        }
    }

    fn inspect_items(&mut self) -> Vec<(u64, u64)> {
        let mut result: Vec<(u64, u64)> = Vec::with_capacity(self.items.len());
        for item in self.items.drain(..).collect::<Vec<u64>>().into_iter() {
            let worry_level = (self.operation)(item, self.operand) / 3;
            result.push((worry_level, self.test(worry_level)))
        }
        result
    }
}

fn main() {
    let file_path = "./input.txt";
    let mut lines = read_lines(file_path)
        .expect("Should be able to read input file")
        .peekable();

    let mut monkeys: HashMap<u64, Monkey> = HashMap::new();
    while lines.peek().is_some() {
        // Collect lines specifying single monkey
        let specificaton: Vec<String> = lines
            .by_ref()
            .take_while(|l| l.is_ok() && !l.as_ref().unwrap().is_empty())
            .map(|s| {
                s.expect("Should be able to read monkey specification")
                    .trim()
                    .to_string()
            })
            .collect();
        assert_eq!(
            specificaton.len(),
            6,
            "Monkey specification should be 6 lines"
        );

        assert!(specificaton[0].starts_with("Monkey"));
        let id: u64 = (&specificaton[0][7..specificaton[0].len() - 1])
            .parse()
            .expect("Monkey IDs should be small integers");

        assert!(specificaton[1].starts_with("Starting items:"));
        let items: Vec<u64> = (&specificaton[1][16..])
            .split(", ")
            .map(|v| {
                v.parse::<u64>()
                    .expect("Items should be identified by small integers")
            })
            .collect();

        assert!(specificaton[2].starts_with("Operation: new = "));
        let calculation: Vec<&str> = (&specificaton[2][17..]).split(' ').collect();
        assert_eq!(
            calculation.len(),
            3,
            "Operation should be specified with two operands and one operator"
        );
        assert_eq!(calculation[0], "old");
        let operation: fn(u64, u64) -> u64;
        let mut operand: u64 = 0;
        if calculation[2] == "old" {
            // Identical operand
            match calculation[1] {
                "*" => operation = |a, _| a * a,
                "+" => operation = |a, _| a + a,
                _ => panic!("Encountered unhandled operator"),
            }
        } else {
            // Different operand
            operand = calculation[2]
                .parse()
                .expect("Operand should be small integer");
            match calculation[1] {
                "*" => operation = |a, b| a * b,
                "+" => operation = |a, b| a + b,
                _ => panic!("Encountered unhandled operator"),
            }
        }

        assert!(specificaton[3].starts_with("Test:"));
        assert!(specificaton[4].starts_with("If true:"));
        assert!(specificaton[5].starts_with("If false:"));
        let test_args: (u64, u64, u64) = (
            specificaton[3]
                .rsplit_once(' ')
                .unwrap()
                .1
                .parse()
                .expect("Divisor should be small integer"),
            specificaton[4]
                .rsplit_once(' ')
                .unwrap()
                .1
                .parse()
                .expect("Monkey ID should be small integer"),
            specificaton[5]
                .rsplit_once(' ')
                .unwrap()
                .1
                .parse()
                .expect("Monkey ID should be small integer"),
        );
        monkeys.insert(id, Monkey::new(items, operation, operand, test_args));
    }

    let mut monkey_ids: Vec<u64> = monkeys.keys().copied().collect();
    monkey_ids.sort_unstable();
    let mut num_inspections: HashMap<u64, u64> =
        HashMap::from_iter(monkeys.keys().map(|k| (*k, 0)));
    for _ in 1..=20 {
        for id in monkey_ids.iter() {
            let inspected_items = monkeys.get_mut(&id).unwrap().inspect_items();
            num_inspections
                .entry(*id)
                .and_modify(|v| *v += inspected_items.len() as u64);
            for (item, target) in inspected_items.into_iter() {
                monkeys.entry(target).and_modify(|m| m.items.push(item));
            }
        }
    }

    let mut inspections = num_inspections.values().copied().collect::<Vec<u64>>();
    inspections.sort_unstable();
    println!(
        "Multiplying two highest numbers of inspections: {} * {} = {}",
        inspections[inspections.len() - 2],
        inspections[inspections.len() - 1],
        inspections[inspections.len() - 2] * inspections[inspections.len() - 1]
    );
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
