use crate::util::linked_list::{print_list, to_list, ListNode};

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = dummy_head.as_mut();

        while head.as_mut().unwrap().next.is_some()
            && head.as_mut().unwrap().next.as_mut().unwrap().next.is_some()
        {
            //println!("==============================");
            let mut first = head.as_mut().unwrap().next.take();
            let mut second = first.as_mut().unwrap().next.take();
            //println!("take first and second");
            // print_list(&first);
            // print_list(&second);
            first.as_mut().unwrap().next = second.as_mut().unwrap().next.take();
            // println!("take first.next");
            // print_list(&first);
            // print_list(&second);
            second.as_mut().unwrap().next = first.take();
            // println!("take second.next");
            // print_list(&first);
            // print_list(&second);
            //
            // // head.as_mut().unwrap().next = second;
            // head.as_mut().unwrap().next = second.as_mut().unwrap().next.take();
            // println!("final");
            // head.as_mut().unwrap().next =
            //     second.as_mut().unwrap().next.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = second;
            head = head.unwrap().next.as_mut().unwrap().next.as_mut();
            // println!("head: {:?}", head);
        }

        // println!();
        // println!("{:?}", dummy_head);
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24() {
        // let vec = Solution::swap_pairs(to_list(vec![1, 2, 3, 4]));
        // println!("final vec:");
        // print_list(&vec);
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3])),
            to_list(vec![2, 1, 3])
        );
        assert_eq!(Solution::swap_pairs(to_list(vec![1])), to_list(vec![1]));
    }
}
