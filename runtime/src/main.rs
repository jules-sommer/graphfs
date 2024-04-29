use hypergraph::{HyperedgeIndex, Hypergraph, VertexIndex};
use jwalk::WalkDir;
use lazy_static::lazy_static;
use parse_display::Display as ParseDisplay;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{
  algo::{dijkstra, min_spanning_tree},
  prelude::DiGraph,
};
use smallstr::SmallString as SmolStr;
use smallvec::SmallVec;
use std::fmt::{Display, Formatter, Result};
use strum::Display;

use petgraph::{data::FromElements, visit::NodeRef};
use rayon::vec;
use regex::Regex;
use std::env::args;
use std::os;
use tracing::{info, span, Level};
use tracing_subscriber::{prelude::*, Registry};
use tracing_tree::HierarchicalLayer;

#[ctor::ctor]
fn init_tracing() {
  let layer = HierarchicalLayer::default()
    .with_writer(std::io::stdout)
    .with_indent_lines(true)
    .with_indent_amount(2)
    .with_ansi(true)
    .with_bracketed_fields(true)
    .with_thread_names(false)
    .with_thread_ids(false)
    .with_targets(true)
    .with_wraparound(5);

  let subscriber = Registry::default().with(layer);
  if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
    println!("Failed to set tracing subscriber: {e}");
  }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Display)]
enum NodeType {
  File,
  Directory,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
// #[display("[{id}]:{name}")]
struct Node {
  id: usize,
  name: SmolStr<[u8; 64]>,
  _type: NodeType,
}

impl Display for Node {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.name)
  }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
// #[display("[{id}]:{name}")]
struct Relation<'a> {
  id: usize,
  cost: usize,
  access_frequency: usize,
  name: &'a SmolStr<[u8; 64]>,
}

#[tokio::main]
async fn main() {
  // Create an undirected graph with `i32` nodes and edges
  // with `()` associated data.
  let mut g = DiGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

  // let mut graph = hypergraph::Hypergraph::<Node,
  // Relation>::new();

  let mut nodes: Vec<NodeIndex> = vec![];

  nodes.push(g.add_node(9));
  nodes.push(g.add_node(20));
  nodes.push(g.add_node(30));

  for node in nodes {
    let index = node.index();
    let id = node.id();
    let weight = g.node_weight(id).unwrap();
    info!(id = ?id, index = ?index, weight = ?weight);
  }

  // Find the shortest path from `1` to `4` using `1` as the
  // cost for every edge.
  let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
  assert_eq!(&1i32, node_map.get(&NodeIndex::new(4)).unwrap());

  // Get the minimum spanning tree of the graph as a new
  // graph, and check that one edge was trimmed.
  let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));
  assert_eq!(g.raw_edges().len() - 1, mst.raw_edges().len());

  // Output the tree to `graphviz` `DOT` format
  println!("{:?}", Dot::with_config(&mst, &[Config::EdgeNoLabel]));
}
