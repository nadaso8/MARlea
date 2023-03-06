use std::env::Args;

pub mod simulation;
fn main () {

    let args: vec<String> = env::args().collect;

let input_stream = csv::ReaderBuilder::new()
    .delimiter(':')
    .from_path(args[1])
    .expect("Error in finding input file");

let output_stream = csv::Writer::from_path(args[2])
    .expect("Error in finding or creating output file");



}