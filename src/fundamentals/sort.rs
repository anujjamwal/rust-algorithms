pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
  for i in 0..v.len() {
    for j in 0..v.len() - 1 - i {
      if v[j] > v[j + 1] {
        v.swap(j, j + 1);
      }
    }
  }
}

pub fn merge_sort<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
  if v.len() <= 1 {
    return v;
  }

  let mut res: Vec<T> = Vec::with_capacity(v.len());
  let b = v.split_off(v.len() / 2);

  let mut a = merge_sort(v).into_iter();
  let mut b = merge_sort(b).into_iter();
  let mut a_peek = a.next();
  let mut b_peek = b.next();

  loop {
    match a_peek {
      Some(ref a_val) => match b_peek {
        Some(ref b_val) => {
          if a_val < b_val {
            res.push(a_peek.take().unwrap());
            a_peek = a.next();
          } else {
            res.push(b_peek.take().unwrap());
            b_peek = b.next();
          }
        }
        None => {
          res.push(a_peek.take().unwrap());
          res.extend(a);
          return res;
        }
      },
      None => {
        if let Some(b_val) = b_peek {
          res.push(b_val);
          res.extend(b);
          return res;
        }
      }
    }
  }
}

pub fn quick_sort<T: PartialOrd>(v: &mut [T]) {
  if v.len() == 0 {
    return;
  }

  let pivot = partition(v);
  quick_sort(&mut v[0..pivot]);
  quick_sort(&mut v[pivot + 1..]);
}

fn partition<T: PartialOrd>(v: &mut [T]) -> usize {
  let mut loc = 0;

  for i in 1..v.len() {
    if v[i] < v[0] {
      loc += 1;
      v.swap(i, loc);
    }
  }

  v.swap(0, loc);
  loc
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bubble_sort_on_empty_vec() {
    let mut input: Vec<i32> = vec![];
    bubble_sort(&mut input);

    assert_eq!(input, vec![]);
  }

  #[test]
  fn test_bubble_sort() {
    let mut input = vec![1, 10, 4, 5, 6];
    bubble_sort(&mut input);

    assert_eq!(input, vec![1, 4, 5, 6, 10]);
  }

  #[test]
  fn test_merge_sort_on_empty_vec() {
    let input: Vec<i32> = vec![];
    let res = merge_sort(input);

    assert_eq!(res, vec![]);
  }

  #[test]
  fn test_merge_sort() {
    let input = vec![1, 10, 4, 5, 6];
    let res = merge_sort(input);

    assert_eq!(res, vec![1, 4, 5, 6, 10]);
  }

  #[test]
  fn test_quick_sort_on_empty_vec() {
    let mut input: Vec<i32> = vec![];
    quick_sort(&mut input);

    assert_eq!(input, vec![]);
  }

  #[test]
  fn test_partition_to_move_smallest_element() {
    let mut input = vec![1, 10, 4, 5, 6, 19, 3];
    let res = partition(&mut input);
    assert_eq!(res, 0);
  }

  #[test]
  fn test_partition_to_move_largest_element() {
    let mut input = vec![51, 10, 4, 5, 6, 19, 3];
    let res = partition(&mut input);
    assert_eq!(res, 6);
  }

  #[test]
  fn test_quick_sort() {
    let mut input = vec![1, 10, 4, 5, 6];
    quick_sort(&mut input);

    assert_eq!(input, vec![1, 4, 5, 6, 10]);
  }
}
