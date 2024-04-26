use jwalk::WalkDir;
use lazy_static::lazy_static;
use regex::Regex;
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

#[tokio::main]
async fn main() {
  for entry in WalkDir::new(".") {
    let entry = entry.unwrap();
    info!(entry = ?entry)
  }
}
