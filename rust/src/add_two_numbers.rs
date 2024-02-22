use crate::list_node::ListNode;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut dummy = Box::new(ListNode::new(0));
        let mut curry = &mut dummy;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(n1) = l1 {
                sum += n1.val;
                l1 = n1.next;
            }
            if let Some(n2) = l2 {
                sum += n2.val;
                l2 = n2.next;
            }
            carry = sum / 10;
            curry.next = Some(Box::new(ListNode::new(sum % 10)));
            curry = curry.next.as_mut().unwrap();
        }
        dummy.next
    }
}

pub struct Solution {}