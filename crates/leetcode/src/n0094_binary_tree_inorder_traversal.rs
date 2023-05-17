//! <https://leetcode.com/problems/binary-tree-inorder-traversal/>

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

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

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn inorder(node: Option<Ref<TreeNode>>, vec: &mut Vec<i32>) {
        let node: Ref<TreeNode> = if let Some(node) = node {
            node
        } else {
            return;
        };

        inorder(node.left.as_ref().map(|root| root.borrow()), vec);

        vec.push(node.val);

        inorder(node.right.as_ref().map(|root| root.borrow()), vec);
    }

    let mut vec = vec![];
    inorder(root.as_ref().map(|root| root.borrow()), &mut vec);

    vec
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

    assert!(inorder_traversal(None) == []);
    assert!(inorder_traversal(Some(Rc::new(RefCell::new(new_node(1, None, None))))) == [1]);
    assert!(
        inorder_traversal(Some(Rc::new(RefCell::new(new_node(
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
            == [4, 10, 12, 15, 18, 22, 24, 25, 31, 35, 44, 50, 66, 70, 90]
    );
}
