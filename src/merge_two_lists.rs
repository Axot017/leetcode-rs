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

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut current = &mut head;
        let mut l1 = l1;
        let mut l2 = l2;

        while l1.is_some() && l2.is_some() {
            if let (Some(mut a), Some(mut b)) = (l1.take(), l2.take()) {
                if a.val < b.val {
                    l1 = a.next.take();
                    l2 = Some(b);
                    current = current.next.get_or_insert(a);
                } else {
                    l1 = Some(a);
                    l2 = b.next.take();
                    current = current.next.get_or_insert(b);
                }
            }
        }

        if l1.is_some() {
            current.next = l1;
        }

        if l2.is_some() {
            current.next = l2;
        }

        return head.next;
    }
}
