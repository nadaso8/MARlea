use crate::trial::reaction_network::reaction::{Reaction, term::{Term, solution::Species}};
use csv::ReaderBuilder;
use std::sync::mpsc::Receiver;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use super::trial::reaction_network::reaction::term::solution::Solution;

pub enum SupportedFileType {
CSV(String),
JSON(String),
XML(String),
Unsuported(String),
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
            Some(other_file_type) => Self::Unsuported(other_file_type.to_string()),
            _=> panic!("no_file_extension_found")
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
                        
                        // Filter empty rows out of the returned reader and panic if reader encounters an error
                        let records = reader.records().filter_map(
                            |record|
                            match record {
                                Ok(mut string) => {
                                    string.trim();
                                    if string[0].is_empty() && string[1].is_empty() {
                                        None
                                    } else {
                                        Some(string)
                                    }
                                }
                                Err(error) => {panic!("{}", error)}
                            }
                        );
                        let mut reactions = HashSet::new();
                        
                        for record in records {
                            let mut reactants: HashSet<Term> = HashSet::new();
                            let mut products: HashSet<Term> = HashSet::new();
                            
                            // Find "=>" and split sides
                            let sides: Vec<&str> = record[0].split("=>").collect();
                            if sides.len() != 2 {
                                panic!("Invalid reaction format - expected 'reactants => products' but received [{}]", record[0].to_string());
                            }
                            let left_side: Vec<&str> = sides[0].split('+').collect();
                            let right_side: Vec<&str> = sides[1].split('+').collect();


                            
                            // Split left side fields into space sign delimited sub fields and parse as reactants
                            for term_string in left_side {
                                if let Some(term) = Term::from(term_string) {reactants.insert(term);}

                            }             

                            // Split right side fields into space delimited subfields and parse as products
                            for term_string in right_side {
                                if let Some(term) = Term::from(term_string) {products.insert(term);}
                            }

                            // Parse the last field as reaction_rate
                            let rate_str = record[1].trim();
                            let rate = rate_str.parse::<u64>().unwrap_or_else(|_| panic!("Invalid reaction rate '{}' provided", record[1].to_string()));

                            reactions.insert(Reaction::new(reactants, products, rate));
                        }

                        return reactions; 
                        
                    }
                    Err(error) => panic!("{}", error),
                } // end of inner match { OK(CSVReader) vs. Error(String) }  
            }, // End of handling CSV files
            Self::JSON(_path) => todo!(),
            Self::XML(_path) => todo!(), 
            Self::Unsuported(file_type) => panic!("Unsupported file type: found {}, expects CSV", file_type), 
            

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
            Self::XML(_path) => unimplemented!(),
            Self::JSON(_path) => unimplemented!(),
            Self::Unsuported(file_type) => panic!("Unsupported file type: found {}, expects CSV", file_type), 
        }
    }



    pub fn write_solution(&self, stable_solution: Vec<(String, f64)>) {
        match self {
            Self::CSV(path) => {
                let mut output_file = csv::WriterBuilder::new().from_path(path).unwrap();
                
                for entry in stable_solution  {
                    output_file.write_record([entry.0, entry.1.to_string()]).unwrap();
                }
            },
            Self::JSON(_path) => todo!(), // implement JSON writing
            Self::XML(_path) => todo!(), // implement XML writing
            Self::Unsuported(other_file_type) => panic!("tried to write unsuported file type {}", other_file_type),
        }
    }
}

enum WriterType {
    CSV{writer: csv::Writer<std::fs::File>, header_written: bool},
}

impl WriterType {
    fn from(file: &SupportedFileType, id: usize) -> Self {
        match file {
            SupportedFileType::CSV(path) => {
                let mut path_with_id = id.to_string();
                path_with_id.push_str(&path);

                return WriterType::CSV{
                    writer: csv::WriterBuilder::new()
                        .flexible(true)
                        .from_path(path_with_id).unwrap(),
                    header_written: false
                }
            }   
            _ => unimplemented!()
        }
    }
}
pub struct TimelineWriter {
    timeline_file: SupportedFileType,
    temp_sub_files: HashMap<usize, WriterType>,
    step_stream: Receiver<(Solution, usize)>,
}

impl TimelineWriter {
    pub fn new(file: SupportedFileType, step_stream: Receiver<(Solution, usize)>) -> Self {
        let temp_sub_files = HashMap::new();

        return TimelineWriter {timeline_file: file, temp_sub_files, step_stream};
    }
    pub fn begin_listen(mut self) {
        loop {
            match self.step_stream.recv() {
                Ok((solution, id)) => {

                    // ensure that a wiriter has been generated for given ID
                    if !self.temp_sub_files.contains_key(&id) {
                        self.temp_sub_files.insert(id, WriterType::from(&self.timeline_file, id));
                    }

                    // Write data with writer at given ID
                    self.temp_sub_files.entry(id)
                    .and_modify(|sub_file|match sub_file {
                        WriterType::CSV{writer,header_written} => {

                            // only write names if there is no names header field in the sub file
                            if !*header_written {
                                let mut names = Vec::new();
                                let mut counts = Vec::new();
                                for (species_name, species_count) in solution.into_iter() {
                                    names.push(species_name.to_string());
                                    counts.push(species_count.to_string());
                                }
                                writer.write_record(names).unwrap();
                                writer.write_record(counts).unwrap();
                                writer.flush().unwrap();
                                *header_written = true;
                            } 

                            // otherwise just ignore them completely and write the species as a new record
                            else {
                                let mut counts = Vec::new();
                                for (_species_name, species_count) in solution.into_iter() {
                                    counts.push(species_count.to_string());
                                }
                                writer.write_record(counts).unwrap();
                                writer.flush().unwrap();
                            }
                        }
                    });
                }
                Err(_msg) => {
                    println!("Stream to Timeline Writer dropped\nShutting down...");
                    // combining files is unimplemented
                    return;
                }
            }
        }
    }
}