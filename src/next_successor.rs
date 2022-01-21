struct BinaryTreeParent {
    value: i32,
    left: Option<BinaryTreeParent>,
    right: Option<BinaryTreeParent>,
    parent: Option<BinaryTreeParent>,
}

pub fn next_successor(tree: Option<BinaryTreeParent>, node: Option<BinaryTreeParent>) -> Option<BinaryTreeParent> {
    match node.right {
        Some(_) => find_left_most_child(node.right),
        None => find_right_most_parent(node),
    }
}

fn find_left_most_child(node: Option<BinaryTreeParent>) -> Option<BinaryTreeParent> {
    match node.left {
        Some(_) =>  find_left_most_child(node.left),
        None => node,
    }
}

fn find_right_most_parent(node: Option<BinaryTreeParent>) -> Option<BinaryTreeParent> {
    if None(node.parent) {
        return None;
    }

    if node.parent.right != node {
        return node;
    }

    find_right_most_parent(node.parent)
}
