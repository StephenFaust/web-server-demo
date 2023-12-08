use std::{cell::RefCell, rc::Rc};

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
    size: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push(&mut self, data: T) {
        let mut new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        let raw_node: *mut Node<T> = &mut *new_node;
        self.head = Some(new_node);
        match self.tail {
            None => self.tail = Some(raw_node),
            Some(_) => (),
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                let data = head.data;
                self.head = head.next;
                if self.head.is_none() {
                    self.tail = None;
                }
                self.size -= 1;
                Some(data)
            }
            None => None,
        }
    }

    fn add(&mut self, data: T) {
        let mut new_node = Box::new(Node { data, next: None });
        let raw_node: *mut _ = &mut *new_node;
        match self.tail {
            Some(tail) => unsafe {
                (*tail).next = Some(new_node);
            },
            None => {
                self.head = Some(new_node);
            }
        }
        self.tail = Some(raw_node);
        self.size += 1;
    }

    fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref head) => Some(&head.data),
            None => None,
        }
    }

    fn get(&self, index: usize) -> Result<Option<&T>, String> {
        let mut current_node = self.head.as_ref();
        if index > self.size {
            return Err("out of range".to_string());
        }
        for _ in 0..index {
            if current_node.is_none() {
                return Ok(None);
            }
            current_node = current_node.unwrap().next.as_ref();
        }
        Ok(Some(&current_node.unwrap().data))
    }

    fn get_size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_push() {
        let mut list = LinkedList::new();
        list.push("1");
        assert_eq!(list.get_size(), 1);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::new();
        list.push("1".to_string());
        list.push("2".to_string());
        for _ in 1..=3 {
            let data = list.pop();
            println!("{:?}", data);
        }
    }
    #[test]
    fn test_add() {
        let mut list = LinkedList::new();
        list.add("1".to_string());
        list.add("2".to_string());
    }

    #[test]
    fn test_get() {
        let mut list = LinkedList::new();
        list.push("1".to_string());
        list.push("2".to_string());
        list.push("3".to_string());
        match list.get(1) {
            Ok(Some(x)) => {
                assert_eq!(*x, "2".to_string());

                let i = x;
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_for() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);
        list.into_iter().for_each(|x| {
            println!("{}", x);
        });
    }

    #[test]
    fn test_ptr() {
        let mut x = vec![1, 2, 3];
        let p: *mut Vec<i32> = &mut x;
        let q = x.as_mut_ptr();
        unsafe { (*p).push(2) }
        println!("{:?} {:?}", p, q)
    }
}
