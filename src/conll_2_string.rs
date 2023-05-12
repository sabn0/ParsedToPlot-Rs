
//
// Under MIT license
//

use super::string_2_conll::Token;
use super::config::configure_structures::Saver;
use super::generic_enums::{Accumulator, Element};
use super::generic_traits::generic_traits::{WalkActions, WalkTree, Structure2PlotBuilder};

/// A Conll2String struct, mainly holds the vec tokens object. This type will implement Structure2PlotBuilder,
/// WalkTree and WalkActions, with an ultimate goal of saving a dependency to file.
pub struct Conll2String {
    tokens: Vec<Token>,
    output: Option<Vec<String>>
}

impl Conll2String {

    /// A method to retrieve the dependency conll after building it from the Vec<token>.
    /// Can be called only after build() has been called. See example on lib.rs.
    fn get_conll(self) -> Vec<String> {
        assert!(self.output.is_some(), "build most be evoked before retrival of conll");
        let conll = self.output.unwrap().clone();
        conll
    }
}

impl Structure2PlotBuilder<Vec<Token>> for Conll2String {
    fn new(structure: Vec<Token>) -> Self {
        Self {
            tokens: structure,
            output: None
        }
    }

    fn build(&mut self, save_to: &str) -> Result<(), Box<dyn std::error::Error>> {
        
        let mut accumulator = Accumulator::C2S(Vec::<String>::new());
        self.walk(None, &mut accumulator)?;

        // move from accumulator vec string to vec string
        let prediction = <&mut Vec<String>>::try_from(&mut accumulator).unwrap();

        // save to file and set output
        vec![prediction.clone()].save_output(save_to)?;
        self.output = Some(prediction.clone());

        Ok(())

    }
}

// The use of WalkTree + WalkActions is almost redundant in Conll2String, because the original string
// can be easily infered from the tokens. Hence most of this implementation is empty.
// get_root_element returns the first token of tokens for compliancy, then init_walk computes
// the accumulator entirly. In a second iteration, get_children_ids returns an empty vector
// for the arbitrary first token that was taken, and the program goes to termination condition.
impl WalkTree for Conll2String {
    fn get_root_element(&self) -> Result<Element, Box<dyn std::error::Error>> {
        let token_id = (&self.tokens).get(0).ok_or("conll is empty")?;
        let element_id = Element::TID(token_id);
        Ok(element_id)
    }

    fn get_children_ids(&self, _element_id: Element) -> Result<Vec<Element>, Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }
}

impl WalkActions for Conll2String {
    fn init_walk(&self, _element_id: Element, data: &mut Accumulator) -> Result<(), Box<dyn std::error::Error>> {
        
        let data_vec = <&mut Vec<String>>::try_from(data)?;
        for token in &self.tokens {
            let token_string = [
                token.get_token_id().to_string(),
                token.get_token_form(),
                token.get_token_lemma(),
                token.get_token_pos(),
                token.get_token_xpos(),
                token.get_token_feats(),
                token.get_token_head().to_string(),
                token.get_token_deprel(),
                token.get_token_deps(),
                token.get_token_misc()
            ].join("\t");
            data_vec.push(token_string);
        }
        Ok(())

    }

    fn finish_trajectory(&self, _element_id: Element, _data: &mut Accumulator) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn on_node(&self, _element_id: Element, _parameters: &mut [f32; 6], _data: &mut Accumulator) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn on_child(&self, _child_element_id: Element, _parameters: &mut [f32; 6], _data: &mut Accumulator) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn post_walk_update(&self, _element_id: Element, _data: &mut Accumulator) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn finish_recursion(&self, _data: &mut Accumulator) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}



#[cfg(test)]
mod tests {

    use super::Conll2String;
    use super::Structure2PlotBuilder;
    use crate::{String2StructureBuilder, String2Conll};

    #[test]
    fn conll() {

        let save_to = String::from("Output/dependency_inverse.txt");
        let example = [
            "0	The	the	DET	_	_	1	det	_	_",
            "1	people	people	NOUN	_	_	2	nsubj	_	_",
            "2	watch	watch	VERB	_	_	2	ROOT	_	_",
            "3	the	the	DET	_	_	4	det	_	_",
            "4	game	game	NOUN	_	_	2	dobj	_	_"
        ].map(|x| x.to_string()).to_vec();
        
        let prediction = inverse_check(example.clone(), save_to);

        assert_eq!(example, prediction, "\n failed, original example: \n {:?} \n != \n prediction: \n {:?}", example, prediction);
    } 


    fn inverse_check(example: Vec<String>, save_to: String) -> Vec<String> { 

        // check by building vec<token> and returning to the original input, expecting x = f(f^-1(x))

        // forward
        let mut dependency = example;
        let mut string2conll: String2Conll = String2StructureBuilder::new();
        string2conll.build(&mut dependency).unwrap();
        let conll = string2conll.get_structure();

        // backward
        let mut conll2string: Conll2String = Structure2PlotBuilder::new(conll);
        conll2string.build(&save_to).unwrap();
        
        conll2string.get_conll()
        
    }


}