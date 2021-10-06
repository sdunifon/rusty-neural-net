#[derive(Clone, Copy)]
pub struct Node<'a, const E: usize> {
    bias: f64,
    value: Option<u8>,
    edges: [Edge<'a, E>; E],
}

impl<'a, const E: usize> Node<'a, E> {
    pub fn new() -> Node<'a, E> {
        Node {
            bias: 0.1,
            value: Option::None,
            edges: [Edge::<E>::new(); E],
        }
    }

    pub fn set(&mut self, value: u8) {
        self.value = Some(value);
    }
}

#[derive(Clone, Copy)]
pub struct Edge<'a, const NEXT_LAYER_SIZE: usize> {
    weight: f64,
    node: Option<&'a Node<'a, NEXT_LAYER_SIZE>>,
}

impl<'a, const NEXT_LAYER_SIZE: usize> Edge<'a, NEXT_LAYER_SIZE> {
    pub fn new() -> Edge<'a, NEXT_LAYER_SIZE> {
        Edge {
            weight: 0.1,
            node: Option::None,
        }
    }
}
