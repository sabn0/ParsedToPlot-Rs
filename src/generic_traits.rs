
//
// Under MIT license
//

/// defines generic traits for building and plotting
pub mod generic_traits {

    use std::error::Error;
    use plotters::prelude::{DrawingBackend, CoordTranslate, ChartContext};
    use crate::generic_enums::{Element, Accumulator};

    // I move this trait to use associated types. Once the user selects the types the implementation
    // for that type is singular, String2StructureBuilder will not be implemented more than once for each type.
    // The associated types define the input and output of the functionalities of the trait.
    /// A trait that contains the needed functionallity to build a string-to-structure process.
    pub trait String2StructureBuilder {

        type Input;
        type Out;

        fn new() -> Self;
        fn get_structure(&self) -> Self::Out;
        fn build(&mut self, input: &mut Self::Input) -> Result<(), Box<dyn Error>>;
    }

    // Structure2PlotBuilder<T> will only work for types that implement WalkTree
    /// A trait that contains the needed functionallity to build a structure-to-plot process.
    pub trait Structure2PlotBuilder<T>: WalkTree {
        fn new(structure: T) -> Self;
        fn build(&mut self, save_to: &str) -> Result<(), Box<dyn Error>>;
    }

    /// A trait that contains the needed functionallity to plot a structure-to-plot that is built.
    /// Not called by the user.
    pub(crate) trait Structure2PlotPlotter<T> {
        fn plot<'a, DB, CT>(&self, chart: &mut ChartContext<'a, DB, CT>, plot_data_vec: Vec<T>, font_style: (&str, i32)) -> Result<(), Box<dyn Error>>
        where DB: DrawingBackend + 'a, CT: CoordTranslate<From = (f32, f32)>;
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
    
    // WalkTree will only work for types that implement WalkActions
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



}