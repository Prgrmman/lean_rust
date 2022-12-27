use std::mem;


pub struct List {
    head: Link,
}

// This form is special:
// it's something about a null pointer optimization in rust:
// since the More element contains a Box which is a heap pointer to a non NULL address,
// rust does not need to store extra "tag" bits with the enum because Empty will always be 0s.
enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

// "impl" associates code with a type
// "Self" is an alias of the type
impl List {
    pub fn new() -> Self {
        List {head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        // We use mem::replace because mutable references cannot move values out without replacement
        self.head = Link::More(new_node);
    }

    //pub fn pop(&mut self) -> Option<i32> {
    //    let result;
    //    match mem::replace(&mut self.head, Link::Empty) {
    //        Link::Empty => {
    //            result = None;
    //        }
    //        Link::More(node) => {
    //            result = Some(node.elem);
    //            self.head = node.next;
    //        }
    //    };
    //    // This is a handy maco: it lets the program compile, but running program will crash (in a controlled way) if it hits it
    //    //unimplemented!()
    //    result
    //}
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