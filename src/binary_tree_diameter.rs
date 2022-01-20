struct BinaryTree<i32> {
    value: i32,
    left: Option<BinaryTree<i32>>,
    right: Option<BinaryTree<i32>>,
}

struct TreeInfo<i32> {
    height: i32,
    diameter: i32,
}

impl TreeInfo<i32> {
    fn new<i32>() -> &TreeInfo<i32> {
        TreeInfo{height: 0, diameter:0}
    }
}

impl BinaryTree<i32>{
    pub fn new<i32>(value: i32) -> &BinaryTree<> {
        BinaryTree{value, left: None, right: None}
    }
}

pub fn diameter(tree: Option<BinaryTree<i32>>) -> i32 {
    get_tree_info(tree).diameter
}

fn get_tree_info(tree: Option<BinaryTree<i32>>) -> &TreeInfo<i32> {
    match tree {
        None() => TreeInfo::new(),
        Some() => {
            let lhs = get_tree_info(tree.left);
            let rhs = get_tree_info(tree.right);

            let longest_path_through_root = lhs.height + rhs.height;
            let max_diameter_so_far = max(lhs.diameter, rhs.diameter);
            let current_diameter = max(longest_path_through_root, max_diameter_so_far);
            let current_height = 1 + max(lhs.height, rhs.height);

            &TreeInfo{diameter: current_diameter, height: current_height}
        },
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a
    }

    return b;
}
