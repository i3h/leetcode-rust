use crate::util::linked_list::{print_list, to_list, ListNode};

pub struct Solution {}

impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        // let mut p1 = &mut dummy_head;
        //  let mut p2 = &mut dummy_head.as_mut().unwrap().next;
        // let v = &mut p1.as_mut().unwrap().val;
        // *v = 109;
        // print_list(p1);

        let mut p1 = &head;
        let mut p2 = &head;

        for _ in 0..k {
            p2 = &p2.as_ref().unwrap().next;
        }

        while p2.as_ref().unwrap().next.is_some() {
            p1 = &p1.as_ref().unwrap().next;
            p2 = &p2.as_ref().unwrap().next;
        }

        println!("=================");
        print_list(p1);
        print_list(p2);
        println!("=================");

        let p3 = &mut p2.as_ref().take();
        // let v = &mut p3.unwrap();
        // let v = &mut p3.as_mut().unwrap().val;
        // v.val = 109;

        // println!("p1: {:?}", p1.as_ref().unwrap().val);
        // let mut p3 = p1.as_ref();
        // p3.as_mut().unwrap().val = 102;
        // v.val = 102;

        println!("");
        println!("head:");
        print_list(&head);

        // dummy_head.unwrap().next

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1721() {
        Solution::swap_nodes(to_list(vec![1, 2, 3, 4, 5]), 2);
        // assert_eq!(
        //     Solution::swap_nodes(to_list(vec![1, 2, 3, 4, 5]), 2),
        //     to_list(vec![1, 4, 3, 2, 5])
        // );
    }
}
