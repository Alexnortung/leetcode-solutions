use super::Solution;

// Definition for singly-linked list.
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

fn helper(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    let num1 = l1.as_ref().map(|node| node.val).unwrap_or(0);
    let num2 = l2.as_ref().map(|node| node.val).unwrap_or(0);
    let sum = num1 + num2 + carry;
    let carry = sum / 10;
    let result = sum - ((sum >= 10) as i32) * 10;
    let mut node = ListNode::new(result);
    let l1_next = l1.map(|node| node.next).flatten();
    let l2_next = l2.map(|node| node.next).flatten();

    if l1_next.is_none() && l2_next.is_none() && carry == 0 {
        // Done?
        return None;
    }

    node.next = helper(l1_next, l2_next, carry);

    Some(Box::new(node))
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // helper(l1, l2, 0)
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        loop {
            let num1 = l1.as_ref().map_or_else(|| 0, |node| node.val);
            let num2 = l2.as_ref().map_or_else(|| 0, |node| node.val);
            let sum = num1 + num2 + carry;
            carry = (sum >= 10) as i32;
            let result = sum - ((sum >= 10) as i32) * 10;

            let l1_next = l1.map(|node| node.next).flatten();
            let l2_next = l2.map(|node| node.next).flatten();

            if l1_next.is_none() && l2_next.is_none() && carry == 0 && result == 0 {
                // Done?
                break;
            }

            let node = ListNode::new(result);
            tail.next = Some(Box::new(node));
            tail = tail.next.as_mut().unwrap();

            l1 = l1_next;
            l2 = l2_next;
        }
        if let Some(node) = head.next {
            return Some(node);
        }
        Some(Box::new(ListNode::new(0)))
    }
}
