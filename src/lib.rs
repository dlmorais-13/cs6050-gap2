use std::collections::HashMap;

use crate::tree::Tree;

pub mod generator;
pub mod paper_input;
pub mod tree;

pub fn algo(t1: &Tree, t2: &Tree) -> usize {
  let mut treedist: HashMap<String, usize> = HashMap::new();
  for i in 0..t1.kr.len() {
    for j in 0..t2.kr.len() {
      calc_treedist(t1, t2, t1.kr[i], t2.kr[j], &mut treedist);
    }
  }

  // Print final matrix.
  // println!("{:#?}", treedist);

  // let mut sorted_treedist: Vec<_> = treedist.iter().collect();
  // sorted_treedist.sort_by_key(|a| a.0);
  // for (key, value) in sorted_treedist {
  //     println!("{}: {}", key, value);
  // }

  // for i in 0..t1.nodes.len() {
  //     for j in 0..t2.nodes.len() {
  //         print!("{:?} ", treedist.get(&format!("{}-{}", i, j)).unwrap());
  //     }
  //     println!("");
  // }

  if treedist.len() > 0 {
    let lrk = format!("{}-{}", t1.nodes.len() - 1, t2.nodes.len() - 1);
    treedist.get(&lrk).unwrap().clone()
  } else {
    0
  }
}

fn calc_treedist(t1: &Tree, t2: &Tree, i: usize, j: usize, treedist: &mut HashMap<String, usize>) {
  let mut forestdist: HashMap<String, usize> = HashMap::new();
  // Use 1 and 0 to force empty representation.
  forestdist.insert(key((None, None), (None, None)), 0);

  for i1 in t1.lm[i]..(i + 1) {
    let prev_key = key((Some(t1.lm[i]), prev_usize(i1)), (None, None));
    forestdist.insert(
      key((Some(t1.lm[i]), Some(i1)), (None, None)),
      forestdist.get(&prev_key).unwrap() + 1,
    );
  }

  for j1 in t2.lm[j]..(j + 1) {
    let prev_key = key((None, None), (Some(t2.lm[j]), prev_usize(j1)));
    forestdist.insert(
      key((None, None), (Some(t2.lm[j]), Some(j1))),
      forestdist.get(&prev_key).unwrap() + 1,
    );
  }

  for i1 in t1.lm[i]..(i + 1) {
    for j1 in t2.lm[j]..(j + 1) {
      let kim1 = key((Some(t1.lm[i]), prev_usize(i1)), (Some(t2.lm[j]), Some(j1)));
      let vim1 = forestdist.get(&kim1).unwrap() + 1;
      let kjm1 = key((Some(t1.lm[i]), Some(i1)), (Some(t2.lm[j]), prev_usize(j1)));
      let vjm1 = forestdist.get(&kjm1).unwrap() + 1;

      let k = key((Some(t1.lm[i]), Some(i1)), (Some(t2.lm[j]), Some(j1)));
      let tdk = format!("{}-{}", i1, j1);

      if t1.lm[i1] == t1.lm[i] && t2.lm[j1] == t2.lm[j] {
        let kbm1 = key((Some(t1.lm[i]), prev_usize(i1)), (Some(t2.lm[j]), prev_usize(j1)));

        let cost = if t1.nodes[i1].label.ne(&t2.nodes[j1].label) {
          1
        } else {
          0
        };
        let vbm1 = forestdist.get(&kbm1).unwrap() + cost;

        forestdist.insert(k.clone(), vec![vim1, vjm1, vbm1].iter().copied().min().unwrap());
        treedist.insert(tdk.clone(), forestdist.get(&k).unwrap().clone());
      } else {
        let kbm1 = key(
          (Some(t1.lm[i]), prev_usize(t1.lm[i1])),
          (Some(t2.lm[j]), prev_usize(t2.lm[j1])),
        );
        let vbm1 = forestdist.get(&kbm1).unwrap() + treedist.get(&tdk).unwrap();

        forestdist.insert(k.clone(), vec![vim1, vjm1, vbm1].iter().copied().min().unwrap());
      }
    }
  }
}

fn prev_usize(v: usize) -> Option<usize> {
  if v > 0 { Some(v - 1) } else { None }
}

fn key(t1_range: (Option<usize>, Option<usize>), t2_range: (Option<usize>, Option<usize>)) -> String {
  let k1 = if t1_range.0.is_none() || t1_range.1.is_none() {
    String::from("∅")
  } else if t1_range.0.unwrap() > t1_range.1.unwrap() {
    String::from("∅")
  } else {
    format!("{}-{}", t1_range.0.unwrap(), t1_range.1.unwrap())
  };

  let k2 = if t2_range.0.is_none() || t2_range.1.is_none() {
    String::from("∅")
  } else if t2_range.0.unwrap() > t2_range.1.unwrap() {
    String::from("∅")
  } else {
    format!("{}-{}", t2_range.0.unwrap(), t2_range.1.unwrap())
  };

  format!("t1: {} / t2: {}", k1, k2)
}
