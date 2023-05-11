

use std::error::Error;
use id_tree::{NodeId};
use crate::{tree_2_plot::TreePlotData, conll_2_plot::WalkData, string_2_conll::Token};

#[derive(Debug)]
pub enum Accumulator {
    TPD(Vec<TreePlotData>),
    T2S(Vec<String>),
    WD(WalkData)
}

impl<'a> TryFrom<&'a mut Accumulator> for &'a mut Vec<TreePlotData> {
    type Error = Box<dyn Error>;
    fn try_from(value: &'a mut Accumulator) -> Result<Self, Self::Error> {
        match value {
            Accumulator::TPD(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<Vec<TreePlotData>>(), value).into())
        }
    }
}

impl<'a> TryFrom<&'a mut Accumulator> for &'a mut Vec<String> {
    type Error = Box<dyn Error>;
    fn try_from(value: &'a mut Accumulator) -> Result<Self, Self::Error> {
        match value {
            Accumulator::T2S(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<Vec<String>>(), value).into())
        }
    }
}

impl<'a> TryFrom<&'a mut Accumulator> for &'a mut WalkData {
    type Error = Box<dyn Error>;
    fn try_from(value: &'a mut Accumulator) -> Result<Self, Self::Error> {
        match value {
            Accumulator::WD(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<WalkData>(), value).into())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Element<'a> {
    NID(&'a NodeId),
    TID(&'a Token),
}


impl<'a> TryFrom<Element<'a>> for &'a NodeId {
    type Error = Box<dyn Error>;
    fn try_from(value: Element<'a>) -> Result<Self, Self::Error> {
        match value {
            Element::NID(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<NodeId>(), value).into())
        }
    }
}

impl<'a> TryFrom<Element<'a>> for &'a Token {
    type Error = Box<dyn Error>;
    fn try_from(value: Element<'a>) -> Result<Self, Self::Error> {
        match value {
            Element::TID(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<Token>(), value).into())
        }
    }
}


pub trait WalkActions {

    // initialize the walk for an empty intial node, returns the root node_id
    fn init_walk(&self, element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    fn finish_trajectory(&self, element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    fn on_node(&self, element_id: Element, parameters: &mut [f32; 6], data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    fn on_child(&self, child_element_id: Element, parameters: &mut [f32; 6], data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    fn post_walk_update(&self, element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    fn finish_recursion(&self, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;

}

pub trait WalkTree: WalkActions {

    fn get_root_element(&self) -> Result<Element, Box<dyn Error>>;

    fn get_children_ids(&self, element_id: Element) -> Result<Vec<Element>, Box<dyn Error>>;

    // Element is an enum that holds a -reference- => &NodeId or &Token
    fn walk(&self, item: Option<Element>, data: &mut Accumulator) -> Result<(), Box<dyn Error>> {

        // walk in DFS

        // handle first iteration, extraction of the root
        if item.is_none() {
            let root_element_id = self.get_root_element()?;
            self.init_walk(root_element_id, data)?;
            self.walk(Some(root_element_id), data)?;
            self.post_walk_update(root_element_id, data)?;
            return Ok(());
        }

        // extract data on current node and its children
        let element_id: Element = item.unwrap();
        let children_ids: Vec<Element> = self.get_children_ids(element_id)?;

        if children_ids.is_empty() {
            // this is a point of leaf encounter
            // action on end trajectory 
            self.finish_trajectory(element_id, data)?;
            return Ok(());
        }
        
        // action on node 
        let mut parameters: [f32; 6] = [0.0; 6];
        self.on_node(element_id, &mut parameters, data)?;

        // do DFS for the children of the current node_id, that has at least one child
        for child_element_id in children_ids {

            //
            // action on child_node
            //
            self.on_child(child_element_id, &mut parameters, data)?;
            self.walk(Some(child_element_id), data)?;
            self.post_walk_update(child_element_id, data)?;
        }

        //
        // action on end recursion
        //
        self.finish_recursion( data)?;
        Ok(())
        
    }


}