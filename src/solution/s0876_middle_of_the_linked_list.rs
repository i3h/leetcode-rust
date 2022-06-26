use crate::util::linked_list::{print_list, to_list, ListNode};

pub struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = &head;
        let mut p2 = &head;
        // let mut dummy_head = Some(Box::new(ListNode::new(0)));
        // let mut p3 = &mut dummy_head;

        while p2.as_ref().unwrap().next.is_some() {
            // p3.as_mut().unwrap().next = Some(Box::new(ListNode::new(p1.as_ref().unwrap().val)));
            // p3 = &mut p3.as_mut().unwrap().next;
            p1 = &p1.as_ref().unwrap().next;
            p2 = &p2.as_ref().unwrap().next;
            match p2.as_ref().unwrap().next {
                Some(_) => p2 = &p2.as_ref().unwrap().next,
                None => break,
            }
        }
        // let p3 = p1.to_owned();

        // println!("====================");
        // print_list(&p3);
        // println!("====================");
        // p3.take()
        p1.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_876() {
        assert_eq!(
            Solution::middle_node(to_list(vec![1, 2, 3, 4, 5])),
            to_list(vec![3, 4, 5])
        );
        assert_eq!(
            Solution::middle_node(to_list(vec![1, 2, 3, 4, 5, 6])),
            to_list(vec![4, 5, 6])
        );
    }
}
