use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

pub struct Encoder {}

pub(crate) struct Block<'a> {
    node: Node<'a>,
    data: [u8; 64000]
}

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
    // TODO: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold_first when stable
    if nodes.len() == 1 {
        nodes.pop().unwrap()
    } else {
        nodes.sort_by_key(|node| node.count);
        let mut parent = Node::new(None, 0);
        let node1 = nodes.remove(nodes.len()-1);
        let node2 = nodes.remove(nodes.len()-1);

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
        let node_list = create_node_list(data);
        assert_eq!(node_list.len(), 4);  // 4 unique bytes
    }

    #[test]
    fn test_node_list_into_tree() {
        let data = b"abbccc!!!!";
        let mut node_list = create_node_list(data);
        assert_eq!(node_list.len(), 4);  // 4 unique bytes

        let tree = node_list_into_tree(node_list);

    }
}
