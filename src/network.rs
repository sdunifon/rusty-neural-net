use rand::random;

pub struct Network {
    num_layers: usize,
    layer_sizes: Vec<usize>,
}

impl Network {
    pub fn new(layer_sizes: Vec<usize>) -> Network {
        let num_layers = layer_sizes.len();
        // const biases: Vec<Vec<f64>> = layer_sizes
        //     .into_iter()
        //     .map(|layer_size| {
        //         const a: [f64; layer_size] = rand::random();
        //         a
        //     })
        //     .collect();
        Network {
            num_layers,
            layer_sizes,
        }
    }

    pub fn random_array(size: usize) -> Vec<f64> {
        let mut v: Vec<f64> = Vec::with_capacity(size);
        for x in 0..size {
            v[x] = random();
        }
        dbg!(&v);
        v.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_random_array() {
    //     let v2 = Network::random_array(5);
    //     dbg!("asdf{}", v2);
    //     // assert_eq!(Network::random_array(5)[..], vec!([1, 2, 3]))
    // }
}
