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
    fn get_perpendicular_directions(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Left, Direction::Right],
            Direction::Down => vec![Direction::Left, Direction::Right],
            _ => vec![Direction::Up, Direction::Down],
        }
    }

    fn possible_destinations(self: &Self, location: &Location, nrows: usize, ncols: usize) -> Vec<Location> {
        let (row, col) = location;
        let mut output: Vec<Location> = Vec::new();
        match self {
            Direction::Up => {
                for i in (*row as i32 - 3)..*row as i32 {
                    let destination = (i as usize, *col);
                    if i > -1 {
                        output.push(destination);
                    }
                }
            },
            Direction::Down => {
                for i in *row + 1..(*row + 4) {
                    let destination = (i as usize, *col);
                    if i < nrows {
                        output.push(destination);
                    }
                }
            },
            Direction::Left => {
                for i in (*col as i32 - 3)..*col as i32 {
                    let destination = (*row, i as usize);
                    if i > -1 {
                        output.push(destination);
                    }
                }
            },
            Direction::Right => {
                for i in *col + 1..(*col + 4) {
                    let destination = (*row, i as usize);
                    if i < ncols {
                        output.push(destination);
                    }
                }
            },
        }
        //println!("{:?} {:?} {:?}", self, location, output);
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
        },
        Memo {
            location: (0, 0),
            previous_direction: Direction::Down,
            heat_loss: 0,
            visited: HashSet::new(),
        },
    ]);
    let mut min_heat_loss = usize::MAX;
    while stack.len() > 0 {
        let mut current = stack.pop().unwrap();
        
        let updated_heat_loss = current.heat_loss + matrix[(current.location.0, current.location.1)];
        // End of path
        if current.location == (nrows - 1, ncols - 1) {
            min_heat_loss = min(updated_heat_loss, min_heat_loss);
            continue;
        }

        // Skip already visited
        if current.visited.contains(&(current.location.0, current.location.1)) {
            continue;
        }
        println!("{:?}", current.location);

        // Track visit
        current.visited.insert((current.location.0, current.location.1));

        let possible_directions = current.previous_direction.get_perpendicular_directions();
        for direction in possible_directions {
            let possible_destinations = direction.possible_destinations(&current.location, nrows, ncols);
            for destination in possible_destinations {
                stack.push(Memo {
                    location: (destination.0, destination.1),
                    previous_direction: direction,
                    heat_loss: updated_heat_loss,
                    visited: current.visited.clone(),
                });
            }
        }
    }
    min_heat_loss
}

fn main() {
    assert_eq!(brute_force("example.txt"), 102);
    // assert_eq!(brute_force("input.txt"), 0);
}
