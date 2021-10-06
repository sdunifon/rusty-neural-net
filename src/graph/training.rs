use super::Graph;

// impl<'a, const LAYER_COUNT: usize, const LAYER_SIZE: usize> Graph<'a, LAYER_COUNT, LAYER_SIZE> {
//     fn train(&self, training_data: &dyn WithTrainingData<LAYER_SIZE>) {
//         todo!()
//     }
// }

// impl<'a, const LAYER_COUNT: usize, const LAYER_SIZE: usize> Graph<'a, LAYER_COUNT, LAYER_SIZE> {
//     fn traina(&mut self, training_input: &dyn WithTrainingData<LAYER_SIZE>) {
//         let layer = self.input_layer();
//         let training_data = training_input.to_training_data();

//         for i in 0..LAYER_SIZE {
//             layer.0[i].set(training_data[i]);
//         }
//     }
// }
pub trait WithTrainingData<const N: usize> {
    fn to_training_data(&self) -> [u8; N];
}

pub trait WithTrainingType<const N: usize> {
    type T = [u8; N];
    fn to_training_data(&self) -> Self::T;
}
