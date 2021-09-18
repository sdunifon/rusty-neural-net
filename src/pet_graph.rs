use petgraph::graph::{NodeIndex, UnGraph};

pub fn pet_graph_main() {
    let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);
    println!("pet_graph_main{:?}", g);
}
