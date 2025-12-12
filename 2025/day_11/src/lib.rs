pub use petgraph::graph::{DiGraph, NodeIndex};
pub use petgraph::visit::Topo;
use std::collections::HashMap;

pub fn parse_graph(value: &str) -> (DiGraph<String, ()>, HashMap<String, NodeIndex>) {
    let mut graph = DiGraph::new();

    let mut lookup = HashMap::new();

    for line in value.lines().map(str::trim).filter(|&s| !s.is_empty()) {
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap().replace(":", "");

        let node = graph.add_node(name.clone());

        lookup.insert(name.clone(), node);
    }

    lookup.insert("out".into(), graph.add_node("out".into()));

    for line in value.lines().map(str::trim).filter(|&s| !s.is_empty()) {
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap().replace(":", "");

        let src = lookup[&name];

        for dest in iter {
            let dst = lookup[dest];
            graph.add_edge(src, dst, ());
        }
    }

    (graph, lookup)
}

pub fn count_paths_dag(g: &DiGraph<String, ()>, start: NodeIndex, end: NodeIndex) -> u64 {
    let mut topo = Topo::new(&g);
    let mut order = Vec::new();
    while let Some(n) = topo.next(&g) {
        order.push(n);
    }

    let mut dp = vec![0u64; g.node_count()];
    dp[start.index()] = 1;

    for &u in &order {
        let ways = dp[u.index()];
        if ways > 0 {
            for v in g.neighbors(u) {
                dp[v.index()] += ways;
            }
        }
    }

    dp[end.index()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let (graph, lookup) = parse_graph(
            "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
",
        );

        assert_eq!(graph.node_count(), 11);
        assert_eq!(graph.node_count(), lookup.len());
        assert_eq!(graph.edge_count(), 17);
    }
}
