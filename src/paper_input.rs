use crate::tree::Node;
use crate::tree::Tree;

/// Creates the trees using the example from the paper.
pub fn input() -> (Tree, Tree) {
  (tree1(), tree2())
}

fn tree1() -> Tree {
  Tree::new(vec![
    Node {
      label: String::from("a"),
      parent: Some(3),
    },
    Node {
      label: String::from("b"),
      parent: Some(2),
    },
    Node {
      label: String::from("c"),
      parent: Some(3),
    },
    Node {
      label: String::from("d"),
      parent: Some(5),
    },
    Node {
      label: String::from("e"),
      parent: Some(5),
    },
    Node {
      label: String::from("f"),
      parent: None,
    },
  ])
}

fn tree2() -> Tree {
  Tree::new(vec![
    Node {
      label: String::from("a"),
      parent: Some(2),
    },
    Node {
      label: String::from("b"),
      parent: Some(2),
    },
    Node {
      label: String::from("d"),
      parent: Some(3),
    },
    Node {
      label: String::from("c"),
      parent: Some(5),
    },
    Node {
      label: String::from("e"),
      parent: Some(5),
    },
    Node {
      label: String::from("f"),
      parent: None,
    },
  ])
}
