#![feature(generic_const_exprs)]
#![feature(test)]
use network::Network;
use std::array;

extern crate image;

mod graph;
mod input;
mod network;
mod new_main;
// mod pet_graph;

use input::main as input_main;

// use pet_graph::pet_graph_main;

static count: u32 = 0;
use graph::*;

fn main() {
    // let a = Network::new([1, 2, 3].to_vec());
    // println!("Hello, world!");
    // dbg!(sigmoid(&[1.0, 2.0, 3.0]));
    //
    let add_to = |x: i32| move |y: i32| x + y;

    let fiver = add_to(5);

    println!("={}", fiver(7));

    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    let graph = Graph::<4, 4>::new();
    let l = Layer::<8, 8>::new();
    input_main();
    // let layers = ["asdf"; 4];
}

fn sigmoid<const N: usize>(array: &[f64; N]) -> Vec<f64> {
    return array
        .clone()
        .into_iter()
        .map(|x| return x.exp())
        .collect::<Vec<f64>>();
}

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
}
