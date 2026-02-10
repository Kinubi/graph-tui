use crate::edge::Edge;
use crate::node_builder::NodeInstance;

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<NodeInstance>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: NodeInstance) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn get_node(&self, id: usize) -> Option<&NodeInstance> {
        self.nodes.iter().find(|node| node.id == id)
    }

    pub fn get_edge(&self, id: u64) -> Option<&Edge> {
        self.edges.iter().find(|edge| edge.id == id)
    }

    pub fn get_new_node_id(&self) -> usize {
        self.nodes
            .iter()
            .map(|node| node.id)
            .max()
            .unwrap_or(0) + 1
    }
}
