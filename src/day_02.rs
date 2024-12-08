use std::fs;

pub fn day_02() {

    println!("Day 02: ");
    let content = fs::read_to_string("src/input02").expect("Error reading the file");


    let mut data: Vec<Vec<i32>> = Vec::new();
    for line in content.lines() {

        let mut row: Vec<i32> = Vec::new();
        for value in line.split_whitespace() {
            row.push(value.parse::<i32>().unwrap());
        }
        data.push(row);

    }

    println!("\t- Part 1: {:?}", part01(&data));
    println!("\t- Part 2: {:?}", part02(&data));


}

fn part01(data : &Vec<Vec<i32>>) -> i32 {

    let mut count = 0;
    for row in data {
        if check_validity(&row) {
            count += 1;
        }
    }

    count
}

enum SequenceType {
    Increasing,
    Decreasing,
}
fn check_validity(row : &&Vec<i32>) -> bool {

    if row.len() < 2 {
       return true;
    }
    let sequence_type;
    if row[0] < row[1] {
        sequence_type = SequenceType::Increasing
    } else {
        sequence_type = SequenceType::Decreasing
    }

    for i in 1..row.len() {
        match sequence_type {
            SequenceType::Increasing => {
                if row[i-1] > row[i] {
                    return false;
                }
            },
            SequenceType::Decreasing => {
                if row[i-1] < row[i] {
                    return false;
                }
            }
        }

        if i32::abs(row[i-1] - row[i]) < 1 || i32::abs(row[i-1] - row[i]) > 3 {
            return false;
        }
    }

    true
}

fn part02(data : &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for row in data {
        for i in 0..row.len() {
            let mut row_copy = row.clone();
            row_copy.remove(i);
            if check_validity(&&row_copy) {
                count += 1;
                break;
            }
        }
    }
    count
}