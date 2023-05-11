
//
// Under MIT license
//

use std::error::Error;
use std::fs::create_dir_all;

const ARGS_LENGTH: usize = 4;
const IMG_TYPE: &str = ".png";
const DEPENDENCY: &str = "d";
const CONSTITUENCY: &str = "c";

pub mod configure_structures {

    use std::error::Error;
    use std::fs::{File, self};
    use std::io::{self, BufRead};
    use std::vec;

    /// Dependency is a vector of dependency string vectors.
    #[derive(Clone)]
    pub(in crate::config) struct Dependency {}

    /// Constituency is a vector of constituency string.
    #[derive(Clone)]
    pub(in crate::config) struct Constituency {}

    /// An enum that wraps the data types supported.
    #[derive(Clone, Debug)]
    pub enum DataType {
        Dependency(Vec<Vec<String>>),
        Constituency(Vec<String>)
    }

    impl TryFrom<DataType> for Vec<String> {
        type Error = Box<dyn Error>;

        fn try_from(value: DataType) -> Result<Self, Self::Error> {
            match value {
                DataType::Constituency(x) => Ok(x),
                _ => Err(format!("could not convert value {:?} to {}", value, std::any::type_name::<Self>()).into())
            }
        }
    }

    impl TryFrom<DataType> for Vec<Vec<String>> {
        type Error = Box<dyn Error>;
        
        fn try_from(value: DataType) -> Result<Self, Self::Error> {
            match value {
                DataType::Dependency(x) => Ok(x),
                _ => Err(format!("could not convert value {:?} to {}", value, std::any::type_name::<Self>()).into())
            }
        }
    }

    impl IntoIterator for DataType {
        type Item = Vec<String>;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            match self {
                DataType::Constituency(x) => vec![x].into_iter(),
                DataType::Dependency(x) => x.into_iter()
            }
        }
    }

    pub(in crate) trait Saver {

        fn save_output(&self, out_path: &str) -> Result<(), Box<dyn Error>>;
    }

    impl Saver for Vec<Vec<String>> {

        fn save_output(&self, out_path: &str) -> Result<(), Box<dyn Error>> {
            
            // each string is a token, line with fields sep by \t
            let mut out_vec = Vec::new();
            for vec in self {
                let string_vec = vec.join("\n").to_owned();
                out_vec.push(string_vec);
            }
            out_vec.save_output(out_path)?;
            Ok(())
        }
    }
    impl Saver for Vec<String> {

        fn save_output(&self, out_path: &str) -> Result<(), Box<dyn Error>> {

            let out_string = self.join("\n");
            fs::write(out_path, out_string).expect("Unable to write file");            
            Ok(())
        }
    }

    /// A trait that supplies reading functionallity over input files.
    /// The trait is used from within the config implementation.
    /// Not called directly by the user.
    pub (in crate::config) trait Reader {
        type Out;
        fn read_input(&self, file_path: &str) -> Result<Self::Out, Box<dyn Error>>;
    }

    impl Reader for Dependency {
        type Out = DataType;
        fn read_input(&self, file_path: &str) -> Result<Self::Out, Box<dyn Error>> {

            // load dependencies
            let in_file = File::open(file_path)?; 
            let lines = io::BufReader::new(in_file).lines();

            let mut sequences = Vec::new();
            let mut depencdency: Vec<String> = Vec::new();
            for (i, line) in lines.enumerate() {
                
                // skip empty first line is exists
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

            return Ok(DataType::Dependency(sequences))

        }
    }


    impl Reader for Constituency {
        type Out = DataType;
        fn read_input(&self, file_path: &str) -> Result<Self::Out, Box<dyn Error>> {

            let in_file = File::open(file_path)?; 
            let lines = io::BufReader::new(in_file).lines();
            let sequences = lines.map(|line| line
                .expect("un string-like line"))
                .collect::<Vec<String>>();
            
            return Ok(DataType::Constituency(sequences))
        }
    }
}

/// An empty struct of configuration process 
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Config {}

use self::configure_structures::{Dependency, Constituency, DataType, Reader};

impl Config {

    ///
    /// A get method to retrive the complete output path (into image convertion)
    /// 
    pub fn get_out_file(out_dir_path: &str, file_name: &str) -> String {
        return out_dir_path.to_string() + "/" + file_name + IMG_TYPE;
    }

    ///
    /// Crate an output directory as requested if possible
    /// 
    pub fn make_out_dir(out_dir: &String) -> Result<(), String> {
        match create_dir_all(out_dir) {
            Ok(()) => Ok(()),
            Err(e) => return Err(format!("create_dir_all error: {}", e.to_string()))
        }
    }


    ///
    /// The Config trait receives the command line array of inputs and parses it.
    /// Expects 3 arguments : Letter selector, input text file, Requested output path to save png images.
    /// Returns a Result over DataType.
    /// 
    /// Examples are given in the lib.rs file
    /// 
    pub fn new(args: &[String]) -> Result<DataType, Box<dyn Error>> {

        // validate number of arguments supplied
        if args.len() != ARGS_LENGTH {
            let custom_err = format!("there should be {} arguments supllied: constituency file and output dir, found {} ", ARGS_LENGTH, args.len());
            return Err(custom_err.into());
        }

        // load output directory path and try to create:
        Config::make_out_dir(&args[3])?;

        // load inputs
        if CONSTITUENCY == args[1] {
            return Box::new (Constituency {}).read_input(&args[2]);
        } else if DEPENDENCY == args[1] {
            return Box::new (Dependency {}).read_input(&args[2]);
        } else {
            return Err(format!("Resulted in error in parsing: input selector {} is invalid", args[1]).into());
        }

    }

}


#[cfg(test)]
mod tests {

    use std::error::Error;
    use super::configure_structures::DataType;
    use super::Config;

    fn config_test_template(selector: &str, input_path: &str, output_path: &str, additional: Option<&str>) -> Result<DataType, Box<dyn Error>> {
        
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