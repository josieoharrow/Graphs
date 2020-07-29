use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use petgraph::graph::{DiGraph};
use petgraph::algo::{tarjan_scc, is_cyclic_directed, dijkstra};

fn main() {

    let contents = fs::read_to_string("../Reductions.csv")
        .expect("Something went wrong reading the file");
    let row_array = contents.split("/n");

    let mut g = DiGraph::<String, String>::new();

    let mut row_count = 1;
    for row in row_array {
        let columns = row.split(",");
        let mut col_count = 1;
        for col in columns {
            if row_count == 1 {
                g.add_node(col_count.to_string());
            }
            let row_index = g.node_indices().find(|i| g[*i] == row_count.to_string()).unwrap();
            let col_index = g.node_indices().find(|i| g[*i] == col_count.to_string()).unwrap();

            if col == "1" {
                g.add_edge(row_index, col_index, 1.);
            }
            col_count += 1;
        }
        row_count += 1;
    }
    let num_nodes = 21;


    let mut scc = tarjan_scc(&g);
    let mut popVal = scc.pop();
    while popVal != None {
        let unwrapped = popVal.unwrap();
        println!("{}", unwrapped.len().to_string());
        if num_nodes == unwrapped.len() {
            println!("Found SCC!");
        }
        popVal = scc.pop();
    }
      //Check for scc, output yes or no and if there is one, list it.
      // plot the scc
}