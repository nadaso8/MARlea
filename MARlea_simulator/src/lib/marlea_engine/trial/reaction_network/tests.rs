
#[cfg(test)]
mod tests {

    use std::collections::{HashSet, HashMap};
    use crate::marlea_engine::*;

    #[test]
    fn test_new() {
        // Initialize reactions and solutions for testing
        let mut reactions = HashSet::new();
        reactions.insert(Reaction::new(vec![Species::new_name("A"), Species::new_name("B")],
                    vec![Species::new_name("C"), Species::new_name("D")]));
        reactions.insert(Reaction::new(vec![Species::new_name("X"), Species::new_count(2)],
                    vec![Species::new_count(3), Species::new_name("Y")]));

        let mut solution = HashMap::new();
        solution.insert(Species::new_name("A"), Species::new_count(1));
        solution.insert(Species::new_name("X"), Species::new_count(2));

        let network = ReactionNetwork::new(reactions, solution);

        // Check initialized fields
        assert!(network.reactions.contains(&Reaction::new(vec![Species::new_name("A"), Species::new_name("B")],
                    vec![Species::new_name("C"), Species::new_name("D")])));
        assert_eq!(*network.solution.get(&Species::new_name("A")).unwrap(), Species::new_count(1));
        assert!(network.null_adjacent_reactions.is_empty());
        assert!(network.possible_reactions.is_empty());
    }
}
