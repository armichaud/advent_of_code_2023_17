use std::cmp::{Reverse, min};
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use nalgebra::DMatrix;
use petgraph::Directed;
use petgraph::prelude::NodeIndex;
use petgraph::graph::Graph;

type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Axis {
    X,
    Y,
}

impl Axis {
    fn opposite(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }

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

#[derive(Eq, PartialEq, Debug)]
struct WeightedNode {
    node_id: NodeIndex,
    weight: usize,
    axis: Axis,
}

impl Ord for WeightedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for WeightedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn update_weight(heap: &mut BinaryHeap<Reverse<WeightedNode>>, node_id: NodeIndex, weight: usize, axis: Axis) {
    let index = heap.iter().position(|&Reverse(ref node)| node.node_id == node_id);
    if index.is_some() {
        heap.retain(|x| x.0.node_id.index() != node_id.index());
        heap.push(Reverse(WeightedNode {
            node_id,
            weight,
            axis,
        }));
    } else {
        println!("Node ID {} not found in the heap.", node_id.index());
    }
}

fn dijkstra(graph: &Graph<Coord, usize, Directed>, start: NodeIndex, goal: NodeIndex, axis: Axis) -> usize {
    let mut priority_queue: BinaryHeap<Reverse<WeightedNode>> = BinaryHeap::new();

    for node in graph.node_indices() {
        priority_queue.push(Reverse(WeightedNode {
            node_id: node,
            weight: usize::max_value(),
            axis,
        }));
    }
    let mut visited = HashSet::<NodeIndex>::new();
    update_weight(&mut priority_queue, start, 0, axis.clone());
    while let Some(Reverse(node)) = priority_queue.pop() {
        let axis = node.axis;
        let node_index: NodeIndex = NodeIndex::new(node.node_id.index() as usize);
        visited.insert(node_index);
        let coord: Coord = graph[node_index];
        //println!("Visiting node {:?} with weight {}.", coord, node.weight);
        if node.node_id.index() == goal.index() {
            return node.weight;
        }
        for neighbor in graph.neighbors(NodeIndex::new(node.node_id.index() as usize)) {
            let (x, y) = graph[neighbor];
            if visited.contains(&neighbor) {
                continue;
            }
            // We only want to consider neighbors on the axis perpendicular to the previous step.
            // This is the entire reason we need to have a custom dijkstra implementation.
            if axis == Axis::X && x != coord.0 {
                continue;
            }
            if axis == Axis::Y && y != coord.1 {
                continue;
            };
            let edge_weight = graph.edges_connecting(NodeIndex::new(node.node_id.index() as usize), neighbor).next().unwrap().weight();
            let Reverse(neighbor_node) = priority_queue.iter().find(|&Reverse(x)| x.node_id == neighbor).unwrap();
            let potential_weight = neighbor_node.weight.min(node.weight + edge_weight);
            update_weight(&mut priority_queue, neighbor, potential_weight, axis.opposite());
        }
    }
    usize::max_value()
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
            weight = 0;
            for i in row + 1..row + 4 {
                if i < matrix.nrows() {
                    let destination = (i, col);
                    weight += matrix[destination];
                    neighbors.push((destination, weight));
                }
            }
            // Left
            weight = 0;
            for i in ((col as i32 - 3)..col as i32).rev() {
                if i > -1 {
                    let destination = (row, i as usize);
                    weight += matrix[destination];
                    neighbors.push((destination, weight));
                }
            }
            // Right
            weight = 0;
            for i in col + 1..col + 4 {
                if i < matrix.ncols() {
                    let destination = (row, i);
                    weight += matrix[destination];
                    neighbors.push((destination, weight));
                }
            }
            for neighbor in neighbors {
                let target = graph.node_indices().find(|i| graph[*i] == neighbor.0).unwrap();
                graph.add_edge(source, target, neighbor.1);
            }
        }
    }
    let start = graph.node_indices().find(|i| graph[*i] == (0,0)).unwrap();
    let goal = graph.node_indices().find(|i| graph[*i] == (matrix.nrows() - 1, matrix.ncols() - 1)).unwrap();
    let shortest_path_starting_horizontal = dijkstra(&graph, start, goal, Axis::X);
    let shortest_path_starting_vertical = dijkstra(&graph, start, goal, Axis::Y);
    println!("Shortest path starting horizontally: {}", shortest_path_starting_horizontal);
    println!("Shortest path starting vertically: {}", shortest_path_starting_vertical);
    min(shortest_path_starting_horizontal, shortest_path_starting_vertical)
}

fn main() {
    assert_eq!(min_heat_loss("example.txt"), 102);
    //assert_eq!(min_heat_loss("input.txt"), 0);
}