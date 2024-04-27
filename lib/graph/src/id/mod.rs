use std::sync::{
  atomic::{AtomicUsize, Ordering},
  Arc,
};

use serde::{Deserialize, Serialize};

/// A function that generates unique IDs for nodes and
/// edges. The function is curried, returning a closure that
/// generates the next ID.
fn id_generator() -> impl FnMut() -> Arc<ID> {
  static ID: AtomicUsize = AtomicUsize::new(0);
  move || {
    let id = ID.fetch_add(1, Ordering::Relaxed) as usize;
    Arc::new(ID::from(id))
  }
}

#[derive(Clone, Debug)]
pub struct ID {
  id: Arc<AtomicUsize>,
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

  #[test]
  fn test_id_generator() {
    let mut next_id = id_generator();
    let id1 = next_id().get();
    let id2 = next_id().get();

    next_id(); // Ignore the next ID

    let id3 = next_id().get();

    assert_eq!(id1, 0);
    assert_eq!(id2, 1);
    assert_eq!(id3, 3);
  }
}
