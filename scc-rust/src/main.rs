use ndarray::arr2;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use petgraph::graph::{DiGraph};
use petgraph::algo::{tarjan_scc};

fn main() {
    let g = DiGraph::<i32, ()>::from_edges(&[
        (1, 2), (2, 3), (3, 4),
        (1, 4)]);
    //Populate and make array of arrays
    //generate graph
    //Check for scc, output yes or no and if there is one, list it.
}