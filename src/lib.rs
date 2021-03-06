use std::collections::hash_map::RandomState;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

use rayon::prelude::*;
use bit_vec::BitVec;

const LEFT: bool = false;
const RIGHT: bool = true;

pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut node_list = create_node_list(data);
    let tree = node_list_into_tree(node_list);
    let mut buffer = BitVec::new();
    data.iter()
        .for_each(|byte| tree.path_for(byte, &mut buffer));
    buffer.to_bytes()
}

#[derive(Debug)]
pub(crate) struct Node<'a> {
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
    datum: Option<&'a u8>,
    count: usize,
}

impl<'a> Node<'a> {
    fn new(datum: Option<&'a u8>, count: usize) -> Self {
        Self {
            datum,
            count,
            right: None,
            left: None,
        }
    }
    /// Extend the bit path for a given datum
    fn path_for(&self, datum: &u8, path: &mut BitVec) {
        if let Some(left) = &self.left {
            if left.contains(datum) {
                path.push(LEFT);
                left.path_for(datum, path);
                return;
            }
        }
        if let Some(right) = &self.right {
            if right.contains(datum) {
                path.push(RIGHT);
                right.path_for(datum, path);
            }
        }
    }
    /// Determine if this node or any of the children contain the datum
    fn contains(&self, datum: &u8) -> bool {
        if let Some(d) = self.datum {
            d == datum
        } else {
            // is a parent node, always has left and right
            self.left.as_ref().unwrap().contains(datum) || self.right.as_ref().unwrap().contains(datum)
        }
    }
}

/// Create a node list from input data
pub(crate) fn create_node_list(input: &[u8]) -> Vec<Node<'_>> {
    let mut mapping = HashMap::new();
    input.iter()
        .for_each(|byte| {
            let count = mapping.entry(byte).or_insert(0);
            *count += 1;
        });
    mapping.iter()
        .map(|(k, v)| Node::new(Some(*k), *v))
        .collect::<Vec<Node>>()
}

/// Convert node list into a tree with one root node
pub(crate) fn node_list_into_tree(mut nodes: Vec<Node<'_>>) -> Node<'_> {
    if nodes.len() == 1 {
        nodes.pop().unwrap()
    } else {
        nodes.sort_by_key(|node| node.count);
        let mut parent = Node::new(None, 0);
        let node1 = nodes.remove(0);
        let node2 = nodes.remove(0);

        parent.count = node1.count + node2.count;
        parent.left = Some(Box::new(node1));
        parent.right = Some(Box::new(node2));
        nodes.push(parent);
        node_list_into_tree(nodes)
    }
}

#[cfg(test)]
mod tests {

    use crate::{create_node_list, node_list_into_tree, compress};
    use bit_vec::BitVec;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_create_node_list() {
        let data = b"abbccc!!!!";
        let mut node_list = create_node_list(data);
        assert_eq!(node_list.len(), 4); // 4 unique bytes
        node_list.sort_by_key(|node| node.count);
        for (i, byte) in b"abc!".iter().enumerate() {
            assert!(node_list[i].contains(byte));
        }
    }

    #[test]
    fn test_node_list_into_tree() {
        let data = b"abbccc!!!!";
        let node_list = create_node_list(data);
        assert_eq!(node_list.len(), 4); // 4 unique bytes

        let tree = node_list_into_tree(node_list);
        assert_eq!(tree.count, data.len());
    }

    #[test]
    fn test_tree() {
        let data = b"abbcccddddeeeeee!!!!abcdefghijklmnopqrstuvwxyz1234567890!#%&/()[]{}$@";
        let node_list = create_node_list(data);
        let unique: HashSet<&u8, RandomState> = HashSet::from_iter(data.iter());
        assert_eq!(node_list.len(), unique.len()); // 4 unique bytes

        let tree = node_list_into_tree(node_list);
        for (i, byte) in b"abcdefghijklmnopqrstuvwxyz".iter().enumerate() {
            assert!(tree.contains(byte));
            let mut path = BitVec::new();
            tree.path_for(byte, &mut path);
            assert!(path.len() > 0);
        }
    }

    #[test]
    fn test_compress() {
        let data = (0..2)
            .map(|_| b"oh what a beautiful day, oh what a beautiful morning!".to_vec())
            .flat_map(|v| v)
            .collect::<Vec<u8>>();
        let compressed = compress(&data);
        assert!(compressed.len() < data.len());
    }
}
