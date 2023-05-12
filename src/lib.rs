//!
//! 
//! Plots constituency trees and dependency trees given by strings. 
//! 
//! 
//! # Input-Output
//! 
//! * The API expects a string input. Multiple string inputs can be delivered in a file, through the command-line.
//! * For constituency trees, the program takes a parsed string given in one line. The string can be syntactic, for example
//! such that represents phrases and parts-of-speech (like the structure in [Berkeley Neural Parser](https://pypi.org/project/benepar/)
//! in python). Such strings will have "double leaves" (see an example below). Alternatively, the strings can have singular leaves,
//! representing, for example, mathematical expressions.
//! * For dependency trees, the programs takes a conll format, in which every token has 10 fields, separated by tab, and
//! presented in a new line. Sentences are separated by an empty line. (see an example below, using an output from
//! [spaCy](https://spacy.io/) in python). 
//! * For multiple inputs of the same type, the program expects 3 arguments from the command line :
//!     * input type ("c" = constituency / "d" = dependency), String
//!     * input file path, String
//!     * output path, String
//!  
//! See examples below. 
//! 
//! # Usage examples
//! ## Constituency
//! 
//! How to use the API in order to produce a png from a single parsed constituency string:
//! 
//! ```rust
//! // Example parsed sentence:
//! // (S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))
//!
//! use parsed_to_plot::Config;
//! use parsed_to_plot::String2Tree;
//! use parsed_to_plot::Tree2Plot;
//! use parsed_to_plot::String2StructureBuilder;
//! use parsed_to_plot::Structure2PlotBuilder;
//! 
//! let mut constituency = String::from("(S (NP (det The) (N people)) (VP (V watch) (NP (det the) (N game))))");
//! let mut string2tree: String2Tree = String2StructureBuilder::new();
//! string2tree.build(&mut constituency).unwrap(); // build the tree from the string
//! let tree = string2tree.get_structure();
//!
//! // build plot from tree and save
//! Config::make_out_dir(&"Output".to_string()).unwrap();
//! let save_to: &str = "Output/constituency_plot.png";
//! let mut tree2plot: Tree2Plot = Structure2PlotBuilder::new(tree);
//! tree2plot.build(save_to).unwrap();
//! ```
//! 
//! ## Dependency  
//! 
//! How to use the API in order to produce a png from a single conll format:
//! 
//! ```rust
//! // Example conll:
//! //  0   The the det _   _   1   det   _   _
//! //  1	people	people	NOUN	_	_	2	nsubj	_	_
//! //  2	watch	watch	VERB	_	_	2	ROOT	_	_
//! //  3	the	the	DET	_	_	4	det	_	_
//! //  4	game	game	NOUN	_	_	2	dobj	_	_
//! 
//! use parsed_to_plot::Config;
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
//! let mut string2conll: String2Conll = String2StructureBuilder::new();
//! string2conll.build(&mut dependency).unwrap(); // build the conll from the vector of strings
//! let conll = string2conll.get_structure();
//!
//! // build plot from conll and save
//! Config::make_out_dir(&"Output".to_string()).unwrap();
//! let save_to: &str = "Output/dependency_plot.png";
//! let mut conll2plot: Conll2Plot = Structure2PlotBuilder::new(conll);
//! conll2plot.build(save_to).unwrap();
//! ```
//! 
//! ## Multiple inputs via file 
//! 
//! You can use a combination of the API and command-line to process multiple inputs of the same type through a file.
//! The command-line format is as follows:
//! ```text
//! cargo run INPUT_TYPE INPUT_FILE OUTPUT_PATH
//! ```
//! 
//! when:
//! * INPUT_TYPE should be replaced with "c" for constituency or "d" for dependency.
//! * INPUT_FILE should be replaced with a path to a txt file with inputs.
//! * OUTPUT_PATH should be replaced with a path to a requested output dir.
//! 
//! For example, you can enter multiple constituencies by using the following command + code (the dependency equivalent is similar) :
//! 
//! ```text
//! cargo run c constituencies.txt Output 
//! ```
//! 
//! ```ignore
//! use parsed_to_plot::Config;
//! use parsed_to_plot::String2Tree;
//! use parsed_to_plot::Tree2Plot;
//! use parsed_to_plot::String2StructureBuilder;
//! use parsed_to_plot::Structure2PlotBuilder;
//! use std::env;
//! 
//! // collect arguments from command line 
//! let args: Vec<String> = env::args().collect();
//! // note: your command line args should translate to something similar to the following:
//! // let args: Vec<String> = ["PROGRAM_NAME", "c", "Input/constituencies.txt", "ConOutput"].map(|x| x.to_string()).to_vec();
//! 
//! // run configuration protocol and inspectations
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
//!     // build tree from consituency
//!     let mut string2tree: String2Tree = String2StructureBuilder::new();
//!     string2tree.build(&mut constituency).unwrap();
//!     let tree = string2tree.get_structure();
//!
//!     // build plot from tree
//!     let mut tree2plot: Tree2Plot = Structure2PlotBuilder::new(tree);
//!     tree2plot.build(save_to).unwrap();
//! }
//! ```
//! 
//! Those will save png images of constituency trees drawn for the inputs in constituencies.txt, in an Output dir.
//! 
//! ##  String reconstruction
//! 
//! As of version 0.2.0 you can create a string from a built structure, tree or Vec<Token>. This can be useful, for example,
//! to assert the built tree made from a string x, by making sure that x = Structure2String(String2Structure(x)).
//! For example, on a dependency string (the constituency equivalent is similar) :
//! 
//! ```rust
//! //  0   The the det _   _   1   det   _   _
//! //  1	people	people	NOUN	_	_	2	nsubj	_	_
//! //  2	watch	watch	VERB	_	_	2	ROOT	_	_
//! //  3	the	the	DET	_	_	4	det	_	_
//! //  4	game	game	NOUN	_	_	2	dobj	_	_
//!
//! use parsed_to_plot::Config;
//! use parsed_to_plot::String2Conll;
//! use parsed_to_plot::Conll2String;
//! use parsed_to_plot::String2StructureBuilder;
//! use parsed_to_plot::Structure2PlotBuilder;
//! 
//! let example = [
//!    "0	The	the	DET	_	_	1	det	_	_",
//!    "1	people	people	NOUN	_	_	2	nsubj	_	_",
//!    "2	watch	watch	VERB	_	_	2	ROOT	_	_",
//!    "3	the	the	DET	_	_	4	det	_	_",
//!    "4	game	game	NOUN	_	_	2	dobj	_	_"
//! ].map(|x| x.to_string()).to_vec();
//! let mut dependency = example.clone();
//! 
//! let mut string2conll: String2Conll = String2StructureBuilder::new();
//! string2conll.build(&mut dependency).unwrap(); // build the conll from the vector of strings
//! let conll = string2conll.get_structure();
//!
//! // from v0.2.0 - reconstruction of the original dependency from the built conll
//! Config::make_out_dir(&"Output".to_string()).unwrap();
//! let save_to: &str = "Output/dependency_reconstruction.txt";
//! let mut conll2string: Conll2String = Structure2PlotBuilder::new(conll);
//! conll2string.build(save_to).unwrap();
//! let dependency_reproduction = conll2string.get_conll();
//! assert_eq!(dependency_reproduction, example);
//! ```
//! 
//! # References
//! * I used the crates: [id-tree](https://crates.io/crates/id_tree), [plotters](https://crates.io/crates/plotters).
//! * I used [spaCy](https://spacy.io/) to create a couple of dependency-parsed examples for illustration.
//! 
//! # License
//! Under MIT license. 
//! 
//! 

mod config;
mod string_2_tree;
mod string_2_conll;
mod tree_2_plot;
mod conll_2_plot;
mod tree_2_string;
mod conll_2_string;
mod sub_tree_children;
mod generic_traits;
mod generic_enums;

pub use config::Config;
pub use string_2_tree::String2Tree;
pub use string_2_conll::String2Conll;
pub use tree_2_plot::Tree2Plot;
pub use conll_2_plot::Conll2Plot;
pub use tree_2_string::Tree2String;
pub use conll_2_string::Conll2String;
pub use generic_traits::generic_traits::String2StructureBuilder;
pub use generic_traits::generic_traits::Structure2PlotBuilder;