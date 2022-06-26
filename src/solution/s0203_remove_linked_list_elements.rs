use crate::util::linked_list::{print_list, to_list, ListNode};

pub struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut p = &mut dummy_head;

        while p.as_mut().unwrap().next.is_some() {
            if p.as_mut().unwrap().next.as_mut().unwrap().val == val {
                p.as_mut().unwrap().next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            } else {
                p = &mut p.as_mut().unwrap().next;
            }
        }

        // print_list(&dummy_head);
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_203() {
        // Solution::remove_elements(to_list(vec![1, 2, 6, 3, 4, 5, 6]), 6);
        assert_eq!(
            Solution::remove_elements(to_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
            to_list(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            Solution::remove_elements(to_list(vec![]), 1),
            to_list(vec![])
        );
        assert_eq!(
            Solution::remove_elements(to_list(vec![7, 7, 7, 7]), 7),
            to_list(vec![])
        );
    }
}
