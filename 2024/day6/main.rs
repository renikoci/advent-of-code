use std::fs::File;
use std::io::BufRead;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_direction(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn main() {
    let mut grid = get_grid("input.txt");

    // default direction
    let mut direction = Direction::Up;

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

fn get_initial_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == '^' || cell == '>' || cell == '<' || cell == 'v' {
                return (row_index, col_index);
            }
        }
    }

    // default if nothing found
    (0, 0)
}
fn get_cell(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> Option<char> {
    if pos.0 >= 0 && pos.1 >= 0 {
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        if x < grid.len() && x < grid[0].len() {
            return Some(grid[x][y]);
        }
    }
    None
}

fn move_guard(
    grid: &Vec<Vec<char>>,
    current_position: &mut (usize, usize),
    current_direction: &mut Direction,
) -> bool {
    let delta = current_direction.get_direction();
    let new_pos = (
        current_position.0 as i32 + delta.0,
        current_position.1 as i32 + delta.1,
    );

    match get_cell(&grid, new_pos) {
        Some('#') => {
            *current_direction = current_direction.turn_clockwise();
            true
        }

        Some(_) => {
            *current_position = (new_pos.0 as usize, new_pos.1 as usize);
            true
        }

        None => false, // out of bounds
    }
}
