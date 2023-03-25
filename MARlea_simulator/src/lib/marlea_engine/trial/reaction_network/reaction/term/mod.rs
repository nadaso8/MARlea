pub mod solution;

use solution::Species;

/// Contains the data for a single term within a larger reaction.
/// Species is a reference to a named value in solution which will be added to or subtracted from. 
/// Coefficient is the value to add or subtract
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct  Term {
    species_name: Species,
    coefficient: u8,
}

impl Term {

    /// creates a new term from a string slice
    /// returns none if the term sould be null
    pub fn from(term: &str) -> Option<Self> {
        let mut species_name = None;
        let mut coefficient = None;
        let parts: Vec<&str> = term.split(" ").filter(|possible_part| !possible_part.is_empty()).collect();

        for part in parts {

            // if there is no coefficent try to parse as coefficent else parse as a name 
            let possible_coefficient = part.trim().parse::<u8>();
            let possible_name = part.trim().to_string();

            match possible_coefficient {
                Ok(value) => {
                    if let None = coefficient {coefficient = Some(value)}
                    else {panic!("more than one numeric value provided: it is unclear which is desired coefficient")}
                }
                Err(_) => {
                    if let None = species_name {species_name = Some(possible_name)}
                    else {
                        // non catastrophic error warn user with a print to console. 
                        print!("more than one possible name found in Term {}.\n{} was used as parsed name and coefficient was assumed to be 1.", term, species_name.clone().unwrap());
                    }
                }
            }

        }

        if let Some(parsed_name) = species_name {
            match coefficient {
                Some(parsed_value) => {
                    return Some(Term::new(parsed_name, parsed_value));
                },
                None => {
                    return Some(Term::new(parsed_name, 1 as u8));
                }
            }
        } else {return None}
    }

    pub fn new(name: String, coefficient: u8) -> Self {
        return Term{species_name:Species::Name(name), coefficient};
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

#[test]
fn test_from() {
    let term_1 = "2 water";
    let term_2 = " NaOH";
    let term_3 = "5 O2";
    let term_4 = "2water NaCl"; // minorly invalid input 
    let term_5 = "";// null input

    let expected_1 = Some(Term::new(String::from("water"), 2));
    let expected_2 = Some(Term::new(String::from("NaOH"), 1)); //default coefficient should be 1 if not specified
    let expected_3 = Some(Term::new(String::from("O2"), 5));
    let expected_4 = Some(Term::new(String::from("2water"), 1));
    let expected_5 = None;

    assert_eq!(Term::from(&term_1), expected_1);
    assert_eq!(Term::from(&term_2), expected_2);
    assert_eq!(Term::from(&term_3), expected_3);
    assert_eq!(Term::from(&term_4), expected_4);
    assert_eq!(Term::from(&term_5), expected_5);

}

}

