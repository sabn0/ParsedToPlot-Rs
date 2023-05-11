
//
// Under MIT license
//

use std::error::Error;
use crate::generic_traits::generic_traits::String2StructureBuilder;

const CONLL_SIZE: usize = 10;

/// A struct that wraps the fields of a conll. The token struct and impl are not used by the user, rather The String2Conll implementation 
#[derive(Clone, Debug)]
pub struct Token {
    id: f32,
    form: String,
    lemma: String,
    pos: String,
    xpos: String,
    feats: String,
    head: f32,
    deprel: String,
    deps: String,
    misc: String
}

impl Token {

    ///
    /// A get method to retrive the token id of self
    /// 
    pub fn get_token_id(&self) -> f32 {
        return self.id
    }
    ///
    /// A get method to retrive the token head of self
    /// 
    pub fn get_token_head(&self) -> f32 {
        return self.head
    }
    ///
    /// A get method to retrive the token form of self
    /// 
    pub fn get_token_form(&self) -> String {
        return self.form.clone()
    }
    ///
    /// A get method to retrive the token pos of self
    /// 
    pub fn get_token_pos(&self) -> String {
        return self.pos.clone()
    }
    ///
    /// A get method to retrive the token deprel of self
    /// 
    pub fn get_token_deprel(&self) -> String {
        return self.deprel.clone()
    }
    ///
    /// A get method to retrive the lemma form of self (might be empty)
    /// 
    pub fn get_token_lemma(&self) -> String {
        return self.lemma.clone()
    }
    ///
    /// A get method to retrive the xpos form of self (might be empty)
    /// 
    pub fn get_token_xpos(&self) -> String {
        return self.xpos.clone()
    }
    ///
    /// A get method to retrive the feats form of self (might be empty)
    /// 
    pub fn get_token_feats(&self) -> String {
        return self.feats.clone()
    }
    ///
    /// A get method to retrive the deps form of self (might be empty)
    /// 
    pub fn get_token_deps(&self) -> String {
        return self.deps.clone()
    }
    ///
    /// A get method to retrive the misc form of self (might be empty)
    /// 
    pub fn get_token_misc(&self) -> String {
        return self.misc.clone()
    }
    
    fn new(input: Vec<String>) -> Token {

        assert!(input.len() == CONLL_SIZE, "input line does not satisfy Token requirments");
        let mut iter = input.into_iter();

        // id (int), form, lemma, upos, xpos, feats, head, deprel, deps, misc
        // for the needs of plotting dependency only id, form, pos, head and deprel are used
        let id = iter.next().unwrap().to_string().parse::<f32>().unwrap();
        let form = iter.next().unwrap().to_string();
        let lemma = iter.next().unwrap().to_string();
        let pos = iter.next().unwrap().to_string();
        let xpos = iter.next().unwrap().to_string();
        let feats = iter.next().unwrap().to_string();
        let head = iter.next().unwrap().to_string().parse::<f32>().unwrap();
        let deprel = iter.next().unwrap().to_string();
        let deps = iter.next().unwrap().to_string();
        let misc = iter.next().unwrap().to_string();
        assert!(iter.next().is_none());

        Self {
            id: id,
            form: form,
            lemma: lemma,
            pos: pos,
            xpos: xpos,
            feats: feats,
            head: head,
            deprel: deprel,
            deps: deps,
            misc: misc
        }
    }

}

/// A String2StructureBuilder sturct, mainly holds the tokens object. This type will implement the String2StructureBuilder,
/// with a dependency vec string as Input and a made Vec<Token> as output.
#[derive(Clone)]
pub struct String2Conll {
    tokens: Vec<Token>
}

impl String2StructureBuilder for String2Conll {

    type Input = Vec<String>;
    type Out = Vec<Token>;

    /// 
    /// Initialization of a String2Conll object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use parsed_to_plot::String2Conll;
    /// use parsed_to_plot::String2StructureBuilder;
    /// 
    /// let _string2conll: String2Conll = String2StructureBuilder::new();
    /// ```
    ///  
    fn new() -> Self {
        
        Self {
            tokens: Vec::new()
        }
    }

    ///
    /// Get a copy of the conll (should be called after build)
    /// 
    fn get_structure(&self) -> Self::Out {
        assert!(!self.tokens.is_empty(), "get_structure() should be called after using build(...)");
        return self.tokens.clone()
    }

    /// 
    /// A recursive method that builds a mutable Vec<Token> structure from a dependency vec string
    /// Returns Ok if the process was succesful (error otherwise)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use parsed_to_plot::String2Conll;
    /// use parsed_to_plot::String2StructureBuilder;
    /// 
    /// let mut dependency = [
    ///     "0	The	the	DET	_	_	1	det	_	_",
    ///     "1	people	people	NOUN	_	_	2	nsubj	_	_",
    ///     "2	watch	watch	VERB	_	_	2	ROOT	_	_",
    ///     "3	the	the	DET	_	_	4	det	_	_",
    ///     "4	game	game	NOUN	_	_	2	dobj	_	_"
    /// ].map(|x| x.to_string()).to_vec();
    /// let gold_first_token_form = "The";
    /// let gold_last_token_id = 4.0;
    /// 
    /// let mut string2conll: String2Conll = String2StructureBuilder::new();
    /// 
    /// if let Err(e) = string2conll.build(&mut dependency) {
    ///     panic!("{}", e); 
    /// }
    ///
    /// let conll = string2conll.get_structure();
    /// 
    /// let prediction_first_token_form = conll.first().unwrap().get_token_form();
    /// assert_eq!(prediction_first_token_form, gold_first_token_form);
    /// 
    /// let prediction_last_token_id = conll.last().unwrap().get_token_id();
    /// assert_eq!(prediction_last_token_id, gold_last_token_id);
    /// ```
    /// 
    fn build(&mut self, input: &mut Self::Input) -> Result<(), Box<dyn Error>> {

        // the input is a vector of strings, each string is a line in conll (token string represenation)
        for line in input.iter() {
    
            let token_vec: Vec<String> = line.split("\t").map(|s| s.to_string()).collect();
            let token = Token::new(token_vec);
            self.tokens.push(token);
        }

        return Ok(())
    }

}

#[cfg(test)]
mod tests {

    use super::String2Conll;
    use crate::generic_traits::generic_traits::String2StructureBuilder;

    #[test]
    fn load_sequence() {
        
        let mut dependency = [
            "0	The	the	DET	_	_	1	det	_	_",
            "1	people	people	NOUN	_	_	2	nsubj	_	_",
            "2	watch	watch	VERB	_	_	2	ROOT	_	_",
            "3	the	the	DET	_	_	4	det	_	_",
            "4	game	game	NOUN	_	_	2	dobj	_	_"
        ].map(|x| x.to_string()).to_vec();
        let gold_first_token_form = "The";
        let gold_last_token_id = 4.0;

        let mut string2conll: String2Conll = String2StructureBuilder::new();
        string2conll.build(&mut dependency).unwrap();
        let conll = string2conll.get_structure();

        let prediction_first_token_form = conll.first().unwrap().get_token_form();
        assert_eq!(prediction_first_token_form, gold_first_token_form);

        let prediction_last_token_id = conll.last().unwrap().get_token_id();
        assert_eq!(prediction_last_token_id, gold_last_token_id);
    }
}