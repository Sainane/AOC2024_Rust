use std::collections::{HashMap, HashSet};
use std::fs;
use std::mem::offset_of;

pub fn day_10() {
    println!("Day 10: ");
    let content = fs::read_to_string("src/input10").expect("Error reading the file");


    let data: &mut Vec<Vec<i32>> = &mut Vec::new();
    for line in content.lines() {
        let res: Vec<i32> = line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
        data.push(res);
    }

   println!("\t- Part 1: {}", part01(data));
   println!("\t- Part 2: {}", part02(data));
}

fn part01(data : &mut Vec<Vec<i32>>) -> i32 {
    count_trailhead(data, false)

}

fn part02(data : &mut Vec<Vec<i32>>) -> i32 {
    count_trailhead(data, true)
}

fn count_trailhead(data : &Vec<Vec<i32>>, distinct : bool) -> i32 {
    let mut start_pos :Vec<(usize, usize)> = Vec::new();
    for line in 0..data.len() {
        for value in 0..data[line].len() {
            if data[line][value] == 0 {
                start_pos.push((line, value));
            }
        }
    }
    let mut count = 0;
    for (x, y) in start_pos {
        count += evaluate_trailhead(&data, x as i32, y as i32, distinct);
    }
    count
}
fn evaluate_trailhead(data : &Vec<Vec<i32>>, x : i32, y : i32, distinct :  bool) -> i32 {
    let mut res : Vec<(i32, i32)> = Vec::new();
    check(&mut data.clone(), &mut res, x, y, 0, distinct);

    res.len() as i32
}

fn check_bounds(i : i32, j : i32, nb_rows : usize, nb_cols : usize) -> bool {
    ! (i < 0 || j < 0 || i >= (nb_rows as i32) || j >= (nb_cols as i32))
}

fn check(map: &mut Vec<Vec<i32>>, set: &mut Vec<(i32, i32)>, x : i32, y : i32, height :i32, distinct : bool) {
    if map[x as usize][y as usize] == height {
        if map[x as usize][y as usize] == 9 {
            if distinct || !set.contains(&(x, y)) {
                set.push((x, y));
            }
            return;
        }

        if check_bounds(x + 1, y, map.len(), map[0].len()) {
                check(map, set, x + 1, y, height + 1, distinct);
            }
            if check_bounds(x, y + 1, map.len(), map[0].len()) {
                check(map, set, x, y + 1, height + 1, distinct);
            }
            if check_bounds(x - 1, y, map.len(), map[0].len()) {
                check(map, set, x - 1, y, height + 1, distinct);
            }
            if check_bounds(x, y - 1, map.len(), map[0].len()) {
                check(map, set, x, y - 1, height + 1, distinct);
            }
        }
    }
