use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use super::{Reaction, Term, Species};

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

/*#[test]
fn test_csv_file() {
    let file_path = Some("/path/to/file.csv".to_owned());
    let mock_solution = HashMap::from_iter(vec![
        (Species::Name("species_one".to_owned()), Species::Count(1)),
        // add more mocked species here
    ].into_iter());
    let reactions = HashSet::from_iter(vec![/* add a few Reaction objects here */].into_iter());
    let supported_file = SupportedFileType::CSV(file_path.clone().unwrap());

    MockCsvReader::setup_mock_read(file_path.unwrap(), vec![
        vec!["species_one", "1", ""], 
        vec!["species_two", "0", ""], 
        // add more vecs of csv data here
    ]);
    
    let actual_solution = supported_file.parse_initial_solution(&reactions);
    
    assert_eq!(
        actual_solution,
        mock_solution,
        "Error occurred while reading CSV file",
    );
}*/
