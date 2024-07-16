use std::collections::HashMap; 

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub freq: usize,                // Frequency of the character
    pub char: Option<char>,         // Character stored in the node
    pub left: Option<Box<Node>>,    // Left child node
    pub right: Option<Box<Node>>,   // Right child node
}

impl Node {
    // Generate a new leaf node
    pub fn new_leaf(freq: usize, char: char) -> Self {
        Node {
            freq,
            char: Some(char),
            left: None,
            right: None,
        }
    }

    // Generate a new internal node with left and right children
    pub fn new_internal(freq: usize, left: Node, right: Node) -> Self {
        Node {
            freq,
            char: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl Ord for Node {
    // Compares nodes by frequency (for priority queue).
    fn cmp(&self, other_node: &Self) -> std::cmp::Ordering {
        other_node.freq.cmp(&self.freq) 
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other_node: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other_node))
    }
}

