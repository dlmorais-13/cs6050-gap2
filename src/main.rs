use gap_2::{algo, paper_input};

/// Main executor for the Tree Edit Distance algorithm
/// that uses the example from the paper as input.
fn main() {
  println!("Tree Edit Distance - Zhang & Shasha");
  let (tree1, tree2) = paper_input::input();
  let dist = algo(&tree1, &tree2);
  print!("dist: {}", dist);
}
