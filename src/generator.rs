use crate::tree::{Node, Tree};

/// Generate a random recursive tree of n nodes.
pub fn generate_random_tree(n: u64) -> Tree {
  let mut nodes = vec![Node {
    label: String::from("N0"),
    parent: None,
  }];
  for i in 1..n {
    nodes.push(Node {
      label: format!("N{}", i),
      parent: Some(rand::random_range(0..i).try_into().unwrap()),
    });
  }
  Tree::new(nodes)
}
