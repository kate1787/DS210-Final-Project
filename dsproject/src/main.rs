use petgraph::graph::DiGraph;
use petgraph::algo::dijkstra;

use petgraph::visit::{IntoNodeReferences, VisitMap, Visitable};
use std::collections::{HashMap, BTreeMap};

use std::fs::File;
use std::io::{self, BufRead};


// Step 1. 
// Function to read the file and construct the graph
// 

fn construct_graph_from_file(path: &str) -> io::Result<DiGraph<(), ()>> {
    let mut graph = DiGraph::new();
    let mut node_indices = Vec::new();

    //

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    //

    for line in reader.lines() {
        let line = line?;
        if !line.starts_with('#') {
            let nodes: Vec<&str> = line.split_whitespace().collect();
            if nodes.len() == 2 {
                let from = nodes[0].parse::<usize>().unwrap();

                let to = nodes[1].parse::<usize>().unwrap();


                while node_indices.len() <= from || node_indices.len() <= to {
                    node_indices.push(graph.add_node(()));
                }

                graph.add_edge(node_indices[from], node_indices[to], ());
            }
        }
    }


    Ok(graph)

}



// Step 2. 
// Function to perform basic network analysis

fn basic_network_analysis(graph: &DiGraph<(), ()>) {

    // Degree Distribution
    let mut degree_distribution = HashMap::new();
    for node_ref in graph.node_references() {
        let degree = graph.neighbors(node_ref.0).count();
        *degree_distribution.entry(degree).or_insert(0) += 1;
    }


    // Print basic network statistics
    println!("Basic Network Analysis:");

    println!("Number of nodes: {}", graph.node_count());
    println!("Number of edges: {}", graph.edge_count());
    println!("Degree Distribution: {:?}", degree_distribution);


}



// Step 3. 
//  Function for Degree Distributions Analysis to return data

fn degree_distributions_analysis(graph: &DiGraph<(), ()>) -> HashMap<usize, usize> {

    let mut second_degree_distribution = HashMap::new();
    for node_ref in graph.node_references() {
        let mut visited = graph.visit_map();
        let neighbors = graph.neighbors(node_ref.0);
        let mut second_degree_count = 0;


        for neighbor in neighbors {
            for second_neighbor in graph.neighbors(neighbor) {
                if !visited.is_visited(&second_neighbor) {
                    second_degree_count += 1;
                    visited.visit(second_neighbor);
                }
            }
        }

        *second_degree_distribution.entry(second_degree_count).or_insert(0) += 1;
    }

    second_degree_distribution
}



// Step 4. 
// Function for Closeness Centrality Analysis

fn closeness_centrality_analysis(graph: &DiGraph<(), ()>) -> BTreeMap<usize, f64> {

    let mut centrality_scores = BTreeMap::new();
    let node_count = graph.node_count().min(1000); // Limit to first 1000 nodes


    for (i, node_ref) in graph.node_references().take(node_count).enumerate() {
        let paths = dijkstra(graph, node_ref.0, None, |_| 1);
        let total_distance: usize = paths.values().map(|&d| d).sum();
        let closeness_centrality = if total_distance > 0 { 1.0 / total_distance as f64 } else { 0.0 };
        centrality_scores.insert(i, closeness_centrality);
    }


    centrality_scores
}



// main
fn main() {


    let path = "amazon0302.txt"; 
    match construct_graph_from_file(path) {
        Ok(graph) => {
            println!("Graph constructed successfully!");
            basic_network_analysis(&graph);


            let _degree_distribution = degree_distributions_analysis(&graph); 


            let centrality_scores = closeness_centrality_analysis(&graph);
            
            println!("Closeness Centrality Scores:");
            for (node, score) in centrality_scores.iter().take(10) {
                println!("Node {}: Closeness Centrality = {:.20}", node, score);
            }
        }
        Err(e) => println!("Error constructing graph: {}", e),

    }

}

// Test

#[cfg(test)]
mod tests {
    use super::*;

    // Test the graph construction from the file
    #[test]
    fn test_construct_graph_from_file() {
        let path = "amazon0302.txt"; 
        let graph = construct_graph_from_file(path).unwrap();

        // 
        assert_eq!(graph.node_count(), 262111);
        assert_eq!(graph.edge_count(), 1234877);
    }


    // Test basic network analysis function
    #[test]
    fn test_basic_network_analysis() {
        let path = "amazon0302.txt"; 
        let graph = construct_graph_from_file(path).unwrap();

        // Capture the printed output 
        // Ensure if it runs without error
        basic_network_analysis(&graph);
    }


    // Test degree distributions analysis
    #[test]
    fn test_degree_distributions_analysis() {
        let path = "amazon0302.txt"; // Adjust the path as needed
        let graph = construct_graph_from_file(path).unwrap();
    
        let degree_distribution = degree_distributions_analysis(&graph);

        // define a margin of error
        let margin = 100; 
        let expected_degree_count = 4541;
        let actual_degree_count = *degree_distribution.get(&0).unwrap_or(&0);

        assert!(
            actual_degree_count >= expected_degree_count - margin 
        && actual_degree_count <= expected_degree_count + margin
        );
    
        // Specific degree check
        // In order to check if the number of nodes with a degree of n is m
        // assert_eq!(*degree_distribution.get(&5).unwrap_or(&0), 4541);
        
    }


    // Test closeness centrality analysis
    #[test]
    fn test_closeness_centrality_analysis() {
        let path = "amazon0302.txt"; // Adjust the path as needed
        let graph = construct_graph_from_file(path).unwrap();

        let centrality_scores = closeness_centrality_analysis(&graph);

        // Perform some basic checks, like ensuring some nodes have centrality scores
        assert!(!centrality_scores.is_empty());
        
    }
}