use std::fs;
struct Direction {
    x : i32,
    y : i32
}

const POSSIBLE_DIRECTIONS: [Direction; 8] = [
    Direction { x: -1, y:  0 },
    Direction { x: -1, y:  1 },
    Direction { x:  0, y:  1 },
    Direction { x:  1, y:  1 },
    Direction { x:  1, y:  0 },
    Direction { x:  1, y: -1 },
    Direction { x:  0, y: -1 },
    Direction { x: -1, y: -1 }

];
pub fn day_04() {
    let content = fs::read_to_string("src/input04").unwrap();
    println!("Part 1 : {}", part01(&content));
    println!("Part 2 : {}", part02(&content));

}

fn get_indexes(index : usize,  nb_col : usize) -> (usize, usize){
    (index/(nb_col), index%(nb_col))
}

fn part01(content : &String) -> i32 {
    let to_find = "XMAS";
    let mut nb_word = 0;
    let re = regex::Regex::new(r"X");
    let grid : Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    re.unwrap().find_iter(&content).for_each(|cap| {
        let (i, j) = get_indexes(cap.start(),  grid[0].len() + 1);

        for dir in POSSIBLE_DIRECTIONS {
           if find_in_dir(to_find.parse().unwrap(), dir, i as i32, j as i32, &grid) {
               nb_word += 1
           }
        }
    });

    nb_word


}

fn check_bounds(i : i32, j : i32, nb_rows : usize, nb_cols : usize) -> bool {
    ! (i < 0 || j < 0 || i >= (nb_rows as i32) || j >= (nb_cols as i32))
}

fn part02(content : &String) -> i32 {
    let re = regex::Regex::new(r"A");
    let grid : Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let mut nb_word = 0;
    re.unwrap().find_iter(&content).for_each(|cap| {
        let (i, j) = get_indexes(cap.start(),  grid[0].len() + 1);

        if (find_in_dir("MAS".parse().unwrap(), Direction{x: 1, y: 1}, (i as i32)-1, (j as i32)-1 , &grid) || find_in_dir("SAM".parse().unwrap(), Direction{x: 1, y: 1}, (i as i32)-1, (j as i32)-1 , &grid)) && (find_in_dir("MAS".parse().unwrap(), Direction{x: 1, y: -1}, (i as i32)-1, (j as i32)+1 , &grid) || find_in_dir("SAM".parse().unwrap(), Direction{x: 1, y: -1}, (i as i32)-1, (j as i32)+1 , &grid))  {

            nb_word += 1
        }
    });
    nb_word
}

fn find_in_dir(to_find : String, dir : Direction, i : i32, j : i32, grid : &Vec<Vec<char>>) -> bool {
    let mut found = true;
    for a  in 0..to_find.len() {
        let (x, y) = (i + (a as i32) * dir.x, j + (a as i32) * dir.y);
        if !check_bounds(x, y, grid.len(), grid[0].len()) || !grid[x as usize][y as usize].eq(&to_find.chars().nth(a).unwrap()) {
            found = false;
            break
        }
    }

    found
}