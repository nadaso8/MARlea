/// Author: Marceline Sorensen 
/// Email: nadaso8th@gmail.com 
/// Date: 08/03/2023
/// 
/// # Description
/// This is the main simulation engine of MARlea it takes a set of reactions as well as a set of initial species valeus. 
/// It uses these to simulate the average stable case of a chemical reaction network stochastically. 
/// 
/// # Arguments (input_path, init_path)
/// - <input_path>
///     Specifies an input file location 
/// - <init_path> 
///     Specifies a file location to read initial species values from
///     Is of type Option which may be None
///     - If none will simply initialize all values to 0
/// - <out_path>
///     Specifies an output file location 
///     Is of type Option which may be None
/// - <num_trials>
///     Specifies the number of trials to be used in making a predicted average
///     Is of type Option which may be None
///     - if None will defualt to 100 trials to provide a simplistic estimation
/// - <max_runtime>
///     Specifies the maximum time the code my run for in seconds
///     Is of type Option which may be None 
///     - if None the simulation will run indefinitely
/// 
/// Accepted file types: 
///     - CSV
///     - XML UNIMPLEMENTED! 
///     - JSON UNIMPLEMENTED!

use std::collections::{HashMap, HashSet};
use std::fs::{File, FileType};
use std::io::prelude::*;
use std::path::Path;
use csv::ReaderBuilder;
use self::trial::reaction_network::{ReactionNetwork, reaction::{Reaction, term::{Term, species::Species}}};


mod trial;

pub struct MarleaEngine {
pub input_path: String,
pub init_path: Option<String>,
pub out_path: Option<String>,
pub out_type: Option<String>,
pub num_trials: Option<usize>,
pub max_runtime: Option<u64>,

prime_network: ReactionNetwork,
}

impl MarleaEngine {
pub fn new(
    input_path: String,
    init_path: Option<String>,
    out_path: Option<String>,
    num_trials: Option<usize>,
    max_runtime: Option<u64>,
) -> Self {
    todo!()
}

pub fn run(&self) -> Result<bool, &'static str> {

    // Set up simulation loop
    let mut time = 0.0;
    let mut trial_count = 0;
    while trial_count <= match self.num_trials{Some(number) => number, None => 100} {

    }

    // Write output file
    todo!();

    Ok(true)
}

}

enum ReactionNetworkParts {
    Reactions(HashSet<Reaction>),
    Solution(HashMap<Species, Species>),
}

enum SupportedFileType {
CSV(String),
XML(String),
JSON(String),
Unsuported,
}

impl SupportedFileType {
    pub fn from(file_path: String) -> Self{
        // get file extension to determine the type
        let extension = Path::new(&file_path)
            .extension()
            .and_then(|os_str| os_str.to_str());

        match extension {
            Some("csv") => Self::CSV(file_path),
            Some("json") => Self::JSON(file_path),
            Some("xml") => Self::XML(file_path),
            _ => Self::Unsuported,
        }
    }

    // A function that parses a file into a `ReactionNetworkParts` enum Type
    pub fn parse_reactions(&self) -> ReactionNetworkParts {
        // Handle different types of supported files, starts here with CSV
        match self {
            Self::CSV(path) => {
                // Read and create CSVReader object
                let reader = ReaderBuilder::new()
                    .has_headers(false) // CSV has no headers/spaces as first row
                    .delimiter(b'+')  // Use plus sign '+' as delimiter
                    .from_path(path); // Path to the CSV file
                
                match reader {
                    Ok(mut reader) => { // If file exists and was opened successfully
                        
                        // Filter records read by the returned CSVReader iterator and Ignore errors
                        let records = reader.records().filter_map(Result::ok);
                        let mut reactions = HashSet::new();
                        
                        for record in records {
                            let mut reactants: HashSet<Term> = HashSet::new();
                            let mut products: HashSet<Term> = HashSet::new();
                            let mut reaction_rate = 0; 
                            
                            // Before "=>" pattern split into plus sign delimited values into fields
                            let sides: Vec<&str> = record[0].split("=>").collect();
                            
                            if sides.len() != 2 {
                                panic!("Invalid reaction format - expected 'reactants => products' but received '{}'", record[0].to_string());
                            }
                            
                            // Split those fields into space delimited sub fields
                            let left_side: Vec<&str> = sides[0].split(' ').collect();
                            
                            for i in (0..left_side.len()).step_by(2) {
                                let name = left_side[i].trim();
                                
                                let coeff_result = left_side[i+1].trim().parse::<usize>();
                                match coeff_result {
                                    Ok(coeff) => {
                                        let term = Term::new(name.to_string(), coeff);
                                        reactants.insert(term);
                                    },
                                    Err(_) => {
                                        let term = Term::new(name.to_string(), 1);
                                        reactants.insert(term);
                                    }
                                }
                                
                                if reactants.len() > 2 {
                                    panic!("Invalid number of reactants - expected two or less while received {}", reactants.len());
                                }
                            }
                            
                            // After "=>" pattern split into plus sign delimited values into fields
                            let right_side: Vec<&str> = sides[1].split(' ').collect();
                            
                            for i in (0..right_side.len()).step_by(2) {
                                let name = right_side[i].trim();
                                
                                let coeff_result = right_side[i+1].trim().parse::<usize>();
                                match coeff_result {
                                    Ok(coeff) => {
                                        let term = Term::new(name.to_string(), coeff);
                                        products.insert(term);
                                    },
                                    Err(_) => {
                                        let term = Term::new(name.to_string(), 1);
                                        products.insert(term);
                                    }
                                }
                                
                                if products.len() > 2 {
                                    panic!("Invalid number of products - expected two or less while received {}", products.len());
                                }
                            }
                            
                            // Parse the last field as reaction_rate
                            let rate_str = record[1].trim();
                            let rate = rate_str.parse::<usize>().unwrap_or_else(|_| panic!("Invalid reaction rate '{}' provided", record[1].to_string()));

                            reactions.insert(Reaction::new(reactants, products, rate));
                        }
                        
                        // return parsed reactions
                        ReactionNetworkParts::Reactions(reactions)
                    },
                    Err(error) => todo!(),
                } // end of inner match { OK(CSVReader) vs. Error(String) }
                
            }, // End of handling CSV files
            Self::JSON(path) => todo!(), // Implement this later for JSON file handling
            Self::XML(path) => todo!(), // Implement this later for XML file handling
            Self::Unsuported => panic!("Unsupported file type"), // When the user inputs an unknown file type.
        } //  End of outer match {Self} (SupportedFileType enum Type)
    }

    fn parse_species_field(field: &str) -> Option<Species> {
        let mut chars = field.chars().filter(|c| !c.is_whitespace());
        match (chars.next(), chars.clone().next()) {
            (Some('"'), Some('"')) => Some(Species::Name(chars.skip(1).take_while(|c| c != &'"').collect())),
            _ => Some(Species::Name(chars.collect())),
        }
    }
    
    fn parse_count_field(field: &str) -> Option<Species> {
        let trimmed = field.trim();
        if let Ok(count) = trimmed.parse::<u64>() {
            Some(Species::Count(count))
        } else {
            None
        }
    }
    
    fn parse_threshold_field(field: &str) -> Option<Species> {
        unimplemented!()
    }
    

    /// Parses initial solution from a reaction network based on the file type (CSV, JSON, XML) 
    /// Self: is a parsed set of reactions which will be added to solution with count of 0 if not specieifed in init data
    /// returns ReactionNetworkParts which contains the parsed initial solution
    pub fn parse_initial_solution(&self, reactions: HashSet<Reaction>) -> ReactionNetworkParts {
        // Get possible species from reactions
        let mut initial_solution: HashMap<Species, Species> = HashMap::new();
        
        for reaction in reactions {
            for reactant in reaction.get_reactants() {
                if !initial_solution.contains_key(&reactant.get_species_name()) {
                    initial_solution.insert(reactant.get_species_name().clone(), Species::Count(0));
                }
            }
            for product in reaction.get_products() {
                if !initial_solution.contains_key(&product.get_species_name()) {
                    initial_solution.insert(product.get_species_name().clone(), Species::Count(0));
                }
            }
        }

        // Match and handle different file types
        match self {
            Self::CSV(path) => { // CSV file handling
                let reader = ReaderBuilder::new()
                    .has_headers(false)
                    .delimiter(b',')
                    .from_path(path);

                match reader { 
                    Ok(mut reader) => {
                        // Get records and filter out any errors
                        let records = reader.records().filter_map(Result::ok);

                        for record in records {
                            // Convert each record's fields into a vec of string slices
                            let fields: Vec<&str> = record.iter().map(|field| field.as_ref()).collect();
                            
                            let mut species_name = Species::Name(String::new());
                            let mut species_count = Species::Count(0);

                            // Ignoring spaces parse first non empty field as Species::name

                            // Ignoring spaces parse second non empty field as Species::count

                            // If any is present, parse third non empty field as species threshold
                                //UNIMPLEMENTED!

                            // Add species reference and data to initial solution HashMap
                            initial_solution.entry(species_name)
                                .and_modify(|count| *count = species_count);
                        }

                        // Return initial solution as part of ReactionNetworkParts
                        ReactionNetworkParts::Solution(initial_solution)
                    },
                    Err(error) => panic!("error occurred while reading csv file: {}", error), // Handle reader error here
                }
            }
            _ => panic!("Unsupported file type"), // Unsupported type handling
        }
    }



    pub fn write(&self, content: &str) -> std::io::Result<()> {
        match self {
            Self::CSV(path) => {
                let mut file = File::create(path)?;
                file.write_all(content.as_bytes())
            },
            Self::JSON(path) => todo!(), // implement JSON writing
            Self::XML(path) => todo!(), // implement XML writing
            Self::Unsuported => Err(std::io::Error::new(
                std::io::ErrorKind::Other, 
                "Unsupported file type"
            )),
        }
    }
}
