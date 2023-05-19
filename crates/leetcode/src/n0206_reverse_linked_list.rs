//! <https://leetcode.com/problems/reverse-linked-list/>

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

type Node = ListNode;
type List = Option<Box<Node>>;

fn pop_front(list: &mut List) -> List {
    let mut head = list.take()?;
    *list = head.next.take();
    Some(head)
}

fn push_front(list: &mut List, mut node: Box<Node>) {
    debug_assert!(node.next.is_none());
    node.next = list.take();
    *list = Some(node);
}

pub fn reverse_list(list: List) -> List {
    let mut original = list;
    let mut reversed = None;

    while let Some(head) = pop_front(&mut original) {
        push_front(&mut reversed, head);
    }

    reversed
}

#[test]
fn test() {
    pub fn new_node(val: i32, next: impl Into<Option<Node>>) -> Node {
        Node {
            val,
            next: next.into().map(Box::new),
        }
    }

    fn new_list(items: &[i32]) -> List {
        let mut list: List = None;
        let mut current = &mut list;

        for &item in items {
            let item = Some(Box::new(new_node(item, None)));

            if let Some(curr) = current {
                curr.next = item;
                current = &mut curr.next;
            } else {
                *current = item;
            }
        }

        list
    }

    assert!(reverse_list(new_list(&[])) == new_list(&[]));
    assert!(reverse_list(new_list(&[0])) == new_list(&[0]));
    assert!(reverse_list(new_list(&[1, 2, 3, 4])) == new_list(&[4, 3, 2, 1]));
    assert!(reverse_list(new_list(&[4, 3, 2, 1])) == new_list(&[1, 2, 3, 4]));

    assert!(true);
}
