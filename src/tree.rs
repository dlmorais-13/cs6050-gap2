#[derive(Debug, Clone)]
pub struct Node {
  pub label: String,
  pub parent: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Tree {
  pub nodes: Vec<Node>,
  pub kr: Vec<usize>,
  pub lm: Vec<usize>,
}

impl Tree {
  pub fn new(mut nodes: Vec<Node>) -> Self {
    let mut children = Tree::compute_children(&nodes);
    Tree::sort_by_postorder(&mut nodes, &children);
    // Recompute children after reordering.
    children = Tree::compute_children(&nodes);
    let lm = Tree::compute_leftmost(&nodes, &children);
    let kr = Tree::compute_keyroots(&nodes, &lm);
    Self { nodes, kr, lm }
  }

  fn compute_children(nodes: &Vec<Node>) -> Vec<Vec<usize>> {
    let mut children: Vec<Vec<usize>> = vec![vec![]; nodes.len()];
    for (i, n) in nodes.iter().enumerate() {
      if let Some(parent) = n.parent {
        children[parent].push(i);
      }
    }
    children
  }

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
