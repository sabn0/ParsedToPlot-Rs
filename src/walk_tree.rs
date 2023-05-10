

use std::error::Error;
use id_tree::{NodeId, Tree};
use crate::tree_2_plot::TreePlotData;

#[derive(Debug)]
pub enum Accumulator {
    TPD(Vec<TreePlotData>),
    T2S(Vec<String>)
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

pub trait WalkActions {

    // initialize the walk for an empty intial node, returns the root node_id
    fn init_walk(&self, root_node_id: &NodeId, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    fn finish_trajectory(&self, node_id: &NodeId, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    fn on_node(&self, node_id: &NodeId, parameters: &mut [f32; 6], data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    fn on_child(&self, child_node_id: &NodeId, parameters: &mut [f32; 6], data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    fn finish_recursion(&self, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;

}

pub trait WalkTree: WalkActions {

    fn get_tree(&self) -> &Tree<String>;

    fn walk(&self, item: Option<&NodeId>, data: &mut Accumulator) -> Result<(), Box<dyn Error>> {

        // walk in DFS

        // handle first iteration, extraction of the root
        if item.is_none() {
            let root_node_id: &NodeId = self.get_tree().root_node_id().ok_or("input tree is empty")?;
            self.init_walk(root_node_id, data)?;
            return self.walk(Some(root_node_id), data);
        }

        // extract data on current node and its children
        let node_id = item.unwrap();
        let children_ids: Vec<&NodeId> = self.get_tree().children_ids(node_id)?.collect();

        if children_ids.is_empty() {
            // this is a point of leaf encounter
            // action on end trajectory 
            return self.finish_trajectory(node_id, data);
        }
        
        // action on node 
        let mut parameters: [f32; 6] = [0.0; 6];
        self.on_node( node_id, &mut parameters, data)?;

        // do DFS for the children of the current node_id, that has at least one child
        for child_node_id in children_ids {

            //
            // action on child_node
            //
            self.on_child(child_node_id, &mut parameters, data)?;
            self.walk(Some(child_node_id), data)?;
        }

        //
        // action on end recursion
        //
        self.finish_recursion( data)?;
        Ok(())
        
    }


}