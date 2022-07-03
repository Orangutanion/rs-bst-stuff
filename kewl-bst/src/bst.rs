/// Reference for children nodes. Nullable and all nodes will live on heap.
type Child = Option<Box>;

/// Each node can point to two subsequent nodes: one whose value is less, one whose value is greater.
/// This implementation won't allow two nodes of the same value.
struct Node<T: Ord + Copy> {
    data: T,
    lhs: Child<T>,
    rhs: Child<T>,
}

/// Entrypoint for BST. This buffer lives on the stack.
pub struct BST<T: Ord + Copy> {
    pub size: usize,
    head: Child<T>,
    min: Child<T>,
    max: Child<T>,
}

impl<T: Ord + Copy> BST<T> {
    /// Create empty BST. This should also be run by default trait.
    pub fn new() -> self {
        return BST {
            size: 0,
            head: None,
            min: None,
            max: None,
        }
    }

    /// Create a BST from an existing item
    pub fn from(first: &T) -> self {
        unimplemented!();
    }

    /// Puts item into BST. Returns true if item hadn't exited before and was added, false if did exist/not added
    pub fn put(&mut self, item: &T) -> bool {
        unimplemented!();
    }

    /// Checks if BST contains item
    pub fn contains(&self, item: &T) -> bool {
        unimplemented!();
    }

    /// Attempts to remove an item. Returns true if it was removed, false if it wasn't.
    pub fn remove(&mut self, item: &T) -> bool {
        unimplemented!();
    }

    /// If there's a minimum value, returns a copy of it on stack with full ownership
    pub fn min(&self) -> Option<T> {
        unimplemented!();
    }

    /// If there's a maximum value, reteurns a copy of it on stack with full ownership
    pub fn max(&self) -> Option<T> {
        unimplemented!();
    }
}