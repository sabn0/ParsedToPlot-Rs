
//
// Under MIT license
//

// A trait to get an iterator over the sub-tree-children-ids a node has
// using the -id-tree children impl. Available within crate.
pub mod sub_tree_children {

    use std::collections::HashMap;
    use std::error::Error;
    use id_tree::{Tree, NodeId};

    pub(in crate) trait SubChildren {
        fn is_leaf(&self, node_id: &NodeId) -> Result<bool, Box<dyn Error>>;
        fn get_sub_children(&mut self, as_leaves: bool) -> Result<HashMap<NodeId, usize>, Box<dyn Error>>;
    }

    impl SubChildren for Tree<String> {

        // 
        // A method that checks if a node is a leaf (i.e has no children)
        // Input is a &NodeId. Returns a true result if leaf, or false result otherwise.
        // 
        // This method is used via get_sub_children method. 
        // 
        fn is_leaf(&self, node_id: &NodeId) -> Result<bool, Box<dyn Error>> {
            let children_node_ids  = self.children_ids(node_id)?;
            return Ok(children_node_ids.peekable().peek().is_none())
        }

        //
        // A method that given a tree returns a mapping between node_id and number of children in 
        // the node's sub tree, from node to leaves. The method receives a boolean parameter as_leaves.
        // The method treats leaves as having 1 child (themselves).
        // 
        // When as_leaves is true : counts only the leaves in each node's sub tree.
        // When as_leaves is false: counts all the nodes in some node's sub tree, from node to leaves, including the node itself.
        // 
        fn get_sub_children(&mut self, as_leaves: bool) -> Result<HashMap<NodeId, usize>, Box<dyn Error>> {

            let root_id = match self.root_node_id() {
                Some(root_id) => root_id,
                None => panic!("self tree was not initialized, no root id")
            };

            let account_for_node = !as_leaves as usize;
            let mut map: HashMap<NodeId, usize> = HashMap::new();
            let post_order_iter = self.traverse_post_order_ids(root_id)?;
            for node_id in post_order_iter {

                // this is a post order traversal, so I add the leaves to the map first,
                // then I add them to their parents counts in O(1) time.
                let node_id_copy = node_id.clone();
                if self.is_leaf(&node_id).unwrap() {
                    map.insert(node_id_copy, 1);
                } else {
                    map.insert(node_id_copy, account_for_node);
                    let vec: Vec<&NodeId> = self.children_ids(&node_id).unwrap().collect();
                    for child in vec {
                        let prev_calc = map.get(child).unwrap().clone();
                        *map.get_mut(&node_id).unwrap() += prev_calc;
                    }
                }
            }
            
            Ok(map)

        }


    }

}

#[cfg(test)]
mod tests {

    use crate::generic_traits::generic_traits::String2StructureBuilder;
    use crate::string_2_tree::String2Tree;
    use super::sub_tree_children::SubChildren;
    use id_tree::{NodeId, PreOrderTraversalIds};
    use std::collections::HashMap;

    fn sub_children_template(example: &str, golden: HashMap<&str, i32>, as_leaves: bool) {

        let mut sequence = String::from(example);
        let mut string2tree: String2Tree = String2StructureBuilder::new();
        string2tree.build(&mut sequence).unwrap();
        
        let mut tree = string2tree.get_structure();
        let n_sub_leaves = match tree.get_sub_children(as_leaves) {
            Ok(n_sub_leaves) => n_sub_leaves,
            Err(e) => panic!("{}", e)
        };

        let mut iter: PreOrderTraversalIds<String> = tree.traverse_pre_order_ids(tree.root_node_id().unwrap()).unwrap();
        while let Some(node_id) = iter.next() {

            let node = tree.get(&node_id).unwrap().data().as_str();

            let node_prediction_n_leaves = match n_sub_leaves.get(&node_id) {
                Some(node_prediction_n_leaves) => *node_prediction_n_leaves as i32,
                None => panic!("missed nodes within tree")
            };
            
            let node_gold_n_leaves = golden.get(node).unwrap();
            assert_eq!(node_prediction_n_leaves, *node_gold_n_leaves);

        }
    }

    #[test]
    fn as_leaves_validation() {

        let example = "(0 (1 (2) (3 (4) (5))))";
        let golden = HashMap::from([
            ("0", 3), ("1", 3), ("2", 1), ("3", 2), ("4", 1), ("5", 1)
        ]);

        sub_children_template(example, golden, true);

    }

    #[test]
    fn not_as_leaves_validation_1() {

        let example = "(0 (1 (2) (3 (4) (5))))";
        let golden = HashMap::from([
            ("0", 6), ("1", 5), ("2", 1), ("3", 3), ("4", 1), ("5", 1)
        ]);

        sub_children_template(example, golden, false);

    }

    #[test]
    fn not_as_leaves_validation_2() {

        let example = "(0 (1 (2)) (3 (4) (5)))";
        let golden = HashMap::from([
            ("0", 6), ("1", 2), ("2", 1), ("3", 3), ("4", 1), ("5", 1)
        ]);

        sub_children_template(example, golden, false);
    }

    #[test]
    fn is_leaf_test() {

        let mut string2tree: String2Tree = String2StructureBuilder::new();
        let mut constituency = String::from("(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
        string2tree.build(&mut constituency).unwrap();

        let tree = string2tree.get_structure();
        let root_node_id: &NodeId = tree.root_node_id().unwrap();
        let is_leaf_result: bool = tree.is_leaf(root_node_id).unwrap();
        
        assert_eq!(is_leaf_result, false);

    }

}
