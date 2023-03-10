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

fn main () {

}

