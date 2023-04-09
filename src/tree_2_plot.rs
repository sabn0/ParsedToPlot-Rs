
//
// Under MIT license
//

use id_tree::*;
use plotters::{prelude::*, style::text_anchor::*};
use std::{collections::HashMap};
use crate::string_2_tree::SubChildren;
use crate::{Structure2PlotBuilder, Structure2PlotPlotter};

const DIM_CONST: usize = 640;
const FONT_CONST: f32 = 0.0267;
const FONT_SIZE: u32 = 15;
const INIT_LEFT_BOUND: f32 = -5.0;  // left and right bound are arbitrary
const INIT_RIGHT_BOUND: f32 = 5.0;
const Y_AX_LABEL: &str = "Depth";

/// A struct that wraps the needed fileds to plot a node
#[derive(Clone)]
pub struct PlotData {
    positional_args: [f32; 6],  // save x1 y1 x2 y2 left_bound right_bound
    label_arg: String,           // save label
}

/*
Note: Options & Results are mainly handled implicitly during this module.
The reason is that this module is based on two components:
1) the output of the string_2_tree module, in which Options & Results 
are aready handled explictly. 2) Relative simple line series and point series.
 */

/// A struct that wraps the needed fileds to plot a tree
 pub struct Tree2Plot {
    tree: Tree<String>,
    node_id2n_sub_children: HashMap<NodeId, usize>,
}

///
/// This is a building process of a plot.
/// Called after using String2Structure.
/// See lib.rs for usage examples
/// 
impl Structure2PlotBuilder<Tree<String>> for Tree2Plot {


    fn new(mut structure: Tree<String>) -> Self {
        
        // extract number of leaves for each node's sub tree
        let node_id2n_sub_children = match structure.get_sub_children(true) {
            Ok(node_id2n_sub_children) => node_id2n_sub_children,
            Err(_e) => panic!("could not infer sub children from tree")
        };

        Self {
            node_id2n_sub_children: node_id2n_sub_children,
            tree: structure
        }
    }

    fn build(&mut self, save_to: &str) {
        
        // run the recursive extraction
        let mut plot_data_vec: Vec<PlotData> = Vec::new();
        self.walk(None, None, &mut plot_data_vec);

        // calculate dimensions of plot based on tree height
        // extract height from tree
        let tree_height = self.tree.height();

        let tree_length = self.node_id2n_sub_children.get(self.tree.root_node_id().unwrap()).unwrap();
        let height = (DIM_CONST * tree_height / tree_length) as u32;
        let length = (DIM_CONST * tree_length / tree_height) as u32;
        let fig_dims: (u32, u32) = (length, height);
        let font_style: (&str, i32) = ("sans-serif", ((height as f32) * FONT_CONST) as i32);

        // initialization of backend settings
        let root_area = BitMapBackend::new(save_to, fig_dims).into_drawing_area();
        root_area.fill(&WHITE).unwrap();
    
        let x_spec = std::ops::Range{start:INIT_LEFT_BOUND, end:INIT_RIGHT_BOUND};
        let y_spec = std::ops::Range{start:(tree_height-1) as f32, end: 0.0};

        // x axis is removed thus doesn't need much space compared to y axis
        let mut chart = ChartBuilder::on(&root_area)
        .margin(FONT_SIZE)
        .x_label_area_size(10)
        .y_label_area_size(50)
        .build_cartesian_2d(x_spec, y_spec).unwrap();
        
        chart
        .configure_mesh()
        .bold_line_style(&BLACK)
        .disable_x_mesh()
        .disable_y_mesh()
        .disable_x_axis()
        .y_labels(tree_height as usize)
        .y_desc(Y_AX_LABEL)
        .y_label_style(font_style)
        .axis_desc_style(font_style)
        .y_label_formatter(&|x| format!("{}", *x as i32))
        .draw()
        .unwrap();

        self.plot(&mut chart, plot_data_vec, font_style);

    }

}

///
/// This is a plotting helper implementation.
/// The methods should not be called direcly by the user.
/// See lib.rs for usage examples
/// 
impl Structure2PlotPlotter<NodeId, PlotData, Option<PlotData>> for Tree2Plot {

    fn plot<'a, DB, CT>(&self, chart: &mut ChartContext<'a, DB, CT>, plot_data_vec: Vec<PlotData>, font_style: (&str, i32)) 
    where DB: DrawingBackend + 'a, CT: CoordTranslate<From = (f32, f32)> {
        
        // TEXT STYLE
        let text_style = TextStyle::from(font_style)
        .transform(FontTransform::None)
        .font.into_font().style(FontStyle::Bold)
        .with_color(&BLACK)
        .with_anchor::<RGBColor>(Pos::new(HPos::Center, VPos::Center))
        .into_text_style(chart.plotting_area());

        for plot_data in plot_data_vec {
            
            // extracting plot location
            let label = &plot_data.label_arg;
            let [x1, y1, x2, y2]: [f32; 4] = plot_data.positional_args[..4].try_into().unwrap();

            // order matters
            // plus 0.1 is a workaround for visualization purposes
            chart.draw_series(LineSeries::new(vec![(x1, y1+0.1), (x2, y2-0.1)], &BLACK)).unwrap();
            chart.draw_series(PointSeries::of_element(
                vec![(x2, y2)],
                FONT_SIZE,
                &BLACK,
                &|c, _s, _st| {
                    return EmptyElement::at(c)
                    + Circle::new((0, 0), 10, ShapeStyle{color: WHITE.into(), filled: true, stroke_width: 1})
                    + Text::new(format!("{}", label), (0,0), &text_style);
                },
                // points + circle on text are correnly a workaround for presentation
            )).unwrap();
        }
    }

    fn walk(&self, item: Option<&NodeId>, walk_args: Option<PlotData>, plot_data_vec: &mut Vec<PlotData>) {
        
        if item.is_none() {

            match self.tree.root_node_id() {
                None => panic!("input tree is empty"),
                Some(root_node_id) => {
                    // get root node label and send with initial positional args to plot
                    // bounds are set to -+ 5 but this is arbitrary at this point.
                    let root_node = self.tree.get(root_node_id).unwrap();
                    let root_node_data = root_node.data();
                    let root_plot_args = PlotData {
                        positional_args: [0.0, 0.0, 0.0, 0.0, INIT_LEFT_BOUND, INIT_RIGHT_BOUND],
                        label_arg: root_node_data.to_owned()
                    };

                    self.walk(Some(&root_node_id.clone()), Some(root_plot_args.clone()), plot_data_vec);
                    plot_data_vec.push(root_plot_args);
                    return
                }, 
            };
        }

        // normal node extraction
        
        // get node's children, if no children finish
        let children_ids: Vec<&NodeId> = match self.tree.children_ids(item.unwrap()) {
            Ok(children_ids) => children_ids.collect(),
            Err(e) => panic!("{}", e)
        };

        // stopping condition of route
        if children_ids.is_empty() {
            return
        }

        // extract positional args
        let walk_args = match walk_args {
            Some(walk_args) => walk_args,
            None => panic!("None positions args are supplied with Some node")
        };
        let [x2, y2, left_bound, right_bound]: [f32; 4] = walk_args.positional_args[2..].try_into().unwrap();

        // for positional computation, get the total number of sub_children that are leaves for this node
        // every child of the node will be positioned by the proportion of its sub_tree compared to the 
        // total number of leaves in this sub tree.
        let n_leaves = match self.node_id2n_sub_children.get(item.unwrap()) {
            Some(n_leaves) => *n_leaves as f32,
            None => panic!("didn't find node id in mapping to sub children")
        };

        // iterate over children, save plotting data for each child recursivly
        let mut space_allocated: f32 = 0.0;
        for child_node_id in children_ids {
            
            // get label for this child;
            let label = self.tree.get(child_node_id).unwrap().data().to_owned();

            // calculate positional args for this child
            // for positional computation, get the total number of sub_children that are leaves for this node
            let c_leaves = match self.node_id2n_sub_children.get(child_node_id) {
                Some(c_leaves) => *c_leaves as f32,
                None => panic!("didn't find node id in mapping to sub children")
            };

            let allocation: f32 = (right_bound - left_bound) * (c_leaves / n_leaves);
        
            let new_left_bound = left_bound + space_allocated;
            let new_right_bound = left_bound + space_allocated + allocation;
            let new_x2: f32 = (new_left_bound + new_right_bound) / 2 as f32;
            let new_y2: f32 = y2 + 1 as f32;
            space_allocated = space_allocated + allocation;

            // create plot data for this child
            let child_walk_args = PlotData {
                positional_args: [x2, y2, new_x2, new_y2, new_left_bound, new_right_bound],
                label_arg: label
            };
            
            self.walk(Some(child_node_id), Some(child_walk_args.clone()), plot_data_vec);
            plot_data_vec.push(child_walk_args);

        }

    }


}

