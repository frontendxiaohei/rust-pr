
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}


fn main() {
    let mut root = Node::new(1);

    root.left = Some(Box::new(Node::new(2)));
    root.right = Some(Box::new(Node::new(3)));


    // println!("Inorder traversal{:?}", root);

}

