use std::fs::File;
use std::io::{BufReader, BufRead};
use nalgebra::DMatrix;
use petgraph::Directed;
use petgraph::graph::Graph;
use petgraph::algo::dijkstra;

type Coord = (usize, usize);

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


fn min_heat_loss(file: &str) -> usize {
    let matrix = build_matrix(file);
    let mut graph = Graph::<Coord, usize, Directed>::new();
    for row in 0..matrix.nrows() {
        for col in 0..matrix.ncols() {
            graph.add_node((row, col));
        }
    }
    for row in 0..matrix.nrows() {
        for col in 0..matrix.ncols() {
            let location = (row, col);
            let source = graph.node_indices().find(|i| graph[*i] == location).unwrap();
            let mut neighbors: Vec<(Coord, usize)> = Vec::new();
            // Up
            let mut weight = 0;
            for i in ((row as i32 - 3)..row as i32).rev() {
                if i > -1 {
                    let destination = (i as usize, col);
                    weight += matrix[destination];
                    neighbors.push((destination, weight));
                }
            }
            // Down
            let mut weight = 0;
            for i in row + 1..row + 4 {
                if i < matrix.nrows() {
                    let destination = (i, col);
                    weight += matrix[destination];
                    neighbors.push((destination, weight));
                }
            }
            // Left
            let mut weight = 0;
            for i in ((col as i32 - 3)..col as i32).rev() {
                if i > -1 {
                    let destination = (row, i as usize);
                    weight += matrix[destination];
                    neighbors.push((destination, weight));
                }
            }
            // Right
            let mut weight = 0;
            for i in col + 1..col + 4 {
                if i < matrix.ncols() {
                    let destination = (row, i);
                    weight += matrix[destination];
                    neighbors.push((destination, weight));
                }
            }
            for neighbor in neighbors {
                let target = graph.node_indices().find(|i| graph[*i] == neighbor.0).unwrap();
                println!("{:?} -> {:?} = {}", location, neighbor.0, neighbor.1);
                graph.add_edge(source, target, neighbor.1);
            }
        }
    }
    let start = graph.node_indices().find(|i| graph[*i] == (0,0)).unwrap();
    let goal = graph.node_indices().find(|i| graph[*i] == (matrix.nrows() - 1, matrix.ncols() - 1)).unwrap();
    let node_map = dijkstra(&graph, start, Some(goal), |e| *e.weight());
    *node_map.get(&goal).unwrap()
}

fn main() {
    assert_eq!(min_heat_loss("example.txt"), 102);
    // assert_eq!(dijkstra("input.txt"), 0);
}