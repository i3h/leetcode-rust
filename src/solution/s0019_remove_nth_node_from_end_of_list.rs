use crate::util::linked_list::{print_list, to_list, ListNode};

pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;

        let mut p = dummy_head.as_ref();
        while p.unwrap().next.is_some() {
            len += 1;
            p = p.unwrap().next.as_ref();
        }
        // println!("len: {}", len);

        let i = len - n;
        let mut p = dummy_head.as_mut();
        for _ in 0..i {
            p = p.unwrap().next.as_mut();
        }
        let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        p.as_mut().unwrap().next = next;

        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19() {
        // Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2);
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}
