
//
// Under MIT license
//

use id_tree::NodeId;
use plotters::prelude::{DrawingBackend, CoordTranslate, ChartContext};

/// A trait that contains the needed functionallity to build a string-to-structure process.
/// It is implemented by both constituency and dependency.
pub trait String2StructureBuilder<T, U> {
    fn new() -> Self;
    fn get_structure(&self) -> T;
    fn build(&mut self, input: &mut U) -> Result<(), String>;
    fn update_parent(&mut self, _item_id: &NodeId, _n: i32) -> Result<(), String> { todo!() } // optional
}

/// A trait that contains the needed functionallity to build a structure-to-plot process.
/// It is implemented by both constituency and dependency.
pub trait Structure2PlotBuilder<V> {
    fn new(structure: V) -> Self;
    fn build(&mut self, save_to: &str);
}

/// A trait that contains the needed functionallity to plot a structure-to-plot that is built.
/// It is implemented by both constituency and dependency.
/// Not called directly by the user.
pub trait Structure2PlotPlotter<W, X, Y> {
    fn plot<'a, DB, CT>(&self, chart: &mut ChartContext<'a, DB, CT>, plot_data_vec: Vec<X>, font_style: (&str, i32)) 
    where DB: DrawingBackend + 'a, CT: CoordTranslate<From = (f32, f32)>;
    fn walk(&self, item: Option<&W>, walk_args: Y, plot_data_vec: &mut Vec<X>);
    fn extract(&self, _item: &W, _walk_args: Y) -> X { todo!() } // optional
}
