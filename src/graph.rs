fn main() {
    println!("Hello, world!");
}
//pub type NodeIndex = usize;
// pub type EdgeIndex = usize;
// pub type SensorValue = f64;
///
// pub struct NodeData {
//     first_outgoing_edge: Option<EdgeIndex>,
// }
///
// pub struct Graph {
//     nodes: Vec<NodeData>,
//     edged: Vec<EdgeData>,
// }
///
// pub struct EdgeData {
//     target: NodeIndex,
//     next_outgoing_edge: Option<EdgeIndex>,
// }
///
// pub struct Sensor {
//     name: Option<String>,
//     value: SensorValue,
// }
// pub struct Result {
//     id: usize,
//     // name: String,
// }
///
#[derive(Clone, Copy)]
pub struct Node<'a, const E: usize> {
    bias: f64,
    edges: [Edge<'a, E>; E],
}

#[derive(Clone, Copy)]
pub struct Edge<'a, const NEXT_LAYER_SIZE: usize> {
    weight: f64,
    node: Option<&'a Node<'a, NEXT_LAYER_SIZE>>,
}

#[derive(Clone, Copy)]
pub struct Layer<'a, const THIS_LAYER_SIZE: usize, const NEXT_LAYER_SIZE: usize>(
    [Node<'a, NEXT_LAYER_SIZE>; THIS_LAYER_SIZE],
);

// #[derive(Clone, Copy)]
// pub struct NodeColumn<const N: usize> {
//     array: [T; N],
// }
#[derive(Clone, Copy)]
pub struct Graph<'a, const LAYER_COUNT: usize, const LAYER_SIZE: usize> {
    layers: [Layer<'a, LAYER_SIZE, LAYER_SIZE>; LAYER_COUNT],
}

// impl<T, W, H> Graph {
impl<'a, const LAYER_COUNT: usize, const LAYER_SIZE: usize> Graph<'a, LAYER_COUNT, LAYER_SIZE> {
    pub fn new() -> Graph<'a, LAYER_COUNT, LAYER_SIZE> {
        // ) -> () {
        Graph {
            layers: [Layer::<'a, LAYER_SIZE, LAYER_SIZE>::new(); LAYER_COUNT],
        }
    }
}

impl<'a, const THIS_LAYER_SIZE: usize, const NEXT_LAYER_SIZE: usize>
    Layer<'a, THIS_LAYER_SIZE, NEXT_LAYER_SIZE>
{
    pub fn new() -> Layer<'a, THIS_LAYER_SIZE, NEXT_LAYER_SIZE> {
        Layer([Node::<NEXT_LAYER_SIZE>::new(); THIS_LAYER_SIZE])
    }
}

impl<'a, const E: usize> Node<'a, E> {
    pub fn new() -> Node<'a, E> {
        Node {
            bias: 0.1,
            edges: [Edge::<E>::new(); E],
        }
    }
}
impl<'a, const NEXT_LAYER_SIZE: usize> Edge<'a, NEXT_LAYER_SIZE> {
    pub fn new() -> Edge<'a, NEXT_LAYER_SIZE> {
        Edge {
            weight: 0.1,
            node: Option::None,
        }
    }
}
///
// impl <'a, const THIS_LAYER_SIZE: usize, const NEXT_LAYER_SIZE: usize> Layer<'a, THIS_LAYER_SIZE,NEXT_LAYER_SIZE>{
//     pub fn new(x: T) -> Layer<'a, THIS_LAYER_SIZE,NEXT_LAYER_SIZE> {
//         // let a: Column<T, N> = Column { array: [x; N] };
//         let a = Layer<NEXT_LAYER_SIZE>

//         return a;
//     }
// }

struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

#[cfg(test)]
mod tests {
    use super::*;
    ///
    #[test]
    fn graph_test() {}
}
