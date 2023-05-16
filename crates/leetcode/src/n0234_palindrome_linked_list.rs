//! <https://leetcode.com/problems/palindrome-linked-list/>

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //
//                                             ListNode                                           //
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        new_node(val, None)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //
//                                               Node                                             //
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //

pub type Node = ListNode;

pub fn new_node(val: i32, next: impl Into<Option<Node>>) -> Node {
    Node {
        val,
        next: next.into().map(Box::new),
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //
//                                               List                                             //
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //

pub struct List {
    pub head: Option<Box<Node>>,
}

impl List {
    // 0(1*)
    pub fn new(head: impl Into<Option<Node>>) -> Self {
        Self {
            head: head.into().map(Box::new),
        }
    }

    // 0(1)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // 0(n)
    pub fn len(&self) -> usize {
        self.into_iter().count()
    }

    // 0(1)
    pub fn pop_front(&mut self) -> Option<Box<Node>> {
        let mut head = self.head.take()?;
        let next = head.next.take();
        self.head = next;
        Some(head)
    }

    // 0(1)
    pub fn push_front(&mut self, mut node: Box<Node>) {
        debug_assert!(node.next.is_none());

        let head = self.head.take();
        node.next = head;
        self.head = Some(node);
    }

    // 0(n)
    pub fn last_mut(&mut self) -> Option<&mut Box<Node>> {
        let mut last = self.head.as_mut()?;

        loop {
            if last.next.is_some() {
                last = last.next.as_mut().unwrap();
            } else {
                break;
            }
        }

        Some(last)
    }

    // 0(n)
    pub fn link(&mut self, other: Self) {
        if let Some(last) = self.last_mut() {
            debug_assert!(last.next.is_none());
            last.next = other.head;
        }
    }

    // 0(n)
    pub fn reverse(&mut self, first: usize) {
        let mut first_half = List::new(None);

        for _ in 0..first {
            first_half.push_front(self.pop_front().unwrap());
        }

        first_half.link(List {
            head: self.head.take(),
        });

        *self = first_half;
    }

    // 0(n)
    pub fn is_palindrome(&mut self) -> bool {
        let len = self.len();

        self.reverse(len / 2);

        let first_half = self.into_iter().take(len / 2);
        let second_half = self
            .into_iter()
            .skip(if len % 2 == 0 { len / 2 } else { len / 2 + 1 });

        first_half
            .zip(second_half)
            .all(|(first, second)| first == second)
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = i32;
    type IntoIter = ListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { head: &self.head }
    }
}

impl FromIterator<i32> for List {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut head: Option<Box<Node>> = None;
        let mut current = &mut head;

        for val in iter {
            let item = Some(Box::new(new_node(val, None)));

            if let Some(curr) = current {
                curr.next = item;
                current = &mut curr.next;
            } else {
                *current = item;
            }
        }

        List { head }
    }
}

impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.into_iter().collect::<Vec<_>>(), f)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //
//                                             ListIter                                           //
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //

pub struct ListIter<'a> {
    head: &'a Option<Box<Node>>,
}

impl<'a> Iterator for ListIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head) = &self.head {
            let val = head.val;
            self.head = &head.next;
            Some(val)
        } else {
            None
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //
//                                               Test                                             //
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ //

#[test]
fn test() {
    for (list, is_palindrome) in [
        (&[] as &[i32], true),
        (&[1, 2, 3], false),
        (&[3, 2, 1], false),
        (&[1, 1, 1], true),
        (&[1, 2, 3, 3, 2, 1], true),
        (&[1, 2, 3, 2, 1], true),
    ] {
        let mut list = list.into_iter().copied().collect::<List>();
        assert_eq!(list.is_palindrome(), is_palindrome);
    }
}
