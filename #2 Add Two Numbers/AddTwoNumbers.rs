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
pub fn box_list_node_to_vec(bl: Box<ListNode>) -> Vec<i32>  {
    let mut bll = bl; 
    let mut tvec = Vec::new();
    'one: loop {
        tvec.push(bll.val);
        match (*bll).next {
            None => {break 'one;},
            Some(bln) => {bll = bln;}
        }
    }
    tvec
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v1 = box_list_node_to_vec(l1.unwrap());
        let mut v2 = box_list_node_to_vec(l2.unwrap());

        if v1.len() < v2.len() {v1.resize(v2.len(), 0);}
        else if v2.len() < v1.len() {v2.resize(v1.len(), 0);}

        let mut up_remainder = 0;
        for (i,n) in v1.iter_mut().enumerate() {
            let sum = *n + v2[i] + up_remainder;
            up_remainder = sum/10;
            *n = sum%10;
        }
        if up_remainder != 0 {v1.push(up_remainder);}
        // println!("{:?}", v1);

        let mut num_iter = v1.into_iter().rev();
        let mut nl = ListNode::new(num_iter.next().unwrap());
        for digit in num_iter{
            nl = ListNode{val: digit, next: Some(Box::new(nl))};
        }
        Some(Box::new(nl))
    }
}
