
//
// Under MIT license
//

use std::error::Error;

use crate::generic_traits::generic_traits::String2StructureBuilder;

const CONLL_SIZE: usize = 10;

/// A struct that wraps the -needed- fields to draw a token
/// The token struct and impl are not used by the user, rather
/// The String2Conll implementation
#[derive(Clone, Debug)]
pub struct Token {
    id: f32,
    form: String,
    pos: String,
    head: f32,
    deprel: String
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
    
    fn new(input: Vec<String>) -> Token {

        assert!(input.len() == CONLL_SIZE, "input line does not satisfy Token requirments");
        let mut iter = input.into_iter();

        // id (int), form, lemma, upos, xpos, feats, head, deprel, deps, misc
        // uses only:
        // id , form, upos, head, deprel, 

        let id = iter.next().unwrap().to_string().parse::<f32>().unwrap();
        let form = iter.next().unwrap().to_string();
        let _lemma = iter.next();
        let pos = iter.next().unwrap().to_string();
        let _xpos = iter.next();
        let _feats = iter.next();
        let head = iter.next().unwrap().to_string().parse::<f32>().unwrap();
        let deprel = iter.next().unwrap().to_string();

        Self {
            id: id,
            form: form,
            pos: pos,
            head: head,
            deprel: deprel,
        }
    }

}

/// A struct that holds sequence of tokens in vector
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
        return self.tokens.clone()
    }

    /// 
    /// An implementation of the build method, construction of tokens 
    /// Receives a vector of strings that represents a conll format.
    /// 
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
    /// let mut string2conll: String2Conll = String2StructureBuilder::new();
    /// let _res = match string2conll.build(&mut dependency) {
    ///     Ok(_res) => {},
    ///     Err(e) => panic!("{}", e) 
    /// };
    ///
    /// let conll = string2conll.get_structure();
    /// 
    /// let first_token = conll.first().unwrap();
    /// assert!(first_token.get_token_form() == "The");
    /// let last_token = conll.last().unwrap();
    /// assert!(last_token.get_token_id().to_string() == "4");
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

    use crate::generic_traits::generic_traits::String2StructureBuilder;
    use super::String2Conll;

    #[test]
    fn load_sequence() {
        
        let mut dependency = [
            "0	The	the	DET	_	_	1	det	_	_",
            "1	people	people	NOUN	_	_	2	nsubj	_	_",
            "2	watch	watch	VERB	_	_	2	ROOT	_	_",
            "3	the	the	DET	_	_	4	det	_	_",
            "4	game	game	NOUN	_	_	2	dobj	_	_"
        ].map(|x| x.to_string()).to_vec();

        let mut string2conll: String2Conll = String2StructureBuilder::new();
        let _res = match string2conll.build(&mut dependency) {
            Ok(_res) => {},
            Err(e) => panic!("{}", e) 
        };

        let conll = string2conll.get_structure();
        let first_token = conll.first().unwrap();
        assert!(first_token.get_token_form() == "The");
        let last_token = conll.last().unwrap();
        assert!(last_token.get_token_id().to_string() == "4");
    }
}