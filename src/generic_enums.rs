

use std::error::Error;
use id_tree::{NodeId};
use crate::{tree_2_plot::TreePlotData, conll_2_plot::WalkData, string_2_conll::Token};

#[derive(Debug)]
pub enum Accumulator {
    TPD(Vec<TreePlotData>), // for Tree2Plot
    T2S(String),            // for Tree2String
    WD(WalkData),           // for Conll2Plot
    C2S(Vec<String>)        // for Conll2String
}

impl<'a> TryFrom<&'a mut Accumulator> for &'a mut Vec<TreePlotData> {
    type Error = Box<dyn Error>;
    fn try_from(value: &'a mut Accumulator) -> Result<Self, Self::Error> {
        match value {
            Accumulator::TPD(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<Vec<TreePlotData>>(), value).into())
        }
    }
}

impl<'a> TryFrom<&'a mut Accumulator> for &'a mut String {
    type Error = Box<dyn Error>;
    fn try_from(value: &'a mut Accumulator) -> Result<Self, Self::Error> {
        match value {
            Accumulator::T2S(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<String>(), value).into())
        }
    }
}

impl<'a> TryFrom<&'a mut Accumulator> for &'a mut WalkData {
    type Error = Box<dyn Error>;
    fn try_from(value: &'a mut Accumulator) -> Result<Self, Self::Error> {
        match value {
            Accumulator::WD(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<WalkData>(), value).into())
        }
    }
}

impl<'a> TryFrom<&'a mut Accumulator> for &'a mut Vec<String> {
    type Error = Box<dyn Error>;
    fn try_from(value: &'a mut Accumulator) -> Result<Self, Self::Error> {
        match value {
            Accumulator::C2S(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<Vec<String>>(), value).into())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Element<'a> {
    NID(&'a NodeId),
    TID(&'a Token),
}


impl<'a> TryFrom<Element<'a>> for &'a NodeId {
    type Error = Box<dyn Error>;
    fn try_from(value: Element<'a>) -> Result<Self, Self::Error> {
        match value {
            Element::NID(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<NodeId>(), value).into())
        }
    }
}

impl<'a> TryFrom<Element<'a>> for &'a Token {
    type Error = Box<dyn Error>;
    fn try_from(value: Element<'a>) -> Result<Self, Self::Error> {
        match value {
            Element::TID(x) => Ok(x),
            _ => Err(format!("could not convert to {:?} from {:?}", std::any::type_name::<Token>(), value).into())
        }
    }
}

