use std::{hash::Hash, sync::{
  atomic::{AtomicUsize, Ordering},
  Arc,
}};

use serde::{Deserialize, Serialize};
use tracing::trace;

/// A function that generates unique IDs for nodes and
/// edges. The function is curried, returning a closure that
/// generates the next ID.
fn id_generator() -> impl FnMut() -> ID {
  static ID: AtomicUsize = AtomicUsize::new(0);
  move || {
    let id = ID.fetch_add(1, Ordering::Relaxed);
    ID::from(id)
  }
}

#[derive(Clone, Debug, Copy)]
pub struct ID {
  id: Arc<AtomicUsize>,
}

impl Hash for ID {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
      self.id.load(Ordering::Relaxed).hash(state);
  }
  fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
      where
          Self: Sized, {
      for id in data {
          id.hash(state);
      }
  }
}

impl PartialEq for ID {
  fn eq(&self, other: &Self) -> bool {
    self.id.load(Ordering::Relaxed) == other.id.load(Ordering::Relaxed)
  }
}

impl Eq for ID {}

impl From<usize> for ID {
  fn from(id: usize) -> Self {
    trace!(id = ?id, "converting usize to ID");
    Self {
      id: Arc::new(AtomicUsize::new(id)),
    }
  }
}

impl From<i32> for ID {
  fn from(id: i32) -> Self {
    trace!(id = ?id, "converting i32 to ID");
    Self {
      // TODO: should this be a cast? seems sus
      id: Arc::new(AtomicUsize::new(id as usize)),
    }
  }
}

impl Into<usize> for ID {
  fn into(self) -> usize {
    trace!(id = ?self, "converting ID to usize");
    self.id.load(Ordering::Relaxed)
  }
}

impl ID {
  pub fn from<T: Into<usize>>(id: T) -> Self {
    Self {
      id: Arc::new(AtomicUsize::new(id.into())),
    }
  }

  pub fn get(&self) -> usize {
    self.id.load(Ordering::Relaxed)
  }

  pub fn set(&self, value: usize) {
    self.id.store(value, Ordering::Relaxed);
  }
}

impl Serialize for ID {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let value = self.id.load(Ordering::Relaxed);
    value.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for ID {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let value = usize::deserialize(deserializer)?;
    Ok(Self::from(value))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tracing::debug;
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

  #[test]
  fn test_id_generator() {
    let mut next_id = id_generator();
    let id1 = next_id().get();
    let id2 = next_id().get();

    next_id(); // Ignore the next ID

    let id3 = next_id().get();

    debug!(id1 = ?id1, id2 = ?id2, id3 = ?id3);

    assert_eq!(id1, 0);
    assert_eq!(id2, 1);
    assert_eq!(id3, 3);
  }
}
