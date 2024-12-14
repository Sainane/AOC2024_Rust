use std::fs;
use regex::Regex;

pub fn day_13() {
    println!("Day 13: ");
    let content = fs::read_to_string("src/input13").expect("Error reading the file");

    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    let mut data: Vec<Machine> = Vec::new();
    let mut data2: Vec<Machine> = Vec::new();
    for cap in re.captures_iter(&content) {
        let button_a_x: i128 = cap[1].parse().unwrap();
        let button_a_y: i128 = cap[2].parse().unwrap();
        let button_b_x: i128 = cap[3].parse().unwrap();
        let button_b_y: i128 = cap[4].parse().unwrap();
        let prize_x: i128 = cap[5].parse().unwrap();
        let prize_y: i128 = cap[6].parse().unwrap();

        data.push(Machine {
            a: Button { x: button_a_x, y: button_a_y },
            b: Button { x: button_b_x, y: button_b_y },
            prize: (prize_x, prize_y),
        });
        data2.push(Machine {
            a: Button { x: button_a_x, y: button_a_y },
            b: Button { x: button_b_x, y: button_b_y },
            prize: (prize_x + 10000000000000, prize_y + 10000000000000),
        });
    }

    println!("\t- Part 1: {:?}", part01(&data));
    println!("\t- Part 2: {:?}", part02(&data2));
}

struct Machine {
    a: Button,
    b: Button,
    prize: (i128, i128),
}

struct Button {
    x: i128,
    y: i128,
}

fn part01(data: &Vec<Machine>) -> i128 {
    let mut result = 0;
    for machine in data {
        let sol = solve(machine);
        if check_validity(sol) {
            result += nb_tokens(sol);
        }
    }
    result
}

fn nb_tokens((a, b): (i128, i128)) -> i128 {
    a * 3 + b
}

fn part02(data: &Vec<Machine>) -> i128 {
    let mut result = 0;
    for machine in data {
        let sol = solve(machine);
        if check_validity2(sol) {
            result += nb_tokens(sol);
        }
    }
    result
}

fn det(m: &Machine) -> i128 {
    m.a.x * m.b.y - (m.b.x * m.a.y)
}

fn check_validity((a, b): (i128, i128)) -> bool {
    if a > 100 || b > 100 || b < 0 || a < 0 {
        return false;
    }
    true
}
fn check_validity2((a, b): (i128, i128)) -> bool {
    if  b < 0 || a < 0 {
        return false;
    }
    true
}

fn solve(m: &Machine) -> (i128, i128) {
    let det = det(m);

    let nb_a = (m.b.y * m.prize.0 - (m.prize.1 * m.b.x)) / det;
    let nb_b = (m.a.x * m.prize.1 - (m.prize.0 * m.a.y)) / det;
    if (m.b.y * m.prize.0 - (m.prize.1 * m.b.x)) % det != 0 || (m.a.x * m.prize.1 - (m.prize.0 * m.a.y)) % det != 0 {
        return (-1, -1);
    }
    (nb_a, nb_b)
}