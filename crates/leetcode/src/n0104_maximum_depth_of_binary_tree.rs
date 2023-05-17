//! <https://leetcode.com/problems/maximum-depth-of-binary-tree/>

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_depth_(node: Option<Ref<TreeNode>>, depth: i32, max: &mut i32) {
        let node = if let Some(node) = node {
            node
        } else {
            if depth > *max {
                *max = depth;
            }

            return;
        };

        let left = node.left.as_ref();
        let right = node.right.as_ref();

        max_depth_(left.map(|node| node.borrow()), depth + 1, max);
        max_depth_(right.map(|node| node.borrow()), depth + 1, max);
    }

    let mut max = 0;
    max_depth_(root.as_ref().map(|node| node.borrow()), 0, &mut max);
    max
}

#[test]
fn test() {
    fn new_node(
        val: i32,
        left: impl Into<Option<TreeNode>>,
        right: impl Into<Option<TreeNode>>,
    ) -> TreeNode {
        TreeNode {
            val,
            left: left.into().map(RefCell::new).map(Rc::new),
            right: right.into().map(RefCell::new).map(Rc::new),
        }
    }

    assert!(
        max_depth(Some(Rc::new(RefCell::new(new_node(
            1,
            new_node(2, new_node(3, None, None), new_node(4, None, None),),
            new_node(2, new_node(4, None, None), new_node(3, None, None),),
        )))))
            == 3
    );
    assert!(
        max_depth(Some(Rc::new(RefCell::new(new_node(
            25,
            None,
            new_node(
                50,
                new_node(35, new_node(31, None, None), new_node(44, None, None)),
                new_node(70, new_node(66, None, None), new_node(90, None, None)),
            ),
        )))))
            == 4
    );
    assert!(
        max_depth(Some(Rc::new(RefCell::new(new_node(
            25,
            new_node(
                15,
                new_node(10, new_node(4, None, None), new_node(12, None, None)),
                new_node(22, new_node(18, None, None), new_node(24, None, None)),
            ),
            new_node(
                50,
                new_node(35, new_node(31, None, None), new_node(44, None, None)),
                new_node(70, new_node(66, None, None), new_node(90, None, None)),
            ),
        )))))
            == 4
    );
}
