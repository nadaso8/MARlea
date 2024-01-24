/// Author: Marceline Sorensen 
/// Email: nadaso8th@gmail.com
/// Date: 15/1/2024
/// 
/// #Description
/// This is the main module for marlea. It manages initializing the GUI as well as command line/headless opperation. 
/// 
/// #Usage 
/// MARlea_simulator.exe <Query> <Options>
/// 
/// ## Queries 
/// None : starts the application with default GUI allowing user to graphically select a rxn network 
/// "Simulate" <input_file> : starts the application and gui with
///  only runtime feedback plot. Then attempts to immediately 
///  parse and simulate the rxn network in input file.  
/// 
/// ## Options 
/// -no_gui : runs the application without starting a gui of any sort dissabling any runtime feedback
/// -o --output <file_path> : saves output to a new file 
/// -r --runtime <seconds> : specifies a time limit after which the application will force quit without finishing the simulation
/// -sensitivity <steps>: specifies the number of steps for which a simulation trial can exist in a semi stable state
/// -t --trials <count> : specifies the number of trials to simulate before taking the final average 
/// -timeline : saves timeline to a new file if no output was specified it will save to the input_file directory 
/// -cores : set the number of workers to allow the compute threadpool
/// 


// Import necessary modules
mod marlea_save;
mod marlea_gui;

use marlea_engine::{
    MarleaEngine,
    MarleaResult,
    MarleaEngineError,
};
use MARlea_parser::{
    MarleaParser,
    MarleaParserError,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "MarleaOpts")]
struct MarleaOpts {
    /// An optional command to allow for starting simulations via the command line without input from the gui.
    /// Additional functionality such as transpilation into a .csv rxn set TBA when I get around to it.  
    #[structopt(subcommand)]
    query: Option<MarleaSubcmd>,
}

#[derive(Debug, StructOpt)]
enum MarleaSubcmd {
    
    simulate {
        /// specify rxn network file to simulate
        /// currently supported extensions
        ///  - .csv
        #[structopt(parse(from_os_str))]
        input: std::path::PathBuf,

        /// start the application in headless mode
        #[structopt(long)]
        no_gui: bool,

        /// specify an output file path
        #[structopt(short, long, parse(from_os_str))]
        output: Option<std::path::PathBuf>,

        /// specify a maximum runtime in seconds
        /// EX: -r 32 will quit the application after 32 seconds have elapsed
        #[structopt(short, long)] 
        runtime: Option<u64>,

        /// ADVANCED! specify maximum count of semi stable a trial may simulate steps before stable state is assumed.
        /// higher values will make the simulation less likely to assume a stable state
        #[structopt(long)]
        sensitivity: Option<usize>,

        /// specify the number of trials to simulate before calculating final average
        #[structopt(short, long)]
        trials: Option<usize>,

        /// specify timeline output file path
        #[structopt(long, parse(from_os_str))]
        timeline: Option<std::path::PathBuf>,

    },
}

#[derive(Debug, Clone)]
enum MarleaError {
    Unknown(&'static str),
    InvalidOptions(&'static str),
    ParserError(),
    SimulationError(MarleaEngineError)
}

fn main() -> Result<(), MarleaError> {
    let opts = MarleaOpts::from_args();

    match opts.query {
        Some(query) => {
            match query {
                MarleaSubcmd::simulate { 
                    input, 
                    no_gui,
                    output, 
                    runtime, 
                    sensitivity, 
                    trials, 
                    timeline 
                } => {
                    // parse network from input path or propagate error
                    let rxn_network = match MarleaParser::parse(&input) {
                        Ok(result) => result,
                        Err(err) => {
                            let msg = match err.clone() {
                                MarleaParserError::InvalidFile(msg) => msg,
                                MarleaParserError::ParseFailed(msg) => msg,
                                MarleaParserError::UnsupportedExt(msg) => msg
                            };

                            println!("{}",msg);

                            return Result::Err(MarleaError::ParserError());
                        },
                    };

                    // check for unimplemented options
                    if no_gui {
                        todo!("no gui is imlemented on current build")
                    }

                    if let Some(output) = output {
                        todo!("no file writer is implemented on current build")
                    }

                    if let Some(path) = timeline {
                        todo!("no file writer is implemented on current build")
                    }

                    // build backend instance from opts
                    let mut marlea_engine_builder = marlea_engine::Builder::new(rxn_network);

                    if let Some(time) = runtime {
                        marlea_engine_builder = marlea_engine_builder.runtime(time);
                    }

                    if let Some(steps) = sensitivity {
                        marlea_engine_builder = marlea_engine_builder.tolerance(steps);
                    }

                    if let Some(count) = trials {
                        marlea_engine_builder = marlea_engine_builder.trials(count);
                    }

                    let (backend, backend_reciever)= marlea_engine_builder.no_response().build();
                    drop(backend_reciever);//drop unused receiver

                    let result = match backend.run() {
                        Ok(result) => match result {
                            MarleaResult::Final(val) => val,
                            MarleaResult::Intermediary(_) => return Result::Err(MarleaError::SimulationError(MarleaEngineError::Unknown("huh?".to_string())))
                        },
                        Err(msg) => return Result::Err(MarleaError::SimulationError(msg))
                    };

                    for (name, avg) in result {
                        println!("{} - {}", name.0, avg.0);
                    }

                    return Result::Ok(());
                },
                _ => {
                    println!("invalid query");
                    return Result::Err(MarleaError::InvalidOptions(&"invalid query"));
                }, 
            }
        },
        None => {
            todo!("implement gui behavior")
        },
    };
    
}
