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

use std::path::PathBuf;
use structopt::StructOpt;

mod marlea_engine;

#[derive(Debug)]
enum Query {
    Simulate(SimulateOption),
    Validate(ValidateOption),
    Help,
}

#[derive(Debug)]
enum InitOption {
    None,
    FilePath(PathBuf),
}

#[derive(Debug)]
enum OutputOption {
    None,
    File(PathBuf)
}

#[derive(Debug)]
struct SimulateOption {
    num_trials: usize,
    max_runtime: f32,
    init_path: InitOption,
    out_path: OutputOption,
}

#[derive(Debug)]
struct ValidateOption {}

#[derive(Debug, StructOpt)]
#[structopt(name = "MARlea D-CRN Stochastic Simulator", usage = "marlea <query <input file> <options>>")]
struct Config {
    #[structopt(subcommand)]
    query: Option<QueryType>,
    #[structopt(short, long, parse(from_os_str), default_value = "./data/sample_input.csv")]
    input_path: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    init: Option<PathBuf>,
    #[structopt(short, long, parse(from_os_str))]
    out: Option<PathBuf>,
    #[structopt(short, long, default_value = "100")]
    trials: usize,
    #[structopt(short, long, default_value = "30")]
    time: i32,
}

impl Config { 
    const USAGE: &'static str = 
    "Usage: marlea <query <input file> <options>>
    From the command line this function takes arguments in the following format
    <Query, Input File Path, Options>

    Queries:
    \tsimulate
    \t\t Simulates a chemical reaction network 

    \tvalidate UNIMPLEMENTED!
    \t\t Validates the syntax of a provided input file 

    \thelp
    \t\t Prints Usage to command line

    Options:

    \t-init <Initial Condition File Path>
    \t\t Specifies a file to read starting conditions from.
    \t\t If ommited it's assumed that starting conditions are provided in a marked section of the Input file
    \t\t If not provided in either location it's assumed that all species start at count 0

    \t-out <Output File Path>
    \t\t Specifies a file where the program should write it's results
    \t\t If ommited program will only print to command line


    \t-trials <Numbe Of Trials To Simulate>
    \t\t Specifies the number of times the simulation should simulate the chemical reaction network. 

    \t-time <Maximum Runtime>
    \t\t Specifies the maximum time the simulation is allowed to run for.";     
}

#[derive(Debug, StructOpt)]
enum QueryType {
    #[structopt(name = "simulate")]
    Simulate {
        #[structopt(flatten)]
        simulate_option: SimulateOptionConfig,
    },
    #[structopt(name = "validate")]
    Validate,
    #[structopt(name = "help")]
    Help,
}

#[derive(Debug, StructOpt)]
struct SimulateOptionConfig {
    #[structopt(long, short, default_value = "100")]
    num_trials: usize,

    #[structopt(long, short, default_value = "30")]
    max_runtime: i32,
}
fn main() {
    let config = Config::from_args();
    let input_path = config.input_path;
    let init_path = match config.init {
        Some(path) => InitOption::FilePath(path),
        None => InitOption::None,
    };
    
    let out_path = match config.out {
        Some(path) => OutputOption::File(path),
        None => OutputOption::None,
    };
    
    let num_trials = config.trials;
    let max_runtime = config.time as f32;

    let query = match config.query {
        Some(query_type) => match query_type {
            QueryType::Simulate { simulate_option } =>
                Query::Simulate(SimulateOption {
                    num_trials: simulate_option.num_trials,
                    max_runtime: simulate_option.max_runtime,
                    init_path,
                    out_path,
                }),
            QueryType::Validate => Query::Validate(ValidateOption {}),
            QueryType::Settings => Query::Settings(SettingsOption {}),
            QueryType::Help => Query::Help,
        },
        None => {
            eprintln!("No subcommand given. Use '-h' option for usage details.");
            return;
        }
    };

    match query {
        Query::Simulate(simulate_option) => {
            let simulator =
                marlea_engine::MarleaEngine::new(&input_path, &simulate_option.init_path, &simulate_option.out_path, simulate_option.num_trials, simulate_option.max_runtime);
            if let Ok(_) = simulator.run() {
                println!("Simulation finished");
            } else {
                eprintln!("Error occurred during simulation");
            }
        }
        Query::Validate(_validate_option) => unimplemented!(),
        Query::Settings(_settings_option) => unimplemented!(),
        Query::Help => println!("{}", Config::USAGE),
    }
}
