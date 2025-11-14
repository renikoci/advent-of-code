use std::fs::File;
use std::io::BufRead;
use std::time::SystemTime;

#[derive(PartialEq)]
enum Decision {
    Continue,
    Loop,
    OutOfBonds,
}

#[derive(Copy, Clone)]
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

    fn get_direction_string(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

fn main() {
    let now = SystemTime::now();
    let mut grid = get_grid("test.txt");

    // default direction
    let mut direction = Direction::Up;

    // initial position
    let mut position = get_initial_position(&grid);

    let mut count = 0;
    loop {
        if grid[position.0 as usize][position.1 as usize] == '.'
        // || grid[position.0 as usize][position.1 as usize] == '^'
        {
            let mut temp_grid = grid.clone();
            let mut temp_position = position;
            let mut temp_direction = direction;
            let next_pos = get_new_pos(&temp_direction, &temp_position);

            // Temporary block the cell forward to the guard and check for loops
            if next_pos.0 >= 0
                && next_pos.0 < grid.len() as i32
                && next_pos.1 >= 0
                && next_pos.1 < grid[0].len() as i32
            {
                temp_grid[next_pos.0 as usize][next_pos.1 as usize] = '#';

                loop {
                    print_grid(&temp_grid);
                    match move_guard_and_check_loop(
                        &temp_grid,
                        &mut temp_position,
                        &mut temp_direction,
                    ) {
                        Decision::Loop => {
                            count += 1;
                            println!("Loop found {}{}", temp_position.0, temp_position.1);
                            break;
                        }
                        Decision::OutOfBonds => break,
                        Decision::Continue => {
                            temp_grid[temp_position.0 as usize][temp_position.1 as usize] =
                                temp_direction.get_direction_string();
                        }
                    }
                }
                // Set back path
                temp_grid[next_pos.0 as usize][next_pos.1 as usize] = '.';
            }
        }
        grid[position.0 as usize][position.1 as usize] = direction.get_direction_string();

        if move_guard_and_check_loop(&grid, &mut position, &mut direction) == Decision::OutOfBonds {
            break;
        }
    }

    println!("{} ", count);
    println!(
        "Elapsed time: {}ms ",
        now.elapsed().expect("I hate rust").as_millis()
    );
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
        if x < grid.len() && y < grid[0].len() {
            return Some(grid[x][y]);
        }
    }
    None
}

fn move_guard_and_check_loop(
    grid: &Vec<Vec<char>>,
    current_position: &mut (usize, usize),
    current_direction: &mut Direction,
) -> Decision {
    let new_pos = get_new_pos(current_direction, current_position);

    match get_cell(&grid, new_pos) {
        Some('#') => {
            *current_direction = current_direction.turn_clockwise();
            // Check for loop when turning
            let temp_position = get_new_pos(current_direction, current_position);
            if grid[temp_position.0 as usize][temp_position.1 as usize]
                == current_direction.get_direction_string()
            {
                return Decision::Loop;
            }
            Decision::Continue
        }

        Some(_) => {
            *current_position = (new_pos.0 as usize, new_pos.1 as usize);
            Decision::Continue
        }

        None => Decision::OutOfBonds, // out of bounds
    }
}

fn get_new_pos(direction: &Direction, position: &(usize, usize)) -> (i32, i32) {
    let delta = direction.get_direction();
    let new_pos = (position.0 as i32 + delta.0, position.1 as i32 + delta.1);
    new_pos
}
