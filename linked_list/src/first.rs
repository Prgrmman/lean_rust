use std::mem;
// mark the List as public so that others can use it, but as a struct to hide implementation
// details.
// Structs with one element are the same size as that element: a zero-cost abstraction.
pub struct List {
    head: Link,
}

// link is used to chain elements together
// This used the null pointer optimization, because there is no need to store the 'Empty' tag (can
// be all 0s)
enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

/* Multiline comment cool
 * Now we need to specify an implementation for our List
 * "impl" blocks associate code with a type
 * A normal function inside of an "impl" block is a static method
 *
 * Methods are a special type of function in rust because of the "self" arg
 *
 * There are three primary forms of ownership in Rust:
 *  - self: Value
 *  - &mut self: mutable reference
 *      Gotcha: you can't remove a value without replacement
 *  - &self: shared reference
 */
impl List {
    pub fn new() -> Self { // "Self" is an alias to List
        List { head: Link::Empty} // implicit return as last expression
    }
    /* push element to front of the list. */
    pub fn push(&mut self, elem: i32) { // I suppose this function does not return anything

        /* The code does not work because of exception safety.
         * One would think that we could move an element out of List as long as we replace it
         * later. However, if an exception occurred, and the code unwrapped, we need some guarantee
         * that memory will be valid there. This is why we must use mem::replace. */
        //let new_node = Box::new(Node {
        //    elem: elem,
        //    next: self.head,
        //});
        //self.head = Link::More(new_node);
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty), // temporarily replace head of the list with Empty and assign next to the previous head.
        });
        self.head = Link::More(new_node);
    }
    /* remove element from the front of the list.
     * Option represents a type that might be Some<T> or None. */
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        /* An example of enum pattern matching! */
        /* we have to do the replace trick because we can't move values
         * out of shared references without replacing them. */
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
            result = None;
            }
            Link::More(node) => {
                /* It should be noted that the compiler is doing something tricky here,
                 * known as Deref coercion. Basically, if a type U implements Deref<Target=T>,
                 * then values of &U will automatically be coerced to &T. So in this case, 
                 * the compiler is actually coercing Box<Node> into Node so we can access the elem
                 * field */
                result = Some(node.elem);
                self.head = node.next;
            }
        };
        result
    }
}

/* The guide claims that the default destructor for our linked list would be recursive
 * and that recursion would be unbounded. This is why we explicitly implement the Drop
 * trait for our list. */
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        /* "while let" is a form of pattern matching.
         * It means "do this thing while this pattern matches" */
        while let Link::More(mut boxed_node) = cur_link {
            /* These next few lines are pretty clever.
             * The pattern matching statement above moves ownership of the boxed_node
             * into the body of the while loop. The Link stored in boxed_node.next is then replaced
             * with an empty link (Link::Empty), and that value is now assigned to cur_link.
             * So boxed_node will go out of scope, be dropped, and therefore cleaned up.
             * This eliminates unbounded recursion. This is the reason why we have to set
             * boxed_node.next to Link::Empty, so it doesn't recurse down the list
             */
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
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
