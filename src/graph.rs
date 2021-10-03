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
    node: &'a Node<'a, NEXT_LAYER_SIZE>,
}

#[derive(Clone, Copy)]
pub struct Row<T, const N: usize> {
    array: [T; N],
}
///

pub type Layer<'a, const THIS_LAYER_SIZE: usize, const NEXT_LAYER_SIZE: usize> = [Node<'a, NEXT_LAYER_SIZE>; THIS_LAYER_SIZE];

// #[derive(Clone, Copy)]
// pub struct NodeColumn<const N: usize> {
//     array: [T; N],
// }
#[derive(Clone, Copy)]
pub struct Graph<'a, const LAYER_COUNT: usize, const NEXT_LAYER_SIZE: usize> {
    layers: Layer<'a, LAYER_COUNT, NEXT_LAYER_SIZE>,
}

// impl<T, W, H> Graph {
pub impl<'a, const LAYER_COUNT: usize, const NEXT_LAYER_SIZE: usize>
    Graph<'a, LAYER_COUNT, NEXT_LAYER_SIZE>
{
    pub fn new(h: usize, w: usize) {}
}
///
impl <'a, const THIS_LAYER_SIZE: usize, const NEXT_LAYER_SIZE: usize> Layer<'a, THIS_LAYER_SIZE,NEXT_LAYER_SIZE>{ 
    pub fn new(x: T) -> Layer<'a, THIS_LAYER_SIZE,NEXT_LAYER_SIZE> {
        // let a: Column<T, N> = Column { array: [x; N] };
        Layer<NEXT_LAYER_SIZE>


        return a;
    }
}

impl<T: Copy, const N: usize> Row<T, N> {
    fn new(x: T) -> Row<T, N> {
        let a: Row<T, N> = Row { array: [x; N] };
        return a;
    }
}

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
