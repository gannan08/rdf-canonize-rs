use std::collections::HashMap;

type DirectionMap<'a> = HashMap<&'a str, bool>;

#[derive(Debug, PartialEq)]
pub struct Permuter<'a> {
  current: &'a mut Vec<&'a str>,
  done: bool,
  dir: DirectionMap<'a>,
}

impl Permuter<'_> {
  /**
   * A Permuter iterates over all possible permutations of the given array
   * of elements.
   *
   * @param list the array of elements to iterate over.
   */
  pub fn new<'a>(current: &'a mut Vec<&'a str>) -> Permuter<'a> {
    // indicates whether there are more permutations
    let done = false;
    // directional info for permutation algorithm
    let mut dir = DirectionMap::new();
    for v in current.iter() {
      dir.insert(v, true);
    }
    Permuter { current, done, dir }
  }

  pub fn elements<'a>(list: &mut [&'a str]) -> Vec<&'a str> {
    // original array
    list.sort_unstable();

    let mut elements = Vec::with_capacity(list.len());
    elements.extend_from_slice(list);

    elements
  }
}

impl Iterator for Permuter<'_> {
  type Item = Vec<String>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      return None;
    }
    // copy current permutation to return it
    let current = &mut self.current[..];
    let dir = &mut self.dir;
    let rval = current.iter().map(|x| x.to_string()).collect();

    /* Calculate the next permutation using the Steinhaus-Johnson-Trotter
    permutation algorithm. */

    // get largest mobile element k
    // (mobile: element is greater than the one it is looking at)
    let mut k: Option<&str> = None;
    let mut k_is_none = true;
    let mut pos = 0;
    let length = current.len();
    for (i, element) in current.iter().enumerate() {
      let left = dir.get(element).unwrap();
      k_is_none = k.is_none();
      if (k_is_none || *element > k.unwrap())
        && ((*left && i > 0 && element > &current[i - 1])
          || (!left && i < (length - 1) && element > &current[i + 1]))
      {
        k = Some(element);
        pos = i;
      }
    }

    // no more permutations
    if k_is_none {
      self.done = true;
    } else {
      let k_val = &k.unwrap();
      // swap k and the element it is looking at
      let swap = if *dir.get(k_val).unwrap() {
        pos - 1
      } else {
        pos + 1
      };

      current.swap(pos, swap);

      // reverse the direction of all elements larger than k
      for element in current.iter() {
        if element > k_val {
          dir.insert(element, !dir.get(element).unwrap());
        }
      }
    }

    Some(rval)
  }
}
