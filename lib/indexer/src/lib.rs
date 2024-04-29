use graph::Node;
use hypergraph::{HyperedgeIndex, Hypergraph, VertexIndex};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub enum FileType {
  File,
  Directory,
}

#[derive(Debug, Clone)]
pub struct File {}
