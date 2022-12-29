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

// some notes on the drop trait
// - it's like a destructor in C++

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}


// this cfg line means only be used if we are compiling for tests
#[cfg(test)]
mod test {
    use super::List; // you have to pull this module in explicitly
    #[test]
    fn basic() {
        let mut list = List::new();

        // check empty list
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        // check removal of items
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // push some more elements (make sure no memory corruption)
        list.push(4);
        list.push(5);

        // Check removal one more time
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // check exhaustion (removal till empty)
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }
}