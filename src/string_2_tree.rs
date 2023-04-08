

use id_tree::*;
use id_tree::InsertBehavior::*;
use id_tree::{Tree, NodeId, NodeIdError};
use std::collections::HashMap;
use std::fmt::Error;
use crate::String2StructureBuilder;

const NODE_DELIMITER: &str = " (";
const ROOT_DELIMITER: &str = "(S";
const ROOT_LABEL: &str = "S";
const EMPTY_TREE: &str = "(S)";
const NULL_TREE: &str = "()";
const CLOSE_BRACKETS: char = ')';

/// A String2Tree object, wrap the Tree<String> id_tree object
pub struct String2Tree {
    tree: Tree<String>,
    parent_node_id: Option<NodeId>,
}

impl String2StructureBuilder<Tree<String>, String> for String2Tree {

    /// 
    /// Initialization of a String2Tree object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use parsed_to_plot::String2Tree;
    /// use parsed_to_plot::String2StructureBuilder;
    /// 
    /// let _string2tree: String2Tree = String2StructureBuilder::new();
    /// ```
    /// 
    fn new() -> Self {
        Self {
            tree: Tree::new(),
            parent_node_id: None
        }
    }

    ///
    /// Get a copy of a tree (should be called after build)
    /// 
    fn get_structure(&self) -> Tree<String> {
        return self.tree.clone();
    }

    /// 
    /// A recursive method that builds a Tree<String> object from a constituency string
    /// Returns Ok if the process was succesful (error otherwise)
    ///
    /// # Examples
    /// 
    /// ```
    /// use parsed_to_plot::String2Tree;
    /// use parsed_to_plot::String2StructureBuilder;
    /// 
    /// let mut string2tree: String2Tree = String2StructureBuilder::new();
    /// let mut constituency = String::from("(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
    /// let _res = match string2tree.build(&mut constituency) {
    ///     Ok(_res) => Ok(_res),
    ///     Err(e) => Err(e)
    /// };
    /// let mut tree = string2tree.get_structure();
    /// 
    /// assert_eq!("S", tree.get(tree.root_node_id().unwrap()).unwrap().data());
    /// ```
    /// 
    fn build(&mut self, input: &mut String) -> Result<(), String> {

        // if the string is empty the algoritm has finished
        if input.is_empty() {
            return Ok(());
        }

        // if constituency does not have open delimiter it's the last iteration
        // else, split by the delimeter
        let (left, right) = match input.split_once(NODE_DELIMITER) {
            Some((left, right)) => (left.trim(), right.trim()),
            None => (input.trim(), "")
        };
        let mut right = right.to_string();

        // if root is found, add to tree and update parent, then recurse . else, panic
        match left {
            ROOT_DELIMITER | EMPTY_TREE => {
                let node_content = String::from(ROOT_LABEL);
                let node = Node::new(node_content);                
                let node_id = match self.tree.insert(node, AsRoot) {
                    Ok(node_id) => node_id,
                    Err(_e) => return Err(format!("could not insert root to tree"))
                };
                self.parent_node_id = Some(node_id);
                self.build(&mut right)?;
                return Ok(());
            }, // the case of empty tree is valid
            NULL_TREE => return Err(format!("The input tree is null and not valid")),
            _ => {
                if self.parent_node_id.is_none() {
                    panic!("The input did not start with root")
                }
            }
        }

        // number of closers in left determines action:
        // 0 closers means treating as node.
        // with at least 1 closer, the tree can either be a either a constituency tree
        // with "double leafs", or a regular tree with only one leaf.
        let mut closers = left.matches(CLOSE_BRACKETS).count() as i32;
        match closers.cmp(&0) {
            std::cmp::Ordering::Greater => {

                let split_vec: Vec<&str> = left.trim_matches(CLOSE_BRACKETS).split(' ').collect();
                
                // handle cases of one leaf or two leafs using match guard for creating the parent in both cases
                // but only creating a child in the two leaves case
                let child_id = match split_vec.len() {
                    n @ 1..=2 => {
                        
                        let upper_leaf = split_vec[0].trim();
                        let parent_id = match self.parent_node_id.as_ref() {
                            Some(parent_id) => parent_id,
                            None => return Err(format!("could not find ancestor for node that is not root"))
                        };
                        
                        let child_node_content = String::from(upper_leaf);
                        let child_node = Node::new(child_node_content);
                        let child_id = match self.tree.insert(child_node, UnderNode(parent_id)) {
                            Ok(child_id) => child_id,
                            Err(_e) => return Err(format!("could not insert child under parent"))
                        };
                        
                        if n == 2 {
                            let lower_leaf = split_vec[1].trim();
                            let grand_child_node_content = String::from(lower_leaf);
                            let grand_child_node = Node::new(grand_child_node_content);
                            let _grand_child_id = self.tree.insert(grand_child_node, UnderNode(&child_id)
                            ).unwrap(); // we know this is a good child_id from the upper case
                        }
                        child_id

                    },
                    _ => panic!("leaf input has an invalid structure")
                };

                if right.is_empty() {
                    closers = closers -1;
                }
                self.update_parent(&child_id, closers)?;

            },
            std::cmp::Ordering::Equal => {
                
                if right.is_empty() {
                    panic!("Did not find trailing closers")
                }

                let parent_id = match self.parent_node_id.as_ref() {
                    Some(parent_id) => parent_id,
                    None => panic!("could not find ancestor for node that is not root")
                };

                let child_node_content = String::from(left.to_string());
                let child_node = Node::new(child_node_content);
                let child_node_id: NodeId = self.tree.insert(child_node, UnderNode(parent_id)
                ).unwrap(); // we know this is a good child_id from the upper case
                self.parent_node_id = Some(child_node_id);

            },
            std::cmp::Ordering::Less => panic!("found less closers than openings")
        }

        self.build(&mut right)?;
        Ok(())
        
    }


    /// A method that updates the current parent node in the parsing process.
    /// No need to call this method directly as users.
    fn update_parent(&mut self, item_id: &NodeId, n: i32) -> Result<(), String> {

        let mut ancestors_ids_iterator = match self.tree.ancestor_ids(item_id) {
            Ok(ancestors_ids_iterator) => ancestors_ids_iterator,
            Err(_e) => return Err(format!("could not find ancestors for node_id"))
        };

        let mut parent_node_id: Option<NodeId> = None::<NodeId>;
        for _i in 1..=n {
            
            parent_node_id = match ancestors_ids_iterator.next() {
                Some(parent_node_id) => Some(parent_node_id.to_owned()),
                None => return Err(format!("inconsistent number of closers and ancestors for node id"))
            };
        }        
        self.parent_node_id = parent_node_id;
        Ok(())

    }



}


/// A trait to get an iterator over the sub-children-ids a node has
/// using the -id-tree children impl
pub trait SubChildren {
    fn is_leaf(&self, node_id: &NodeId) -> Result<bool, Error>;
    fn get_sub_children(&mut self, as_leaves: bool) -> Result<HashMap<NodeId, usize>, NodeIdError>;
}

impl SubChildren for Tree<String> {

    /// 
    /// A method that checks if a node is a leaf (i.e has no children)
    /// Input is a &NodeId. Returns a true result if leaf, or false result otherwise.
    /// 
    /// This method is used via get_sub_children method, but can be used without it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use id_tree::NodeId;
    /// use parsed_to_plot::{String2Tree, SubChildren, String2StructureBuilder};
    /// 
    /// let mut string2tree: String2Tree = String2StructureBuilder::new();
    /// let mut constituency = String::from("(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
    /// let _res = match string2tree.build(&mut constituency) {
    ///     Ok(_res) => Ok(_res),
    ///     Err(e) => Err(e)
    /// };
    /// let tree = string2tree.get_structure();
    /// let root_node_id: &NodeId = tree.root_node_id().unwrap();
    /// let is_leaf_result: bool = tree.is_leaf(root_node_id).unwrap();
    /// 
    /// assert_eq!(is_leaf_result, false);
    /// ```
    /// 
    fn is_leaf(&self, node_id: &NodeId) -> Result<bool, Error> {
        
        let children_node_ids  = match self.children_ids(node_id) {
            Ok(sub_children_ids) => sub_children_ids,
            Err(e) => panic!("error happened in children extraction: {}", e)
        };
        return Ok(children_node_ids.peekable().peek().is_none())
    }

    ///
    /// A method that given a tree return a mapping between node id and number of children in 
    /// each node's sub tree, from node to leaves. The method receives a boolean parameter as_leaves.
    /// 
    /// The method accounts for leaves as having 1 child (themselves).
    /// When as_leaves is true : counts only the leaves in each node's sub tree.
    /// When as_leaves is false: counts all the nodes in some node's sub tree, from node to leaves,
    /// including the node itself.
    /// 
    /// # Examples
    /// 
    /// ## As_leaves
    /// 
    /// ```
    /// use parsed_to_plot::{String2Tree, SubChildren, String2StructureBuilder};
    /// use id_tree::PreOrderTraversalIds;
    /// 
    /// let mut string2tree: String2Tree = String2StructureBuilder::new();
    /// let mut sequence = String::from("(S (0 (1) (2 (3) (4))))");
    /// let _res = match string2tree.build(&mut sequence) {
    ///     Ok(_res) => Ok(_res),
    ///     Err(e) => Err(e)
    /// };
    /// let mut tree = string2tree.get_structure();
    /// 
    /// // an example for as_leaves :
    /// let as_leaves_node2num = match SubChildren::get_sub_children(&mut tree, true) {
    ///     Ok(as_leaves_node2num) => as_leaves_node2num,
    ///     Err(_e) => panic!("could not infer sub children from tree")
    /// };
    /// 
    /// let mut iter: PreOrderTraversalIds<String> = tree.traverse_pre_order_ids(tree.root_node_id().unwrap()).unwrap();
    /// // checking the first 3 nodes in pre_order:
    /// assert_eq!(*as_leaves_node2num.get(&iter.next().unwrap()).unwrap(), 3);
    /// assert_eq!(*as_leaves_node2num.get(&iter.next().unwrap()).unwrap(), 3);
    /// assert_eq!(*as_leaves_node2num.get(&iter.next().unwrap()).unwrap(), 1);
    /// ```
    /// 
    /// ## not As_leaves
    /// 
    /// ```
    /// use parsed_to_plot::{String2Tree, SubChildren, String2StructureBuilder};
    /// use id_tree::PreOrderTraversalIds;
    /// 
    /// let mut string2tree: String2Tree = String2StructureBuilder::new();
    /// let mut sequence = String::from("(S (0 (1) (2 (3) (4))))");
    /// let _res = match string2tree.build(&mut sequence) {
    ///     Ok(_res) => Ok(_res),
    ///     Err(e) => Err(e)
    /// };
    /// let mut tree = string2tree.get_structure();
    /// 
    /// // an example for not as_leaves :
    /// let not_as_leaves_node2num = match SubChildren::get_sub_children(&mut tree, false) {
    ///     Ok(not_as_leaves_node2num) => not_as_leaves_node2num,
    ///     Err(_e) => panic!("could not infer sub children from tree")
    /// };
    /// 
    /// let mut iter: PreOrderTraversalIds<String> = tree.traverse_pre_order_ids(tree.root_node_id().unwrap()).unwrap();
    /// // checking the first 4 nodes in pre_order:
    /// assert_eq!(*not_as_leaves_node2num.get(&iter.next().unwrap()).unwrap(), 6);
    /// assert_eq!(*not_as_leaves_node2num.get(&iter.next().unwrap()).unwrap(), 5);
    /// assert_eq!(*not_as_leaves_node2num.get(&iter.next().unwrap()).unwrap(), 1);
    /// assert_eq!(*not_as_leaves_node2num.get(&iter.next().unwrap()).unwrap(), 3);
    /// 
    /// ```
    /// 
    fn get_sub_children(&mut self, as_leaves: bool) -> Result<HashMap<NodeId, usize>, NodeIdError> {

        let root_id = match self.root_node_id() {
            Some(root_id) => root_id,
            None => panic!("self tree was not initialized, no root id")
        };

        let account_for_node = !as_leaves as usize;
        let mut map: HashMap<NodeId, usize> = HashMap::new();
        let post_order_iter = self.traverse_post_order_ids(root_id).unwrap();
        for node_id in post_order_iter {

            // this is a post order traversal, so I add the leaves to the map (if have no children)
            // then I add them to their parents counts in O(1).

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



#[cfg(test)]
mod tests {


    use crate::String2StructureBuilder;
    use super::String2Tree;
    use super::SubChildren;
    use super::{Node, NodeId};
    use id_tree::{PostOrderTraversal, LevelOrderTraversal, PreOrderTraversal, PreOrderTraversalIds};
    use std::collections::HashMap;
    

    enum Traversal<'a> {
        Pre(PreOrderTraversal<'a, String>),
        Level(LevelOrderTraversal<'a, String>),
        Post(PostOrderTraversal<'a, String>)
    }

    impl<'a> Iterator for Traversal<'a> {
        type Item = &'a Node<String>;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Traversal::Pre(t) => t.next(),
                Traversal::Post(t) => t.next(),
                Traversal::Level(t) => t.next(),
            }
        }
    }

    fn string2tree_template(example: &str, golden: Vec<&str>, order: &str) {

        let mut constituency = String::from(example);
        let mut string2tree: String2Tree = String2StructureBuilder::new();
        
        let _result = match string2tree.build(&mut constituency) {
            Ok(_result) => {},
            Err(e) => panic!("{}", e)
        };

        let tree = string2tree.get_structure();
        let root = match tree.root_node_id() {
            Some(root) => root,
            None => panic!("did not find root")
        };

        let mut iter = match order {
            "pre" => Traversal::Pre(tree.traverse_pre_order(root).unwrap()),
            "level" => Traversal::Level(tree.traverse_level_order(root).unwrap()),
            "post" => Traversal::Post(tree.traverse_post_order(root).unwrap()),
            _ => panic!("incorrect order given")
        };


        let mut prediction = Vec::new();
        while let Some(ref mut next) = iter.next() {
            prediction.push(next.data());
        }

        assert_eq!(golden, prediction);

    }


    #[test]
    fn level_order() {
        let example = "(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))";
        let golden = vec!["S", "NP", "VP", "det", "N", "V", "NP", "The", "people", "watch", "det", "N", "the", "game"];
        string2tree_template(example, golden, "level");
    }

    #[test]
    fn pre_order() {
        let example = "(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))";
        let golden = vec!["S", "NP", "det", "The", "N", "people", "VP", "V", "watch", "NP", "det", "the", "N", "game"];
        string2tree_template(example, golden, "pre");
    }

    #[test]
    fn math_mode() {
        let example = "(S (0 (1) (2 (3)))";
        let golden = vec!["S", "0", "1", "2", "3"];
        string2tree_template(example, golden, "pre");
    }

    #[test]
    fn empty_tree() {
        let example = "(S)";
        let golden = vec!["S"];
        string2tree_template(example, golden, "post");
    }

    #[test]
    #[should_panic(expected = "The input tree is null and not valid")]
    fn null_tree() {
        let example = "()";
        let golden = vec![""];
        string2tree_template(example, golden, "");
    }

    #[test]
    #[should_panic(expected = "Did not find trailing closers")]
    fn missing_closures() {
        let example = "(S (NP (det";
        let golden = vec!["S", "NP", "det"];
        string2tree_template(example, golden, "pre");
    }

    #[test]
    #[should_panic(expected = "The input did not start with root")]
    fn missing_root() {
        let example = "(NP (det))";
        let golden = vec!["NP", "det"];
        string2tree_template(example, golden, "pre");
    }

    #[test]
    #[should_panic(expected = "The input did not start with root")]
    fn missing_opening() {
        let example = "S (NP (det";
        let golden = vec!["S", "NP", "det"];
        string2tree_template(example, golden, "pre");
    }

    #[test]
    #[should_panic(expected = "inconsistent number of closers and ancestors for node id")]
    fn inconsistent_closers() {
        let example = "(S (NP)) (det the)";
        let golden = vec!["VP", "V", "Making"];
        string2tree_template(example, golden, "pre");
    }

    fn sub_children_template(example: &str, golden: HashMap<&str, i32>, as_leaves: bool) {

        let mut sequence = String::from(example);
        let mut string2tree: String2Tree = String2StructureBuilder::new();

        let _result = match string2tree.build(&mut sequence) {
            Ok(_result) => {},
            Err(_e) => panic!("error building tree")
        };

        let mut tree = string2tree.get_structure();
        let n_sub_leaves = match tree.get_sub_children(as_leaves) {
            Ok(n_sub_leaves) => n_sub_leaves,
            Err(_e) => panic!("could not calculate number of sub children")
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

        let example = "(S (0 (1) (2 (3) (4))))";
        let golden = HashMap::from([
            ("S", 3), ("0", 3), ("1", 1), ("2", 2), ("3", 1), ("4", 1)
        ]);

        sub_children_template(example, golden, true);

    }

    #[test]
    fn not_as_leaves_validation_1() {

        let example = "(S (0 (1) (2 (3) (4))))";
        let golden = HashMap::from([
            ("S", 6), ("0", 5), ("1", 1), ("2", 3), ("3", 1), ("4", 1)
        ]);

        sub_children_template(example, golden, false);

    }

    #[test]
    fn not_as_leaves_validation_2() {

        let example = "(S (0 (1)) (2 (3) (4)))";
        //let example = "(S (0 (1) (2 (3) (4))))";
        let golden = HashMap::from([
            ("S", 6), ("0", 2), ("1", 1), ("2", 3), ("3", 1), ("4", 1)
        ]);

        sub_children_template(example, golden, false);
    }

    #[test]
    fn is_leaf_test() {

        let mut string2tree: String2Tree = String2StructureBuilder::new();
        let mut constituency = String::from("(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
        let _res = match string2tree.build(&mut constituency) {
                Ok(_res) => Ok(_res),
            Err(e) => Err(e)
        };
        let tree = string2tree.get_structure();
        let root_node_id: &NodeId = tree.root_node_id().unwrap();
        let is_leaf_result: bool = tree.is_leaf(root_node_id).unwrap();
        
        assert_eq!(is_leaf_result, false);

    }



}
