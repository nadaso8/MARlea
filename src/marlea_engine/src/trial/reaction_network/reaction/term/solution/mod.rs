use std::{collections::HashMap, fmt::Display};

/// Contains a Name, or count for some species
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Species {
    Name(String),
    Count(u64),
}

impl Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Species::Name(name) => write!(f, "{}", name),
            Species::Count(count) => write!(f, "{}", count),
        }
    }
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

impl IntoIterator for Solution {
    type Item = (Species, Species);
    type IntoIter = std::vec::IntoIter<Self::Item>;


    fn into_iter(self) -> Self::IntoIter {
        // Make an ordered copy of self
        let mut self_as_vector: Vec<(Species,Species)> = Vec::new();
        for entry in self.species_counts {
            self_as_vector.push(entry);
        }
        self_as_vector.sort();

        return self_as_vector.into_iter();
    }

    
}


impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the ordered vector as a string
        let mut formatted_string = String::new();
        for entry in self.clone().into_iter() {
            formatted_string.push_str(&format!("{},{},",entry.0.to_string(),entry.1.to_string()));
        }

        // Write the formatted string to the provided Formatter
        write!(f, "{}", formatted_string)
    }
}