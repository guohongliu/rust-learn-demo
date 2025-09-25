use std::ops::Deref;

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
struct Solution;
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let get_number_by_list_node = |first_node: Option<Box<ListNode>> | {
            let mut index = 0;
            let mut result = 0;
            let mut current = first_node.unwrap();

            loop {
                let value = current.val;
                result += value * 10i32.pow(index);

                if !current.next.is_some() { break };

                current = current.next.unwrap();
                index += 1;
            };

            result
        };

        let l1_number = get_number_by_list_node(l1);
        let l2_number = get_number_by_list_node(l2);
        let sum = l1_number + l2_number;

        let convert_to_listnode = |sum: i32| {
            let mut num = sum;

            if num == 0 {
                return Some(Box::new(ListNode::new(0)));
            }

            // 创建哑节点（简化头节点处理）
            let mut dummy = Box::new(ListNode::new(0));
            let mut current = &mut dummy;

            while num != 0 {
                let digit = num % 10;
                let new_node = Box::new(ListNode::new(digit));
                current.next = Some(new_node);
                current = current.next.as_mut().unwrap();
                num /= 10;
            }

            // 哑节点的next就是真正的头节点
            dummy.next
        };

        convert_to_listnode(sum)
    }

    pub fn add_two_numbers_v1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 创建哑节点简化头节点处理
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;

        let mut carry = 0;
        let mut p1 = l1;
        let mut p2 = l2;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let val1 = p1.as_ref().map_or(0, |node| node.val);
            let val2 = p2.as_ref().map_or(0, |node| node.val);
            // 计算当前位总和（包含进位）
            let sum = val1 + val2 + carry;
            // 当前位结果 = 总和 % 10
            let current_val = sum % 10;
            // 新进位 = 总和 / 10
            carry = sum / 10;

            current.next = Some(Box::new(ListNode::new(current_val)));
            current = current.next.as_mut().unwrap();

            p1 = p1.and_then(|node| node.next);
            p2 = p2.and_then(|node| node.next);

        }

        dummy.next
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let get_test_number : fn(Vec<i32>) -> Option<Box<ListNode>> = |vec_num: Vec<i32>| {
            let mut dummy = Box::new(ListNode::new(0));
            let mut current = &mut dummy;
            for x in vec_num {
                let new_node = Box::new(ListNode::new(x));
                current.next = Some(new_node);
                current = current.next.as_mut().unwrap();
            }
            dummy.next
        };

        let l1 = get_test_number(vec![9]);
        let l2 = get_test_number(vec![1,9,9,9,9,9,9,9,9,9]);
        let result = get_test_number(vec![0,0,0,0,0,0,0,0,0,0,0,1]);
        assert_eq!(Solution::add_two_numbers_v1(l1, l2), result);
    }
}