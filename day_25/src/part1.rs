use petgraph::prelude::*;
use petgraph::Graph;
use std::collections::HashMap;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::Result;

pub fn process(data: &str) -> usize {
    let mut cons: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut graph: Graph<&str, String, Undirected> = Graph::new_undirected();
    for line in data.lines() {
        let (comp1, connects) = line.split_once(": ").unwrap();
        for connect in connects.split_whitespace() {
            cons.entry(comp1).or_default().push(connect);
            cons.entry(connect).or_default().push(comp1);
        }
    }
    let node_map: HashMap<&str, NodeIndex> = cons.keys().map(|&n| (n, graph.add_node(n))).collect(); 
    for (node_name, connects) in cons.iter() {
        for connector in connects.iter() {
            graph.update_edge(node_map[node_name], node_map[connector], format!("{}-{}", node_name, connector));
        }
    }

    // Brute force solution never finishes
    // for edges in graph.edge_references().combinations(3) {
    //     let mut temp_graph = graph.clone();
    //     for edge in edges.iter() {
    //         temp_graph.remove_edge(edge.id());
    //     }
    //     if connected_components(&temp_graph) == 2 {
    //         dbg!(&edges.iter().map(|e| e.weight()).collect::<Vec<_>>());
    //         break
    //     }
    // }
    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_min_cut, partition) = min_cut_res.unwrap().unwrap();
    (node_map.len() - partition.len()) * partition.len()

}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 54);
    }

    #[test]
    fn test_graph_process() {
        use petgraph::algo::connected_components;
        use petgraph::dot::{Config, Dot};
        use petgraph::prelude::*;
        use petgraph::Graph;

        let mut graph: Graph<(), (), Undirected> = Graph::new_undirected();
        let a = graph.add_node(()); // node with no weight
        let b = graph.add_node(());
        let c = graph.add_node(());
        let d = graph.add_node(());
        let e = graph.add_node(());
        let f = graph.add_node(());
        let g = graph.add_node(());
        let h = graph.add_node(());

        graph.extend_with_edges(&[
            (a, b),
            (b, c),
            (c, d),
            // (d, a),
            (e, f),
            (f, g),
            (g, h),
            (h, e),
        ]);
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        // a ----> b       e ----> f
        // ^       |       ^       |
        // |       v       |       v
        // d <---- c       h <---- g

        assert_eq!(connected_components(&graph), 2);
    }
}
