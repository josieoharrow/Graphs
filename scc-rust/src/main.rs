#![allow(unused)]
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use petgraph::Graph;
use petgraph::graph::{DiGraph};
use petgraph::dot::{Dot, Config};
use petgraph::algo::{tarjan_scc, is_cyclic_directed, dijkstra};
use petgraph::visit::EdgeRef;
use petgraph::visit::IntoEdgeReferences;
use petgraph::visit::IntoNodeReferences;

#[derive(Debug)]
#[derive(PartialEq)]
enum ReductionSource {
    
    Original,
    Karp,
    Outside
}

fn edge_attributes<G>(graph: G, edge: G::EdgeRef) -> String where G: IntoEdgeReferences, G::EdgeWeight: PartialEq<ReductionSource>,
{
    //do something to color code here.

    if (*(EdgeRef::weight(&edge)) == ReductionSource::Original) {
        return "color = \"black\"".to_string();

    } else if (*(EdgeRef::weight(&edge)) == ReductionSource::Karp) {
        return "color = \"deeppink\"".to_string();
    } else if (*(EdgeRef::weight(&edge)) == ReductionSource::Outside) {
        return "color = \"palegreen4\"".to_string();
    } else {
        return "".to_string();
    }
}

fn empty_string<G>(graph: G, node: G::NodeRef) -> String where G: IntoNodeReferences {
    return "".to_string();
}

fn main() -> std::io::Result<()> {


    println!("\n\n\n\n\n\n\n\n");


    let contents = fs::read_to_string("../21_Complete_Data/reductions.csv")
        .expect("Something went wrong reading the file");
    let tags = fs::read_to_string("../21_Complete_Data/problems.txt")
        .expect("woops");

    let row_array: Vec<&str> = contents.split("\n").collect();
    let tags: Vec<&str> = tags.split("\n").collect();

    let mut g: petgraph::Graph<String, ReductionSource> = DiGraph::<String, ReductionSource>::new();
    //let mut g: petgraph::Graph<String, i32> = DiGraph::<String, i32>::new();

    let mut row_count = 1;
    for row in row_array {
        let columns: Vec<&str> = row.split(",").collect();
        let columns_length = columns.len();
        let mut col_count = 1;
        for col in columns {
            let row_tag = &tags[row_count - 1].to_string();
            let col_tag = &tags[col_count - 1].to_string();
            if row_count == 1 {
                g.add_node(col_tag.to_string());
            }
            let row_index = g.node_indices().find(|i| g[*i] == row_tag.to_string()).unwrap();
            let col_index = g.node_indices().find(|i| g[*i] == col_tag.to_string()).unwrap();

            let reduction_value: &str = col.trim();
            if reduction_value == "1" {
                g.add_edge(row_index, col_index, ReductionSource::Original);
            } else if reduction_value == "2" {
                g.add_edge(row_index, col_index, ReductionSource::Karp);
            } else if reduction_value == "3" {
                g.add_edge(row_index, col_index, ReductionSource::Outside);
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
            let mut count = 1;
            for i in unwrapped {
                println!("{}", &tags[count - 1].to_string());

                println!("{}", g[i]);
                count = count + 1;
            }

            println!("~~~~~~~~~~~~~~ Found SCC! ~~~~~~~~~~~~~~");
            tree = true;
            //To get the dot file, use 
        }
        pop_val = scc.pop();
    }
    if tree == false {
        println!("No SCC was found :(");
    }
    println!("\n");

    //Create .dot file
    let mut file = File::create("../21_Complete_Data/graph.dot")?;
    //file.write_all(format!("{:?}", Dot::new(&g)).as_bytes())?;
    //Uncomment below to have unlabeled edges.
    file.write_all(format!("{:?}", Dot::with_attr_getters(&g, &[Config::EdgeNoLabel], &edge_attributes, &empty_string)).as_bytes())?;
    Ok(())
}