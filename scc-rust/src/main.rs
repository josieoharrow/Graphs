use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use petgraph::graph::{DiGraph};
use petgraph::dot::{Dot};
use petgraph::algo::{tarjan_scc, is_cyclic_directed, dijkstra};

fn main() {
    println!("\n\n\n\n\n\n\n\n");


    let contents = fs::read_to_string("../reductions.csv")
        .expect("Something went wrong reading the file");

    let row_array: Vec<&str> = contents.split("\n").collect();
    let mut g = DiGraph::<String, String>::new();

    let mut row_count = 1;
    for row in row_array {
        let columns: Vec<&str> = row.split(",").collect();
        let columns_length = columns.len();
        let mut col_count = 1;
        for col in columns {
            if row_count == 1 {
                g.add_node(col_count.to_string());
            }
            let row_index = g.node_indices().find(|i| g[*i] == row_count.to_string()).unwrap();
            let col_index = g.node_indices().find(|i| g[*i] == col_count.to_string()).unwrap();

            if col.trim() == "1" {
                g.add_edge(row_index, col_index, format!("{}{}", row_count.to_string(), col_count.to_string()));//(row_count).to_string());
            }
            col_count += 1;
        }
        row_count += 1;
    }

    let num_nodes = 21;
    let mut tree = false;

    let mut scc = tarjan_scc(&g);
    let mut pop_val = scc.pop();
    while pop_val != None {
        let unwrapped = pop_val.unwrap();

        //Check for scc, output yes if there is one and list it.
        if num_nodes == unwrapped.len() {

            for i in unwrapped {
                println!("{}", g[i]);
            }

            println!("             Found SCC!");
            tree = true;
            //To get the dot file, use Dot::new(&g)
        }
        pop_val = scc.pop();
    }
    if tree == false {
        println!("               No SCC was found :(");
    }
    println!("\n\n\n\n\n\n\n\n");

}