use super::Node;
#[derive(Clone, Copy)]
pub struct Layer<'a, const THIS_LAYER_SIZE: usize, const NEXT_LAYER_SIZE: usize>(
    pub [Node<'a, NEXT_LAYER_SIZE>; THIS_LAYER_SIZE],
);

impl<'a, const THIS_LAYER_SIZE: usize, const NEXT_LAYER_SIZE: usize>
    Layer<'a, THIS_LAYER_SIZE, NEXT_LAYER_SIZE>
{
    pub fn new() -> Layer<'a, THIS_LAYER_SIZE, NEXT_LAYER_SIZE> {
        Layer([Node::<NEXT_LAYER_SIZE>::new(); THIS_LAYER_SIZE])
    }
}
