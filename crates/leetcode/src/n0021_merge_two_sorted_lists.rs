//! <https://leetcode.com/problems/merge-two-sorted-lists/>

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type List = Option<Box<ListNode>>;

#[inline]
fn pop_front(list: &mut List) -> List {
    let mut head = list.take()?;
    *list = head.next.take();
    Some(head)
}

#[inline]
fn push_back(last: &mut List, next: Box<ListNode>) -> &mut List {
    if let Some(last_) = last {
        last_.next = Some(next);
        &mut last_.next
    } else {
        *last = Some(next);
        last
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;

    let mut head: List = None;
    let mut last: &mut List = &mut head;

    loop {
        match (&list1, &list2) {
            (None, None) => return head,
            (None, Some(_)) => last = push_back(last, pop_front(&mut list2).unwrap()),
            (Some(_), None) => last = push_back(last, pop_front(&mut list1).unwrap()),
            (Some(head1), Some(head2)) => {
                if head1.val < head2.val {
                    last = push_back(last, pop_front(&mut list1).unwrap());
                } else {
                    last = push_back(last, pop_front(&mut list2).unwrap());
                }
            }
        }
    }
}

#[test]
fn test() {
    pub fn node(val: i32, next: impl Into<Option<ListNode>>) -> ListNode {
        ListNode {
            val,
            next: next.into().map(Box::new),
        }
    }

    fn list(items: &[i32]) -> List {
        let mut list: List = None;
        let mut current = &mut list;

        for &item in items {
            let item = Some(Box::new(node(item, None)));

            if let Some(curr) = current {
                curr.next = item;
                current = &mut curr.next;
            } else {
                *current = item;
            }
        }

        list
    }

    assert!(merge_two_lists(list(&[]), list(&[])) == list(&[]));
    assert!(merge_two_lists(list(&[20]), list(&[10])) == list(&[10, 20]));
    assert!(
        merge_two_lists(list(&[0, 2, 4, 6, 8]), list(&[1, 3, 5, 7]))
            == list(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
    );

    assert!(merge_two_lists(list(&[5]), list(&[1, 2, 4])) == list(&[1, 2, 4, 5]));
    assert!(merge_two_lists(list(&[-9, 3]), list(&[5, 7])) == list(&[-9, 3, 5, 7]));
}
