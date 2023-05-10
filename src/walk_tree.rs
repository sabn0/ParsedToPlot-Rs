

use std::error::Error;
use id_tree::{Tree, NodeId};
use std::any::Any;
use crate::tree_2_plot::TreePlotData;

pub trait Accumulateable {
    fn as_any(&self) -> &dyn Any;
    fn as_base(&self) -> &dyn Accumulateable;
    fn clone_box<'a>(&'a self) -> Box<dyn Accumulateable + 'a>;
}

impl Accumulateable for TreePlotData{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_base(&self) -> &dyn Accumulateable {
        self as &dyn Accumulateable
    }
    fn clone_box<'a>(&'a self) -> Box<dyn Accumulateable + 'a> {
        Box::new(self.clone())
    }
}

impl Accumulateable for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_base(&self) -> &dyn Accumulateable {
        self as &dyn Accumulateable
    }
    fn clone_box<'a>(&'a self) -> Box<dyn Accumulateable + 'a> {
        Box::new(self.clone())
    }
}
pub trait Accumulator {
    fn push_item(&mut self, item: &dyn Accumulateable);
    fn check_is_empty(&self) -> bool;
    fn peak_last(&self) -> Result<&dyn Accumulateable, Box<dyn Error>>;
    fn as_base(&self) -> &dyn Accumulator;
    fn clone_box<'a>(&'a self) -> Box<dyn Accumulator +'a>;
    fn as_any(&self) -> &dyn Any;
}

impl Accumulator for Vec<&dyn Accumulateable> {
    fn push_item(&mut self, item: &dyn Accumulateable) {
        self.push(item);
    }
    fn check_is_empty(&self) -> bool {
        return self.is_empty()
    }
    fn peak_last(&self) -> Result<&dyn Accumulateable, Box<dyn Error>> {
        let last = self.last().expect(&format!("Vec<Accumulateable> is empty, probabaly with Non-empty node input"));
        Ok(last.as_base())
    }
    fn as_base(&self) -> &dyn Accumulator {
        
        self as &dyn Accumulator
        //let x = self
        //.iter()
        //.map(|x| x.as_base())
        //.collect::<Vec<&dyn Accumulateable>>();
        
        //&x as &dyn Accumulator
    }
    fn clone_box<'a>(&'a self) -> Box<dyn Accumulator +'a> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

}

pub trait WalkActions {

    // initialize the walk for an empty intial node, returns the root node_id
    fn init_walk(&self, root_node_id: &NodeId, data: &mut Box<dyn Accumulator>);
    fn finish_trajectory(&self, node_id: &NodeId, data: &mut Box<dyn Accumulator>) -> Result<(), Box<dyn Error>>;
    fn on_node(&self, node_id: &NodeId, parameters: &mut [f32; 6], data: &mut Box<dyn Accumulator>) -> Result<(), Box<dyn Error>>;
    fn on_child(&self, child_node_id: &NodeId, parameters: &mut [f32; 6], data: &mut Box<dyn Accumulator>);
    fn finish_recursion(&self, data: &mut Box<dyn Accumulator>);
}

pub struct WalkTree {
    tree: Tree<String>
}

impl WalkTree {

    pub fn new(tree: Tree<String>) -> Self {
        Self { tree: tree }
    }


    pub fn walk(&self, item: Option<&NodeId>, actions: &impl WalkActions, data: &mut Box<dyn Accumulator>) -> Result<(), Box<dyn Error>> {

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