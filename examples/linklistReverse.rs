// 反转链表
// 输入：head = [1,2,3,4,5]
// 输出：[5,4,3,2,1]

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

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
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
    let reversed = reverse_list(head);
    println!("{:?}", list_to_vec(reversed)); // 输出: [5, 4, 3, 2, 1]
}



