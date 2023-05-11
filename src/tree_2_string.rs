
//
// Under MIT license
//

use id_tree::*;
use std::{error::Error};

use crate::generic_enums::{Accumulator, Element};
use crate::generic_traits::generic_traits::{WalkActions, WalkTree, Structure2PlotBuilder};

const CLOSE_BRACKETS: &str = ")";
const OPEN_BRACKET: &str = "(";

 pub struct Tree2String {
    tree: Tree<String>,
    double_leaf: bool
}


impl Structure2PlotBuilder<Tree<String>> for Tree2String {

    fn new(structure: Tree<String>) -> Self {

        let root_data = structure.get(structure.root_node_id().unwrap()).unwrap().data();
        let (is_double, _data) = root_data.split_once('-').unwrap();
        let double_leaf = match is_double {
            "1" => true,
            "0" => false,
            _ => panic!("incorrect bin for is double")
        };

        Self {
            double_leaf: double_leaf,
            tree: structure
        }
    }

    fn build(&mut self, _save_to: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}


impl WalkTree for Tree2String {

    fn get_root_element(&self) -> Result<Element, Box<dyn Error>> {
        let root_node_id = self.tree.root_node_id().ok_or("tree is empty")?;
        let root_element_id = Element::NID(root_node_id);
        Ok(root_element_id)
    }

    fn get_children_ids(&self, element_id: Element) -> Result<Vec<Element>, Box<dyn Error>> {
        let node_id = <&NodeId>::try_from(element_id)?;
        let children_ids = self.tree.children_ids(node_id)?.map(|x| Element::NID(x)).collect::<Vec<Element>>();
        return Ok(children_ids)
    }

}

impl WalkActions for Tree2String {

    fn init_walk(&self, _element_id: Element, _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn finish_trajectory(&self, element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn Error>> {

        let node_id = <&NodeId>::try_from(element_id)?;

        // if the tree is a double leaf tree (constituency) then
        let data_vec = <&mut Vec<String>>::try_from(data)?; 
        let node_data = self.tree.get(node_id)?.data();
        match self.double_leaf {
            true => data_vec.push(format!("{}", node_data)),
            false => data_vec.push(format!("{}{}{}", OPEN_BRACKET.to_string(), node_data, CLOSE_BRACKETS.to_string()))
        };
        Ok(())
    }

    fn on_node(&self, element_id: Element, _parameters: &mut [f32; 6], data: &mut Accumulator) -> Result<(), Box<dyn Error>> {

        let node_id = <&NodeId>::try_from(element_id)?;
        let node_data = self.tree.get(node_id)?.data();
        let data_vec = <&mut Vec<String>>::try_from(data)?;
        data_vec.push(format!("{}{}", OPEN_BRACKET.to_string(), node_data));
        Ok(())
    }

    fn on_child(&self, _child_element_id: Element, _parameters: &mut [f32; 6], _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn post_walk_update(&self, _element_id: Element, _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn finish_recursion(&self, data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        let data_vec = <&mut Vec<String>>::try_from(data)?;
        let last = data_vec.pop().unwrap();
        data_vec.push(format!("{}{}", last, CLOSE_BRACKETS.to_string()));
        Ok(())
    }


}




#[cfg(test)]
mod tests {

    use crate::string_2_tree::String2Tree;
    use crate::generic_enums::{Accumulator};
    use crate::generic_traits::generic_traits::WalkTree;
    use crate::{String2StructureBuilder, Structure2PlotBuilder};
    use crate::tree_2_string::Tree2String;

    #[test]
    fn double_leaf() {

        let example = String::from("(1-S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
        let prediction = inverse_check(example.clone());
        assert_eq!(example, prediction, "failed, original example: {} != prediction: {}", example, prediction);
    } 

    #[test]
    fn single_leaf() {

        let example = String::from("(0-S (36 (9 (3) (3)) (4 (2) (2))))");
        let prediction = inverse_check(example.clone());
        assert_eq!(example, prediction, "failed, original example: {} != prediction: {}", example, prediction);
    } 

    fn inverse_check(example: String) -> String { 

        // check by building tree and returning to the original input

        // forward
        let mut constituency = example;
        let mut string2tree: String2Tree = String2StructureBuilder::new();
        if let Err(e) = string2tree.build(&mut constituency) {
            panic!("{}", e);
        }
        let tree = string2tree.get_structure();

        // backward
        let mut accumulator = Accumulator::T2S(Vec::<String>::new());
        let tree2string: Tree2String = Structure2PlotBuilder::new(tree);
        if let Err(e) = tree2string.walk(None, &mut accumulator) {
            panic!("{}", e);
        }

        let string_vec = <&mut Vec<String>>::try_from(&mut accumulator).unwrap();
        let prediction = string_vec.join(" ");

        prediction
        
    }

}