
//
// Under MIT license
//

use id_tree::*;
use std::{error::Error};

use crate::{walk_tree::{WalkActions, Accumulator, Accumulateable}};

const CLOSE_BRACKETS: char = ')';
const OPEN_BRACKET: &str = "(";
const SPACE: &str = " ";

 pub struct Tree2String {
    tree: Tree<String>,
    doube_leaf: bool
}

impl WalkActions for Tree2String {

    fn init_walk(&self, _root_node_id: &NodeId, _data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>) {}

    fn finish_trajectory(&self, node_id: &NodeId, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>) -> Result<(), Box<dyn Error>> {

        // if the tree is a double leaf tree (constituency) then 
        let node_data = self.tree.get(node_id)?.data();
        match self.doube_leaf {
            true => data.push_item(Box::new(format!("{}{}", SPACE.to_string(), node_data))),
            false => data.push_item(Box::new(format!("{}{}{}{}", SPACE.to_string(), OPEN_BRACKET.to_string(), node_data, CLOSE_BRACKETS.to_string())))
        };
        Ok(())
    }

    fn on_node(&self, node_id: &NodeId, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>) -> Result<(), Box<dyn Error>> {

        let pad = if data.check_is_empty() { "" } else { " " };
        let node_data = self.tree.get(node_id)?.data();
        data.push_item(Box::new(format!("{}{}{}", pad.to_string(), OPEN_BRACKET.to_string(), node_data)));
        Ok(())
    }

    fn on_child() {
        todo!()
    }

    fn finish_recursion() {
        todo!()
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

    fn walk(&self, item: Option<&NodeId>, constituency: &mut String) -> Result<(), Box<dyn Error>> {

        if item.is_none() {

            // handle first iteration over tree
            let root_node_id: &NodeId = self.tree.root_node_id().ok_or("input tree is empty")?;
            
            self.walk(Some(root_node_id), constituency)?;
            
            return Ok(());
        }

        // first print the data of this node
        let node_id = item.unwrap();
        let children_ids: Vec<&NodeId> = self.tree.children_ids(node_id)?.collect();

        // if got here, handle the case of a leaf
        if children_ids.is_empty() {
            let node_data = self.tree.get(node_id)?.data();
            match self.doube_leaf {
                true => *constituency += &format!("{}{}", SPACE.to_string(), node_data),
                false => *constituency += &format!("{}{}{}{}", SPACE.to_string(), OPEN_BRACKET.to_string(), node_data, CLOSE_BRACKETS.to_string())
            };

            return Ok(());
        }
        
        // not padding the root with left side space
        let pad = if constituency.is_empty() { "" } else { " " };
        let node_data = self.tree.get(node_id)?.data();
        *constituency += &format!("{}{}{}", pad.to_string(), OPEN_BRACKET.to_string(), node_data);
        
        // do DFS for the children of the current node_id, that has at least one child
        for child_id in children_ids {
            self.walk(Some(child_id), constituency)?;
        }

        // here a node case is continuing its recursive calling, 
        // so we need to pad with another bracket that signals
        // the end of the sub tree 
        *constituency += &format!("{}", CLOSE_BRACKETS.to_string());
        Ok(())
        
    }

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