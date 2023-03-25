use std::{collections::HashMap, fmt::Display};

/// Contains a Name, or count for some species
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Species {
    Name(String),
    Count(u64),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Threshold {
    LessThan(u64),
    LessThanOrEqual(u64),
    GreaterThan(u64),
    GreaterThanOrEqual(u64),
}

#[derive(Eq, PartialEq, Clone)]
pub struct Solution {
    pub species_counts: HashMap<Species, Species>,
}

impl std::hash::Hash for Solution {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for entry in self.species_counts.iter() {
            entry.hash(state);
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Make an ordered copy of self
        let mut self_as_vector: Vec<(String, String)> = Vec::new();
        for entry in self.species_counts {
            self_as_vector.push((
                match entry.0 {
                    Species::Name(name) => { name }
                    Species::Count(count) => { count.to_string() }
                }, 
                match entry.1 {
                    Species::Name(name) => { name }
                    Species::Count(count) => { count.to_string() }
                }
            ));
        }
        self_as_vector.sort();

        // Format the ordered vector as a string
        let mut formatted_string = String::new();
        for entry in self_as_vector {
            formatted_string.push_str(&format!("{}, {}\n", entry.0, entry.1));
        }

        // Write the formatted string to the provided Formatter
        write!(f, "{}", formatted_string)
    }
}