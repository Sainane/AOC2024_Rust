use std::fs;


pub fn day_12() {
    println!("Day 12");

    let content = fs::read_to_string("src/input12").expect("Error reading the file");

    let data: &mut Vec<Vec<char>> = &mut Vec::new();
    for line in content.lines() {
        let res: Vec<char> = line.chars().collect();
        data.push(res);
    }

    println!("\t- Part 1: {}", part01(&mut data.clone()));
    println!("\t- Part 2: {}", part02(&mut data.clone()));
}

fn part01(data : &mut Vec<Vec<char>>) -> i32 {
    let plots = find_plots(data);

    let mut price = 0;
    for plot in plots {
        price += area(&plot) * perimeter(&plot);
    }
    price
}

fn part02(data : &mut Vec<Vec<char>>) -> i32 {
    let plots = find_plots(data);
    let mut price = 0;
    for plot in plots {
        price += sides(&plot, data.len() as i32, data[0].len() as i32) * area(&plot);
    }
    price
}
fn find_plots(data : &mut Vec<Vec<char>>) -> Vec<Plot> {
    let mut plots: Vec<Plot> = Vec::new();
    for line in 0..data.len() {
        for value in 0..data[line].len() {
            if data[line][value] == '#' {
                continue
            } else {
                let plot = &mut Plot {vec: Vec::new(), letter: data[line][value]};
                find_plot(data, line as i32, value as i32, plot);
                plots.push(plot.clone());

            }
        }
    }
    plots
}
fn find_plot(data : &mut Vec<Vec<char>>, x : i32, y : i32, plot: &mut Plot) {
    if check_bounds(x, y, data.len(), data[0].len()) && plot.letter == data[x as usize][y as usize] {
        plot.vec.push((x, y));
        data[x as usize][y as usize] = '#';
        find_plot(data, x + 1, y, plot);
        find_plot(data, x - 1, y, plot);
        find_plot(data, x, y + 1, plot);
        find_plot(data, x, y - 1, plot);
     
    }
}

#[derive(Clone)]
struct Plot {
    vec: Vec<(i32, i32)>,
    letter: char
}

fn area(plot : &Plot) -> i32 {
    plot.vec.len() as i32
}

fn perimeter(plot: &Plot) -> i32 {
    let mut perimeter = 0;
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for &(x, y) in &plot.vec {
        for &(dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;
            if !plot.vec.contains(&(nx, ny)) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn sides(plot: &Plot, nb_rows: i32, nb_cols: i32) -> i32 {
    let mut borders = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for &(x, y) in &plot.vec {
        for (i, &(dx, dy)) in directions.iter().enumerate() {
            let nx = x + dx;
            let ny = y + dy;
            if !plot.vec.contains(&(nx, ny)) {
                if dy == 0 {
                    borders[i].push((nx, ny));
                } else {
                    borders[i].push((ny, nx));
                }
            }
        }
    }

    let mut visualizations = vec![
        vec![vec![0; (nb_cols + 2) as usize]; (nb_rows + 2) as usize],
        vec![vec![0; (nb_cols + 2) as usize]; (nb_rows + 2) as usize],
        vec![vec![0; (nb_rows + 2) as usize]; (nb_cols + 2) as usize],
        vec![vec![0; (nb_rows + 2) as usize]; (nb_cols + 2) as usize],
    ];

    for (i, border) in borders.iter().enumerate() {
        update_visualization(&mut visualizations[i], border);
    }

    let mut nb_sides = 0;
    for visualization in visualizations {
        nb_sides += find_sides(visualization);
    }
    nb_sides
}

fn update_visualization(visualization: &mut Vec<Vec<i32>>, border: &Vec<(i32, i32)>) {
    for &(x, y) in border {
        visualization[(x + 1) as usize][(y + 1) as usize] += 1;
    }
}
fn find_sides(border : Vec<Vec<i32>>) -> i32 {
    let mut last_val = 0;
    let mut count = 0;
  for line in border {
      for value in line {
          if value != 0 && last_val == 0  {

              count += value;
          }
          last_val = value;
      }
  }

    count
}
fn print_data(data : &Vec<Vec<i32>>) {
    for line in data {
        for value in line {
         print!("{}", value);
        }
        println!();
    }
    println!();
}

pub fn check_bounds(i : i32, j : i32, nb_rows : usize, nb_cols : usize) -> bool {
    ! (i < 0 || j < 0 || i >= (nb_rows as i32) || j >= (nb_cols as i32))
}