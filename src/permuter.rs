#[derive(Copy, Clone, Debug, PartialEq)]
struct PermutationElement<'a> {
  direction: bool,
  value: &'a str,
  empty: bool,
}

type PermutationElements<'a> = Vec<PermutationElement<'a>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Permuter<'a> {
  current: PermutationElements<'a>,
  done: bool,
}

impl Permuter<'_> {
  /**
   * A Permuter iterates over all possible permutations of the given array
   * of elements.
   *
   * @param list the array of elements to iterate over.
   */
  pub fn new(list: &mut Vec<String>) -> Permuter {
    let list_clone = &mut list[..];
    // original array
    list_clone.sort();
    // indicates whether there are more permutations
    let done = false;
    let mut current = vec![];
    for value in list_clone.iter() {
      current.push(PermutationElement {
        direction: true,
        value,
        empty: false,
      })
    }
    Permuter { current, done }
  }
}

impl Iterator for Permuter<'_> {
  type Item = Vec<String>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      return None;
    }
    // copy current permutation to return it
    let current = &mut self.current;
    let rval = current.iter().map(|x| x.value.to_string()).collect();
    /* Calculate the next permutation using the Steinhaus-Johnson-Trotter
    permutation algorithm. */

    // get largest mobile element k
    // (mobile: element is greater than the one it is looking at)
    let mut k = PermutationElement {
      direction: true,
      value: "",
      empty: true,
    };
    let mut pos = 0;
    let length = current.len();
    for (i, permutator_elment) in current.iter().enumerate() {
      let left = permutator_elment.direction;
      let element = permutator_elment.value;
      if (k.empty || element > k.value)
        && ((left && i > 0 && element > current[i - 1].value)
          || (!left && i < (length - 1) && element > current[i + 1].value))
      {
        k = current[i];
        pos = i;
      }
    }

    // no more permutations
    if k.empty {
      self.done = true;
    } else {
      // swap k and the element it is looking at
      let swap = if current[pos].direction {
        pos - 1
      } else {
        pos + 1
      };
      current[pos] = current[swap];
      current[swap] = k;

      // reverse the direction of all elements larger than k
      for permutator_elment in current.iter_mut() {
        let element = &permutator_elment.value;
        if element > &k.value {
          permutator_elment.direction = !permutator_elment.direction;
        }
      }
    }

    Some(rval)
  }
}
