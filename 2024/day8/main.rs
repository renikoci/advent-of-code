use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn is_inside(&self, max_x: i32, max_y: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < max_x && self.y < max_y
    }
}

struct Combination {
    c1: Coordinates,
    c2: Coordinates,
}

impl Combination {
    fn get_antinodes(&self, max_x: i32, max_y: i32) -> Vec<Coordinates> {
        let dx: i32 = self.c2.x - self.c1.x;
        let dy: i32 = self.c2.y - self.c1.y;

        let mut coordinates = vec![self.c1, self.c2];

        let mut cur = self.c1;
        loop {
            let next = Coordinates {
                x: cur.x - dx,
                y: cur.y - dy,
            };
            if !next.is_inside(max_x, max_y) {
                break;
            }
            coordinates.push(next);
            cur = next;
        }

        let mut cur = self.c2;
        loop {
            let next = Coordinates {
                x: cur.x + dx,
                y: cur.y + dy,
            };
            if !next.is_inside(max_x, max_y) {
                break;
            }
            coordinates.push(next);
            cur = next;
        }

        coordinates
    }
}

fn read_grid() -> io::Result<Vec<Vec<char>>> {
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Line not found");
        grid.push(line.chars().collect());
    }

    Ok(grid)
}
fn get_coordinate_combinations(grid: &[Vec<char>]) -> Vec<Combination> {
    let mut map: HashMap<char, Vec<Coordinates>> = HashMap::new();
    let mut combs = Vec::new(); // combinations

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '.' {
                // skip
                continue;
            }

            let coord = Coordinates {
                x: i as i32,
                y: j as i32,
            };

            let entry = map.entry(c).or_insert_with(Vec::new);

            for &prev in entry.iter() {
                combs.push(Combination {
                    c1: prev,
                    c2: coord,
                });
            }

            entry.push(coord);
        }
    }

    combs
}

fn main() {
    let now = Instant::now();
    let grid = read_grid().expect("Sth went wrong");
    let rows = grid.len();
    let cols = grid[0].len();

    let combinations = get_coordinate_combinations(&grid);

    let mut unique: HashSet<Coordinates> = HashSet::new();

    for combo in &combinations {
        for coord in combo.get_antinodes(rows as i32, cols as i32) {
            unique.insert(coord);
        }
    }
    println!("Elapsed time: {}ns ", now.elapsed().as_nanos());
    println!("Unique anti-nodes: {}", unique.len());
}
