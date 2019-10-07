/* NOTE: code was initially copied from first.rs */

/* Adding generics. */
pub struct List<T> {
    head: Link<T>,
}

// type alias Link
// the old version of Link is now reimplemented using Option
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None}
    }
    /* push element to front of the list. */
    pub fn push(&mut self, elem: T) { // I suppose this function does not return anything

        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    /* remove element from the front of the list.
     * Option represents a type that might be Some<T> or None. */
    pub fn pop(&mut self) -> Option<T> {
        //match self.head.take() {
        //    None => None,
        //    Some(node) => {
        //        self.head = node.next;
        //        Some(node.elem)
        //    }
        //}
        /* This particular match idiom is so common, that Option
         * does it with a method called "map". It will apply a function to
         * a value wrapped in Option, otherwise it will return None.
         * Below, we take advantage of a closure.
         * I'm going to assume that the type of node is a non mutable value,
         * where ownership is transferred into the closure block.
         * The last expression "node.elem" moves ownership back out of the closure (I think)
         */
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    /* Map takes self by value, which will move Option out of the thing that it's in.
     * To work around this, we need as_ref, which "demotes" Option to an Option reference. */
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    /* mutable version. */
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            /* the explicit "&mut" is most likely needed because of the automatic derefernce. */
            &mut node.elem
        })
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        /* notice the take method: this is method from Option, that implements the idiom
         * mem::replace(&mut option, None) */
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }

    }
}

/* Add the IntoIter - T wrapper around list */
/* This is a tuple struct. We use it here to make a basic wrapper object
 * around our list. */
pub struct IntoIter<T>(List<T>);
impl <T> List<T> {
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

/* add the Iter - &T. */
pub struct Iter<T> {
    next: Option<&Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.map(|node| &node) }
    }
}

impl<T> Iterator for Iter<T> {
    type Item = &T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.map(|node| &node);
            &node.elem
        })
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
    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3)); // this is interesting...didn't think you could have a reference to a literal. */
        assert_eq!(list.peek_mut(), Some(&mut 3));

        /* This is a test to see if we can actually change the calue of the first element.*/
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

        let mut iter = list.into_iter(); // note: value moved here
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }



}
