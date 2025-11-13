use std::fs::File;
use std::io::BufRead;

static UP: (i32, i32) = (-1, 0);
static DOWN: (i32, i32) = (1, 0);
static LEFT: (i32, i32) = (0, -1);
static RIGHT: (i32, i32) = (0, 1);

fn main() {
    let mut grid = get_grid("input.txt");

    // default direction
    let mut direction = 'u';

    // initial position
    let mut position = get_initial_position(&grid);

    let mut count = 0;

    loop {
        // print_grid(&grid);
        if grid[position.0 as usize][position.1 as usize] != 'X' {
            grid[position.0 as usize][position.1 as usize] = 'X';
            count += 1;
        }
        if !move_guard(&grid, &mut position, &mut direction) {
            break;
        }
    }

    println!("{}", count);
}

fn get_grid(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Plis fix this");
    let reader = std::io::BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    lines
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for (_, row) in grid.iter().enumerate() {
        for (_, &cell) in row.iter().enumerate() {
            print!("{}", cell)
        }
        println!();
    }
    println!("*********************************************");
    println!("*********************************************");
    println!("*********************************************");
}

fn get_initial_position(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == '^' || cell == '>' || cell == '<' || cell == 'v' {
                return (row_index as i32, col_index as i32);
            }
        }
    }

    // default if nothing found
    (0, 0)
}

fn move_guard(
    grid: &Vec<Vec<char>>,
    current_position: &mut (i32, i32),
    current_direction: &mut char,
) -> bool {
    if *current_direction == 'u' {
        let new_x = current_position.0 + UP.0;
        let new_y = current_position.1 + UP.1;
        if new_x < 0 || new_x >= grid.len() as i32 || new_y < 0 || new_y >= grid[0].len() as i32 {
            return false;
        }
        if grid[new_x as usize][new_y as usize] == '#' {
            *current_direction = 'r';
        } else {
            *current_position = (new_x, new_y);
        }
    } else if *current_direction == 'd' {
        let new_x = current_position.0 + DOWN.0;
        let new_y = current_position.1 + DOWN.1;
        if new_x < 0 || new_x >= grid.len() as i32 || new_y < 0 || new_y >= grid[0].len() as i32 {
            return false;
        }
        if grid[new_x as usize][new_y as usize] == '#' {
            *current_direction = 'l';
        } else {
            *current_position = (new_x, new_y);
        }
    } else if *current_direction == 'l' {
        let new_x = current_position.0 + LEFT.0;
        let new_y = current_position.1 + LEFT.1;
        if new_x < 0 || new_x >= grid.len() as i32 || new_y < 0 || new_y >= grid[0].len() as i32 {
            return false;
        }
        if grid[new_x as usize][new_y as usize] == '#' {
            *current_direction = 'u';
        } else {
            *current_position = (new_x, new_y);
        }
    } else if *current_direction == 'r' {
        let new_x = current_position.0 + RIGHT.0;
        let new_y = current_position.1 + RIGHT.1;
        if new_x < 0 || new_x >= grid.len() as i32 || new_y < 0 || new_y >= grid[0].len() as i32 {
            return false;
        }
        if grid[new_x as usize][new_y as usize] == '#' {
            *current_direction = 'd';
        } else {
            *current_position = (new_x, new_y);
        }
    }
    true
}
