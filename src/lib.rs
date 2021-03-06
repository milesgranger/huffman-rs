use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

use bit_vec::BitVec;


pub struct Encoder {}

static LEFT: bool = false;
static RIGHT: bool = true;

pub(crate) struct Block<'a> {
    node: Node<'a>,
    data: [u8; 64000]
}

#[derive(Debug)]
pub(crate) struct Node<'a> {
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
    datum: Option<&'a u8>,
    count: usize
}

impl<'a> Node<'a> {
    fn new(datum: Option<&'a u8>, count: usize) -> Self {
        Self { datum, count, right: None, left: None }
    }
    /// Extend the bit path for a given datum
    fn path_for(&self, datum: &u8, path: &mut BitVec) {
        if let Some(right) = &self.right {
            if let Some(d) = right.datum {
                if d == datum {
                    path.push(RIGHT);
                }
            }
        } else if let Some(left) = &self.left {
            if let Some(d) = left.datum {
                if d == datum {
                    path.push(LEFT)
                }
            }
        } else {
          if self.left_contains(datum) {
              if let Some(left) = &self.left {
                  left.path_for(datum, path)
              }
          } else if self.right_contains(datum) {
              if let Some(right) = &self.right {
                  right.path_for(datum, path)
              }
          }
        }
    }
    /// Determine if this node or any of the children contain the datum
    fn contains(&self, datum: &u8) -> bool {
        if let Some(d) = self.datum {
            d == datum
        } else {
            if let Some(left) = &self.left {
                if left.contains(datum) {
                    true
                } else {
                    if let Some(right) = &self.right {
                        right.contains(datum)
                    } else {
                        false
                    }
                }
            } else {
                false
            }
        }
    }
    /// Determine if the left node, or any of its children contain the datum
    fn left_contains(&self, datum: &u8) -> bool {
        if let Some(left) = &self.left {
            left.contains(datum)
        } else {
            false
        }
    }
    /// Determine if the right node, or an of its children contain the datum
    fn right_contains(&self, datum: &u8) -> bool {
        if let Some(right) = &self.right {
            right.contains(datum)
        } else {
            false
        }
    }
}

/// Create a node list from input data
pub(crate) fn create_node_list(input: &[u8]) -> Vec<Node<'_>> {
    let set: HashSet<&u8, RandomState> = HashSet::from_iter(input.iter());
    set.iter()
        .map(|key| {
            let n_occurances = input.iter().filter(|byte| byte == key).count();
            Node::new(Some(*key), n_occurances)
        })
        .collect()
}

/// Sort node list
pub(crate) fn sort_node_list(nodes: &mut Vec<Node<'_>>) {
    nodes.sort_by_key(|node| node.count)
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

    use crate::{create_node_list, node_list_into_tree};

    #[test]
    fn test_create_node_list() {
        let data = b"abbccc!!!!";
        let mut node_list = create_node_list(data);
        assert_eq!(node_list.len(), 4);  // 4 unique bytes
        node_list.sort_by_key(|node| node.count);
        for (i, byte) in b"abc!".iter().enumerate() {
            assert!(node_list[i].contains(byte));
        }
    }

    #[test]
    fn test_node_list_into_tree() {
        let data = b"abbccc!!!!";
        let mut node_list = create_node_list(data);
        assert_eq!(node_list.len(), 4);  // 4 unique bytes

        let tree = node_list_into_tree(node_list);
        assert_eq!(tree.count, data.len());
    }

    #[test]
    fn test_tree() {
        let data = b"abbccc!!!!";
        let mut node_list = create_node_list(data);
        assert_eq!(node_list.len(), 4);  // 4 unique bytes

        let tree = node_list_into_tree(node_list);
        for byte in b"abc!".iter() {
            assert!(tree.contains(byte));
        }
    }
}
