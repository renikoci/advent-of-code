use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;

struct Pair {
    result: u64,
    values: VecDeque<u64>,
}

impl Pair {
    fn push_values_to_deque(&mut self, value: u64) {
        self.values.push_back(value);
    }

    fn calculate_result(&self, ops: Vec<char>) -> u64 {
        // Create a temporary queue
        let mut deq = self.values.clone();

        for op in ops.iter() {
            let value1 = deq.pop_front().unwrap();
            let value2 = deq.pop_front().unwrap();
            match op {
                '*' => {
                    deq.push_front(value1 * value2);
                }
                '+' => {
                    deq.push_front(value1 + value2);
                }
                '|' => {
                    let s = format!("{}{}", value1, value2);
                    deq.push_front(s.parse().expect("parsing failed"));
                }
                _ => {
                    println!("unknown operation")
                }
            }
        }

        return deq
            .pop_front()
            .expect("No value found.This shouldn't happen");
    }
}

static OPERATIONS: [char; 3] = ['+', '*', '|'];

fn main() {
    let pairs: Vec<Pair> = get_data();

    let mut res = 0;

    for pair in pairs {
        let mut ops = vec!['+'; pair.values.len() - 1];
        if backtrack(&pair, &mut ops, 0) {
            res += pair.result;
        }
    }

    println!("{}", res);
}

fn backtrack(p: &Pair, ops: &mut Vec<char>, start: usize) -> bool {
    if start == ops.len() {
        if p.result == p.calculate_result(ops.to_vec()) {
            // println!("Found - {}", p.result);
            return true;
        }
        return false;
    }

    let prev: char = ops[start];

    for &op in &OPERATIONS {
        ops[start] = op;

        if backtrack(p, ops, start + 1) {
            return true;
        }
    }
    ops[start] = prev;

    false
}

fn get_data() -> Vec<Pair> {
    let file = File::open("input.txt").expect("Plis fix this");
    let reader = std::io::BufReader::new(file);

    let mut pairs: Vec<Pair> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Plis read line");

        let parts: Vec<&str> = line.split(":").collect();

        if parts.len() != 2 {
            panic!("YO WTF HAPPENED");
        }

        let result = parts[0].trim().parse().expect("Parsing failed");

        let mut pair = Pair {
            result,
            values: VecDeque::new(),
        };

        for val in parts[1].split_whitespace() {
            let parsed: u64 = val.parse().expect("Parsing failed");
            pair.push_values_to_deque(parsed);
        }

        pairs.push(pair);
    }

    pairs
}
