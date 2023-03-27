use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use crate::marlea_engine::{MarleaEngine, TrialResult, supported_file_type::SupportedFileType, reaction_network::reaction::{Reaction, term::{Term, solution::Species}}};

#[test]
fn test_no_path() {
    let reactions = HashSet::from_iter(vec![
        Reaction::new(HashSet::from([Term::new("zooble".to_string(), 6), Term::new("crand".to_string(), 4)]), HashSet::from([Term::new("gubble".to_string(), 1)]), 14),
        Reaction::new(HashSet::from([Term::new("gobble".to_string(), 1), Term::new("gubble".to_string(), 1)]), HashSet::from([Term::new("crangle".to_string(), 1)]), 6),
        Reaction::new(HashSet::from([Term::new("gubble".to_string(), 1)]), HashSet::from([Term::new("gobble".to_string(), 1), Term::new("zooble".to_string(), 10), Term::new("crand".to_string(), 5)]), 100),
    ].into_iter());
    let expected_solution = HashMap::from_iter(vec![
        (Species::Name("zooble".to_owned()), Species::Count(0)),
        (Species::Name("gobble".to_owned()), Species::Count(0)),
        (Species::Name("gubble".to_owned()), Species::Count(0)),
        (Species::Name("crand".to_owned()), Species::Count(0)),
        (Species::Name("crangle".to_owned()), Species::Count(0)),
    ].into_iter());

    let actual_solution = super::MarleaEngine::solution_from(None, &reactions);

    assert_eq!(
        actual_solution,
        expected_solution,
        "No path provided, did not generate correct initial solution",
    );
}

#[cfg(test)]
#[test]
fn test_average_trials() {
    let steps = 0;
    // Prepare input data
    let mut trial_results = HashSet::new();
    
    let mut result1 = HashMap::new();
    result1.insert(Species::Name("Giraffe".to_string()), Species::Count(2));
    result1.insert(Species::Name("Elephant".to_string()), Species::Count(2));
    
    let mut result2 = HashMap::new();
    result2.insert(Species::Name("Elephant".to_string()), Species::Count(3));
    result2.insert(Species::Name("Giraffe".to_string()), Species::Count(1));

    let mut result3 = HashMap::new();
    result3.insert(Species::Name("Giraffe".to_string()), Species::Count(1));
    result3.insert(Species::Name("Elephant".to_string()), Species::Count(2));

    trial_results.insert(TrialResult{ result: result1 , steps});
    trial_results.insert(TrialResult{ result: result2 , steps});
    trial_results.insert(TrialResult{ result: result3 , steps});

    // Call the function
    let averaged_values = MarleaEngine::average_trials(trial_results);

    // Assert the result
    assert_eq!(averaged_values, vec![("Elephant".to_string(), 2.0), ("Giraffe".to_string(), 1.0)]);
}

#[test]
fn parse_initial_solution_gives_correct_counts_for_basic_reactions() {

    let reactions: HashSet<Reaction> = HashSet::from_iter(vec![
        Reaction::new(HashSet::from([Term::new("zooble".to_string(), 6), Term::new("crand".to_string(), 4)]), HashSet::from([Term::new("gubble".to_string(), 1)]), 14),
        Reaction::new(HashSet::from([Term::new("gobble".to_string(), 1), Term::new("gubble".to_string(), 1)]), HashSet::from([Term::new("crangle".to_string(), 1)]), 6),
        Reaction::new(HashSet::from([Term::new("gubble".to_string(), 1)]), HashSet::from([Term::new("gobble".to_string(), 1), Term::new("zooble".to_string(), 10), Term::new("crand".to_string(), 5)]), 100),
    ].into_iter());

    let mut solution = HashMap::new(); 
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
    SupportedFileType::CSV("init.csv".to_string()).parse_initial_solution(&mut solution);

    let mut expected_solution = HashMap::from([
        (Species::Name("crand".to_string()), Species::Count(50)),
        (Species::Name("crangle".to_string()), Species::Count(0)),
        (Species::Name("gobble".to_string()), Species::Count(0)),
        (Species::Name("gubble".to_string()), Species::Count(0)),
        (Species::Name("zooble".to_string()), Species::Count(2)),
    ]);

    assert_eq!(solution, expected_solution);

    /*
    assert_eq!(SupportedFileType::XML() parse_initial_solution(&reactions), expected_solution );
    assert_eq!(SupportedFileType::JSON() parse_initial_solution(&reactions), expected_solution );
     */
}

#[cfg(test)]
#[test]
fn test_new() {
    let input_path = String::from("input.csv");

    // test case with init_path and num_trials
    let init_path = Some(String::from("init.csv"));
    let num_trials = Some(10);
    let max_runtime = None;
    let app = MarleaEngine::new(input_path.clone(), init_path, None, num_trials, max_runtime, None);
    // assertions
    assert_eq!(app.out_path, None);
    assert_eq!(app.num_trials, num_trials);
    assert_eq!(app.max_runtime, max_runtime);
    assert_eq!(app.prime_network.get_number_reactions(), 3); // example value, update based on actual input
    assert_eq!(app.prime_network.get_solution().len(), 5); // example value, update based on actual input

    // test case with all parameters none
    let app = MarleaEngine::new(input_path.clone(), None, None, None, None, None);
    // assertions
    assert_eq!(app.out_path, None);
    assert_eq!(app.num_trials, None);
    assert_eq!(app.max_runtime, None);
    assert_eq!(app.prime_network.get_number_reactions(), 3); // example value, update based on actual input
    assert_eq!(app.prime_network.get_solution().len(), 5); // example value, update based on actual input

    // test case with out_path and max_runtime
    let out_path = Some(String::from("out.csv"));
    let max_runtime = Some(60);
    let app = MarleaEngine::new(input_path, None, out_path.clone(), None, max_runtime, None);
    // assertions
    assert_eq!(app.out_path, out_path);
    assert_eq!(app.num_trials, None);
    assert_eq!(app.max_runtime, max_runtime);
    assert_eq!(app.prime_network.get_number_reactions(), 3); // example value, update based on actual input
    assert_eq!(app.prime_network.get_solution().len(), 5); // example value, update based on actual input
}

#[test]
fn test_parse_reactions_csv() {
    // Prepare input data
    let path = "input.csv".to_string();
    let file = SupportedFileType::CSV(path);

    // Call the function and get output
    let result = file.parse_reactions();

    // Define expected output (manually created HashSet with expected reactions)
    let expected: HashSet<Reaction> = HashSet::from_iter(vec![
        Reaction::new(HashSet::from([Term::new("zooble".to_string(), 6), Term::new("crand".to_string(), 4)]), HashSet::from([Term::new("gubble".to_string(), 1)]), 14),
        Reaction::new(HashSet::from([Term::new("gobble".to_string(), 1), Term::new("gubble".to_string(), 1)]), HashSet::from([Term::new("crangle".to_string(), 1)]), 6),
        Reaction::new(HashSet::from([Term::new("gubble".to_string(), 1)]), HashSet::from([Term::new("gobble".to_string(), 1), Term::new("zooble".to_string(), 10), Term::new("crand".to_string(), 5)]), 100),
    ].into_iter());

    // Compare actual and expected output
    assert_eq!(result, expected);
}
