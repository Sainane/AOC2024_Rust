use std::{
    fs,
};

#[derive(Clone, Copy, PartialEq)]
enum MoveResult {
    Out,
    Ok,
    Stuck,
    Loop,
}

#[derive(Clone, Copy)]
struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    fn equals(&self, other: &Direction) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn is_opposite(&self, other: &Direction) -> bool {
        self.x + other.x + self.y + other.y == 0
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn move_position(&self, direction: &Direction) -> Position {
        Position {
            x: self.x + direction.x,
            y: self.y + direction.y,
        }
    }
}

struct Result {
    nb_pos: usize,
    is_loop: bool,
}

const DIRECTIONS: [Direction; 4] = [
    Direction { x: -1, y: 0 },
    Direction { x: 0, y: 1 },
    Direction { x: 1, y: 0 },
    Direction { x: 0, y: -1 },
];

fn check_move(grid: &Vec<Vec<u8>>, pos: Position, dir: Direction, nb_iter: usize) -> MoveResult {
    let new_pos = pos.move_position(&dir);
    if new_pos.x < 0 || new_pos.y < 0 || new_pos.x as usize >= grid.len() || new_pos.y as usize >= grid[0].len() {
        return MoveResult::Out;
    }
    if grid[new_pos.x as usize][new_pos.y as usize] == b'#' {
        return MoveResult::Stuck;
    }
    if nb_iter > grid.len() * grid[0].len() {
        return MoveResult::Loop;
    }
    MoveResult::Ok
}

fn move_in_map(grid: &mut Vec<Vec<u8>>, mut pos: Position, mut dir: Direction) -> Result {
    let mut nb_pos = 1;
    let mut dir_index = 0;
    let mut nb_iter = 0;
    dir = DIRECTIONS[dir_index];

    loop {
        match check_move(&grid, pos, dir, nb_iter) {
            MoveResult::Out => return Result { nb_pos, is_loop: false },
            MoveResult::Ok => {
                pos = pos.move_position(&dir);
                nb_iter += 1;
                if grid[pos.x as usize][pos.y as usize] == b'.' {
                    grid[pos.x as usize][pos.y as usize] = b'X';
                    nb_pos += 1;
                }
            }
            MoveResult::Stuck => {
                dir_index = (dir_index + 1) % 4;
                dir = DIRECTIONS[dir_index];
            }
            MoveResult::Loop => return Result { nb_pos, is_loop: true },
        }
    }
}

fn copy_grid(grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    grid.clone()
}

pub fn day_06()  {
    let path = "src/input06";
    let content = fs::read_to_string(path).unwrap();

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut start_pos = Position { x: 0, y: 0 };

    for (i, line) in content.lines().enumerate() {
        grid.push(line.bytes().collect());
        if let Some(j) = line.find('^') {
            start_pos = Position { x: i as isize, y: j as isize };
            grid[i][j] = b'X';
        }
    }

    let result = move_in_map(&mut copy_grid(&grid), start_pos, DIRECTIONS[0]);
    println!("Part 1: {}", result.nb_pos);

    let mut nb_loop = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'.' {
                let mut new_grid = copy_grid(&grid);
                new_grid[i][j] = b'#';
                let result = move_in_map(&mut new_grid, start_pos, DIRECTIONS[0]);
                if result.is_loop {
                    nb_loop += 1;
                }
            }
        }
    }
    println!("Part 2: {}", nb_loop);

}
