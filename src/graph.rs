#![allow(unused)]

use std::{
    collections::HashSet,
    fmt,
    ops::{Index, IndexMut},
};

use indexmap::{IndexMap, IndexSet};

struct NodeIdGenerator {
    next_id: u32,
}

impl NodeIdGenerator {
    fn new() -> Self {
        Self { next_id: 0 }
    }

    fn next_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        NodeId(id)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NodeId(u32);

impl From<&Node> for NodeId {
    fn from(node: &Node) -> Self {
        node.id
    }
}

#[derive(Debug)]
pub struct Node {
    id: NodeId,
    neighbors: IndexSet<NodeId>,
}

impl Node {
    fn new(id: NodeId) -> Node {
        Self {
            id,
            neighbors: IndexSet::new(),
        }
    }

    pub fn neighbors(&self) -> &IndexSet<NodeId> {
        &self.neighbors
    }
}

pub struct Graph {
    id_generator: NodeIdGenerator,
    nodes: GraphAttribute<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            id_generator: NodeIdGenerator::new(),
            nodes: GraphAttribute::new(),
        }
    }

    pub fn get_node(&self, node_id: NodeId) -> &Node {
        &self.nodes[node_id]
    }

    pub fn add_node(&mut self) -> NodeId {
        let new_node = Node::new(self.id_generator.next_id());
        let new_node_id = new_node.id;
        self.nodes.insert(new_node_id, new_node);
        new_node_id
    }

    pub fn add_edge(&mut self, start: NodeId, end: NodeId) {
        self.nodes[start].neighbors.insert(end);
        self.nodes[end].neighbors.insert(start);
    }

    pub fn get_node_attribute(&self) -> &GraphAttribute<Node> {
        &self.nodes
    }
}

pub struct GraphAttribute<T> {
    nodes: IndexMap<NodeId, T>,
}

impl<T> GraphAttribute<T> {
    pub fn new() -> Self {
        Self {
            nodes: IndexMap::new(),
        }
    }

    pub fn get(&self, node: impl Into<NodeId>) -> Option<&T> {
        self.nodes.get(&node.into())
    }

    pub fn get_mut(&mut self, node: impl Into<NodeId>) -> Option<&mut T> {
        self.nodes.get_mut(&node.into())
    }

    pub fn insert(&mut self, node: impl Into<NodeId>, attr: T) -> Option<T> {
        self.nodes.insert(node.into(), attr)
    }

    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &T)> + '_ {
        self.nodes.iter().map(|(id, val)| (*id, val))
    }
}

impl<T, I: Into<NodeId>> Index<I> for GraphAttribute<T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index.into()).expect("Node not in GraphAttribute.")
    }
}

impl<T, I: Into<NodeId>> IndexMut<I> for GraphAttribute<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(index.into())
            .expect("Node not in GraphAttribute.")
    }
}

type TypeErasedAttribute<'a> = dyn Fn(NodeId) -> Option<&'a dyn fmt::Debug> + 'a;
pub struct DumpDot<'a> {
    graph: &'a Graph,
    attrs: Vec<(&'a str, Box<TypeErasedAttribute<'a>>)>,
}

impl<'a> DumpDot<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            attrs: Vec::new(),
        }
    }

    pub fn with<A: fmt::Debug + 'a>(
        mut self,
        attr_name: &'a str,
        attr: &'a GraphAttribute<A>,
    ) -> Self {
        let erased_attr = |id: NodeId| attr.get(id).map(|a| a as &dyn fmt::Debug);

        self.attrs.push((attr_name, Box::new(erased_attr)));

        self
    }

    pub fn dump(&self, mut out: impl std::io::Write) -> anyhow::Result<()> {
        let mut node_dumped = HashSet::new();

        writeln!(out, "graph {{")?;

        for (node_id, node) in self.graph.nodes.iter() {
            let label = self.attrs.iter().flat_map(|(attr_name, attr)| {
                let result = format!("{:?}", attr(node_id)?);
                let result = result.escape_default();
                Some(format!("{}={}", attr_name, result))
            });

            let label = itertools::join(label, "\n");

            writeln!(out, "\t{}[label=\"{}\"];", node_id.0, label)?;

            for neighbor in node.neighbors() {
                if node_dumped.contains(neighbor) {
                    continue;
                }

                writeln!(out, "\t {} -- {};", node_id.0, neighbor.0)?;
            }

            node_dumped.insert(node_id);
        }

        writeln!(out, "}}")?;
        out.flush()?;
        Ok(())
    }
}
