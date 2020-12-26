#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PermutationElement<'a> {
  direction: bool,
  value: &'a str,
}

pub type PermutationElements<'a> = Vec<&'a mut PermutationElement<'a>>;

#[derive(Debug, PartialEq)]
pub struct Permuter<'a> {
  current: &'a mut PermutationElements<'a>,
  done: bool,
}

impl Permuter<'_> {
  /**
   * A Permuter iterates over all possible permutations of the given array
   * of elements.
   *
   * @param list the array of elements to iterate over.
   */
  pub fn new<'a>(current: &'a mut PermutationElements<'a>) -> Permuter<'a> {
    // indicates whether there are more permutations
    let done = false;
    Permuter { current, done }
  }

  pub fn elements<'a>(list: &mut Vec<&'a str>) -> Vec<PermutationElement<'a>> {
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
      // swap k and the element it is looking at
      let swap = if k.unwrap().direction {
        pos - 1
      } else {
        pos + 1
      };
      current.swap(pos, swap);

      // reverse the direction of all elements larger than k
      for permutator_elment in current.iter_mut() {
        let element = &permutator_elment.value;
        if element > &k.unwrap().value {
          permutator_elment.direction = !permutator_elment.direction;
        }
      }
    }

    Some(rval)
  }
}
