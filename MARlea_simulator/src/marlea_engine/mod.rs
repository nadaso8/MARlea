use std::collections::{HashMap, HashSet};
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

use std::fs::File;
use std::io::prelude::*;
use csv::ReaderBuilder;

use self::trial::reaction_network::reaction::{Reaction, term::{Term, species::Species}};


mod trial;

pub struct MarleaEngine {
    input_path: String,
    init_path: Option<String>,
    out_path: Option<String>,
    num_trials: Option<usize>,
    max_runtime: Option<u64>,
}

impl MarleaEngine {
    pub fn new(
        input_path: String,
        init_path: Option<String>,
        out_path: Option<String>,
        num_trials: Option<usize>,
        max_runtime: Option<u64>,
    ) -> Self {
        Self {
            input_path,
            init_path,
            out_path,
            num_trials,
            max_runtime,
        }
    }

    pub fn run(&self) -> Result<bool, &'static str> {
        // Read in reactions data
        let reactions: HashSet<Reaction> = HashSet::new();
        let mut input_file = File::open(&self.input_path)
            .map_err(|_| "Failed to open input file")?;
        let mut input_data = String::new();
        input_file.read_to_string(&mut input_data)
            .map_err(|_| "Failed to read input file")?;
        let input_reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(input_data.as_bytes());

        // Read in initial species values (or create new ones)
        let solution: HashMap<&Species,Species> = HashMap::new();
        let init_values: Option<String> = match &self.init_path {
            Some(path) => {
                let mut init_file = File::open(path)
                    .map_err(|_| "Failed to open initial values file")?;
                let mut init_data = String::new();
                init_file.read_to_string(&mut init_data)
                    .map_err(|_| "Failed to read initial values file")?;
                // TODO: parse data from file into initial values struct
                unimplemented!()
            },
            None => {
                // TODO: generate random initial values
                unimplemented!()
            }
        };

        // Set up simulation loop
        let mut time = 0.0;
        let mut trial_count = 0;
        while trial_count <= match self.num_trials{Some(number) => number, None => 100} {

        }

        // Write or print simulation output


        Ok(true)
    }

}