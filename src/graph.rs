pub type NodeIndex = usize;
pub type EdgeIndex = usize;
pub type SensorValue = f64;

pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

pub struct Graph {
    nodes: Vec<NodeData>,
    edged: Vec<EdgeData>,
}

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

pub struct Sensor {
    name: Option<String>,
    value: SensorValue,
}
pub struct Result {
    id: usize,
    name: String,
}
