mod data_structures;
mod layer;
mod training;

use data_structures::*;
use layer::*;
use training::*;

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
    pub fn input_layer(&mut self) -> &'a mut Layer<LAYER_SIZE, LAYER_SIZE> {
        return self.layers.first_mut().unwrap();
    }
    fn train(&mut self, training_input: &dyn WithTrainingData<LAYER_SIZE>) {
        let layer = self.input_layer();
        let training_data = training_input.to_training_data();

        for i in 0..LAYER_SIZE {
            layer.0[i].set(training_data[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    ///
    #[test]
    fn graph_test() {}
}
