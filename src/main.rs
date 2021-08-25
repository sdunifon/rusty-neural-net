use std::array;

fn main() {
    println!("Hello, world!");
    dbg!(sigmoid(&[1.0, 2.0, 3.0]));
}

fn sigmoid<const N: usize>(array: &[f64; N]) -> Vec<f64> {
    return array
        .clone()
        .into_iter()
        .map(|x| return x.exp())
        .collect::<Vec<f64>>();
    // .into_iter()
    // .map(|x| return x)
    // .collect::<[f64; N]>();
    // .collect::<&[f64; N]>();
    // let exp_array: &[f64; N] = array.iter().map(|x| return x).collect::<&[f64; N]>();

    // return exp_array.into_iter().collect();
}

// fn sigmoid_vector<T: Iterator>(array: &T) -> Vec<T> {
//     let exp_array = array.into_iter().map(|x| return x);

//     return &exp_array.collect();
// }
#[cfg(test)]
mod tests {
    use std::future::pending;

    use super::*;

    #[test]
    fn sigmoid_test() {
        assert_eq!(
            sigmoid(&[1.0, 2.0, 3.0]),
            &[2.718281828459045, 7.38905609893065, 20.085536923187668]
        )
    }
    // #[test]
    // fn sigmoid_vector_test() {
    //     pending();
    //     assert_eq!(
    //         sigmoid(vec!([1.0, 2.0, 3.0])),
    //         &[2.71828183, 7.3890561, 20.08553692]
    //     );
    // }
}
