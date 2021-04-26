#[derive(Debug)]
struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

#[allow(dead_code)]
impl<T> LinkedList<T> {
  /**
   * push an element to the front of the list
   */
  fn push_front(&mut self, item: T) {
    self.0 = Some((item, Box::new(LinkedList(self.0.take()))));
  }

  // Reads an element from the front of the list
  // and returns a borrow of the value.
  fn peek(&self) -> Option<&T> {
    match self.0.as_ref() {
      Some((a, _)) => Some(a),
      None => None,
    }
  }

  // Removes the first element of the list
  fn pop_front(&mut self) -> Option<T> {
    match self.0.take() {
      None => None,
      Some((item, mut child)) => {
        self.0 = child.0.take();
        Some(item)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_linked_list() {
    let list: LinkedList<i32> = LinkedList(None);
    assert_eq!(None, list.peek());
  }

  #[test]
  fn test_add_element_to_empty_list() {
    let mut list = LinkedList(None);
    list.push_front(29);
    assert_eq!(Some(&29), list.peek().take());
  }

  #[test]
  fn test_add_element_to_nonempty_list() {
    let mut list = LinkedList(None);
    list.push_front(29);
    list.push_front(31);
    list.push_front(201);
    assert_eq!(Some(&201), list.peek().take());
  }

  #[test]
  fn test_remove_from_empty_list() {
    let mut list: LinkedList<i32> = LinkedList(None);

    assert_eq!(None, list.pop_front());
  }

  #[test]
  fn test_remove_element_from_nonempty_list() {
    let mut list = LinkedList(None);
    list.push_front(29);
    list.push_front(31);
    list.push_front(201);
    assert_eq!(Some(201), list.pop_front());
    assert_eq!(Some(&31), list.peek().take());
  }

  #[test]
  fn test_remove_all_element_from_nonempty_list() {
    let mut list = LinkedList(None);
    list.push_front(29);
    list.push_front(31);
    list.push_front(201);
    assert_eq!(Some(201), list.pop_front());
    assert_eq!(Some(31), list.pop_front());
    assert_eq!(Some(29), list.pop_front());
    assert_eq!(None, list.pop_front());
    assert_eq!(None, list.peek().take());
  }

  #[test]
  fn test_remove_multiple_elements_from_nonempty_list() {
    let mut list = LinkedList(None);
    list.push_front(29);
    list.push_front(31);
    list.push_front(201);
    assert_eq!(Some(201), list.pop_front());
    assert_eq!(Some(31), list.pop_front());
    assert_eq!(Some(&29), list.peek().take());
    list.push_front(91);
    assert_eq!(Some(91), list.pop_front());
    assert_eq!(Some(&29), list.peek().take());
  }
}
