
//
// Under MIT license
//

use id_tree::*;
use id_tree::InsertBehavior::*;
use id_tree::{Tree, NodeId};
use crate::generic_traits::generic_traits::String2StructureBuilder;

const NODE_DELIMITER: &str = " (";
const ROOT_DELIMITER: &str = "(S";
const ROOT_LABEL: &str = "S";
const EMPTY_TREE: &str = "(S)";
const NULL_TREE: &str = "()";
const CLOSE_BRACKETS: char = ')';

/// A String2Tree object, wrap the Tree-String id_tree object
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
    /// A recursive method that builds a Tree-String object from a constituency string
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
        // with at least 1 closer, the tree can either be a constituency tree
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



#[cfg(test)]
mod tests {

    use crate::generic_traits::generic_traits::String2StructureBuilder;
    use super::String2Tree;
    use id_tree::{Node, PostOrderTraversal, LevelOrderTraversal, PreOrderTraversal};
    
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

}
