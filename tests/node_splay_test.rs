use adv_algo_rs::splaytree::node::Node;

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