use std::fs;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

pub fn day_08()  {
    println!("Day 08: Handheld Halting");
    let data = fs::read_to_string("src/input08").expect("Error reading input");
    let lines: Vec<&str> = data.lines().collect();

    let mut map = initialize_map(&lines);
    let antenna = extract_antenna_positions(&lines);

    let (_, count1) = mark_antinodes(&mut map, &antenna, calculate_antinode1);
    let (_, count2) = mark_antinodes(&mut map, &antenna, calculate_antinode2);


    println!("\t- Part 1 : {}", count1);
    println!("\t- Part 2 : {}", count2);

}

fn initialize_map(lines: &[&str]) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn extract_antenna_positions(lines: &[&str]) -> std::collections::HashMap<char, Vec<Position>> {
    let mut antenna = std::collections::HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, val) in line.chars().enumerate() {
            if val != '.' {
                antenna.entry(val).or_insert_with(Vec::new).push(Position { x: i as isize, y: j as isize });
            }
        }
    }
    antenna
}

fn mark_antinodes<F>(
    map: &mut Vec<Vec<char>>,
    antenna: &std::collections::HashMap<char, Vec<Position>>,
    find_antinode: F,
) -> (Vec<Vec<char>>, usize)
where
    F: Fn(Position, Position, usize, usize) -> Vec<Position>,
{
    let max_x = map.len();
    let max_y = map[0].len();
    let mut map_copy = map.clone();
    let mut count = 0;

    for positions in antenna.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let antinodes = find_antinode(positions[i], positions[j], max_x, max_y);
                for antinode in antinodes {
                    if map_copy[antinode.x as usize][antinode.y as usize] != '#' {
                        count += 1;
                        map_copy[antinode.x as usize][antinode.y as usize] = '#';
                    }
                }
            }
        }
    }
    (map_copy, count)
}

fn print_map(map: &[Vec<char>]) {
    for line in map {
        for &val in line {
            print!("{}", val);
        }
        println!();
    }
}

fn get_double_distance(pos1: Position, pos2: Position) -> Position {
    Position {
        x: pos2.x + 2 * (pos1.x - pos2.x),
        y: pos2.y + 2 * (pos1.y - pos2.y),
    }
}

fn get_distance(pos1: Position, pos2: Position, mult: isize) -> Position {
    Position {
        x: pos2.x + mult * (pos1.x - pos2.x),
        y: pos2.y + mult * (pos1.y - pos2.y),
    }
}

fn check_bounds(pos: Position, max_x: usize, max_y: usize) -> bool {
    pos.x >= 0 && pos.x < max_x as isize && pos.y >= 0 && pos.y < max_y as isize
}

fn calculate_antinode1(pos1: Position, pos2: Position, max_x: usize, max_y: usize) -> Vec<Position> {
    let mut positions = Vec::new();
    let pos1_double = get_double_distance(pos1, pos2);
    let pos2_double = get_double_distance(pos2, pos1);
    if check_bounds(pos1_double, max_x, max_y) {
        positions.push(pos1_double);
    }
    if check_bounds(pos2_double, max_x, max_y) {
        positions.push(pos2_double);
    }
    positions
}

fn calculate_antinode2(pos1: Position, pos2: Position, max_x: usize, max_y: usize) -> Vec<Position> {
    let mut positions = Vec::new();
    positions.extend(calculate_antinodes(pos1, pos2, max_x, max_y));
    positions.extend(calculate_antinodes(pos2, pos1, max_x, max_y));
    positions
}

fn calculate_antinodes(start: Position, end: Position, max_x: usize, max_y: usize) -> Vec<Position> {
    let mut antinodes = Vec::new();
    let mut val = 1;
    loop {
        let antinode = get_distance(start, end, val);
        if !check_bounds(antinode, max_x, max_y) {
            break;
        }
        antinodes.push(antinode);
        val += 1;
    }
    antinodes
}
