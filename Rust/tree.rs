// Define a tree structure in Rust
#[derive(Debug)]
enum Tree<T> {
    Node(T, Vec<Tree<T>>), // A node with a value and children
    Leaf(T),               // A leaf node with just a value
}

// Implementing basic functionality for the Tree
impl<T> Tree<T> {
    // Create a new leaf node
    pub fn new_leaf(value: T) -> Self {
        Tree::Leaf(value)
    }

    // Create a new node with children
    pub fn new_node(value: T, children: Vec<Tree<T>>) -> Self {
        Tree::Node(value, children)
    }

    // Add a child to a node
    pub fn add_child(&mut self, child: Tree<T>) {
        if let Tree::Node(_, ref mut children) = self {
            children.push(child);
        } else {
            println!("Cannot add a child to a leaf node.");
        }
    }
}

// Traversing the tree with a depth-first search (DFS)
impl<T: std::fmt::Debug> Tree<T> {
    pub fn dfs(&self) {
        match self {
            Tree::Node(value, children) => {
                println!("Node: {:?}", value);
                for child in children {
                    child.dfs();
                }
            }
            Tree::Leaf(value) => {
                println!("Leaf: {:?}", value);
            }
        }
    }
}

fn main() {
    // Creating a tree with nodes and leaves
    let mut root = Tree::new_node(1, vec![]); // Root node with value 1

    let child1 = Tree::new_node(2, vec![
        Tree::new_leaf(3),    // A child node with two leaf children
        Tree::new_leaf(4),
    ]);

    let child2 = Tree::new_leaf(5); // A child with no children (leaf)

    root.add_child(child1); // Add first child node
    root.add_child(child2); // Add second child node

    // Perform DFS traversal and print the tree
    root.dfs();
}
