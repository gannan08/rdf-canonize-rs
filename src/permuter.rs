use std::collections::HashMap;

type DirectionMap = HashMap<String, bool>;

#[derive(Clone, Debug, PartialEq)]
pub struct Permuter {
  current: Vec<String>,
  done: bool,
  dir: DirectionMap,
}

impl Permuter {
  /**
   * A Permuter iterates over all possible permutations of the given array
   * of elements.
   *
   * @param list the array of elements to iterate over.
   */
  pub fn new(list: &mut Vec<String>) -> Permuter {
    let current = &mut list[..];
    // original array
    current.sort();
    // indicates whether there are more permutations
    let done = false;
    // directional info for permutation algorithm
    let mut dir = DirectionMap::new();
    for v in current.iter() {
      dir.insert(v.to_string(), true);
    }
    let mut vec = vec![];
    vec.extend_from_slice(current);
    Permuter {
      current: vec,
      done,
      dir,
    }
  }

  /**
   * Returns true if there is another permutation.
   *
   * @return true if there is another permutation, false if not.
   */
  pub fn has_next(&self) -> bool {
    !self.done
  }

  /**
   * Gets the next permutation. Call has_next() to ensure there is another one
   * first.
   *
   * @return the next permutation.
   */
  pub fn next(&mut self) -> Vec<String> {
    // copy current permutation to return it
    let current = &mut self.current;
    let dir = &mut self.dir;
    let rval = current.to_vec();
    println!("[Permuter] current: {:?}", current);
    println!("[Permuter] dir: {:?}", dir);
    println!("[Permuter] rval: {:?}", rval);
    /* Calculate the next permutation using the Steinhaus-Johnson-Trotter
    permutation algorithm. */

    // get largest mobile element k
    // (mobile: element is greater than the one it is looking at)
    let mut k: Option<String> = None;
    let mut k_val = String::from("");
    let mut k_is_none = true;
    let mut pos = 0;
    let length = current.len();
    for (i, element) in current.iter().enumerate() {
      let left = dir.get(element).unwrap();
      if let Some(tmp_k) = &k {
        k_is_none = false;
        k_val = tmp_k.to_string();
      } else {
        k_is_none = true;
        k_val = "".to_string();
      }
      if (k_is_none || element > &k_val)
        && ((*left && i > 0 && element > &current[i - 1])
          || (!left && i < (length - 1) && element > &current[i + 1]))
      {
        k = Some(element.to_string());
        pos = i;
      }
    }

    // no more permutations
    if k_is_none {
      self.done = true;
    } else {
      // swap k and the element it is looking at
      let swap = if *dir.get(&k_val).unwrap() {
        pos - 1
      } else {
        pos + 1
      };
      println!("[Permuter] current: {:?}", current);
      println!("[Permuter] pos: {:?}", pos);
      println!("[Permuter] swap: {:?}", swap);
      println!("[Permuter] current[pos]: {:?}", current[swap].to_string());
      println!("[Permuter] current[swap]: {:?}", k_val);
      let swap_val = &current[pos].clone();
      println!("[Permuter] test: {:?}", swap_val);
      current[pos] = current[swap].to_string();
      println!("[Permuter] test 2: {:?}", swap_val);
      current[swap] = swap_val.to_string();

      // reverse the direction of all elements larger than k
      for element in current.iter() {
        println!("[Permuter] element: {:?}", element);
        println!("[Permuter] k: {:?}", &mut k_val.to_string());
        println!(
          "[Permuter] element > k: {:?}",
          element > &mut k_val.to_string()
        );
        if element > &mut k_val.to_string() {
          println!("[Permuter] reverse element: {:?}", element);
          println!(
            "[Permuter] reverse dir: {:?}",
            !dir.get(&element.to_string()).unwrap()
          );
          dir.insert(
            element.to_string(),
            !dir.get(&element.to_string()).unwrap(),
          );
        }
      }
    }

    println!("[Permuter] return rval: {:?}", rval);
    rval
  }
}
