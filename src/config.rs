

use std::fs::{self, File};
use std::io::{self, BufRead};
use std::vec;

const ARGS_LENGTH: usize = 4;
const IMG_TYPE: &str = ".png";
const DEPENDENCY: &str = "d";
const CONSTITUENCY: &str = "c";

/// Dependency is a vector of dependency string vectors.
#[derive(Clone)]
struct Dependency {}

/// Constituency is a vector of constituency string one-liners.
#[derive(Clone)]
struct Constituency {}

/// An enum that wraps the data types supported.
#[derive(Clone)]
pub enum Input {
    Dependency(Vec<Vec<String>>),
    Constituency(Vec<String>)
}

impl TryFrom<Input> for Vec<String> {
    type Error = ();

    fn try_from(value: Input) -> Result<Self, Self::Error> {
        match value {
            Input::Constituency(x) => Ok(x),
            _ => Err(())
        }
    }
}

impl TryFrom<Input> for Vec<Vec<String>> {
    type Error = ();

    fn try_from(value: Input) -> Result<Self, Self::Error> {
        match value {
            Input::Dependency(x) => Ok(x),
            _ => Err(())
        }
    }
}

impl IntoIterator for Input {
    type Item = Vec<String>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Input::Constituency(x) => vec![x].into_iter(),
            Input::Dependency(x) => x.into_iter()
        }
    }
}


/// A trait to supplies reading functionallity over input files.
/// The trait is used from within the config implementation.
/// Not called directly by the user.
trait Reader {
    fn read_input(&self, file_path: &str) -> Result<Input, String>;
}

impl Reader for Dependency {
    
    fn read_input(&self, file_path: &str) -> Result<Input, String> {

        // load dependencies
        let lines = match File::open(file_path) {
            Ok(f) => io::BufReader::new(f).lines(),
            Err(e) => return Err(e.to_string())
        };

        let mut sequences = Vec::new();
        let mut depencdency: Vec<String> = Vec::new();
        for (i, line) in lines.enumerate() {
            
            // skip empty first line
            if i == 0 && line.as_ref().unwrap().trim().is_empty() {
                continue;
            }

            if line.as_ref().unwrap().trim().is_empty() {
                sequences.push(depencdency);
                depencdency = Vec::new();
            } else {
                depencdency.push(line.unwrap());
            }
        }

        if depencdency.len() > 0 {
            sequences.push(depencdency);
        }

        return Ok(Input::Dependency(sequences))

    }
}


impl Reader for Constituency {

    fn read_input(&self, file_path: &str) -> Result<Input, String> {

        // load constituencies
        let mut sequences = Vec::new();
        let lines = match File::open(file_path) {
            Ok(f) => io::BufReader::new(f).lines(),
            Err(e) => return Err(e.to_string())
        };

        for line in lines {
            sequences.push(line.unwrap());
        }
 
        return Ok(Input::Constituency(sequences))
    }
}


/// An empty struct of configuration process 
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Config {}

impl Config {

    ///
    /// A get method to retrive the complete output path (into png convertion)
    /// 
    pub fn get_out_file(out_dir_path: &str, file_name: &str) -> String {
        return out_dir_path.to_string() + "/" + file_name + IMG_TYPE;
    }

    ///
    /// The Config trait receives the command line array of inputs and parses it.
    /// Expects 3 arguments : Letter selector, Input text file, Requested output path to save png images.
    /// Returns a Result over Input type.
    /// 
    /// Examples are given in the lib.rs file
    /// 
    pub fn new(args: &[String]) -> Result<Input, String> {

        println!("{:?}", args);

        // validate number of arguments supplied
        if args.len() != ARGS_LENGTH {
            let custom_err = format!("there should be {} arguments supllied: constituency file and output dir, found {} ", ARGS_LENGTH, args.len());
            return Err(custom_err);
        }

        // load output directory path and try to create:
        let out_dir_path = &args[3];
        match fs::create_dir_all(out_dir_path) {
            Ok(()) => {},
            Err(e) => return Err(format!("create_dir_all error: {}", e.to_string()))
        }

        // load inputs
        if CONSTITUENCY == args[1] {
            return Box::new (Constituency {}).read_input(&args[2]);
        } else if DEPENDENCY == args[1] {
            return Box::new (Dependency {}).read_input(&args[2]);
        } else {
            return Err(format!("Resulted in error in parsing: input selector {} is invalid", args[1]));
        
        }

    }

}


#[cfg(test)]
mod tests {

    use super::Input;
    use super::Config;

    fn config_test_template(selector: &str, input_path: &str, output_path: &str, additional: Option<&str>) -> Result<Input, String> {
        
        let mut args = vec![
            "PROGRAM_NAME".to_string(),
            selector.to_string(),
            input_path.to_string(),
            output_path.to_string()
        ];

        match additional {
            Some(additional) => { args.push(additional.to_string()); }
            None => {}
        }

        let sequences = Config::new(&args);
        return sequences;
    }

    #[test]
    #[ignore]
    fn constituency() {

        let sequences = config_test_template("c", "Input/constituencies.txt", "Output", None);
        match sequences {
            Ok(_sequences) => {},
            Err(e) => panic!("{}", e)
        }
        let save_to = &Config::get_out_file("Output", "img".to_string().as_str());
        assert_eq!(save_to, "Output/img.png");
    }

    #[test]
    #[ignore]
    fn dependency() {

        let sequences = config_test_template("d", "Input/conll.txt", "Output", None);
        match sequences {
            Ok(_sequences) => {},
            Err(e) => panic!("{}", e)
        }
        let save_to = &Config::get_out_file("Output", "img".to_string().as_str());
        assert_eq!(save_to, "Output/img.png");
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "Resulted in error in parsing: input selector e is invalid")]
    fn invalid_selector() {
        
        let selector = "e";
        let sequences = config_test_template(selector, "Input/constituency.txt", "Output", None);
        match sequences {
            Ok(_sequences) => {},
            Err(e) => panic!("{}", e)
        }
        
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "there should be 4 arguments supllied: constituency file and output dir, found 5")]
    fn invalid_length() {

        let selector = "c";
        let sequences = config_test_template(selector, "Input/constituency.txt", "Output", Some("---"));
        match sequences {
            Ok(_sequences) => {},
            Err(e) => panic!("{}", e)
        }

    }

}