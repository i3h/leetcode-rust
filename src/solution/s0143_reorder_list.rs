use std::collections::VecDeque;

use crate::util::linked_list::{print_list, to_list, ListNode};

pub struct Solution {}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut p1 = &*head;
        let mut p2 = &*head;

        let mut count = 0;
        while p2.as_ref().unwrap().next.is_some() {
            p1 = &p1.as_ref().unwrap().next;
            count += 1;
            p2 = &p2.as_ref().unwrap().next;
            match p2.as_ref().unwrap().next {
                Some(_) => {
                    p2 = &p2.as_ref().unwrap().next;
                }
                None => break,
            }
        }
        p2 = p1;
        p1 = head;

        let mut s1 = VecDeque::new();
        for _ in 0..count {
            s1.push_back(p1.as_ref().unwrap().val);
            p1 = &p1.as_ref().unwrap().next;
        }
        // println!("s1: {:?}", s1);
        let mut s2 = VecDeque::new();
        while p2.is_some() {
            s2.push_back(p2.as_ref().unwrap().val);
            p2 = &p2.as_ref().unwrap().next;
        }
        // println!("s2: {:?}", s2);

        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut p = &mut dummy_head;
        while !s1.is_empty() && !s2.is_empty() {
            let v1 = s1.pop_front().unwrap();
            let v2 = s2.pop_back().unwrap();
            p.as_mut().unwrap().next = Some(Box::new(ListNode::new(v1)));
            p = &mut p.as_mut().unwrap().next;
            p.as_mut().unwrap().next = Some(Box::new(ListNode::new(v2)));
            p = &mut p.as_mut().unwrap().next;
        }
        while !s1.is_empty() {
            let v1 = s1.pop_front().unwrap();
            p.as_mut().unwrap().next = Some(Box::new(ListNode::new(v1)));
            p = &mut p.as_mut().unwrap().next;
        }
        while !s2.is_empty() {
            let v2 = s2.pop_back().unwrap();
            p.as_mut().unwrap().next = Some(Box::new(ListNode::new(v2)));
            p = &mut p.as_mut().unwrap().next;
        }

        head.as_mut().unwrap().next = dummy_head.unwrap().next.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_143() {
        {
            let vec = vec![1, 2, 3, 4];
            let mut list = to_list(vec);
            Solution::reorder_list(&mut list);
            assert_eq!(list, to_list(vec![1, 4, 2, 3]));
        }
        {
            let vec = vec![1, 2, 3, 4, 5];
            let mut list = to_list(vec);
            Solution::reorder_list(&mut list);
            assert_eq!(list, to_list(vec![1, 5, 2, 4, 3]));
        }
    }
}
