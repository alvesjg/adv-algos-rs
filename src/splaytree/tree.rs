use crate::splaytree::node::Node;

type NodePair<T> = (Option<Box<Node<T>>>, Option<Box<Node<T>>>);

pub struct SplayTree<T>{
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> SplayTree<T>{
    pub fn new() -> Self{
        Self {root : None}
    }

    pub fn insert(&mut self, key : T){
        if let Some(mut old_root) = self.root.take(){
            let (left,right) = Node::split(old_root,&key);
            // old_root = Node::join(Some(Node::join(left,Some(Box::new(Node::new(key))))),right);
            // self.root = Some(Node::splay(old_root,&key));
            self.root = Some(Node::join(Some(Node::join(left,Some(Box::new(Node::new(key))))),right));
        }
        else{
            self.root = Some(Box::new(Node::new(key)));
        };
    }

    pub fn delete(&mut self, key : &T){
        if let Some(old_root) = self.root.take() {
            let mut splayed_root = Node::splay(old_root,key);

            if splayed_root.key == *key{
                let left_child  = splayed_root.left.take();
                let right_child = splayed_root.right.take();

                self.root = Some(Node::join(left_child,right_child));
            }
            else{
                self.root = Some(splayed_root);
            }
        }
    }

    pub fn contains(&mut self, key : &T) -> bool{
        if let Some(old_root) = self.root.take(){
            let splayed_root = Node::splay(old_root,key);
            self.root = Some(splayed_root);
            return self.root.as_ref().unwrap().key == *key;
        }
        else{
            return false;
        }
        
    }
}