#[derive(Debug)]
// 定义二叉树节点结构
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // 创建新节点
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 递归计算树的深度
    fn depth(&self) -> i32 {
        let left_depth = match &self.left {
            Some(node) => node.depth(),
            None => 0,
        };
        let right_depth = match &self.right {
            Some(node) => node.depth(),
            None => 0,
        };
        1 + std::cmp::max(left_depth, right_depth)
    }

    // 递归打印树的值（中序遍历）
    fn inorder(&self) {
        if let Some(left) = &self.left {
            left.inorder();
        }
        println!("{}", self.value);
        if let Some(right) = &self.right {
            right.inorder();
        }
    }
}

fn main() {
    // 创建一个简单的二叉树
    let mut root = TreeNode::new(1);
    
    // 添加左子树
    let mut left_node = TreeNode::new(2);
    left_node.left = Some(Box::new(TreeNode::new(4)));
    left_node.right = Some(Box::new(TreeNode::new(5)));
    root.left = Some(Box::new(left_node));
    
    // 添加右子树
    let mut right_node = TreeNode::new(3);
    right_node.left = Some(Box::new(TreeNode::new(6)));
    root.right = Some(Box::new(right_node));

    // 打印树的结构
    println!("Tree structure: {:?}", root);
    
    // 计算并打印树的深度
    println!("Tree depth: {}", root.depth());
    
    // 执行中序遍历
    println!("Inorder traversal:");
    root.inorder();
}