#![allow(unused)]
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use petgraph::Graph;
use petgraph::adj::NodeIndex;
use petgraph::graph::{DiGraph};
use petgraph::dot::{Dot, Config};
use petgraph::algo::{tarjan_scc, is_cyclic_directed, dijkstra, all_simple_paths};
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

    //************* CHANGE PARAMS HERE **********/
    let mut sat_only: bool = true;
    //*******************************************/

    //println!("\n\n\n\n\n\n\n\n");


    let contents = fs::read_to_string("../21_Complete_Data/reductions.csv")
        .expect("Something went wrong reading the file");
    let tags = fs::read_to_string("../21_Complete_Data/problems.txt")
        .expect("woops");

    let row_array: Vec<&str> = contents.split("\n").collect();
    let tags: Vec<&str> = tags.split("\n").collect();

    let mut g: petgraph::Graph<String, ReductionSource> = DiGraph::<String, ReductionSource>::new();

    //A var to keep track of special sat node id.
    let mut sat_node: Option<petgraph::stable_graph::NodeIndex> = None;

    let mut row_count = 1;
    for row in row_array {
        let columns: Vec<&str> = row.split(",").collect();
        let columns_length = columns.len();
        let mut col_count = 1;
        for col in columns {
            let row_tag = &tags[row_count - 1].to_string();
            let col_tag = &tags[col_count - 1].to_string();
            if row_count == 1 {
                if (col_tag.to_string().contains("SAT") && !col_tag.to_string().contains("3")) {
                    sat_node = Some(g.add_node(col_tag.to_string()));
                } else {
                    //Don't keep track of the node ID if it isn't sat.
                    g.add_node(col_tag.to_string());
                }
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
    let mut biggest_longest: Option<Vec<petgraph::stable_graph::NodeIndex>> = None;

    let mut longest = 0;


    if (!sat_only) {
        //let mut longest_simple_path: Option<Vec<petgraph::stable_graph::NodeIndex>> = ways.pop().unwrap();
        //Looking for longest simple path
        for current_node in g.node_indices() {
            let mut ways: Vec<Vec<petgraph::stable_graph::NodeIndex>> = all_simple_paths::<Vec<_>, _>(&g, current_node, current_node, 1, None)
                .collect::<Vec<_>>();
            ways.sort_by(|b, a| b.len().cmp(&a.len()));
            if (ways.len() > 0) {
                //pop while same length here
                let mut longest_simple_path: Vec<petgraph::stable_graph::NodeIndex> = ways.pop().unwrap();
                //println!("Longest simple path containing {} is length: {}", g.node_weight(current_node).unwrap_or(&("Unknown".to_string())).to_string(), (longest_simple_path.len() - 1).to_string());
                if (longest_simple_path.len() > 0) {
                    if (longest_simple_path.len() > longest) {
                        longest = longest_simple_path.len();
                        biggest_longest = Some(longest_simple_path);
                    }
                /*  let mut node_index: Option<petgraph::stable_graph::NodeIndex> = longest_simple_path.pop();
                    while (!node_index.is_none()) {
                        println!("{}", g.node_weight(node_index.unwrap()).unwrap().to_string());
                        node_index = longest_simple_path.pop();
                    }*/
                }
            }
        }
        if (biggest_longest.is_some()) {
            let mut longest_simple_path: Vec<petgraph::stable_graph::NodeIndex> = biggest_longest.unwrap();
            let mut node_index: Option<petgraph::stable_graph::NodeIndex> = longest_simple_path.pop();
            while (!node_index.is_none()) {
                println!("{}", g.node_weight(node_index.unwrap()).unwrap().to_string());
                node_index = longest_simple_path.pop();
            }
        }
    } else {
        //SAT only.
        if (!sat_node.is_none()) {
            let sat_node: petgraph::stable_graph::NodeIndex = sat_node.unwrap();
            let mut ways: Vec<Vec<petgraph::stable_graph::NodeIndex>> = all_simple_paths::<Vec<_>, _>(&g, sat_node, sat_node, 1, None)
                .collect::<Vec<_>>();
            ways.sort_by(|b, a| b.len().cmp(&a.len()));
            if (ways.len() > 0) {
                let mut longest_simple_path: Vec<petgraph::stable_graph::NodeIndex> = ways.pop().unwrap();
                if (longest_simple_path.len() > 0) {

                    let longest_length = longest_simple_path.len();
                    while (longest_simple_path.len() == longest_length) {
                    //Print out all longest circuits
                        println!("*********** Longest simple path sat to sat {}", (longest_simple_path.len() - 1).to_string());
                        let mut node_index: Option<petgraph::stable_graph::NodeIndex> = longest_simple_path.pop();
                        while (!node_index.is_none()) {
                            println!("{}", g.node_weight(node_index.unwrap()).unwrap().to_string());
                            node_index = longest_simple_path.pop();
                        }
                        longest_simple_path = ways.pop().unwrap();
                    }
                }
               
            }
        }
    }    
   
    //Looking for scc. This code is weird. No reason to have a loop.
    // TODO: Fix this up. Whatever for now.
    let mut scc = tarjan_scc(&g);
    let mut pop_val = scc.pop();
    while pop_val != None {
        let unwrapped = pop_val.unwrap();

        //Check for scc, output yes if there is one and list it.
        if num_nodes == unwrapped.len() {
            let mut count = 1;
            for i in unwrapped {
                //println!("{}", &tags[count - 1].to_string());

                //println!("{}", g[i]);
                count = count + 1;
            }

            //println!("~~~~~~~~~~~~~~ Found SCC! ~~~~~~~~~~~~~~");
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