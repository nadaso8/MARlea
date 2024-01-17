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
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "MARlea")]
struct MARlea {
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
        runtime: Option<u32>,

        /// ADVANCED! specify maximum count of semi stable a trial may simulate steps before stable state is assumed.
        /// higher values will make the simulation less likely to assume a stable state
        #[structopt(long)]
        sensitivity: Option<u32>,

        /// specify the number of trials to simulate before calculating final average
        #[structopt(short, long)]
        trials: Option<u32>,

        /// specify timeline output file path
        #[structopt(long, parse(from_os_str))]
        timeline: Option<std::path::PathBuf>,

        /// specify maximum the number of cores able to be used by marlea_engine
        /// defaults to using all cores availible
        #[structopt(long)]
        cores: Option<u8>,
    },
}

fn main() {
    let opts = MARlea::from_args();

    match opts.query {
        Some(simulate) => {

        },
        None => {
            
        },
    };
    
}
