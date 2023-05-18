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

use std::sync::mpsc::{
    sync_channel,
    SyncSender, Receiver,
};
use std::usize;
use supported_file_type::SupportedFileType; 
use threadpool::ThreadPool;
use trial::{
    results::TrialResult, 
    reaction_network::{
        ReactionNetwork, 
        reaction::{
            Reaction, 
            term::solution::{
                Species,
                Solution,
            }
        }
    }
};

use self::supported_file_type::TimelineWriter; 


pub mod trial;
mod supported_file_type; 
//mod tests;

pub struct MarleaEngine {
    // set externally
    out_path: Option<String>,
    out_timeline: Option<String>,
    num_trials: Option<usize>,
    max_runtime: Option<u64>,
    max_semi_stable_steps: Option<i32>,

    // constructed by struct
    computation_threads: ThreadPool,
    computations_threads_sender: SyncSender<TrialResult>,
    computation_threads_reciever: Receiver<TrialResult>,
    prime_network: ReactionNetwork
}

impl MarleaEngine {
    pub fn custom_block ( custom_network: ReactionNetwork ) -> Self {

        let computation_threads = threadpool::Builder::new()
            .thread_name("compute_thread".into())
            .build();
        let computation_threads_channels = sync_channel(0);

        Self { 
            out_path: None,
            out_timeline: None, 
            num_trials: None, 
            max_runtime: None, 
            max_semi_stable_steps: None, 
            computation_threads: computation_threads, 
            computations_threads_sender: computation_threads_channels.0, 
            computation_threads_reciever: computation_threads_channels.1, 
            prime_network: custom_network
        }
    }

    pub fn new(
        input_path: String,
        init_path: Option<String>,
        out_path: Option<String>,
        out_timeline: Option<String>,
        num_trials: Option<usize>,
        max_runtime: Option<u64>,
        max_semi_stable_steps: Option<i32>,
    ) -> Self { 

        let reactions = SupportedFileType::from(input_path).parse_reactions();
        let solution = Self::solution_from(init_path, &reactions);
        let prime_network = ReactionNetwork::new(reactions, solution);
        let computation_threads = threadpool::Builder::new()
            .thread_name("compute_thread".into())
            .build();
        let computation_threads_channels = sync_channel(0);

        Self{
            out_path,
            out_timeline,
            num_trials,
            max_runtime,
            max_semi_stable_steps,
            computation_threads,
            computations_threads_sender: computation_threads_channels.0,
            computation_threads_reciever: computation_threads_channels.1,
            prime_network,
        } 
    }

    pub fn run(&self) -> Vec<(String, f64)> {
        // set containing all trial results
        let mut simulation_results = HashSet::new();

        // setup loop variables
        let mut trials_recieved = 0;
        let mut trials_created = 0;
        let max_trials = match self.num_trials{Some(number) => number, None => 100};        

        // setup timeline writer if one is needed
        let (timeline_writer_sender, timeline_writer_reciever) = sync_channel(0);
        if let Some(path) = &self.out_timeline {
            let timeline_writer = TimelineWriter::new(SupportedFileType::from(path.clone()), timeline_writer_reciever);
            self.computation_threads.execute(move|| timeline_writer.begin_listen());
        }
  
        // start runtime timer
        let (timer_sender, timer_reciever) = sync_channel(0);
        if let Some(time) = self.max_runtime {
            self.computation_threads.execute(move|| Self::engine_runtime_timer(time, timer_sender));
        }

        // create trials 
        match &self.out_timeline {
            Some(_) => {
                while trials_created < max_trials {
                    let mut current_trial = trial::Trial::from(self.prime_network.clone(), self.max_semi_stable_steps, trials_created);
                    let trial_sender = self.computations_threads_sender.clone();
                    self.computation_threads.execute(move|| current_trial.simulate_with_timeline(trial_sender));
                    trials_created += 1;
                }
            }
            None => {
                while trials_created < max_trials {
                    let mut current_trial = trial::Trial::from(self.prime_network.clone(), self.max_semi_stable_steps, trials_created);
                    let trial_sender = self.computations_threads_sender.clone();
                    self.computation_threads.execute(move || current_trial.simulate(trial_sender));                    
                    trials_created += 1;
                }
            }
        }

        // poll for trial results
        while trials_recieved < max_trials {
            if let Ok(result) = self.computation_threads_reciever.try_recv() {
                match result {
                    TrialResult::StableSolution(solution, steps) => {
                        trials_recieved += 1;
                        println!("Trial stable after {} steps", steps);
                        println!("Recieved {} trials", trials_recieved);
                        simulation_results.insert(solution);
                    }
                    TrialResult::TimelineEntry(solution, id) => {
                        timeline_writer_sender.send((solution, id)).unwrap();
                    }
                }
            }
            
            if let Ok(_) = timer_reciever.try_recv() {
                println!("forced termination because max time was reached\n\nWARNING: returned results may not be accurate and should be used for debugging purposes only");
                break;
            }
        }

        drop(timeline_writer_sender);

        return self.terminate(simulation_results);

    }
    
    fn average_trials(simulation_results: HashSet<Solution>) -> Vec<(String, f64)> {
        let mut summed_values = HashMap::<String, f64>::new();
        let num_trials = simulation_results.len() as f64;
    
        // Sum values of each species across all trials
        for result in &simulation_results {
            for (name, count) in result.species_counts.clone() {
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
    

    fn solution_from(file_path: Option<String>, reactions: &HashSet<Reaction>) -> Solution {
        let mut species_counts: HashMap<Species, Species> = HashMap::new();

        // Get possible species from reactions
        for reaction in reactions {
            for reactant in reaction.get_reactants() {
                // if no such species exists in the map generate a new map entry using the reactant species name and default value 0 
                species_counts.insert(reactant.get_species_name().clone(), Species::Count(0));
            }
            for product in reaction.get_products() {
                // if no such species exists in the map generate a new map entry using the product species name and default value 0 
                species_counts.insert(product.get_species_name().clone(), Species::Count(0));
            }
        }

        if let Some(path) = file_path {
            SupportedFileType::from(path).parse_initial_solution(&mut species_counts);
        }

        return Solution{species_counts}; 
    }

    fn terminate(&self, simulation_results: HashSet<Solution>) -> Vec<(String, f64)> {
        
        let average_stable_solution = Self::average_trials(simulation_results);

        //write results if output option ennabled
        if let Some(path) = &self.out_path {
            let output_file = SupportedFileType::from(path.clone());
            output_file.write_solution(average_stable_solution.clone());
        } else {
            for entry in average_stable_solution.clone() {
                println!("{},{}", entry.0 , entry.1);
            }
        }

        return average_stable_solution;
    }

    fn engine_runtime_timer(runtime: u64, tx: SyncSender<bool>) {
        let max_runtime = std::time::Duration::from_secs(runtime);
        std::thread::sleep(max_runtime);
        tx.send(true).unwrap();
        return;
    } 

}