pub use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub fn parse_graph(value: &str) -> (DiGraph<String, ()>, NodeIndex, NodeIndex) {
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


    let origin = lookup["you"];
    let end = lookup["out"];

    (graph, origin, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let (graph, origin, end) = parse_graph(
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

        eprintln!("{graph:?}");
        eprintln!("{origin:?}");
        eprintln!("{end:?}");

        assert_eq!(graph.node_count(), 11);
        assert_eq!(graph.edge_count(), 17);
    }
}
