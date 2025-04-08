#![allow(unused)]

#[derive(Debug, Default)]
struct List {
    head: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl List {
    fn new() -> Self {
        Default::default()
    }
    fn push(&mut self, value: i32) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(node);
    }
    fn pop(&mut self) -> Option<i32> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = List::new();
        list.push(3);
        list.push(2);
        list.push(1);
        assert!(list.head.is_some());
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }
}
