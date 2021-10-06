/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
 
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut bl1 = l1.unwrap();
        let mut bl2 = l2.unwrap();
        let mut head = Box::new(ListNode::new(0));
        let mut curr = &mut head;
        
        // summing listnodes to vec v1
        let mut carry = 0;
        let mut one = false;
        let mut two = false;
        'main: loop {
            let mut n = carry;
            if !one {
                n += bl1.val;
                match (*bl1).next {
                    None => {one = true;},
                    Some(bln) => {bl1 = bln;},
                };
            }
            if !two {
                n += bl2.val;
                match (*bl2).next {
                    None => {two = true;},
                    Some(bln) => {bl2 = bln;},
                };
            }
            carry = n/10;
            curr.next = Some(Box::new(ListNode::new(n%10)));
            curr = curr.next.as_mut().unwrap();
            println!("{:?}", curr);
            if one && two {break 'main;}
        }
        if carry > 0 {curr.next = Some(Box::new(ListNode::new(carry)));}
        head.next
    }
}
