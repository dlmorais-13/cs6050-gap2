/// Structure that represents each node in a tree.
#[derive(Debug, Clone)]
pub struct Node {
  /// Label of the node.
  pub label: String,
  /// Index of node's parent. None if the node is the root of the tree.
  pub parent: Option<usize>,
}

/// Structure that represents a tree.
#[derive(Debug, Clone)]
pub struct Tree {
  /// List of the nodes in the tree.
  pub nodes: Vec<Node>,
  /// List of the keyroots of the tree.
  pub kr: Vec<usize>,
  /// List with the leftmost node for each node in the tree.
  /// The index must match the index in property "nodes".
  pub lm: Vec<usize>,
}

impl Tree {
  /// Function to build a new tree.
  /// 
  /// This function needs only the list of nodes and will
  /// calculate the other properties itself.
  /// 
  /// The list of nodes do not need to be in any particular order, but
  /// they will be stored in left-to-right postorder in the tree.
  pub fn new(mut nodes: Vec<Node>) -> Self {
    let mut children = Tree::compute_children(&nodes);
    Tree::sort_by_postorder(&mut nodes, &children);
    // Recompute children after reordering.
    children = Tree::compute_children(&nodes);
    let lm = Tree::compute_leftmost(&nodes, &children);
    let kr = Tree::compute_keyroots(&nodes, &lm);
    Self { nodes, kr, lm }
  }

  /// Compute an array with the children for each node, using the same
  /// index from the node list.
  fn compute_children(nodes: &Vec<Node>) -> Vec<Vec<usize>> {
    let mut children: Vec<Vec<usize>> = vec![vec![]; nodes.len()];
    for (i, n) in nodes.iter().enumerate() {
      if let Some(parent) = n.parent {
        children[parent].push(i);
      }
    }
    children
  }

  /// Compute the list with the leftmost node for each node, using the same 
  /// index from the node list.
  fn compute_leftmost(nodes: &Vec<Node>, children: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut leftmost: Vec<usize> = vec![];
    for (i, _n) in nodes.iter().enumerate() {
      if children[i].is_empty() {
        leftmost.push(i);
      } else {
        leftmost.push(leftmost[children[i][0]]);
      }
    }
    leftmost
  }

  /// Compute the keyroots of the tree.
  fn compute_keyroots(nodes: &Vec<Node>, leftmost: &Vec<usize>) -> Vec<usize> {
    let mut keyroots: Vec<usize> = vec![];
    for (i, n) in nodes.iter().enumerate() {
      if n.parent.is_none() {
        keyroots.push(i);
      } else if leftmost[i] != leftmost[n.parent.unwrap()] {
        keyroots.push(i);
      }
    }
    keyroots
  }

  /// Sort the nodes from input by left-to-right postorder.
  fn sort_by_postorder(nodes: &mut Vec<Node>, chidren: &Vec<Vec<usize>>) {
    if nodes.is_empty() {
      return;
    }

    // Get the old indices in postorder sequence
    let postorder_old_indices = Tree::postorder_indices(nodes, chidren);

    // Create an inverse mapping from old_index -> new_index
    let mut old_to_new = vec![0; nodes.len()];
    for (new_idx, &old_idx) in postorder_old_indices.iter().enumerate() {
      old_to_new[old_idx] = new_idx;
    }

    // Reconstruct the node vector in postorder and update parent pointers
    let mut new_nodes = Vec::with_capacity(nodes.len());
    for &old_idx in &postorder_old_indices {
      let mut node = nodes[old_idx].clone();
      node.parent = node.parent.map(|p| old_to_new[p]);
      new_nodes.push(node);
    }

    *nodes = new_nodes;
  }

  /// Walk through the nodes in left-to-right postorder and find the index for
  /// each node in this walk order.
  fn postorder_indices(nodes: &Vec<Node>, children: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = nodes.len();
    if n == 0 {
      return Vec::new();
    }

    let mut roots = Vec::new();
    for (idx, node) in nodes.iter().enumerate() {
      if node.parent.is_none() {
        roots.push(idx);
      }
    }

    let mut result = Vec::with_capacity(n);

    fn dfs(curr: usize, children: &[Vec<usize>], result: &mut Vec<usize>) {
      for &child in &children[curr] {
        dfs(child, children, result);
      }
      result.push(curr);
    }

    for root in roots {
      dfs(root, &children, &mut result);
    }

    result
  }
}
