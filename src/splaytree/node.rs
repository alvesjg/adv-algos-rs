pub struct Node<T>{
    pub key   : T,
    pub left  : Option<Box<Node<T>>>,
    pub right : Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> Node<T> {
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

    pub fn rotate_left_left(mut node: Box<Self>) -> Box<Self>{
        node = Node::rotate_left(node);
        node = Node::rotate_left(node);
        return node;
    }

    pub fn rotate_right_right(mut node: Box<Self>) -> Box<Self>{
        node = Node::rotate_right(node);
        node = Node::rotate_right(node);
        return node;
    }

    pub fn rotate_left_right(mut node: Box<Self>) -> Box<Self>{
        if let Some(left_child) = node.left.take(){
            node.left = Some(Node::rotate_left(left_child));
            return Node::rotate_right(node);
            
        }
        return node;
    }

    pub fn rotate_right_left(mut node: Box<Self>) -> Box<Self>{
        if let Some(right_child) = node.right.take(){
            node.right = Some(Node::rotate_right(right_child));
            return Node::rotate_left(node);
        }
        return node;
    }

    pub fn splay(mut root : Option<Box<Self>>, key : &mut T) -> Option<Box<Self>> {
        if let Some(old_root) = root.take(){
            let mut dir : i32 = 0;
            return Some(Self::priv_splay(old_root,key,0, &mut dir));
        }
        else{
            return None;
        }
    }

    pub fn priv_splay(mut node : Box<Self>, key : &mut T , depth :usize, dir : &mut i32) -> Box<Self>{
        if node.key == *key{
            return node;
        }
        if node.key > *key{ // Node is in the left subtree
            if let Some(left_node) = node.left.take(){
                let left_splayed = Self::priv_splay(left_node,key,depth+1,dir);
                if left_splayed.key == *key{
                    *dir = -1;
                    node.left = Some(left_splayed);
                    if depth == 0{
                        return Self::rotate_right(node);
                    }
                }
                else{
                    node.left = Some(left_splayed);
                    if *dir == -1 {
                        return Self::rotate_right_right(node);
                    }
                    else{
                        return Self::rotate_left_right(node);
                    }
                }
            }
            else{ // there is no node with such key in the tree
                *key = node.key.clone();
                return node;
            }
        }
        else{ // Node is in the right subtree
            if let Some(right_node) = node.right.take(){
                let right_splayed = Self::priv_splay(right_node,key,depth+1,dir);
                if right_splayed.key == *key{
                    *dir = 1;
                    node.right = Some(right_splayed);
                    if depth == 0{
                        return Self::rotate_left(node);
                    }
                }
                else{
                    node.right = Some(right_splayed);
                    if*dir == -1{
                        return Self::rotate_right_left(node);
                    }
                    else{
                        return Self::rotate_left_left(node);
                    }
                }
            }
            else{ // there is no node with such key in the tree
                *key = node.key.clone();
                return node;
            }
        }
        return node;
    }

    pub fn join(left_node : Option<Box<Self>>, right_node : Option<Box<Self>>) -> Box<Self> {
        todo!();
    }

    pub fn split(mut node : Box<Self>, key: &mut T)-> (Option<Box<Self>>,Option<Box<Self>>) {
        todo!();
    }

}