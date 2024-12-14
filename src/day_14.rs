use std::collections::HashMap;
use std::fmt::Debug;
use std::{fs, thread};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::time::Duration;
use regex::Regex;

pub fn day_14() {
    println!("Day 14: ");
    let content = fs::read_to_string("src/input14").expect("Error reading the file");

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut data: Vec<Robot> = Vec::new();
    for line in content.lines() {
        let cap = re.captures(line).unwrap();
        let p = Vector { x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap() };
        let v = Vector { x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap() };
        let robot = Robot { p, v };
        data.push(robot.clone());
    }

    println!("\t- Part 1: {:?}", part01(&data));
    println!("\t- Part 2: {:?}", part02(&data));
}

#[derive(Debug, Clone)]
struct Robot {
    p: Vector,
    v: Vector,
}
#[derive(Debug, Clone)]
struct Vector {
    x: i128,
    y: i128,
}

fn part01(data: &Vec<Robot>) -> i128 {
    let mut quad : HashMap<i128, i128> = HashMap::new();
    let seconds = 100;
    let size_x = 101;
    let size_y = 103;

    for robot in data {
        let x = (robot.p.x + seconds * robot.v.x).rem_euclid(size_x);
        let y = (robot.p.y + seconds * robot.v.y).rem_euclid(size_y);
        quad.insert(quadrant_number(x, y, size_x, size_y), quad.get(&quadrant_number(x, y, size_x, size_y)).unwrap_or(&0) + 1);
    }
    let mut result = 1;
    for (key, value) in quad.iter() {
        if *key != 0 {
            result *= *value;
        }
    }
    result
}

fn quadrant_number(x : i128, y : i128, wide : i128, height : i128) -> i128 {
    if x < wide / 2 && y < height / 2 {
        1
    } else if x >= wide / 2 + 1 && y < height / 2 {
        2
    } else if x < wide / 2 && y >= height / 2 + 1{
        3
    } else  if x >= wide / 2 + 1 && y >= height / 2 + 1{
        4
    } else {
        0
    }
}

fn part02(data2: &Vec<Robot>) {
    let mut file = File::create("day_14_part_2.txt").unwrap();
    let mut data = data2.clone();
    let mut quad : HashMap<i128, i128> = HashMap::new();
    let seconds = 100;
    let size_x = 101;
    let size_y = 103;
    let mut unique = true;

for i in 1..10000 {
    let mut map = vec![vec!['.'; size_x as usize]; size_y as usize];

    for robot in &mut data {
        let x = (robot.p.x + robot.v.x).rem_euclid(size_x);
        let y = (robot.p.y + robot.v.y).rem_euclid(size_y);
        robot.p.x = x;
        robot.p.y = y;
        if map[robot.p.y as usize][robot.p.x as usize] != '#' {
            map[robot.p.y as usize][robot.p.x as usize] = '#';
        } else {
            unique = false;
        }
    }
    if unique {
        print_map(&map, size_x, size_y, i, &mut file);
    }
    unique = true;

}
}

fn print_map(map: &Vec<Vec<char>>, wide : i128, height : i128, nb_seconds : i32, file: &mut File) {



    let mut buf_reader = BufReader::new(&mut *file);
    let mut contents = &mut String::new();
   
    contents.push_str(&format!("After {} seconds:\n", nb_seconds));
    for line in map {
        for value in line {
            contents.push(*value);
        }
        contents.push('\n');
    }
    println!("{}", contents);


}