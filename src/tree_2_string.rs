
//
// Under MIT license
//

use id_tree::*;
use std::{error::Error};
use std::fs::write;
use crate::generic_enums::{Accumulator, Element};
use crate::generic_traits::generic_traits::{WalkActions, WalkTree, Structure2PlotBuilder};

const CLOSE_BRACKETS: &str = ")";
const OPEN_BRACKET: &str = "(";

 pub struct Tree2String {
    tree: Tree<String>,
    double_leaf: bool,
    output: Option<String>
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
            tree: structure,
            output: None
        }
    }

    fn build(&mut self, save_to: &str) -> Result<(), Box<dyn Error>> {
        
        // run the recursive extraction
        let mut accumulator = Accumulator::T2S(Vec::<String>::new());
        self.walk(None, &mut accumulator)?;

        // move from accumulator vec to string
        let string_vec = <&mut Vec<String>>::try_from(&mut accumulator).unwrap();
        let prediction = string_vec.join(" ");

        // save to file and set output
        write(save_to, prediction.clone()).expect("Unable to write file");
        self.output = Some(prediction);

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

impl Tree2String {

    pub fn get_constituency(self) -> String {
        assert!(self.output.is_some(), "build most be evoked before retrival of constituency");
        self.output.unwrap().clone()
    }

}


#[cfg(test)]
mod tests {

    use crate::{String2StructureBuilder, Structure2PlotBuilder, String2Tree};
    use crate::tree_2_string::Tree2String;

    #[test]
    fn double_leaf() {

        let save_to = String::from("Output/constituency_inverse_double.txt");
        let example = String::from("(1-S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
        let prediction = inverse_check(example.clone(), save_to);
        assert_eq!(example, prediction, "failed, original example: {} != prediction: {}", example, prediction);
    } 

    #[test]
    fn single_leaf() {

        let save_to = String::from("Output/constituency_inverse_single.txt");
        let example = String::from("(0-S (36 (9 (3) (3)) (4 (2) (2))))");
        let prediction = inverse_check(example.clone(), save_to);
        assert_eq!(example, prediction, "failed, original example: {} != prediction: {}", example, prediction);
    } 

    fn inverse_check(example: String, save_to: String) -> String { 

        // check by building tree and returning to the original input

        // forward
        let mut constituency = example;
        let mut string2tree: String2Tree = String2StructureBuilder::new();
        string2tree.build(&mut constituency).unwrap();
        let tree = string2tree.get_structure();

        // backward
        let mut tree2string: Tree2String = Structure2PlotBuilder::new(tree);
        tree2string.build(&save_to).unwrap();

        tree2string.get_constituency()
        
    }

}