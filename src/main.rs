use nalgebra::DMatrix;
use std::cmp::min;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,    
}

impl Direction {
    fn get_all() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn get_all_but(&self) -> Vec<Direction> {
        Direction::get_all()
            .into_iter()
            .filter(|x| x != self)
            .collect::<Vec<_>>()
    }

    fn possible_destinations(self: &Self, location: &Location, nrows: usize, ncols: usize) -> Vec<Location> {
        let (row, col) = location;
        let mut output: Vec<Location> = Vec::new();
        match self {
            Direction::Up => {
                for i in (*row as i32 - 3)..*row as i32 {
                    if i > -1 {
                        output.push((i as usize, *col));
                    }
                }
            },
            Direction::Down => {
                for i in *row..(*row + 3) {
                    if i < nrows {
                        output.push((i as usize, *col));
                    }
                }
            },
            Direction::Left => {
                for i in (*col as i32 - 3)..*col as i32 {
                    if i > -1 {
                        output.push((*row, i as usize));
                    }
                }
            },
            Direction::Right => {
                for i in *col..(*col + 3) {
                    if i < ncols {
                        output.push((*row, i as usize));
                    }
                }
            },
        }
        output
    }
}

type Location = (usize, usize);

#[derive(Debug, Clone)]
struct Memo {
    location: Location,
    previous_direction: Direction,
    heat_loss: usize,
    visited: HashSet<Location>
}

fn build_matrix(filename: &str) -> DMatrix<usize> {
    let file = File::open(filename).unwrap();
    let mut data = Vec::new();
    let reader = BufReader::new(file);
    let mut nrows = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let row = line
            .chars()
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        data.extend(row);
        nrows += 1;
    }
    DMatrix::from_row_slice(nrows, data.len() / nrows, &data)
}

fn brute_force(filename: &str) -> usize {
    let matrix = build_matrix(filename);
    let nrows = matrix.nrows();
    let ncols = matrix.ncols();
    let mut stack: Vec<Memo> = Vec::from(&[
        Memo {
            location: (0, 0),
            previous_direction: Direction::Right,
            heat_loss: 0,
            visited: HashSet::new(),
        }
    ]);
    let mut min_heat_loss = usize::MAX;
    while stack.len() > 0 {
        let current = stack.pop().unwrap();
        println!("{:?}", current);
        if current.location == (nrows - 1, ncols - 1) {
            min_heat_loss = min(current.heat_loss, min_heat_loss);
            continue;
        }

        let mut visited = current.visited.clone();
        visited.insert((current.location.0, current.location.1));
        let possible_directions = current.previous_direction.get_all_but();
        for direction in possible_directions {
            for destination in direction
                    .possible_destinations(&current.location, nrows, ncols)
                    .iter()
                    .filter(|x| !current.visited.contains(x))
                    .collect::<Vec<&Location>>() {
                let heat_loss = current.heat_loss + matrix[(destination.0, destination.1)];
                if heat_loss < min_heat_loss {
                    stack.push(Memo {
                        location: *destination,
                        previous_direction: direction.opposite(),
                        heat_loss,
                        visited: visited.clone(),
                    });
                }
            }
        }
    }
    min_heat_loss
}

fn main() {
    assert_eq!(brute_force("example.txt"), 102);
    // assert_eq!(brute_force("input.txt"), 0);
}
