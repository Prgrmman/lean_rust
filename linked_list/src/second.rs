/* NOTE: code was initially copied from first.rs */

pub struct List {
    head: Link,
}

// type alias Link
// the old version of Link is now reimplemented using Option
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None}
    }
    /* push element to front of the list. */
    pub fn push(&mut self, elem: i32) { // I suppose this function does not return anything

        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    /* remove element from the front of the list.
     * Option represents a type that might be Some<T> or None. */
    pub fn pop(&mut self) -> Option<i32> {
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
}

impl Drop for List {
    fn drop(&mut self) {
        /* notice the take method: this is method from Option, that implements the idiom
         * mem::replace(&mut option, None) */
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }

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
