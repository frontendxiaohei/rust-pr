use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreeNode {
    id: i32,
    parent_id: Option<i32>,
    name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TransferedData {
    id: i32,
    parent_id: Option<i32>,
    children: Vec<TransferedData>
}


fn arr2tree(arr: Vec<TreeNode>) -> TransferedData {
    let mut map: HashMap<i32, TransferedData> = HashMap::new();
    let mut root_id = None;
    
    for node in arr.iter() {
        let node_data = TransferedData {
            id: node.id,
            parent_id: node.parent_id,
            children: Vec::new()
        };
        
        if node.parent_id.is_none() {
            root_id = Some(node.id);
        }
        
        map.insert(node.id, node_data);
    }
    
    for node in arr.iter() {
        if let Some(parent_id) = node.parent_id {
            // 如果有父节点，将当前节点添加到父节点的children中
            if let Some(current) = map.get(&node.id).cloned() {
                if let Some(parent) = map.get_mut(&parent_id) {
                    parent.children.push(current);
                }
            }
        }
    }
    
    root_id.and_then(|id| map.get(&id).cloned())
        .unwrap_or_else(|| TransferedData {
            id: 0,
            parent_id: None,
            children: Vec::new()
        })
}


// 递归版本
fn arr2tree_recursive(arr: Vec<TreeNode>) -> TransferedData {
    let root = arr.iter().find(|node| node.parent_id.is_none())
        .expect("No root node found");
    
    let mut root_data = TransferedData {
        id: root.id,
        parent_id: None,
        children: Vec::new()
    };
    
    build_children(&mut root_data, &arr);
    
    root_data
}

// 递归查找并构建子节点
fn build_children(parent: &mut TransferedData, arr: &[TreeNode]) {
    // 找到所有直接子节点
    let children = arr.iter()
        .filter(|node| node.parent_id == Some(parent.id));
    
    // 为每个子节点递归构建子树
    for child in children {
        let mut child_data = TransferedData {
            id: child.id,
            parent_id: child.parent_id,
            children: Vec::new()
        };
        
        // 递归构建子节点的子树
        build_children(&mut child_data, arr);
        
        // 将构建好的子树添加到父节点
        parent.children.push(child_data);
    }
}


fn main() {
    let arr:Vec<TreeNode> = vec![
        TreeNode {id:1, parent_id:None, name:"1".to_string()},
        TreeNode {id:2, parent_id:Some(1), name:"2".to_string()},
        TreeNode {id:3, parent_id:Some(1), name:"3".to_string()},
        TreeNode {id:4, parent_id:Some(2), name:"4".to_string()},
        TreeNode {id:5, parent_id:Some(3), name:"5".to_string()},
        TreeNode {id:6, parent_id:Some(3), name:"6".to_string()},
        TreeNode {id:7, parent_id:Some(6), name:"7".to_string()},
    ];
    
    println!("非递归：");
    let tree = arr2tree(arr.clone());
    let json = serde_json::to_string_pretty(&tree).unwrap();
    println!("{}", json);
    
    println!("递归：");
    let tree_recursive = arr2tree_recursive(arr);
    let json_recursive = serde_json::to_string_pretty(&tree_recursive).unwrap();
    println!("{}", json_recursive);
}