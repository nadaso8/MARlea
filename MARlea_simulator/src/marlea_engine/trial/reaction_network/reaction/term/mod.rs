pub mod species;

use species::Species;

/// Contains the data for a single term within a larger reaction.
/// Species is a reference to a named value in solution which will be added to or subtracted from. 
/// Coefficient is the value to add or subtract
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct  Term {
    species_name: Species,
    coefficient: u8,
}

impl Term {

    pub fn new(name: String, coefficient: usize) -> Self {
        return Term{species_name:Species::Name(name), coefficient: coefficient.try_into().unwrap()};
    }
    /// returns the coefficient value of a Term
    pub fn get_coefficient (&self) -> u64 {
        return self.coefficient as u64;
    }

    /// Returns a reference to a Species enum
    /// Should always be a name
    pub fn get_species_name(&self) -> &Species {
        return &self.species_name;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let term = Term::new(String::from("water"), 2);
        assert_eq!(term.coefficient, 2);
        assert_eq!(*term.get_species_name(), Species::Name(String::from("water")));
    }

    #[test]
    fn test_get_coefficient() {
        let term = Term::new(String::from("water"), 3);
        assert_eq!(term.get_coefficient(), 3);
    }

    #[test]
    fn test_get_species_name() {
        let term = Term::new(String::from("water"), 1);
        assert_eq!(*term.get_species_name(), Species::Name(String::from("water")));
    }
}

