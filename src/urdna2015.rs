
use std::collections::HashMap;
use crate::nquads::{Dataset, QuadSet};

type Hash = String;

struct BlankNodeInfo {
  quads: QuadSet,
  hash: Hash
}

pub fn main(dataset: &Dataset)  {
  // 4.4) Normalization Algorithm

  // 1) Create the normalization state.

  // 2) For every quad in input dataset:
  for quad in &dataset.quads {
    println!("{:?}", quad);
  }

  println!("{:#?}", dataset);
}
