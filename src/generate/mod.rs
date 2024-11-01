use std::ops::RangeBounds;

use petgraph::graph::UnGraph;
pub mod geng;

use crate::Graph;

/// Generate graphs using the geng program from nauty.
///
/// # Arguments
/// * `n`: The number of vertices in the graphs to generate.
/// * `m`: The number of edges in the graphs to generate. Use '..' for any amount.
/// * `options`: A list of options to pass to geng.
pub fn generate_graphs(
    n: usize,
    m: impl RangeBounds<usize>,
    options: &[geng::GengOption],
) -> std::io::Result<impl Iterator<Item = Graph>> {
    Ok(geng::call_geng_with_args(n, m, options)?.map(graph6_to_graph))
}

fn graph6_to_graph(graph6: String) -> Graph {
    Graph::from_g6(&graph6).expect(format!("Invalid graph6 string: {}", graph6).as_str())
}

pub fn graph_to_petgraph(graph6: Graph) -> UnGraph<(), ()> {
    let mut graph = UnGraph::new_undirected();
    let nodes: Vec<_> = (0..graph6.n).map(|_| graph.add_node(())).collect();

    for u in 0..graph6.n {
        for v in 0..graph6.n {
            if graph6.bit_vec[graph6.n * u + v] == 1 {
                graph.add_edge(nodes[u], nodes[v], ());
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::generate_graphs;
    use super::geng::GengOption;

    #[test]
    fn generate_graphs_of_size_1() {
        let graphs = generate_graphs(1, &[]);
        assert_eq!(graphs.count(), 1);
    }

    #[test]
    fn generate_graphs_of_size_2() {
        let graphs = generate_graphs(2, &[]);
        assert_eq!(graphs.count(), 2);
    }

    #[test]
    fn generate_connected_graphs_of_size_2() {
        let graphs = generate_graphs(2, &[GengOption::Connected]);
        assert_eq!(graphs.count(), 1);
    }

    #[test]
    fn generate_connected_chordal_graphs_of_size_4() {
        let graphs = generate_graphs(4, &[GengOption::Connected, GengOption::Chordal]);
        assert_eq!(graphs.count(), 3);
    }
}
