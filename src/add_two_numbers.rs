#[allow(dead_code)]
pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut current = &mut head;

        let mut nextl1 = l1;
        let mut nextl2 = l2;

        let mut reminder = 0;

        while !nextl1.is_none() && !nextl2.is_none() {
            let nodes_sum = match (&nextl1, &nextl2) {
                (Some(a), Some(b)) => a.val + b.val + reminder,
                (Some(a), None) => a.val + reminder,
                (None, Some(b)) => b.val + reminder,
                (None, None) => reminder,
            };
            reminder = nodes_sum / 10;
            let value = nodes_sum % 10;
            current.val = value;

            nextl1 = if let Some(node) = nextl1 {
                node.next
            } else {
                None
            };
            nextl2 = if let Some(node) = nextl2 {
                node.next
            } else {
                None
            };

            if nextl1.is_none() && nextl2.is_none() {
                return Some(Box::new(head));
            }
            current.next = Some(Box::new(ListNode::new(0)));
            current = current.next.as_mut().unwrap();
        }

        return None;
    }
}
