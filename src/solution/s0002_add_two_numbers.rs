use crate::util::linked_list::{print_list, to_list, ListNode};

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // print_list(&l1);
        // print_list(&l2);

        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        let (mut l1, mut l2) = (l1, l2);
        let mut carry = 0;
        //let mut current = head.as_mut();

        while l1.is_some() || l2.is_some() {
            let x = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => 0,
            };
            let y = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => 0,
            };
            let sum = x + y + carry;
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.as_mut().next.as_mut().unwrap();
            // tail                            &Box
            // tail.as_mut                     &ListNode
            // tail.as_mut.next                Option<Box>
            // tail.as_mut.next.as_mut         Option<&Box>
            // tail.as_mut.next.as_mut.unwrap  &Box
        }

        if carry > 0 {
            tail.next = Some(Box::new(ListNode::new(1)));
        }

        //print_list(&head.next);
        //println!();
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
