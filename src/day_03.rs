use std::fs;

pub fn day_03() {
    println!("Day 03: ");
    let content = fs::read_to_string("src/input03").expect("Error reading the file");

    println!("\t- Part 1: {}", part01(&content));
    println!("\t- Part 2: {}", part02(&content))

}

fn part01(content: &String) -> i32 {
    let mut value = 0;
    let regex = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    regex.captures_iter(&content).for_each(|cap| {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        value += a * b;
    });
    value
}

fn part02(content : &String) -> i32 {
    let mut value = 0;

    let regex = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut curr_state = true;
    regex.captures_iter(&content).for_each(|cap| {
        match &cap[0]
        {
            "do()" => {
                curr_state = true
            }
            "don't()" => {
                curr_state = false
            }
            _ => {
                if curr_state {
                    let a = cap[1].parse::<i32>().unwrap();
                    let b = cap[2].parse::<i32>().unwrap();
                    value += a * b;
                }
            }
        }


    });
    value
}