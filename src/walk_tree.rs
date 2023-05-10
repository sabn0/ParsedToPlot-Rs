

use std::error::Error;
use id_tree::{Tree, NodeId};
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

pub struct WalkTree {
    tree: Tree<String>
}

impl WalkTree {

    // maybe should be initialized with a reference to Structure2PlotBuilder, for example tree_2_plot
    // alternitably, Structure2PlotBuilder should be initialized with WalkTree. 
    // Those are the two options
    pub fn new(tree: Tree<String>) -> Self {
        Self { tree: tree }
    }


    pub fn walk(&self, item: Option<&NodeId>, actions: &impl WalkActions, data: &mut Accumulator) -> Result<(), Box<dyn Error>> {

        // walk in DFS

        // handle first iteration, extraction of the root
        if item.is_none() {
            let root_node_id: &NodeId = self.tree.root_node_id().ok_or("input tree is empty")?;
            actions.init_walk(root_node_id, data)?;
            self.walk(Some(root_node_id), actions, data)?;
            return Ok(());
        }

        // extract data on current node and its children
        let node_id = item.unwrap();
        let children_ids: Vec<&NodeId> = self.tree.children_ids(node_id)?.collect();

        if children_ids.is_empty() {
            // this is a point of leaf encounter
            // action on end trajectory 
            return actions.finish_trajectory(node_id, data);
        }
        
        // action on node 
        let mut parameters: [f32; 6] = [0.0; 6];
        actions.on_node(node_id, &mut parameters, data)?;

        // do DFS for the children of the current node_id, that has at least one child
        for child_node_id in children_ids {

            //
            // action on child_node
            //
            actions.on_child(child_node_id, &mut parameters, data)?;
            self.walk(Some(child_node_id), actions, data)?;
        }

        //
        // action on end recursion
        //
        actions.finish_recursion(data)?;
        Ok(())
        
    }


}