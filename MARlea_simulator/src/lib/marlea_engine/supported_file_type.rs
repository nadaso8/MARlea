
use crate::marlea_engine::{reaction_network::reaction::{Reaction, term::{Term, species::Species}}};
use std::fs::File;
use csv::ReaderBuilder; 
use std::io::Write;
use std::path::Path;
use std::collections::{HashMap, HashSet};
pub enum SupportedFileType {
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
    pub fn parse_reactions(&self) -> HashSet<Reaction> {
        // Handle different types of supported files, starts here with CSV
        match self {
            Self::CSV(path) => {
                // Read and create CSVReader object
                let reader = ReaderBuilder::new()
                    .has_headers(false)
                    .delimiter(b',')  
                    .from_path(path); 
                
                match reader {
                    Ok(mut reader) => { // If file exists and was opened successfully
                        
                        // Filter records read by the returned CSVReader iterator and Ignore errors
                        let records = reader.records().filter_map(Result::ok);
                        let mut reactions = HashSet::new();
                        
                        for record in records {
                            let mut reactants: HashSet<Term> = HashSet::new();
                            let mut products: HashSet<Term> = HashSet::new();
                            
                            // Find "=>" and split sides
                            let sides: Vec<&str> = record[0].split("=>").collect();
                            let left_side: Vec<&str> = sides[0].split('+').collect();
                            let right_side: Vec<&str> = sides[1].split('+').collect();

                            if sides.len() != 2 {
                                panic!("Invalid reaction format - expected 'reactants => products' but received '{}'", record[0].to_string());
                            }
                            
                            // Split left side fields into space sign delimited sub fields and parse as reactants
                            for term in left_side {
                                reactants.insert(Term::from(term));
                            }             

                            // Split right side fields into space delimited subfields and parse as products
                            for term in right_side {
                                products.insert(Term::from(term));
                            }

                            // Parse the last field as reaction_rate
                            let rate_str = record[1].trim();
                            let rate = rate_str.parse::<usize>().unwrap_or_else(|_| panic!("Invalid reaction rate '{}' provided", record[1].to_string()));

                            reactions.insert(Reaction::new(reactants, products, rate));
                        }

                        return reactions; 
                        
                    }
                    Err(error) => panic!("{}", error),
                } // end of inner match { OK(CSVReader) vs. Error(String) }  
            }, // End of handling CSV files
            Self::JSON(path) => todo!(),
            Self::XML(path) => todo!(), 
            Self::Unsuported => panic!("Unsupported file type"), 
            

        } //  End of outer match {Self} (SupportedFileType enum Type)
    }
    
    /// Parses initial solution from a reaction network based on the file type (CSV, JSON, XML) 
    /// Self: is a parsed set of reactions which will be added to solution with count of 0 if not specieifed in init data
    pub fn parse_initial_solution(&self, initial_solution: &mut HashMap<Species, Species>) {

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
                            // Split each record string based on commas and filter empty fields
                            let fields: Vec<&str> = record.iter()
                                .filter_map(|field| {
                                    let trimmed_field = field.trim();
                                    if trimmed_field.is_empty() { None } else { Some(trimmed_field) }
                                })
                                .collect();
                        
                            // If there is less than 2 fields, disregard this record
                            if fields.len() < 2 {
                                continue;
                            }
                        
                            // Ignoring spaces parse first non empty field as Species::name
                            let mut species_name = Species::Name(String::new());
                            if let Some(name_str) = fields.get(0).map(|s| s.trim()).filter(|s| !s.is_empty()) {
                                species_name = Species::Name(name_str.to_owned());
                            }
                        
                            // Ignoring spaces parse second non empty field as Species::count
                            let mut species_count = Species::Count(0);
                            if let Some(count_str) = fields.get(1).map(|s| s.trim()).filter(|s| !s.is_empty()) {
                                if let Ok(count_int) = count_str.parse::<u64>() {
                                    species_count = Species::Count(count_int);
                                }
                            }
                        
                            // If any is present, parse third non empty field as species threshold
                            if let Some(_threshold_str) = fields.get(2).map(|s| s.trim()).filter(|s| !s.is_empty()) {
                                unimplemented!()
                            }

                            // Add species name and data t
                            initial_solution.entry(species_name)
                                .and_modify(|count| *count = species_count);
                        }
                    },
                    Err(error) => panic!("error occurred while reading csv file: {}", error), // Handle reader error here
                }
            }
            Self::XML(path) => unimplemented!(),
            Self::JSON(path) => unimplemented!(),
            _ => panic!("Unsupported file type"),
        }
    }



    pub fn write(&self, content: &String) -> std::io::Result<()> {
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