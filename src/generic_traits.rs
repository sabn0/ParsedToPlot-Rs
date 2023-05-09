
//
// Under MIT license
//

/// defines generic traits for building and plotting
pub mod generic_traits {

    use std::error::Error;
    use plotters::prelude::{DrawingBackend, CoordTranslate, ChartContext};

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

    /// A trait that contains the needed functionallity to build a structure-to-plot process.
    pub trait Structure2PlotBuilder<V> {
        fn new(structure: V) -> Self;
        fn build(&mut self, save_to: &str) -> Result<(), Box<dyn Error>>;
    }

    /// A trait that contains the needed functionallity to plot a structure-to-plot that is built.
    /// Not called by the user.
    pub(crate) trait Structure2PlotPlotter<W, X, Y> {
        fn plot<'a, DB, CT>(&self, chart: &mut ChartContext<'a, DB, CT>, plot_data_vec: Vec<X>, font_style: (&str, i32)) -> Result<(), Box<dyn Error>> where DB: DrawingBackend + 'a, CT: CoordTranslate<From = (f32, f32)>;
        fn walk(&self, item: Option<&W>, walk_args: Y, plot_data_vec: &mut Vec<X>) -> Result<(), Box<dyn Error>> ;
        fn extract(&self, _item: &W, _walk_args: Y) -> X { todo!() } // optional
    }

}