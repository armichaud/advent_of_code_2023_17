use nalgebra::DMatrix;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() {
    println!("{}", build_matrix("example.txt"));
}
