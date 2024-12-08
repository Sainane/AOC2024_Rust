use std::fs;

pub fn day_01() {
    println!("Day 01: ");

    let content = fs::read_to_string("src/input01").expect("Error reading the file");

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(part1), Some(part2)) = (parts.next(), parts.next()) {
            col1.push(part1.parse::<i32>().unwrap());
            col2.push(part2.parse::<i32>().unwrap());
        }
    }

    println!("\t- Part 1: {}", part01(&col1, &col2));
    println!("\t- Part 2: {}", part02(&col1, &col2));

}

fn part01(col1 : &Vec<i32>, col2 : &Vec<i32>) -> i32 {
    let mut col1_sort = col1.clone();
    let mut col2_sort = col2.clone();
    col1_sort.sort();
    col2_sort.sort();
    let mut sum = 0;
    for i in 0..col1.len() {
        sum += i32::abs(col1_sort[i] - col2_sort[i]);
    }
    sum
}

fn part02(col1 : &Vec<i32>, col2 : &Vec<i32>) -> i32 {
    let mut similarity = 0;
    for &value in col1 {
        let nb_occurrences = col2.iter().filter(|&&x| x == value).count() as i32;
        similarity += nb_occurrences * value;
    }
    similarity
}