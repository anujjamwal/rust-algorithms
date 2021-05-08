// 2021-05-08
// https://www.spoj.com/problems/TEST

struct InputReader();

impl Iterator for InputReader {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    let mut buffer = String::new();
    std::io::stdin()
      .read_line(&mut buffer)
      .expect("Failed to read input");

    match buffer.trim().parse::<i32>() {
      Ok(i) => Some(i),
      Err(_) => None,
    }
  }
}

struct Solution<X: Iterator<Item = i32>>(X);

impl<X> Iterator for Solution<X>
where
  X: Iterator<Item = i32>,
{
  type Item = X::Item;

  fn next(&mut self) -> Option<Self::Item> {
    match self.0.next() {
      Some(i) => {
        if i != 42 {
          Some(i)
        } else {
          None
        }
      }
      None => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_without_stop() {
    let input = [1, 2, 88, 3, 9].iter().map(|x| *x);

    let sol = Solution(input);
    let output: Vec<i32> = sol.collect();

    assert_eq!(vec![1, 2, 88, 3, 9], output);
  }

  #[test]
  fn test_code() {
    let input = [1, 2, 88, 42, 3, 9].iter().map(|x| *x);

    let sol = Solution(input);
    let output: Vec<i32> = sol.collect();

    assert_eq!(vec![1, 2, 88], output);
  }
}

fn main() {
  let reader = InputReader();

  let sol = Solution(reader);

  for i in sol {
    println!("{}", i);
  }
}
