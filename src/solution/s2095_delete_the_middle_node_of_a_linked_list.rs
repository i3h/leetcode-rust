use crate::util::linked_list::{print_list, to_list, ListNode};

pub struct Solution {}

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = &head;
        let mut p2 = &head;
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut p3 = &mut dummy_head;

        while p2.as_ref().unwrap().next.is_some() {
            p3.as_mut().unwrap().next = Some(Box::new(ListNode::new(p1.as_ref().unwrap().val)));
            p3 = &mut p3.as_mut().unwrap().next;
            p1 = &p1.as_ref().unwrap().next;
            p2 = &p2.as_ref().unwrap().next;
            match p2.as_ref().unwrap().next {
                Some(_) => p2 = &p2.as_ref().unwrap().next,
                None => break,
            }
        }

        p3.as_mut().unwrap().next = p1.as_ref().unwrap().next.to_owned();

        // println!("p1: {:?}", p1.as_ref().unwrap().val);
        // print_list(&dummy_head);

        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2095() {
        // Solution::delete_middle(to_list(vec![1, 3, 4, 7, 1, 2, 6]));
        assert_eq!(
            Solution::delete_middle(to_list(vec![1, 3, 4, 7, 1, 2, 6])),
            to_list(vec![1, 3, 4, 1, 2, 6])
        );
        assert_eq!(
            Solution::delete_middle(to_list(vec![1, 2, 3, 4])),
            to_list(vec![1, 2, 4])
        );
        assert_eq!(
            Solution::delete_middle(to_list(vec![2, 1])),
            to_list(vec![2])
        );
    }
}
