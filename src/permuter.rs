#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PermutationElement<'a> {
  direction: bool,
  value: &'a str,
}

pub type PermutationElements<'a> = Vec<PermutationElement<'a>>;
pub type PermutationElementRefs<'a> = Vec<&'a mut PermutationElement<'a>>;

#[derive(Debug, PartialEq)]
pub struct Permuter<'a> {
  current: &'a mut PermutationElementRefs<'a>,
  done: bool,
}

impl Permuter<'_> {
  /**
   * A Permuter iterates over all possible permutations of the given array
   * of elements.
   *
   */
  pub fn new<'a>(current: &'a mut PermutationElementRefs<'a>) -> Permuter<'a> {
    // indicates whether there are more permutations
    let done = false;
    Permuter { current, done }
  }

  pub fn permutation_elements<'a>(list: &mut Vec<&'a str>) -> PermutationElements<'a> {
    // original array
    list.sort_unstable();

    let mut elements = Vec::with_capacity(list.len());
    for value in list.iter() {
      elements.push(PermutationElement {
        direction: true,
        value,
      })
    }

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
    let rval = current.iter().map(|x| x.value.to_string()).collect();
    /* Calculate the next permutation using the Steinhaus-Johnson-Trotter
    permutation algorithm. */

    // get largest mobile element k
    // (mobile: element is greater than the one it is looking at)
    let mut k: Option<PermutationElement> = None;
    let mut k_is_none = true;
    let mut pos = 0;
    let length = current.len();
    for (i, permutator_elment) in current.iter().enumerate() {
      let element = permutator_elment.value;
      let left = permutator_elment.direction;
      k_is_none = k.is_none();
      if (k_is_none || element > k.unwrap().value)
        && ((left && i > 0 && element > current[i - 1].value)
          || (!left && i < (length - 1) && element > current[i + 1].value))
      {
        k = Some(*current[i]);
        pos = i;
      }
    }

    // no more permutations
    if k_is_none {
      self.done = true;
    } else {
      let k = k.unwrap();
      // swap k and the element it is looking at
      let swap = if k.direction { pos - 1 } else { pos + 1 };
      current.swap(pos, swap);

      // reverse the direction of all elements larger than k
      for permutator_element in current.iter_mut() {
        if permutator_element.value > k.value {
          permutator_element.direction = !permutator_element.direction;
        }
      }
    }

    Some(rval)
  }
}
