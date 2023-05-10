

use std::error::Error;
use id_tree::{Tree, NodeId};
use std::any::Any;
use crate::tree_2_plot::TreePlotData;

pub trait Accumulateable {
    fn as_any(&self) -> &dyn Any;
    fn as_base(&self) -> &dyn Accumulateable;
}

impl Accumulateable for TreePlotData {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_base(&self) -> &dyn Accumulateable {
        self as &dyn Accumulateable
    }
}

impl Accumulateable for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_base(&self) -> &dyn Accumulateable {
        self as &dyn Accumulateable
    }
}
pub trait Accumulator {
    type Item;
    fn push_item(&mut self, item: Self::Item);
    fn check_is_empty(&self) -> bool;
    fn peak_last(&self) -> Result<&Self::Item, Box<dyn Error>>;
    fn as_base(&self) -> &dyn Accumulator<Item=dyn Accumulateable>;
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
    fn as_base(&self) -> &dyn Accumulator<Item=&dyn Accumulateable> {
        //let a = self as &dyn Accumulator<Item=&dyn Accumulateable>;
        let x = self.iter().map(|x| x.as_base()).collect::<Vec<&dyn Accumulateable>>();
        //let z = <x as Accumulator>::Item = &dyn Accumulateable;
        //let y = &x as &dyn Accumulator::Item=&dyn Accumulateable;
        //let y = &x as &dyn Accumulator::Item=<&dyn Accumulateable>;
        let y = &x as &dyn Accumulator<Item = &dyn Accumulateable>;
        //let x = self.iter().for_each(|x| {x.as_base();});
        y
    }
}

pub trait WalkActions {

    // initialize the walk for an empty intial node, returns the root node_id
    fn init_walk(&self, root_node_id: &NodeId, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>);
    fn finish_trajectory(&self, node_id: &NodeId, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>) -> Result<(), Box<dyn Error>>;
    fn on_node(&self, node_id: &NodeId, parameters: &mut [f32; 6], data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>) -> Result<(), Box<dyn Error>>;
    fn on_child(&self, child_node_id: &NodeId, parameters: &mut [f32; 6], data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>);
    fn finish_recursion(&self, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>);
}

pub struct WalkTree {
    tree: Tree<String>
}

impl WalkTree {

    pub fn new(tree: Tree<String>) -> Self {
        Self { tree: tree }
    }


    pub fn walk(&self, item: Option<&NodeId>, actions: &impl WalkActions, data: &mut Box<dyn Accumulator<Item=Box<dyn Accumulateable>>>) -> Result<(), Box<dyn Error>> {

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
        let mut parameters: [f32; 6] = [0.0; 6];
        actions.on_node(node_id, &mut parameters, data)?;

        // do DFS for the children of the current node_id, that has at least one child
        for child_node_id in children_ids {

            //
            // action on child_node
            //
            actions.on_child(child_node_id, &mut parameters, data);
            self.walk(Some(child_node_id), actions, data)?;
        }

        //
        // action on end recursion
        //
        actions.finish_recursion(data);
        Ok(())
        
    }


}