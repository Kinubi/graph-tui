#[derive(Debug)]
pub struct Node {
    pub id: u64,
    pub label: String,
}
#[derive(Debug)]
pub struct Edge {
    pub from: u64,
    pub to: u64,
    pub label: String,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
