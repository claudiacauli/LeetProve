use num_bigint::BigUint;

// Definition for singly-linked list.
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

// Helper functions
fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in v.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn list_to_vec(mut l: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    while let Some(node) = l {
        result.push(node.val);
        l = node.next;
    }
    result
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sum;
    let mut carry = 0;
    let mut l3 = Some(Box::new(ListNode { val: 0, next: None }));
    let mut head = l3.as_mut();

    let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());

    while l1.is_some() || l2.is_some() {
        sum = 0;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next.as_ref();
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next.as_ref();
        }
        sum += carry;
        carry = sum / 10;
        sum %= 10;
        head.as_mut().unwrap().next = Some(Box::new(ListNode { val: sum, next: None }));
        head = head.unwrap().next.as_mut();
    }

    if carry != 0 {
        head.as_mut().unwrap().next = Some(Box::new(ListNode { val: carry, next: None }));
    }

    l3.unwrap().next
}

pub fn main() {
    let l1 = vec_to_list(vec![2, 4, 3]);
    let l2 = vec_to_list(vec![5, 6, 4]);
    let result = add_two_numbers(l1, l2);
    println!("{:?}", list_to_vec(result)); // Should print the linked list representing the number 807
}



fn vec_to_biguint(digits: &[i32]) -> BigUint {
    digits.iter()
        .rev()
        .fold(BigUint::from(0u32), |acc, &d| acc * 10u32 + d as u32)
}

fn biguint_to_vec(n: &BigUint) -> Vec<i32> {
    let s = n.to_string();
    s.chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn trim_trailing_zeros(mut v: Vec<i32>) -> Vec<i32> {
    while v.len() > 1 && v.last() == Some(&0) {
        v.pop();
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000000))]

        #[test]
        fn prop_add_two_numbers_bigint(
            v1 in prop::collection::vec(0..10i32, 1..100).prop_map(trim_trailing_zeros),
            v2 in prop::collection::vec(0..10i32, 1..100).prop_map(trim_trailing_zeros)
        ) {
            let num1 = vec_to_biguint(&v1);
            let num2 = vec_to_biguint(&v2);
            
            let l1 = vec_to_list(v1);
            let l2 = vec_to_list(v2);
            
            let result = add_two_numbers(l1, l2);
            let result_vec = list_to_vec(result);
            
            let result_num = vec_to_biguint(&result_vec);
            let expected = num1 + num2;
            
            prop_assert_eq!(result_num, expected);
        }
    }
}