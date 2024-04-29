mod id;

use std::{
  collections::HashMap,
  fmt::Debug,
  hash::Hash,
  sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
  },
};

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use strum::Display;
use tracing::trace;

use crate::id::ID;

pub trait Relation<I>
where
  I: Clone + Sync + Send + Hash + Eq + Debug,
{
  type Data;

  fn connect(&mut self, src: I, dest: I, data: Self::Data);
  fn disconnect(&mut self, src: I, dest: I);
}

trait Generator<T> {
  fn next(&mut self) -> T;
}

impl<T> Generator<T> for Box<dyn FnMut() -> T> {
  fn next(&mut self) -> T {
    (self)()
  }
}

pub struct Graph<I, N, E>
where
  I: Clone + Sync + Send + Hash + Eq + Debug,
  N: Clone + Send + Sync + Debug,
  E: Clone + Send + Sync + Debug,
{
  // TODO: Does node need to take an index/id?
  pub nodes: HashMap<I, Node<I, N>>,
  pub adjacency_list: HashMap<I, Vec<Edge<I, E>>>,
  // TODO: This is meant to be a curried HOF generator, is the type correct?
  pub id_generator: Box<dyn FnMut() -> I>,
  pub root: Option<I>,
}

impl<I, N, E> Relation<I> for Graph<I, N, E>
where
  I: Clone + Sync + Send + Hash + Eq + Debug,
  N: Clone + Send + Sync + Debug,
  E: Clone + Send + Sync + Debug,
{
  type Data = E;

  fn connect(&mut self, src: I, dest: I, data: Self::Data) {
    trace!(src = ?src, dest = ?dest, data = ?data);

    if self.has_edge(&src, &dest) {
      // Handle the case where nodes might not exist yet
      return;
    }
  }

  fn disconnect(&mut self, src: I, dest: I) {
    trace!(src = ?src, dest = ?dest);
  }
}

impl<I, N, E> Graph<I, N, E>
where
  I: Clone + Sync + Send + Hash + Eq + Debug,
  N: Clone + Send + Sync + Debug,
  E: Clone + Send + Sync + Debug,
{
  fn next_id(&mut self) -> I {
    self.id_generator.next()
  }

  pub fn add_node(&mut self, data: N) {
    let id = self.next_id();
    self.nodes.insert(
      id.clone(),
      Node {
        id: id.clone(),
        data,
      },
    );
    self
      .adjacency_list
      // TODO: Measure the performance of this clone
      .entry(id)
      .or_insert_with(Vec::new);
  }

  pub fn add_edge(&mut self, src: I, dest: I, data: E) {
    let id = self.next_id();
    let edge = Edge::<I, E>::new(
      id.clone(),
      src.clone(),
      dest,
      Relationship::Directed,
      data,
    );
    self
      .adjacency_list
      .entry(src)
      .or_insert_with(Vec::new)
      .push(edge);
  }

  pub fn has_node(&self, id: &I) -> bool {
    self.nodes.contains_key(id)
  }

  pub fn has_edge(&self, src: &I, dest: &I) -> bool {
    if let Some(edges) = self.adjacency_list.get(src) {
      edges.iter().any(|e| &e.dest == dest)
    } else {
      false
    }
  }

  pub fn remove_node(&mut self, id: &I) {
    self.nodes.remove(id);
    self.adjacency_list.remove(id);
    for edges in self.adjacency_list.values_mut() {
      edges.retain(|e| &e.dest != id);
    }
  }
}

#[derive(
  Clone,
  Debug,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Display,
  Serialize,
  Deserialize,
  strum_macros::EnumIs,
)]
pub enum Relationship {
  /// In a directed graph, each edge has a direction,
  /// pointing from one node (the source) to another node
  /// (the destination). Therefore, the edge can be
  /// represented as an ordered pair of vertices.
  Directed,
  /// In an undirected graph, edges are bidirectional,
  /// meaning they don't have an inherent direction.
  /// Therefore, the edge can be represented as an
  /// undirected pair of vertices.
  Undirected,
}

/// A node in a graph.
///
/// `id` is the unique identifier of the node.
/// `data` is the data associated with the node.
#[derive(Clone, Debug)]
pub struct Node<I, N>
where
  I: Clone + Sync + Send + Hash + Eq + Debug,
  N: Clone + Send + Sync + Debug,
{
  pub id: I,
  pub data: N,
}

impl<I, N> Node<I, N>
where
  I: Clone + Sync + Send + Hash + Eq + Debug,
  N: Clone + Send + Sync + Debug,
{
  pub fn new(id: impl Into<I>, data: N) -> Self {
    Self {
      id: id.into(),
      data,
    }
  }

  pub fn id(&self) -> &I {
    &self.id
  }
}

#[derive(Clone, Debug)]
pub struct Edge<I, E>
where
  I: Clone + Sync + Send + Hash + Eq + Debug,
  E: Clone + Send + Sync + Debug,
{
  pub id: I,
  // `src` is the ID of the source node from which the edge originates.
  pub src: I,
  // `dest` is the ID of the destination node to which the edge connects
  // creating a relationship between the two nodes.
  pub dest: I,
  pub relation: Relationship,
  // `data` is the data associated with the edge. It can be used to store
  // additional information about the edge. For example, if the graph is
  // representing a file system, the data could be associated tags.
  pub data: E,
}

impl<I, E> Edge<I, E>
where
  I: Clone + Sync + Send + Hash + Eq + Debug,
  E: Clone + Send + Sync + Debug,
{
  pub fn new(
    id: impl Into<I>,
    src: impl Into<I>,
    dest: impl Into<I>,
    relation: Relationship,
    data: E,
  ) -> Self {
    let (id, src, dest) = (id.into(), src.into(), dest.into());
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
  pub fn vertices(&self) -> (&I, &I, Relationship) {
    (self.src(), self.dest(), self.relation)
  }

  /// Returns the ID of the source node from which the edge
  /// originates.
  pub fn src(&self) -> &I {
    todo!(
      "I doesn't implement anything that ensures we can actually use as index?"
    )
  }

  /// Returns the ID of the destination node to which the
  /// edge connects creating a relationship between the
  /// two nodes.
  pub fn dest(&self) -> &I {
    todo!()
  }

  /// Returns the data associated with the edge.
  pub fn data(&self) -> &E {
    &self.data
  }
}

#[cfg(test)]
mod tests {
  use tracing::debug;

  use super::*;
  use crate::id::ID;
  use std::{collections::HashMap, usize};

  use super::*;
  use std::sync::Mutex;

  fn generate_id<I>() -> impl FnMut() -> usize {
    Box::new({
      static ID: Arc<AtomicUsize> = Lazy::once(|| Arc::new(AtomicUsize::new(0)));
      move || {
        id += 1;
        ID.fetch_add(1, Ordering::Relaxed).into()
      }
    })
  }

  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Person {
    name: String,
  }

  #[test]
  fn test_id_wrapper() {
    let id = ID::from(0_usize);
    let usize_id = id.get();
    let into: usize = id.into();

    // check if ID is convertible to usize
    // and that .get() returns the same usize
    // as .into().
    assert_eq!(usize_id, into);
    assert_eq!(id.get(), into);
    assert_eq!(id.get(), 0);
  }

  #[test]
  fn test_id_generator() {
    let mut generator = generate_id::<usize>();
    for i in 0..200 {
      let id: usize = generator();
      debug!(i = ?i, id = ?id, "generating ID#{i}...");
      assert_eq!(id, i);
    }
  }

  #[test]
  fn test_edge() {
    let person = Person {
      name: "Jane Doe".into(),
    };

    let edge =
      Edge::<usize, Person>::new(0_usize, 1_usize, 2_usize, Relationship::Directed, person.clone());
    assert_eq!(edge.src(), &1);
    assert_eq!(edge.dest(), &2);
    assert_eq!(edge.data(), &person);
  }
}
