
//
// Under MIT license
//

use id_tree::*;
use std::{error::Error};

use crate::{walk_tree::{WalkActions, Accumulator}};

const CLOSE_BRACKETS: char = ')';
const OPEN_BRACKET: &str = "(";
const SPACE: &str = " ";

 pub struct Tree2String {
    tree: Tree<String>,
    doube_leaf: bool
}

impl WalkActions for Tree2String {

    fn init_walk(&self, _root_node_id: &NodeId, _data: &mut Box<dyn Accumulator>) {}

    fn finish_trajectory(&self, node_id: &NodeId, data: &mut Box<dyn Accumulator>) -> Result<(), Box<dyn Error>> {

        // if the tree is a double leaf tree (constituency) then 
        let node_data = self.tree.get(node_id)?.data();
        match self.doube_leaf {
            true => data.push_item(&format!("{}", node_data)),
            false => data.push_item(&format!("{}{}{}", OPEN_BRACKET.to_string(), node_data, CLOSE_BRACKETS.to_string()))
        };
        Ok(())
    }

    fn on_node(&self, node_id: &NodeId, _parameters: &mut [f32; 6], data: &mut Box<dyn Accumulator>) -> Result<(), Box<dyn Error>> {

        let node_data = self.tree.get(node_id)?.data();
        data.push_item(&format!("{}{}", OPEN_BRACKET.to_string(), node_data));
        Ok(())
    }

    fn on_child(&self, _child_node_id: &NodeId, _parameters: &mut [f32; 6], _data: &mut Box<dyn Accumulator>) {}

    fn finish_recursion(&self, data: &mut Box<dyn Accumulator>) {
        data.push_item(&format!("{}", CLOSE_BRACKETS.to_string()));
    }



}

#[allow(dead_code)]
impl Tree2String {

    fn new(tree: Tree<String>, double_leaf: bool) -> Self {
        Self {
            tree: tree,
            doube_leaf: double_leaf
        }
    }

    // Tree2String now has WalkActions, so it does not need to have an explicit walk
}


#[cfg(test)]
mod tests {

    use crate::string_2_tree::String2Tree;
    use crate::String2StructureBuilder;
    use crate::tree_2_string::Tree2String;

    #[test]
    fn double_leaf() {

        let example = String::from("(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
        let prediction = inverse_check(example.clone(), true);
        assert_eq!(example, prediction, "failed, original example: {} != prediction: {}", example, prediction);
    } 

    #[test]
    fn single_leaf() {

        let example = String::from("(S (36 (9 (3) (3)) (4 (2) (2))))");
        let prediction = inverse_check(example.clone(), false);
        assert_eq!(example, prediction, "failed, original example: {} != prediction: {}", example, prediction);
    } 

    fn inverse_check(example: String, double_leaf: bool) -> String { 

        // check by building tree and returning to the original input

        // forward
        let mut constituency = example;
        let mut string2tree: String2Tree = String2StructureBuilder::new();
        if let Err(e) = string2tree.build(&mut constituency) {
            panic!("{}", e);
        }
        let tree = string2tree.get_structure();

        // backward
        let mut prediction = String::from("");
        let tree2string = Tree2String::new(tree, double_leaf);
        if let Err(e) = tree2string.walk(None, &mut prediction) {
            panic!("{}", e);
        };

        prediction
    }

}