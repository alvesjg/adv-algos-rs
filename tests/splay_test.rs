use adv_algo_rs::SplayTree;

#[test]
fn test_new_splay_tree(){
        let mut st: SplayTree<i32> = SplayTree::new();
        st.insert(&mut 10);
        assert!(st.contains(&mut 10));
    }