//! <https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/>

#[derive(PartialEq, Eq)]
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

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = &mut f.debug_tuple("");

        debug = debug.field(&self.val);

        if let Some(left) = self.left.as_ref() {
            debug = debug.field(&left.borrow());
        } else {
            debug = debug.field(&Option::<()>::None);
        }

        if let Some(right) = self.right.as_ref() {
            debug = debug.field(&right.borrow());
        } else {
            debug = debug.field(&Option::<()>::None);
        }

        debug.finish()
    }
}

use std::{cell::RefCell, rc::Rc};

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        if nums.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
        }

        let mid = nums.len() / 2;

        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: bst(&nums[..mid]),
            right: bst(&nums[mid + 1..]),
        })))
    }

    bst(&nums)
}

#[test]
fn test() {
    // 😱 Harder to test than to solve...

    dbg!(sorted_array_to_bst(vec![]));
    dbg!(sorted_array_to_bst(vec![0]));
    dbg!(sorted_array_to_bst(vec![0, 1]));
    dbg!(sorted_array_to_bst(vec![0, 1, 2]));
    dbg!(sorted_array_to_bst(vec![0, 1, 2, 3]));
    dbg!(sorted_array_to_bst(vec![0, 1, 2, 3, 4]));
}
