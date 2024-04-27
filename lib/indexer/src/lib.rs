use graph::Node;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub enum FileType {
  File,
  Directory,
}

#[derive(Debug, Clone)]
pub struct File {}
