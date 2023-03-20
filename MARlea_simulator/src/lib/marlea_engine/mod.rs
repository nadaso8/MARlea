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

use std::sync::mpsc::{channel, Sender};
use std::usize;
use supported_file_type::SupportedFileType; 
use trial::{*, reaction_network::{ReactionNetwork, reaction::{Reaction, term::species::Species}}}; 


mod trial;
mod supported_file_type; 

mod tests;

pub struct MarleaEngine {
    // set externally
    out_path: Option<String>,
    num_trials: Option<usize>,
    max_runtime: Option<u64>,
    max_semi_stable_steps: Option<i32>,

    // constructed by struct
    prime_network: ReactionNetwork
}

impl MarleaEngine {
    pub fn new(
        input_path: String,
        init_path: Option<String>,
        out_path: Option<String>,
        num_trials: Option<usize>,
        max_runtime: Option<u64>,
        max_semi_stable_steps: Option<i32>,
    ) -> Self { 
        
        let reactions = SupportedFileType::from(input_path).parse_reactions();
        let solution = Self::solution_from(init_path, &reactions);
        let prime_network = ReactionNetwork::new(reactions, solution);

        Self{
        out_path,
        num_trials,
        max_runtime,
        max_semi_stable_steps,
        prime_network,
        } 
    }

    pub fn run(&self) -> bool {
        // vector containing all trial results
        let mut simulation_results = HashSet::new();  

        // setup threadpool 
        let computation_threads = futures::executor::ThreadPool::new().unwrap();
        let results_channel = channel();

        // start runtime timer
        let timer_channel = channel();
        if let Some(time) = self.max_runtime {
            computation_threads.spawn_ok(Self::engine_runtime_timer(time, timer_channel.0));
        }

        // setup loop for assigning new tasks
        let mut trials_recieved = 0;
        let max_trials = match self.num_trials{Some(number) => number, None => 100};

        // tasks for trial results
        while trials_recieved < max_trials {

            let current_trial = trial::Trial::from(self.prime_network.clone(), self.max_semi_stable_steps);
            computation_threads.spawn_ok(Self::new_trial_task(results_channel.0.clone(), current_trial));

            // check for new results
            if let Ok(result) = results_channel.1.try_recv() {
                simulation_results.insert(result);
                trials_recieved += 1;
                println!("recieved {} trials", trials_recieved);
            }

            if let Ok(_) = timer_channel.1.try_recv() {
                println!("forced termination because max time was reached\n\nWARNING: returned results may not be accurate and should be used for debugging purposes only");
                break;
            }
        }

        return self.terminate(simulation_results);

    }
    
    fn average_trials(trial_results: HashSet<TrialResult>) -> Vec<(String, f64)> {
        let mut summed_values = HashMap::<String, f64>::new();
        let num_trials = trial_results.len() as f64;
    
        // Sum values of each species across all trials
        for result in &trial_results {
            for (name, count) in result.result.clone() {
                if let Species::Name(species_name) = name {
                    if let Species::Count(species_count) = count  {
                        summed_values.entry(species_name)
                        .and_modify(|summed_count| *summed_count +=  species_count as f64)
                        .or_insert(species_count as f64);    
                    }
                } else {
                    panic!("Got non-species name when calculating averages");
                }
            }
        }
    
        // Calculate averages and sort alphabetically
        let mut averaged_values: Vec<(String, f64)> = summed_values
                        .into_iter()
                        .map(|(key, value)| (key, value / num_trials))
                        .collect();
        averaged_values.sort_by_key(|(species, _)| species.to_owned());

        return averaged_values;
    }
    

    fn solution_from(file_path: Option<String>, reactions: &HashSet<Reaction>) -> HashMap<Species, Species> {
        let mut solution: HashMap<Species, Species> = HashMap::new();

        // Get possible species from reactions
        for reaction in reactions {
            for reactant in reaction.get_reactants() {
                // if no such species exists in the map generate a new map entry using the reactant species name and default value 0 
                solution.insert(reactant.get_species_name().clone(), Species::Count(0));
            }
            for product in reaction.get_products() {
                // if no such species exists in the map generate a new map entry using the product species name and default value 0 
                solution.insert(product.get_species_name().clone(), Species::Count(0));
            }
        }

        if let Some(path) = file_path {
            SupportedFileType::from(path).parse_initial_solution(&mut solution);
        }

        return solution; 
    }

    fn terminate(&self, simulation_results: HashSet<TrialResult>) -> bool {
        //Generate results content and print to console
        let mut content: String = String::new(); 
        for field in Self::average_trials(simulation_results) {
            print!("{},{}\n", field.0, field.1);
            content.push_str( &format!("{},{}\n", field.0, field.1));
        }

        //write results if output option ennabled
        if let Some(path) = &self.out_path {
            let output_file = SupportedFileType::from(path.clone());
            if let Err(error) = output_file.write(&content) {
                panic!("{}", error);
            }
        }
        return true;
    }

    async fn new_trial_task(tx: Sender<TrialResult>, mut current_trial: Trial) {
        let result = current_trial.simulate();
        tx.send(result).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(30));
        return;
    }

    async fn engine_runtime_timer(runtime: u64, tx: Sender<bool>) {
        let max_runtime = std::time::Duration::from_secs(runtime);
        std::thread::sleep(max_runtime);
        tx.send(true).unwrap();
        return;
    }

}

