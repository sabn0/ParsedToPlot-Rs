
//
// Under MIT license
//

use id_tree::*;
use std::error::Error;

use super::config::configure_structures::Saver;
use super::generic_enums::{Accumulator, Element};
use super::generic_traits::generic_traits::{WalkActions, WalkTree, Structure2PlotBuilder};

const CLOSE_BRACKET: &str = ")";
const OPEN_BRACKET: &str = "(";

/// A Tree2String struct, mainly holds the tree object. This type will implement Structure2PlotBuilder,
/// WalkTree and WalkActions, with an ultimate goal of saving a constituency string of the tree to file.
 pub struct Tree2String {
    tree: Tree<String>,
    output: Option<String>
}

impl Tree2String {

    /// A method to retrieve the constituency string after building it from the tree.
    /// Can be called only after build() has been called. See example on lib.rs.
    fn get_constituency(self, inverse: bool) -> String {

        assert!(self.output.is_some(), "build() most be evoked before retrival of constituency");
        let constituency = self.output.unwrap().clone();

        // The constituency is built in singular mode regardless of the tree it repsresents.
        // for the purpse of checking the inverse tree2string(string2tree(x)) = x, one can use the inverse
        // flag to return the original. This option can have unexpected results for non-double leaf trees.

        if inverse {
            constituency.split(' ').map(|x| {
                if x.starts_with(OPEN_BRACKET) && x.ends_with(CLOSE_BRACKET) {
                    let (left, right) = x.split_once(CLOSE_BRACKET).unwrap();
                    left.split_once(OPEN_BRACKET).unwrap().1.to_string() + right
                } else {
                    x.to_string()
                }
            }).collect::<Vec<String>>().join(" ").to_string()
        } else {
            constituency
        }
    }

}


impl Structure2PlotBuilder<Tree<String>> for Tree2String {

    fn new(structure: Tree<String>) -> Self {

        Self {
            tree: structure,
            output: None
        }
    }

    fn build(&mut self, save_to: &str) -> Result<(), Box<dyn Error>> {
        
        // run the recursive extraction
        let mut accumulator = Accumulator::T2S(String::from(""));
        self.walk(None, &mut accumulator)?;

        // move from accumulator to string
        let prediction = <&mut String>::try_from(&mut accumulator).unwrap();

        // save to file and set output
        vec![prediction.clone()].save_output(save_to)?;
        self.output = Some(prediction.clone());

        Ok(())

    }

}

// WalkTree is very similar to the implementation in Tree2Plot
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

// WalkActions is very similar to the implementation in Tree2Plot, with the distinction beening
// the accumulator and its goal (save to string over plot to img).
impl WalkActions for Tree2String {

    fn init_walk(&self, _element_id: Element, _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn finish_trajectory(&self, element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn Error>> {

        let node_id = <&NodeId>::try_from(element_id)?;

        // double leaves are ignored in the tree2string construction, every leaf is build as if it
        // was a singular leaf (with parenthesis)
        let data_str = <&mut String>::try_from(data)?; 
        let node_data = self.tree.get(node_id)?.data();
        let sep = if data_str.is_empty() { "" } else { " " };
        *data_str += &format!("{}{}{}{}", sep, OPEN_BRACKET.to_string(), node_data, CLOSE_BRACKET.to_string());
        Ok(())
    }

    fn on_node(&self, element_id: Element, _parameters: &mut [f32; 6], data: &mut Accumulator) -> Result<(), Box<dyn Error>> {

        let node_id = <&NodeId>::try_from(element_id)?;
        let node_data = self.tree.get(node_id)?.data();
        let data_str = <&mut String>::try_from(data)?;
        let sep = if data_str.is_empty() { "" } else { " " };
        *data_str += &format!("{}{}{}", sep, OPEN_BRACKET.to_string(), node_data);
        Ok(())
    }

    fn on_child(&self, _child_element_id: Element, _parameters: &mut [f32; 6], _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn post_walk_update(&self, _element_id: Element, _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn finish_recursion(&self, data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        let data_str = <&mut String>::try_from(data)?;
        *data_str += &format!("{}", CLOSE_BRACKET.to_string());
        Ok(())
    }


}


#[cfg(test)]
mod tests {

    use super::Tree2String;
    use super::Structure2PlotBuilder;
    use crate::{String2StructureBuilder, String2Tree};

    #[test]
    fn tree_double_leaf() {

        let save_to = String::from("Output/constituency_inverse_double.txt");
        let example = String::from("(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
        let inverse = true;
        let prediction = inverse_check(example.clone(), save_to, inverse);
        assert_eq!(example, prediction, "\nfailed, original example:\n {}\n != \n prediction: {}", example, prediction);
    } 

    #[test]
    fn tree_single_leaf() {

        let save_to = String::from("Output/constituency_inverse_single.txt");
        let example = String::from("(36 (9 (3) (3)) (4 (2) (2)))");
        let inverse = false;
        let prediction = inverse_check(example.clone(), save_to, inverse);
        assert_eq!(example, prediction, "\nfailed, original example:\n {}\n != \nprediction: {}", example, prediction);
    } 

    fn inverse_check(example: String, save_to: String, inverse: bool) -> String { 

        // check by building tree and returning to the original input, expecting x = f(f^-1(x))

        // forward 
        let mut constituency = example;
        let mut string2tree: String2Tree = String2StructureBuilder::new();
        string2tree.build(&mut constituency).unwrap();
        let tree = string2tree.get_structure();

        // backward
        let mut tree2string: Tree2String = Structure2PlotBuilder::new(tree);
        tree2string.build(&save_to).unwrap();

        tree2string.get_constituency(inverse)
        
    }

}