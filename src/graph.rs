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
struct Row<T, const N: usize> {
    array: [T; N],
}
///
#[derive(Clone, Copy)]
struct Column<T, const N: usize> {
    array: [T; N],
}
pub struct Graph<T, const W: usize, const H: usize> {
    matrix: Column<Row<T, W>, H>,
}
// impl<T, W, H> Graph {
impl<T, const W: usize, const H: usize> Graph<T, W, H> {
    fn new(h: usize, w: usize) {}
}
///
impl<T: Copy, const N: usize> Column<T, N> {
    fn new(x: T) -> Column<T, N> {
        let a: Column<T, N> = Column { array: [x; N] };
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
