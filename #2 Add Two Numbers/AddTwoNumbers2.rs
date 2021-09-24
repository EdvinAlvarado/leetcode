// Definition for singly-linked list.
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
        let mut v1 = Vec::new();
        let mut bl1 = l1.unwrap();
        let mut bl2 = l2.unwrap();
        
        // summing listnodes to vec v1
        let mut up_remainder = 0;
        let mut one = false;
        let mut two = false;
        'main: loop {
            let mut n = up_remainder;
            if !one {
                n += bl1.val;
                match (*bl1).next {
                    None => {one == true;},
                    Some(bln) => {bl1 = bln;},
                };
            }
            if !two {
                n += bl2.val;
                match (*bl2).next {
                    None => {two == true;},
                    Some(bln) => {bl2 = bln;},
                };
            }
            up_remainder = n/10;
            v1.push(n%10);
            if one && two {break 'main;}
        }
        if up_remainder != 0 {v1.push(up_remainder);}
        
        // vec to ListNode
        let mut num_iter = v1.into_iter().rev();
        let mut nl = ListNode::new(num_iter.next().unwrap());
        for digit in num_iter{
            nl = ListNode{val: digit, next: Some(Box::new(nl))};
        }
        Some(Box::new(nl))
    }
}
