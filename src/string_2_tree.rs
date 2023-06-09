
//
// Under MIT license
//

use std::error::Error;
use id_tree::*;
use id_tree::InsertBehavior::*;
use id_tree::{Tree, NodeId};
use crate::generic_traits::generic_traits::String2StructureBuilder;

const NODE_DELIMITER: &str = " ";
const CLOSE_BRACKETS: char = ')';
const OPEN_BRACKETS: char = '(';

/// A String2Tree struct, mainly holds the tree object. This type will implement the String2StructureBuilder, 
/// with a constituency String as Input and a made Tree-String- as output.
pub struct String2Tree {
    tree: Tree<String>,
    parent_node_id: Option<NodeId>,
    level_balance: i32
}

impl String2Tree {

    // A method that updates the current parent node in the parsing process.
    // This method isn't called directly as users, not exposed.
    fn update_parent(&mut self, item_id: &NodeId, closers: usize) -> Result<(), Box<dyn Error>> {

        if closers > 0 {
            let ancestors_ids = self.tree.ancestor_ids(item_id)?.collect::<Vec<&NodeId>>();
            let parent_node_id = ancestors_ids
            .get(closers-1)
            .expect("inconsistent number of closers and ancestors for node id")
            .to_owned()
            .to_owned();        
            self.parent_node_id = Some(parent_node_id);
        } else {
            self.parent_node_id = None;
        }

        Ok(())
    }


}

impl String2StructureBuilder for String2Tree {

    type Input = String;
    type Out = Tree<String>;

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
            parent_node_id: None,
            level_balance: 0,           // a sanity variable during the construction stage
        }
    }

    ///
    /// Get a copy of a tree (should be called after build)
    /// 
    fn get_structure(&self) -> Self::Out {
        assert!(self.tree.root_node_id().is_some(), "get_structure() should be called after using build(...)");
        return self.tree.clone();
    }

    /// 
    /// A recursive method that builds a mutable Tree-String- structure from a constituency string
    /// Returns Ok if the process was succesful (error otherwise)
    ///
    /// # Examples
    /// 
    /// ```
    /// use parsed_to_plot::String2Tree;
    /// use parsed_to_plot::String2StructureBuilder;
    /// 
    /// let mut constituency = String::from("(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
    /// let gold_root_data = "S";
    /// 
    /// let mut string2tree: String2Tree = String2StructureBuilder::new();
    /// if let Err(e) = string2tree.build(&mut constituency) {
    ///     panic!("{}", e);
    /// }
    /// 
    /// let mut tree = string2tree.get_structure();
    /// let prediction_root_data = tree.get(tree.root_node_id().unwrap()).unwrap().data();
    /// 
    /// assert_eq!(prediction_root_data, gold_root_data);
    /// ```
    /// 
    fn build(&mut self, input: &mut Self::Input) -> Result<(), Box<dyn Error>> {

        // If the string is empty the algoritm has finished
        if input.is_empty() {
            assert_eq!(self.level_balance, 0, "number of closers and openers don't match");
            return Ok(());
        }

        // If constituency does not have open delimiter it's the last iteration, (work on right).
        // else, split by the delimeter (work on left, leave right for next iteration).
        let (left, mut right) = match input.split_once(NODE_DELIMITER) {
            Some((left, right)) => (left.trim().to_owned(), right.trim().to_owned()),
            None => (input.trim().to_owned(), "".to_owned())
        };

        // A closure to insert a new node to the tree
        let mut add_node = |node_str: &str, parent_id: &Option<&NodeId>| -> Result<NodeId, Box<dyn Error>> {

            // create a new node from the input str
            let node_string = String::from(node_str);
            let new_node = Node::new(node_string);

            // add the node to the tree. This can either be the root of the tree or another node
            let new_node_id = match parent_id {
                // case of an inner node, parent_id already exists. Add new node under parent.
                Some(parent_id) => self.tree.insert(new_node, UnderNode(parent_id))?,
                // case of a root node, parent_id is None. Add new node as root
                None => self.tree.insert(new_node, AsRoot)?
            };

            Ok(new_node_id)
        };

        // we have done a split by " ". We handle the left size and keep the right to next iter
        // we will validate and match the number of openers and closers in left. 
        let mut closers = left.matches(CLOSE_BRACKETS).count();
        let openers = left.matches(OPEN_BRACKETS).count();
        assert!(openers <= 1, "invalid input structure, consecutive open brackets");
        assert!(openers > 0 || closers > 0, "found a node without matching parenthesis");
        self.level_balance += openers as i32 - closers as i32;
        match closers {
            0 => {

                // If closers = 0, it is an opening node, "(A" . 
                // I asserted the number of openings to validate the structure.
                // Create a new node and add to the tree
                let node_str = left.trim_matches(OPEN_BRACKETS);
                let parent_id = self.parent_node_id.as_ref();
                let new_node_id = add_node(node_str, &parent_id)?;

                // make the new node the parent for next iteration
                self.parent_node_id = Some(new_node_id);

            },
            _ => {
                
                // If closers > 0 , it is a leaf. it can look like "A)" or "(A)", depending on double or singular
                let node_str = left.trim_matches(CLOSE_BRACKETS).trim_matches(OPEN_BRACKETS);
                assert_ne!(node_str, "", "found a null node in input string");

                // Create a new node and add to the tree
                let parent_id = self.parent_node_id.as_ref();
                let new_node_id = add_node(&node_str, &parent_id)?;

                // double or singular leaves change the requested parent for next iteration. In singular leaves,
                // K closures mean that the parent for next iteration is K levels above. In double leaves,
                // K closures mean that the parent for next iteration is K+1 levels above. 
                closers += 1-openers; 

                // ignore the very last closer because there is no global parent beyond the most remote closers
                if right.is_empty() {
                     closers -= 1;
                }
                self.update_parent(&new_node_id, closers)?;               
            }
        }

        self.build(&mut right)?;
        Ok(())
        
    }


}



#[cfg(test)]
mod tests {

    use super::String2Tree;
    use crate::generic_traits::generic_traits::String2StructureBuilder;
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
        
        string2tree.build(&mut constituency).unwrap();
        let tree = string2tree.get_structure();
        let root = tree.root_node_id().unwrap();

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
        let example = "(S (0 (1) (2 (3))))";
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
    fn non_trivial_root() {
        let example = "(0 (1))";
        let golden = vec!["0", "1"];
        string2tree_template(example, golden, "pre");
    }

    #[test]
    #[should_panic(expected = "found a null node in input string")]
    fn null_tree() {
        let example = "()";
        let golden = vec![""];
        string2tree_template(example, golden, "");
    }

    #[test]
    #[should_panic(expected = "number of closers and openers don't match")]
    fn missing_closures() {
        let example = "(S (0 (1";
        let golden = vec!["S", "0", "1"];
        string2tree_template(example, golden, "pre");
    }

    #[test]
    #[should_panic(expected = "found a node without matching parenthesis")]
    fn missing_opening() {
        let example = "S (0 (1";
        let golden = vec!["S", "0", "1"];
        string2tree_template(example, golden, "pre");
    }

    #[test]
    #[should_panic(expected = "inconsistent number of closers and ancestors for node id")]
    fn inconsistent_closers() {
        let example = "(S (0)) (1 2)";
        let golden = vec!["0", "1", "2"];
        string2tree_template(example, golden, "pre");
    }

}
