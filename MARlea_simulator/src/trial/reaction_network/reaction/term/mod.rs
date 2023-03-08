pub mod species;

use species::Species;

/// Contains the data for a single term within a larger reaction.
/// Species is a reference to a named value in solution which will be added to or subtracted from. 
/// Coefficient is the value to add or subtract
#[derive(Eq, PartialEq,Clone)]
pub struct  Term<'term> {
    species: &'term Species,
    coefficient: u8,
}

impl <'term> Term<'term> {

    /// returns the coefficient value of a Term
    pub fn get_coefficient (&self) -> u64 {
        return self.coefficient as u64;
    }

    // returns the species reference in a Term
    pub fn get_species(&self) -> &'term Species {
        return self.species;
    }

}

impl<'term> std::hash::Hash for Term<'term> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.species.hash(state);
        self.coefficient.hash(state);
    }
}
