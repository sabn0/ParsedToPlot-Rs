//! 
//! # Overview
//! 
//! This repo plots constituency trees and dependency trees given by strings, using both the [id-tree](https://crates.io/crates/id_tree) crate and [plotters](https://crates.io/crates/plotters). While primarily written with linguistic syntax in mind, it can serve other inputs, such as mathematical expressions etc. The API first transforms the input to an internal conll / tree, then plots the structure with recursion. It is mostly suitable for short sentences of up to 15-20 tokens.
//! 
//! # Input-Output
//! 
//! * The API expects a string input. Multiple string inputs can be delivered in a file, through the command-line.
//! * For constituency trees, the program takes a parsed string given in one line. The string can be syntactic, for example
//! such that represents phrases and parts-of-speech (like the structure in [Berkeley Neural Parser](https://pypi.org/project/benepar/)
//! in python). Such strings will have "double leaves" (see an example below). Alternatively, the strings can have singular leaves,
//! representing, for example, mathematical expressions.
//! * For dependency trees, the programs takes a conll format, in which every token has 10 fields, separated by tab, and
//! presented in a new line. Sentences are separated by an empty line. (see an example below, using the output of
//! [spaCy](https://spacy.io/) in python). 
//! * For multiple inputs of the same type, the program will expect 3 arguments from the command line :
//!     * input type ("c" = constituency / "d" = dependency), String
//!     * input file path, String
//!     * output path, String
//!  
//! See an example below. 
//! 
//! # Usage examples
//! ## Constituency
//! 
//! This example shows how to use the API in order to produce a png from a single parsed constituency string.
//! 
//! ```
//! 
//! // Example sentence: 
//! // The people watch the game
//! // (S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))
//! 
//! use parsed_to_plot::String2Tree;
//! use parsed_to_plot::Tree2Plot;
//! use parsed_to_plot::String2StructureBuilder;
//! use parsed_to_plot::Structure2PlotBuilder;
//! 
//!
//! let mut constituency = String::from("(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
//! let mut string2tree: String2Tree = String2StructureBuilder::new();
//! string2tree.build(&mut constituency).unwrap(); // build the tree from the string
//! let tree = string2tree.get_structure();
//! 
//! // build plot from tree and save
//! let save_to: &str = "Output/constituency_plot.png";
//! let mut tree2plot: Tree2Plot = Structure2PlotBuilder::new(tree);
//! tree2plot.build(save_to);
//! 
//! ```
//! 
//! ## Dependency  
//! 
//! This example shows how to use the API in order to produce a png from a single sentence in conll.
//! 
//! ```
//! 
//! // The people watch the game
//! //  0   The the det _   _   1   det   _   _
//! //  1	people	people	NOUN	_	_	2	nsubj	_	_
//! //  2	watch	watch	VERB	_	_	2	ROOT	_	_
//! //  3	the	the	DET	_	_	4	det	_	_
//! //  4	game	game	NOUN	_	_	2	dobj	_	_
//! 
//! 
//! use parsed_to_plot::String2Conll;
//! use parsed_to_plot::Conll2Plot;
//! use parsed_to_plot::String2StructureBuilder;
//! use parsed_to_plot::Structure2PlotBuilder;
//! 
//! let mut dependency = [
//!     "0	The	the	DET	_	_	1	det	_	_",
//!     "1	people	people	NOUN	_	_	2	nsubj	_	_",
//!     "2	watch	watch	VERB	_	_	2	ROOT	_	_",
//!     "3	the	the	DET	_	_	4	det	_	_",
//!     "4	game	game	NOUN	_	_	2	dobj	_	_"
//! ].map(|x| x.to_string()).to_vec();
//! 
//! let mut conll2tree: String2Conll = String2StructureBuilder::new();
//! conll2tree.build(&mut dependency).unwrap(); // build the conll from the vector of strings
//! let tree = conll2tree.get_structure();
//! 
//! // build plot from tree and save
//! let save_to: &str = "Output/dependency_plot.png";
//! let mut conll2plot: Conll2Plot = Structure2PlotBuilder::new(tree);
//! conll2plot.build(save_to);
//! 
//! ```
//! 
//! ## Multiple inputs via file 
//! 
//! You can use multiple inputs of the same type in a file, through the command line, as follows:\
//! ``` cargo run INPUT_TYPE INPUT_FILE OUTPUT_PATH ```\
//! when:\
//! * INPUT_TYPE should be replaced with "c" for constituency or "d" for dependency.
//! * INPUT_FILE should be replaced with a path to a txt file with inputs.
//! * OUTPUT_PATH should be replaced with a path to a requested output dir.
//! 
//! For example: cargo run c constituencies.txt Output
//! Will save png images of constituency trees drawn for the inputs in constituencies.txt, in an Output dir.
//!  
//! 
//! ### Constituency
//! 
//! ```ignore
//! 
//! use parsed_to_plot::Config;
//! use parsed_to_plot::String2Tree;
//! use parsed_to_plot::Tree2Plot;
//! use parsed_to_plot::String2StructureBuilder;
//! use parsed_to_plot::Structure2PlotBuilder;
//! use std::env;
//! 
//! // collect arguments from command line 
//! let args: Vec<String> = env::args().collect();
//! // note: your command line args should translate to something like the following:
//! // let args: Vec<String> = ["PROGRAM_NAME", "c", "Input/constituencies.txt", "ConOutput"].map(|x| x.to_string()).to_vec();
//! 
//! // run configuration protocol and inpectations
//! let sequences = match Config::new(&args) {
//!     Ok(sequences) => Vec::<String>::try_from(sequences).unwrap(),
//!     Err(config) => panic!("{}", config) 
//! };
//! 
//! for (i, mut constituency) in sequences.into_iter().enumerate() {
//!            
//!     println!("working on input number {} ...", i);
//!     let save_to = &Config::get_out_file(&args[3], i.to_string().as_str());
//!     
//!     // build tree from consituency, and also do a DFS to build n_sub_children from trait
//!     let mut string2tree: String2Tree = String2StructureBuilder::new();
//!     string2tree.build(&mut constituency).unwrap();
//!     let tree = string2tree.get_structure();
//!
//!     // build plot from tree
//!     let mut tree2plot: Tree2Plot = Structure2PlotBuilder::new(tree);
//!     tree2plot.build(save_to);
//!
//! }
//! 
//! ```
//! 
//! ### Dependency
//! 
//! ```ignore
//! 
//! use parsed_to_plot::Config;
//! use parsed_to_plot::String2Conll;
//! use parsed_to_plot::Conll2Plot;
//! use parsed_to_plot::String2StructureBuilder;
//! use parsed_to_plot::Structure2PlotBuilder;
//! use std::env;
//! 
//! // collect arguments from command line
//! let args: Vec<String> = env::args().collect();
//! // note: your command line args should translate to something like the following:
//! // let args: Vec<String> = ["PROGRAM_NAME", "d", "Input/conll.txt", "DepOutput"].map(|x| x.to_string()).to_vec();
//! 
//! // run configuration protocol and inpectations
//! let sequences = match Config::new(&args) {
//!     Ok(sequences) => Vec::<Vec<String>>::try_from(sequences).unwrap(),
//!     Err(config) => panic!("{}", config) 
//! };
//! 
//! for (i, mut dependency) in sequences.into_iter().enumerate() {
//!        
//!     println!("working on input number {} ...", i);
//!     let save_to = &Config::get_out_file(&args[3], i.to_string().as_str());
//!  
//!     // build conll from string
//!     let mut string2conll: String2Conll = String2StructureBuilder::new();
//!     string2conll.build(&mut dependency).unwrap();
//!     let conll = string2conll.get_structure();
//!
//!     // build plot from conll
//!     let mut conll2plot: Conll2Plot = Structure2PlotBuilder::new(conll);
//!     conll2plot.build(save_to);
//!
//! }
//! 
//! ```

mod config;
mod string_2_conll;
mod conll_2_plot;
mod string_2_tree;
mod tree_2_plot;
mod generic_traits;

pub use config::Config;
pub use config::Input;
pub use string_2_tree::String2Tree;
pub use string_2_tree::SubChildren;
pub use string_2_conll::String2Conll;
pub use tree_2_plot::Tree2Plot;
pub use conll_2_plot::Conll2Plot;
pub use generic_traits::String2StructureBuilder;
pub use generic_traits::Structure2PlotBuilder;
pub use generic_traits::Structure2PlotPlotter;
