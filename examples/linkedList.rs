// 单链表节点定义
#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

#[derive(Debug)]
pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // 增：在链表头插入节点
    pub fn insert(&mut self, val: i32) {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    // 查：查找值为val的节点
    pub fn find(&self, val: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.val == val {
                return true;
            }
            current = &node.next;
        }
        false
    }

    // 改：将第一个值为old_val的节点改为new_val
    pub fn update(&mut self, old_val: i32, new_val: i32) -> bool {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.val == old_val {
                node.val = new_val;
                return true;
            }
            current = &mut node.next;
        }
        false
    }

    // 删：删除第一个值为val的节点
    pub fn delete(&mut self, val: i32) -> bool {
        let mut current = &mut self.head;
        loop {
            match current {
                None => return false,
                Some(node) if node.val == val => {
                    *current = node.next.take();
                    return true;
                },
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }

    // 打印链表
    pub fn print(&self) {
        let mut current = &self.head;
        let mut first = true;
        print!("[");
        while let Some(node) = current {
            if !first {
                print!(", ");
            } else {
                first = false;
            }
            print!("{}", node.val);
            current = &node.next;
        }
        println!("]");
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.print(); // [3, 2, 1]
    println!("find 2: {}", list.find(2)); // true
    println!("update 2 to 5: {}", list.update(2, 5)); // true
    list.print(); // [3, 5, 1]
    println!("delete 3: {}", list.delete(3)); // true
    list.print(); // [5, 1]
}
