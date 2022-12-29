pub struct List<T> {
    head: Link<T>,
}

// This form is special:
// it's something about a null pointer optimization in rust:
// since the More element contains a Box which is a heap pointer to a non NULL address,
// rust does not need to store extra "tag" bits with the enum because Empty will always be 0s.
//enum Link {
//    Empty,
//    More(Box<Node>),
//}
// Here we change Link to be a type alias of Option
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// "impl" associates code with a type
// "Self" is an alias of the type
impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None}
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        // We use mem::replace because mutable references cannot move values out without replacement
        self.head = Some(new_node);
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
    pub fn pop(&mut self) -> Option<T> {
        //match self.head.take() { // here we have replaced the mem::replace function with "take"
        //    None => None,
        //    Some(node) => {
        //        self.head = node.next;
        //        Some(node.elem)
        //    }
        //}
        // Replace the above with the map idiom, which takes a function and makes a closure
        // TODO: probably will need more study of Rust-style closures...
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

// some notes on the drop trait
// - it's like a destructor in C++

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            //cur_link = mem::replace(&mut boxed_node.next, None);
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None
            // so no unbounded recursion occurs.
        }
    }
}

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);

// All collection types should implement that following itertors:
// - IntoIter - T
// - IterMut - &mut T
// - Iter - &T
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}


/* This one is more fun (Also found multi line comments)
We need to hold a pointer to the next element.
However, we need to use the option type because the list may be empty,
or we may be done iterating
*/
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// No lifetime here, List doesn't have any associated lifetimes
//impl<T> List<T> {
    //// We declare a fresh lifetime here for the *exact* borrow that
    //// creates the iter. Now &self needs to be valid as long as the
    //// Iter is around.
    //// as deref will dereference our box type and rewrap as an Option type again
    //pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        //Iter { next: self.head.as_deref() }
    //}
//}
// Same as above, but with lieftime ellision
impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

// We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    // None of this needs to change, handled by the above.
    // Self continues to be incredibly hype and amazing
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { // note explicit lifetime ellision syntax
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // note: we have to do a "take" here because mutable references do not implement the Copy trait
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

// this cfg line means only be used if we are compiling for tests
#[cfg(test)]
mod test {
    use super::List; // you have to pull this module in explicitly
    #[test]
    fn basic() {
        // This line is pretty cool:
        // Rust figures out the type of list based on the arguments I pass it later down
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(1); list.push(2); list.push(3);

        // Odd that we can pass a mutable reference to a literal...
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        // Originally had the line list.peek_mut().map(|&mut value| {
        // however this line is wrong:
        // |&mut value| means "the argument is a mutable reference, but just copy the value it points to into value, please." 
        // instead, we use just |value| and the original type is preserved
        list.peek_mut().map(|value| {
            *value = 42
        });
    
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        
        let mut iter = list.into_iter();
        // list.peek(); this call is illegal!
        // it doesn't work because by creating an iterator type, we move list into the iterator!
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}