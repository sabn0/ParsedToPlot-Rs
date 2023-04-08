
use super::string_2_conll::*;
use plotters::{prelude::*, style::text_anchor::{Pos, HPos, VPos}};
use crate::{Structure2PlotBuilder, Structure2PlotPlotter};

const DIM_CONST: u32 = 640;
const MARGIN: u32 = 15;
const FONT_SIZE: f32 = 15.0;
const FONT_CONST: f32 = 7.5 / 5.0;

/// A struct that wraps the needed fileds to plot a token
#[derive(Clone)]
pub struct PlotData {
    start: f32,                 // start x position
    end: f32,                   // end x position
    deprel: String,
    pos: String,
    form: String,
    height: f32
}

/// A struct that wraps the needed fileds to plot a conll
pub struct Conll2Plot {
    tokens: Vec<Token>,
    leaf_ids: Vec<f32>,
    seq_length: usize,
    y_shift: f32 // room for pos and form
}

///
/// This is a building process of a plot.
/// Called after using String2Structure.
/// See lib.rs for usage examples
/// 
impl Structure2PlotBuilder<Vec<Token>> for Conll2Plot {

    fn new(structure: Vec<Token>) -> Self {
        
        // calculate leaves only
        let seq_length = structure.len();
        let mut leaf_ids: Vec<f32> = (0..seq_length).map(|x| (x as f32)).collect();
        for i in 0..seq_length {
            
            // if not leaf, remove from the list.
            // A token cannot be a leaf if it is the head of another token
            let token = &structure[i as usize];
            let token_head = token.get_token_head();
            match leaf_ids.iter().position(|x| *x == token_head) {
                Some(index) => {
                    // found that the token_head is in leaf, remove it
                    leaf_ids.remove(index);
                },
                None => { () }
            };
        } 
        leaf_ids.dedup();

        Self {
            seq_length: seq_length,
            leaf_ids: leaf_ids,
            tokens: structure,
            y_shift: 2.0
        }
    }

    fn build(&mut self, save_to: &str) {

        // first run the forward part of extracting plot data
        let mut walk_args: Vec<[f32; 2]> = vec![[0.0, 0.0]; self.seq_length];
        let mut plot_data_vec: Vec<PlotData> = Vec::new();
        self.walk(None, &mut walk_args, &mut plot_data_vec);

        // handle plot settings
        // extract length of sequence
        let seq_length = self.seq_length as f32;
        let built_height = self.y_shift + walk_args[0..seq_length as usize].concat().iter().map(|x| *x as usize).max().unwrap() as f32;
        let total_units = 2*DIM_CONST / (seq_length + built_height) as u32;
        let width = total_units * seq_length as u32;
        let height = total_units * built_height as u32;
        let fig_dims: (u32, u32) = (width, height);

        // calculate dynamic font size
        let font_size = (FONT_CONST * (height as f32 / width as f32) * FONT_SIZE) as i32;
        let font_style = ("sans-serif", font_size);

        // initialization of backend settings
        let root_area = BitMapBackend::new(save_to, fig_dims)
        .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let x_spec = std::ops::Range{start: -0.1 as f32, end: seq_length};
        let y_spec = std::ops::Range{start: 0.0 as f32, end: 10.0 as f32};

        // x axis is removed thus doesn't need much space compared to y axis
        let mut chart = ChartBuilder::on(&root_area)
        .margin(MARGIN)
        .x_label_area_size(10)
        .y_label_area_size(50)
        .build_cartesian_2d(x_spec, y_spec).unwrap();

        chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .disable_x_axis()
        .disable_y_axis()
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
impl Structure2PlotPlotter<Token, PlotData, &mut Vec<[f32; 2]>> for Conll2Plot{

    fn plot<'a, DB, CT>(&self, chart: &mut ChartContext<'a, DB, CT>, plot_data_vec: Vec<PlotData>, font_style: (&str, i32)) 
    where DB: DrawingBackend + 'a, CT: CoordTranslate<From = (f32, f32)> {
        
        // TEXT STYLE
        let text_style = TextStyle::from(font_style)
        .transform(FontTransform::None)
        .font.into_font().style(FontStyle::Bold)
        .with_color(&BLACK)
        .with_anchor::<RGBColor>(Pos::new(HPos::Center, VPos::Center))
        .into_text_style(chart.plotting_area());

        let text_draw = |x, y, label: String| {
            return EmptyElement::at((x,y))
            + Text::new(format!("{}", label), (0,0), &text_style
            );
        };

        for plot_data in plot_data_vec {

            if plot_data.height >= 0.0 {

                let a_left = std::cmp::min(plot_data.start as u32, plot_data.end as u32);
                let a_right = std::cmp::max(plot_data.start as u32, plot_data.end as u32);
                let (x_0, a, b) = ((a_right + a_left) as f32 / 2.0, (a_right - a_left) as f32 / 2.0, plot_data.height);
                let (multi, y_shift, epsilon) = (50, self.y_shift, 0.2);
                
                chart.draw_series(LineSeries::new(((multi * a_left as i32) as u32..=(multi * a_right as i32) as u32).map(|x| x as f32 / multi as f32)
                .map(|x| (x, y_shift + (((b*b) - (((b*b) / (a*a))*((x-x_0).powi(2)))).powf(0.5)))), &BLACK)).unwrap();

                chart.draw_series(LineSeries::new(vec![(plot_data.end, y_shift), (plot_data.end + epsilon, y_shift + epsilon)], &BLACK)).unwrap();
                chart.draw_series(LineSeries::new(vec![(plot_data.end, y_shift), (plot_data.end - epsilon, y_shift + epsilon)], &BLACK)).unwrap();
                chart.plotting_area().draw(&text_draw(x_0, y_shift + plot_data.height - epsilon, plot_data.deprel.clone())).unwrap();
            }
            
            chart.plotting_area().draw(&text_draw(plot_data.end, self.y_shift / 2.0, plot_data.pos.clone())).unwrap();
            chart.plotting_area().draw(&text_draw(plot_data.end, 0.0, plot_data.form.clone())).unwrap();
        }

    }

    fn walk(&self, item: Option<&Token>, walk_args: &mut Vec<[f32; 2]>, plot_data_vec: &mut Vec<PlotData>) {
        
        // get root of the sequence if not given
        if item.is_none() {
            let mut root_id: Option<f32> = None;
            for i in 0..self.seq_length {

                let token = &self.tokens[i as usize];
                let token_head = token.get_token_head();
                let token_id = token.get_token_id();

                if token_id != token_head {
                    continue;
                }

                match root_id {
                    Some(_root_id) => panic!("not supporting more than one root"),
                    None => {
                        root_id = Some(token_id)
                    }
                }
            }
            
            let root_token = &self.tokens[root_id.unwrap() as usize];

            self.walk(Some(root_token), walk_args, plot_data_vec);
            
            let this_plot_data = self.extract(root_token, walk_args);
            plot_data_vec.push(this_plot_data);
            
            return

        }

        // get children of root and calculate distance
        let root_id = item.unwrap().get_token_id();
        let mut root_children_ids: Vec<(f32, usize)> = Vec::new();
        for i in 0..self.seq_length {

            let token = &self.tokens[i as usize];
            let token_head = token.get_token_head();
            let token_id = token.get_token_id();

            if token_head == root_id && token_id != root_id {
                let distance = (root_id - token_id).abs() as usize;
                root_children_ids.push((token_id, distance));
            }

        }

        // sort children by distance
        root_children_ids.sort_by(|x, y| x.1.cmp(&y.1));

        // send each child to recursive before plot
        for (child_id, _) in root_children_ids {

            let child_token = &self.tokens[child_id as usize];
            if !self.leaf_ids.contains(&child_id) {
                self.walk(Some(child_token), walk_args, plot_data_vec);
            }

            let this_plot_data = self.extract(child_token, walk_args);
            plot_data_vec.push(this_plot_data);
        }

    }

    fn extract(&self, token: &Token, walk_args: &mut Vec<[f32; 2]>) -> PlotData {

        let token_head = token.get_token_head();
        let token_id = token.get_token_id();

        let mut update = || {

            let index; let start; let end;
            if token_id < token_head {
                index = 0 as usize;
                start = (token_id + 1.0) as usize;
                end = (token_head - 1.0) as usize;

            } else if token_id > token_head {
                index = 1 as usize;
                start = (token_head + 1.0) as usize;
                end = (token_id - 1.0) as usize;

            } else {
                return -1.0 // this is the root case, change height for computability
            }


            // extract height based on counts in the closed interval
            let mut potential_heights: Vec<f32> = Vec::new();
            if start <= end {
                potential_heights = walk_args[start..=end].concat();
            }
            
            let mut bounds = vec![walk_args[token_id as usize][1-index], walk_args[token_head as usize][index]];
            potential_heights.append(&mut bounds);
            let height = 1.0 + potential_heights.iter().map(|x| *x as usize).max().unwrap() as f32;

            walk_args[token_id as usize][1-index] = height;
            walk_args[token_head as usize][index] = height;

            height
        };

        let height = update();

        // plot the token
        let plot_args = PlotData {
            start: token_head,
            end: token_id,
            deprel: token.get_token_deprel(),
            form: token.get_token_form(),
            pos: token.get_token_pos(),
            height: height
        };

        return plot_args;

    }
}
