//! <https://leetcode.com/problems/symmetric-tree/>

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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_sym(left: Option<Ref<TreeNode>>, right: Option<Ref<TreeNode>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                left.val == right.val
                    && is_sym(
                        left.right.as_ref().map(|node| node.borrow()),
                        right.left.as_ref().map(|node| node.borrow()),
                    )
                    && is_sym(
                        left.left.as_ref().map(|node| node.borrow()),
                        right.right.as_ref().map(|node| node.borrow()),
                    )
            }
            _ => false,
        }
    }

    is_sym(
        root.as_ref().map(|root| root.borrow()),
        root.as_ref().map(|root| root.borrow()),
    )
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

    assert!(is_symmetric(Some(Rc::new(RefCell::new(new_node(
        1,
        new_node(2, new_node(3, None, None), new_node(4, None, None),),
        new_node(2, new_node(4, None, None), new_node(3, None, None),),
    ))))));
    assert!(!is_symmetric(Some(Rc::new(RefCell::new(new_node(
        1,
        new_node(2, new_node(3, None, None), new_node(4, None, None),),
        new_node(2, new_node(3, None, None), new_node(4, None, None),),
    ))))));
    assert!(!is_symmetric(Some(Rc::new(RefCell::new(new_node(
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
    ))))));
}
