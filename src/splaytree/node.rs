pub struct Node<T>{
    pub(crate) key   : T,
    pub(crate) left  : Option<Box<Node<T>>>,
    pub(crate) right : Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(key : T) -> Self{
        Self{ key, left: None, right: None}
    }

    pub fn rotate_left(mut node : Box<Self>) -> Box<Self>{
        if let Some(mut right_child) = node.right.take(){
            node.right = right_child.left.take();
            right_child.left = Some(node);
            return right_child;
        } else{
            return node;
        };
    }

    pub fn rotate_right(mut node: Box<Self>) -> Box<Self>{
        if let Some(mut left_child) = node.left.take() {
            node.left = left_child.right.take();
            left_child.right = Some(node);
            return left_child;
        }
        else{
            return node;
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_rotation() {
        let mut root = Box::new(Node::new(10));
        root.left = Some(Box::new(Node::new(5)));
        
        // Pass the owned Box in, get the new owned Box out
        let new_root = Node::rotate_right(root);

        assert_eq!(new_root.key, 5);
        // Use as_ref() to peek inside the Option<Box<Node<T>>>
        assert_eq!(new_root.right.as_ref().unwrap().key, 10);
    }
}