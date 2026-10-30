#[cfg(test)]
mod tests {
  use gap_2::{
    algo,
    generator::generate_random_tree,
    paper_input,
    tree::{Node, Tree},
  };

  #[test]
  fn test_empty_trees() {
    let t1 = Tree::new(vec![]);
    let t2 = Tree::new(vec![]);
    assert_eq!(algo(&t1, &t2), 0);
  }

  #[test]
  fn test_single_node_trees() {
    let t1 = Tree::new(vec![Node {
      label: String::from("a"),
      parent: None,
    }]);
    let t2 = Tree::new(vec![Node {
      label: String::from("b"),
      parent: None,
    }]);
    assert_eq!(algo(&t1, &t2), 1);
  }

  #[test]
  fn test_insert_change() {
    let t1 = Tree::new(vec![Node {
      label: String::from("a"),
      parent: None,
    }]);

    let t2 = Tree::new(vec![
      Node {
        label: String::from("c"),
        parent: Some(1),
      },
      Node {
        label: String::from("b"),
        parent: None,
      },
    ]);

    assert_eq!(algo(&t1, &t2), 2);
  }

  #[test]
  fn test_insert_change_wrong_order() {
    let t1 = Tree::new(vec![Node {
      label: String::from("a"),
      parent: None,
    }]);

    let t2 = Tree::new(vec![
      Node {
        label: String::from("b"),
        parent: None,
      },
      Node {
        label: String::from("c"),
        parent: Some(0),
      },
    ]);

    assert_eq!(algo(&t1, &t2), 2);
  }

  #[test]
  fn test_paper_input() {
    let (t1, t2) = paper_input::input();
    assert_eq!(algo(&t1, &t2), 2);
  }

  #[test]
  fn test_same_trees() {
    let (_, t) = paper_input::input();
    assert_eq!(algo(&t, &t), 0);
  }

  #[test]
  fn test_generator_input() {
    let t = generate_random_tree(10);
    assert_eq!(algo(&t, &t), 0);
  }
}
