use std::{fmt, mem};

pub trait Drop {
    fn drop(&mut self);
}

#[allow(dead_code)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Link::Empty => write!(f, "Empty"),
            Link::More(node) => write!(f, " ({}) -> ", node.elem),
        }
    }
}

struct Node {
    elem: i32,
    next: Link,
}

struct List {
    head: Link,
}

#[allow(dead_code)]
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,

            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = curr_link {
            curr_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        let mut curr_link = &self.head;
        while let Link::More(node) = curr_link {
            write!(f, "{}", node.elem)?;
            curr_link = &node.next;
            if let Link::More(_) = curr_link {
                write!(f, ") -> (")?;
            }
        }
        if let Link::Empty = curr_link {
            write!(f, ") -> (*empty*)")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
