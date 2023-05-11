
//
// Under MIT license
//

/// The module defines generic traits for building, plotting and DFS over trees
pub mod generic_traits {

    use std::error::Error;
    use plotters::prelude::{DrawingBackend, CoordTranslate, ChartContext};
    use crate::generic_enums::{Element, Accumulator};

    // Since 0.2.0 this trait uses associated types. Once the user selects the types the implementation
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

    /// A trait that contains the needed functionallity to build a structure-to-plot process.
    pub trait Structure2PlotBuilder<T> {
        fn new(structure: T) -> Self;
        fn build(&mut self, save_to: &str) -> Result<(), Box<dyn Error>>;
    }

    // A trait that contains the needed functionallity to plot a structure-to-plot that is built.
    // Not called by the user. Normally would be called from Structure2PlotBuilder, but not a must.
    pub(in crate) trait Structure2PlotPlotter<S> {
        fn plot<'a, DB, CT>(&self, chart: &mut ChartContext<'a, DB, CT>, plot_data_vec: Vec<S>, font_style: (&str, i32)) -> Result<(), Box<dyn Error>>
        where DB: DrawingBackend + 'a, CT: CoordTranslate<From = (f32, f32)>;
    }

    // A trait that specifies the actions inside a travel over a structure. 
    // This functionality is needed by the WalkTree trait. 
    pub(in crate) trait WalkActions {
        // initializes a DFS run using the root element.
        fn init_walk(&self, element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
        // actions to be made specifically on a leaf.
        fn finish_trajectory(&self, element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
        // actions to be made specifically on a node.
        fn on_node(&self, element_id: Element, parameters: &mut [f32; 6], data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
        // actions to be made specifically on child of a node.
        fn on_child(&self, child_element_id: Element, parameters: &mut [f32; 6], data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
        // actions to be made specifically after a recursive call.
        fn post_walk_update(&self, element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
        // actions to be made right before termination.
        fn finish_recursion(&self, data: &mut Accumulator) -> Result<(), Box<dyn Error>>;
    
    }
    
    // WalkTree will only work for types that implement WalkActions.
    // A supertrait that contains the organizes a DFS over a structure.
    pub(in crate) trait WalkTree: WalkActions {

        // retrieve the root element of a structure from the structure. Element is an enum that stores references
        // for supported structs, not owed structures.
        fn get_root_element(&self) -> Result<Element, Box<dyn Error>>;
        
        // retrieve the children of an element by id. Element is an enum over references, that the return type
        // is a vector of references, not owed structures.
        fn get_children_ids(&self, element_id: Element) -> Result<Vec<Element>, Box<dyn Error>>;
        
        // The main frame of a DFS walk . Starts with an empty Element (None), and an empty mutable Accumulator,
        // that is a dynamic enum to store the output of the actions during the walk (the goal of the walk could
        // be to plot to an img, save to string, etc..)
        fn walk(&self, item: Option<Element>, data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
    
            // handle first iteration, extraction of the root
            if item.is_none() {
                let root_element_id = self.get_root_element()?;
                self.init_walk(root_element_id, data)?;
                self.walk(Some(root_element_id), data)?;
                self.post_walk_update(root_element_id, data)?;
                return Ok(());
            }
    
            // extract data on current element and its children
            let element_id: Element = item.unwrap();
            let children_ids: Vec<Element> = self.get_children_ids(element_id)?;
    
            if children_ids.is_empty() {
                // this is a point of leaf encounter, action on end trajectory 
                self.finish_trajectory(element_id, data)?;
                return Ok(());
            }
            
            // action on element 
            let mut parameters: [f32; 6] = [0.0; 6];
            self.on_node(element_id, &mut parameters, data)?;
    
            // do DFS for the children of the current element_id, that has at least one child
            for child_element_id in children_ids {
    
                //
                // action on child_element
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