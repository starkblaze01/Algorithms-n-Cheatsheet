// Define a binary tree structure in Rust
#[derive(Debug)]
struct BinaryTree<T> {
    value: T,
    left: Option<Box<BinaryTree<T>>>,  // Left child
    right: Option<Box<BinaryTree<T>>>, // Right child
}

// Implement basic functionality for the BinaryTree
impl<T> BinaryTree<T> {
    // Create a new node (with no children)
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a left child
    pub fn insert_left(&mut self, value: T) {
        self.left = Some(Box::new(BinaryTree::new(value)));
    }

    // Insert a right child
    pub fn insert_right(&mut self, value: T) {
        self.right = Some(Box::new(BinaryTree::new(value)));
    }
}

// Implement tree traversal methods
impl<T: std::fmt::Debug> BinaryTree<T> {
    // Pre-order traversal (root, left, right)
    pub fn preorder(&self) {
        println!("{:?}", self.value); // Visit the root
        if let Some(left) = &self.left {
            left.preorder();
        }
        if let Some(right) = &self.right {
            right.preorder();
        }
    }

    // In-order traversal (left, root, right)
    pub fn inorder(&self) {
        if let Some(left) = &self.left {
            left.inorder();
        }
        println!("{:?}", self.value); // Visit the root
        if let Some(right) = &self.right {
            right.inorder();
        }
    }

    // Post-order traversal (left, right, root)
    pub fn postorder(&self) {
        if let Some(left) = &self.left {
            left.postorder();
        }
        if let Some(right) = &self.right {
            right.postorder();
        }
        println!("{:?}", self.value); // Visit the root
    }
}

fn main() {
    // Create the root of the binary tree
    let mut root = BinaryTree::new(1);

    // Insert children to the root
    root.insert_left(2);
    root.insert_right(3);

    // Insert children to the left child of the root
    if let Some(left_child) = root.left.as_mut() {
        left_child.insert_left(4);
        left_child.insert_right(5);
    }

    // Insert children to the right child of the root
    if let Some(right_child) = root.right.as_mut() {
        right_child.insert_left(6);
        right_child.insert_right(7);
    }

    // Pre-order traversal
    println!("Pre-order traversal:");
    root.preorder();

    // In-order traversal
    println!("\nIn-order traversal:");
    root.inorder();

    // Post-order traversal
    println!("\nPost-order traversal:");
    root.postorder();
}
