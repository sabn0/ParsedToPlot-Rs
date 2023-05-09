

use std::error::Error;
use id_tree::{Tree, NodeId};
use std::any::Any;
use crate::tree_2_plot::TreePlotData;

pub trait Accumulateable {
    fn as_any(&self) -> &dyn Any;
}

impl Accumulateable for TreePlotData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Accumulateable for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
pub trait Accumulator {
    type Item;
    fn push_item(&mut self, item: Self::Item);
    fn check_is_empty(&self) -> bool;
    fn peak_last(&self) -> Result<&Self::Item, Box<dyn Error>>;
}

impl Accumulator for String {
    type Item = String;
    fn push_item(&mut self, item: Self::Item){
        *self += &item;
    }
    fn check_is_empty(&self) -> bool {
        return self.is_empty()
    }
    fn peak_last(&self) -> Result<&String, Box<dyn Error>> {
        return Err(format!("peak last not implemented for str").into())
    }
}

impl Accumulator for Vec<TreePlotData> {
    type Item = TreePlotData;
    fn push_item(&mut self, item: Self::Item){
        self.push(item);
    }
    fn check_is_empty(&self) -> bool {
        return self.is_empty()
    }
    fn peak_last(&self) -> Result<&TreePlotData, Box<dyn Error>> {
        let last = self.last().ok_or(format!("Vec<TreePlotData> is empty, probabaly with Non-empty node input").into());
        last
    }
}

pub trait WalkActions {

    // initialize the walk for an empty intial node, returns the root node_id
    fn init_walk(&self, root_node_id: &NodeId, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>);
    fn finish_trajectory(&self, node_id: &NodeId, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>) -> Result<(), Box<dyn Error>>;
    fn on_node(&self, node_id: &NodeId, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>) -> Result<(), Box<dyn Error>>;
    fn on_child();
    fn finish_recursion();
}

pub struct WalkTree {
    tree: Tree<String>
}

impl WalkTree {

    fn new(tree: Tree<String>) -> Self {
        Self { tree: tree }
    }


    fn walk(&self, item: Option<&NodeId>, actions: &impl WalkActions, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>) -> Result<(), Box<dyn Error>> {

        // walk in DFS

        // handle first iteration, extraction of the root
        if item.is_none() {
            let root_node_id: &NodeId = self.tree.root_node_id().ok_or("input tree is empty")?;
            actions.init_walk(root_node_id, data);
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


        // do DFS for the children of the current node_id, that has at least one child
        for child_node_id in children_ids {

            //
            // action on child_node
            //

            self.walk(Some(child_node_id), actions, data)?;
        }

        //
        // action on end recursion
        //
        Ok(())
        
    }


}