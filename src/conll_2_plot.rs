
//
// Under MIT license
//

use std::error::Error;

use super::string_2_conll::*;
use plotters::{prelude::*, style::text_anchor::{Pos, HPos, VPos}};
use crate::generic_enums::{Element, Accumulator};
use crate::generic_traits::generic_traits::{Structure2PlotBuilder, Structure2PlotPlotter, WalkActions, WalkTree};

const DIM_CONST: u32 = 640;
const MARGIN: u32 = 15;
const FONT_SIZE: f32 = 15.0;
const FONT_CONST: f32 = 7.5 / 5.0;

/// A struct that wraps the needed fileds to plot a token
#[derive(Clone, Debug)]
pub struct ConllPlotData {
    start: f32,                 // start x position
    end: f32,                   // end x position
    deprel: String,
    pos: String,
    form: String,
    height: f32
}

#[derive(Debug)]
pub struct WalkData {
    conll_plot_data: Vec<ConllPlotData>,
    walk_args: Vec<[f32; 2]>
}


/// A struct that wraps the needed fileds to plot a conll
pub struct Conll2Plot {
    tokens: Vec<Token>,
    y_shift: f32 // room for pos and form
}

///
/// This is a building process of a plot.
/// Called after using String2Structure.
/// See lib.rs for usage examples.
/// 
impl Structure2PlotBuilder<Vec<Token>> for Conll2Plot {

    fn new(structure: Vec<Token>) -> Self {
        
        Self {
            tokens: structure,
            y_shift: 2.0
        }
    }

    fn build(&mut self, save_to: &str) -> Result<(), Box<dyn Error>> {

        // first run the forward part: extraction of the plotting data through recursion
        let walk_args: Vec<[f32; 2]> = vec![[0.0, 0.0]; (&self.tokens).len()];
        let plot_data_vec: Vec<ConllPlotData> = Vec::new();
        let walk_data: WalkData = WalkData { conll_plot_data: plot_data_vec, walk_args: walk_args };
        let mut accumulator = Accumulator::WD(walk_data);
        self.walk(None, &mut accumulator)?;

        // return to walk data
        let walk_data = <&mut WalkData>::try_from(&mut accumulator)?;

        // determine general plot settings for the example
        let seq_length = (&self.tokens).len() as f32;
        let built_height = self.y_shift + (&walk_data).walk_args[0..seq_length as usize].concat().iter().map(|x| *x as usize).max().unwrap() as f32;
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

        self.plot(&mut chart, walk_data.conll_plot_data.clone(), font_style)?;
        
        Ok(())
    }

}


///
/// This is a plotting helper implementation of the Structure2PlotPlotter trait.
/// The methods should not be called direcly by the user, rather used by the builder.
/// 
impl Structure2PlotPlotter<ConllPlotData> for Conll2Plot {

    fn plot<'a, DB, CT>(&self, chart: &mut ChartContext<'a, DB, CT>, plot_data_vec: Vec<ConllPlotData>, font_style: (&str, i32)) -> Result<(), Box<dyn Error>>
    where DB: DrawingBackend + 'a, CT: CoordTranslate<From = (f32, f32)> {
        
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

        Ok(())
    }


}


impl WalkTree for Conll2Plot {

    fn get_root_element(&self) -> Result<Element, Box<dyn Error>> {
        
        let mut root_id: Option<f32> = None;
        for i in 0..(&self.tokens).len() {

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
        assert!(root_id.is_some());
        let root_element_id = Element::TID(&self.tokens[root_id.unwrap() as usize]);
        Ok(root_element_id)

    }

    fn get_children_ids(&self, element_id: Element) -> Result<Vec<Element>, Box<dyn Error>> {
        
        let root_token_id = <&Token>::try_from(element_id)?.get_token_id();

        let mut root_children_ids: Vec<(f32, usize)> = Vec::new();
        for i in 0..(&self.tokens).len() {

            let token = &self.tokens[i as usize];
            let token_head = token.get_token_head();
            let token_id = token.get_token_id();

            if token_head == root_token_id && token_id != root_token_id {
                let distance = (root_token_id - token_id).abs() as usize;
                root_children_ids.push((token_id, distance));
            }

        }

        // sort children by distance (ascending order)
        root_children_ids.sort_by(|x, y| x.1.cmp(&y.1));
        let children_ids = root_children_ids.iter().map(|(token_id, _)| 
        Element::TID(&self.tokens[*token_id as usize])).collect::<>();
        
        Ok(children_ids)


    }
}

impl WalkActions for Conll2Plot {

    fn init_walk(&self, _element_id: Element, _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn finish_trajectory(&self, _element_id: Element, _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn on_node(&self, _element_id: Element, _parameters: &mut [f32; 6], _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn on_child(&self, _child_element_id: Element, _parameters: &mut [f32; 6], _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn post_walk_update(&self, element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        let root_token = <&Token>::try_from(element_id)?;
        let walk_data = <&mut WalkData>::try_from(data)?;
        let this_plot_data = self.extract(root_token, walk_data);
        walk_data.conll_plot_data.push(this_plot_data);
        Ok(())
    }

    fn finish_recursion(&self, _data: &mut Accumulator) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

}

impl Conll2Plot {

    fn extract(&self, token: &Token, walk_data: &mut WalkData) -> ConllPlotData {

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
                return -1.0 // this is the root case
            }


            // extract height based on counts in the closed interval
            let mut potential_heights: Vec<f32> = Vec::new();
            if start <= end {
                potential_heights = walk_data.walk_args[start..=end].concat();
            }
            
            let mut bounds = vec![walk_data.walk_args[token_id as usize][1-index], walk_data.walk_args[token_head as usize][index]];
            potential_heights.append(&mut bounds);
            let height = 1.0 + potential_heights.iter().map(|x| *x as usize).max().unwrap() as f32;

            walk_data.walk_args[token_id as usize][1-index] = height;
            walk_data.walk_args[token_head as usize][index] = height;

            height
        };

        let height = update();

        let plot_args = ConllPlotData {
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