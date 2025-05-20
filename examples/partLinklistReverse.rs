// 反转部分链表
// 输入: head = [1,2,3,4,5], left = 2, right = 4
// 输出: [1,4,3,2,5]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn reverse_between(
    mut head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut prev = &mut dummy;
    for _ in 1..left {
        prev = prev.next.as_mut().unwrap();
    }
    let mut curr = prev.next.take();
    let mut next = None;
    for _ in left..=right {
        if let Some(mut node) = curr {
            let tmp = node.next.take();
            node.next = next;
            next = Some(node);
            curr = tmp;
        }
    }
    // prev.next 现在应该指向反转后的子链表头
    let mut tail = &mut prev.next;
    for _ in left..=right {
        if let Some(node) = tail {
            *tail = node.next.take();
        }
    }
    prev.next = next;
    let mut p = prev;
    while p.next.is_some() {
        p = p.next.as_mut().unwrap();
    }
    p.next = curr;
    dummy.next
}

// 辅助函数：将 vector 转为链表
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = current;
        current = Some(node);
    }
    current
}

// 辅助函数：将链表转为 vector
fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    while let Some(node) = head {
        vec.push(node.val);
        head = node.next;
    }
    vec
}

fn main() {
    let head = vec_to_list(vec![1, 2, 3, 4, 5]);
    let left = 2;
    let right = 4;
    let result = reverse_between(head, left, right);
    println!("{:?}", list_to_vec(result)); // 输出: [1, 4, 3, 2, 5]
}
