use std::fs;
use std::ops::Add;
use regex::Regex;

pub fn day_07() {
    println!("Day 07: ");
    let content = fs::read_to_string("src/input07").expect("Error reading the file");

    let re = Regex::new(r"(\d+):\s*((?:\d+\s*)+)").unwrap();
    let mut results = Vec::new();

    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let result: i128 = caps[1].parse().unwrap();
            let numbers: Vec<i128> = caps[2]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            results.push((result, numbers));
        }
    }



    println!("\t- Part 1: {:?}", part01(&results));
    println!("\t- Part 1: {:?}", part02(&results));
}

fn part01(data: &Vec<(i128, Vec<i128>)>) -> i128 {
    let mut result = 0;
    let ops = vec![Operation::Plus, Operation::Times];
    for d in data {
        if is_valid(d, &ops, 0, 0) {
            result += d.0;
        }
    }
    result
}

fn part02(data: &Vec<(i128, Vec<i128>)>) -> i128 {
    let mut result = 0;
    let ops = vec![Operation::Plus, Operation::Times, Operation::Or];
    for d in data {
        if is_valid(d, &ops, 0, 0) {
            result += d.0;
        }
    }
    result
}

enum Operation {
    Plus,
    Times,
    Or
}

fn apply_operation(op: &Operation, a: i128, b: i128) -> i128 {

    match op {
        Operation::Plus => a + b,
        Operation::Times => a * b,
        Operation::Or => {
            a.to_string().add(&b.to_string()).parse().unwrap()
        }
    }
}
fn is_valid(data: &(i128, Vec<i128>), ops : &Vec<Operation>, index : usize, current :i128) -> bool {

    if index == data.1.len()  {
        if current == data.0 {
            return true;
        }
        return false;
    }
    if current >= data.0 {
        return false
    }

    for op in ops {
        let new_current = apply_operation(op, current, data.1[index]);
        if is_valid(data, ops, index + 1, new_current) {
            return true;
        }
    }

    false
}