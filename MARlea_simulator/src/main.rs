/// Author: Marceline Sorensen 
/// Email: nadaso8th@gmail.com
/// Date: 09/03/2023
/// 
/// #Description
/// This is a command line program used in the simulation of DNA based chemical Reaction Networks. 
/// It takes some CSV as input preforms stochastic simulation on the provided system 
/// then prints results either to a specified output file or the command line. 
/// 
/// #Usage 
/// From the command line this function takes arguments in the following format
/// <`Query`, `Input File Path`, `Options`>
/// 
/// ## Queries 
/// - `settings` UNIMPLEMENTED!
///     Prints current settings file and defaults to console
/// 
/// - `validate` UNIMPLEMENTED!
///     Validates the syntax of a provided input file 
/// 
/// - `simulate`
///     Simulates a chemical reaction network 
/// 
/// - `help`
///     Prints Usage to command line
/// 
/// ## Options 
/// 
/// - `-Init <Initial Condition File Path>`
///     Specifies a file to read starting conditions from.
///     If ommited it's assumed that starting conditions are provided in a marked section of the Input file
///     If not provided in either location it's assumed that all species start at count 0
/// 
/// - `-Out <Output File Path>`
///     Specifies a file where the program should write it's results
///     If ommited program will only print to command line
///     
/// 
/// - `-Trials <Numbe Of Trials To Simulate>`
///     Specifies the number of times the simulation should simulate the chemical reaction network. 
/// 
/// - `-Time <Maximum Runtime>`
///     Specifies the maximum time the simulation is allowed to run for. 
/// 


// Import necessary modules
use std::{path::{Path, PathBuf}, process::ExitStatus};
use futures::stream::TryBuffered;
use structopt::StructOpt;
use crate::lib::marlea_engine;

mod lib;

#[derive(Debug, StructOpt)]
#[structopt(name = "Marlea", about = "A command line program for simulating DNA based chemical reaction networks")]

// Define Query enum
#[derive(PartialEq)]
enum Query {
    #[structopt(name = "settings")]
    //Settings,
    #[structopt(name = "validate")]
    //Validate,
    #[structopt(name = "simulate")]
    Simulate,
    #[structopt(name = "help")]
    Help,
}


#[derive(Debug, StructOpt)]
#[structopt(
    name = "Marlea",
    about = "A stochastic CRN simulator"
)]
struct MarleaOpts {
    query: Query,
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
    #[structopt(short="-i", long="--init")]
    init_file: Option<String>,
    #[structopt(short="-o", long="--out")]
    output_file: Option<String>,
    #[structopt(short="-timeline", long="-timeline")]
    output_timeline: Option<String>,
    #[structopt(short="-t", long="--trials")]
    num_trials: Option<usize>,
    #[structopt(short="-r", long="--runtime")]
    max_runtime: Option<u64>,
    #[structopt(short="-s", long="--stability_tolerance")]
    max_semi_stable_steps: Option<i32>,
}



impl std::str::FromStr for Query {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            //"settings" => Ok(Query::Settings),
            //"validate" => Ok(Query::Validate),
            "simulate" => Ok(Query::Simulate),
            "help" => Ok(Query::Help),
            _ => Err(format!("Invalid query '{}'", s)),
        }
    }
}


fn main () {
    let opts = MarleaOpts::from_args();

    // Match query and perform appropriate action
    match opts.query {
        // Just print Usage information if `help` query is provided
        Query::Help => println!("{}", MarleaOpts::clap().about(&*format!(
            "A command line program for simulating DNA based chemical reaction networks.
            
            This program takes some CSV (other formats remain unimplemented) as input and performs stochastic simulation on the provided system,
            then prints results either to a specified output file or the command line.
            
            Usage: marlea <QUERY> <INPUT_FILE> [Options]
            Arguments:
                <QUERY>                          Specify the operation/query to perform. Possible values: \"settings\", \"validate\", \"simulate\", \"help\".
                <INPUT_FILE>                     Input file path to use.
            
            Options:
                -i, --init <FILE_NAME>      Specifies a file to read starting conditions from. If omitted it is assumed that starting conditions are provided in a marked section of the input file; if not provided in either location it is assumed that all species start at count 0.
                -o, --out <FILE_NAME>    Specifies a file where the program should write its results. If omitted program will only print to the command line.
                -t, --trials <NUM_TRIALS>    Specifies the number of times the simulation should simulate the chemical reaction network.
                -r, --runtime <MAX_RUNTIME> Specifies the maximum time the simulation is allowed to run for in seconds.
                -s, --stability_tolerance <SEMI_STABLE_TRIALS> **Advanced** Specifies the number of successive semi stable trials to run before terminated"
        ))),

        // If `simulate` query is provided, create new instance of MarleaEngine with parsed options, then run it
        Query::Simulate => {
            let engine = marlea_engine::MarleaEngine::new(
                opts.input_file.to_string_lossy().into_owned(), 
                opts.init_file, 
                opts.output_file, 
                opts.output_timeline,
                opts.num_trials, 
                opts.max_runtime,
                opts.max_semi_stable_steps,
            );
            // Run MarleaEngine
            engine.run();
        },

        // Print error message if unsupported query is requested
        _ => println!("Error: Unsupported query requested.")
    }

    return; 
}
