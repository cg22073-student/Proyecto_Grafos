use petgraph::dot::Dot;
use petgraph::graph::UnGraph;

fn main() {
    let mut g = UnGraph::<String, u32>::new_undirected();

    let ber = g.add_node("Berlin".to_owned());
    let del = g.add_node("New Delhi".to_owned());
    let mex = g.add_node("Mexico City".to_owned());
    let syd = g.add_node("Sydney".to_owned());

    g.extend_with_edges(&[
        (ber, del, 6_000),
        (ber, mex, 10_000),
        (ber, syd, 16_000),
        (del, mex, 14_000),
        (del, syd, 12_000),
        (mex, syd, 15_000),
    ]);

    let dot = format!("{:?}", Dot::new(&g));

    println!("{}", dot);

    std::fs::write("flight_network.dot", dot).unwrap();
}