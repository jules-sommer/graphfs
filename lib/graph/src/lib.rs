mod id;

use std::{
  collections::HashMap,
  hash::Hash,
  sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
  },
};

use crate::id::ID;

pub trait Relation<I> {
  type Data;

  fn connect(&mut self, src: I, dest: I, data: Self::Data);
  fn disconnect(&mut self, src: I, dest: I);
}

pub struct Graph<I, N, E>
where
  N: Hash + Eq + Clone + Send + Sync,
  E: Hash + Eq + Clone + Send + Sync,
{
  nodes: HashMap<I, Node<N>>,
  adjacency_list: HashMap<I, Vec<Edge<E>>>,
  root: Option<I>,
}

// pub struct Graph<T: Hash + Eq + Clone> {
//   pub nodes: HashMap<T, Node<File>>,
//   pub adjacency_list: HashMap<T, Vec<(T, Relationship)>>,
// }

#[derive(Clone, Debug, Copy)]
pub enum Relationship {
  Directed,
  Undirected,
}

/// A node in a graph.
///
/// `id` is the unique identifier of the node.
/// `data` is the data associated with the node.
#[derive(Clone, Debug)]
pub struct Node<N: Clone + Send + Sync> {
  pub id: ID,
  pub data: N,
}

impl<N: Clone + Send + Sync> Node<N> {
  pub fn new(data: N, id: impl Into<usize>) -> Self {
    Self {
      id: ID::from(id),
      data,
    }
  }

  pub fn id(&self) -> usize {
    self.id.get()
  }
}

#[derive(Clone, Debug)]
pub struct Edge<N: Clone + Send + Sync> {
  pub id: ID,
  // `src` is the ID of the source node from which the edge originates.
  pub src: ID,
  // `dest` is the ID of the destination node to which the edge connects
  // creating a relationship between the two nodes.
  pub dest: ID,
  pub relation: Relationship,
  // `data` is the data associated with the edge. It can be used to store
  // additional information about the edge. For example, if the graph is
  // representing a file system, the data could be associated tags.
  pub data: N,
}

impl<N: Clone + Send + Sync> Edge<N> {
  pub fn new(
    id: impl Into<usize>,
    src: ID,
    dest: ID,
    relation: Relationship,
    data: N,
  ) -> Self {
    let id = ID::from(id);
    Edge {
      id,
      src,
      dest,
      relation,
      data,
    }
  }

  /// Returns the ID's of the source and destination nodes
  /// of the edge, i.e., the vertices of the edge.
  pub fn vertices(&self) -> (usize, usize, Relationship) {
    (self.src(), self.dest(), self.relation)
  }

  /// Returns the ID of the source node from which the edge
  /// originates.
  pub fn src(&self) -> usize {
    self.src.get()
  }

  /// Returns the ID of the destination node to which the
  /// edge connects creating a relationship between the
  /// two nodes.
  pub fn dest(&self) -> usize {
    self.dest.get()
  }

  /// Returns the data associated with the edge.
  pub fn data(&self) -> &N {
    &self.data
  }
}
