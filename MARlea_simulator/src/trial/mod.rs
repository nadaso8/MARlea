pub mod reaction_network;

use reaction_network::{ReactionNetwork, reaction::Reaction, reaction::term::species::Species};
use std::collections::{HashSet, HashMap};

use self::reaction_network::reaction;

#[derive(Clone)]
pub enum TrialState<'calculating_trial_results> {
    Initial(ReactionNetwork<'calculating_trial_results>), 
    Unstable(ReactionNetwork<'calculating_trial_results>),
    SemiStable(ReactionNetwork<'calculating_trial_results>, i32),
    Stable(ReactionNetwork<'calculating_trial_results>),
}

impl<'reacting> TrialState<'reacting> {

    pub fn simulate (&mut self) -> HashMap<&Species, Species> {
        loop {
            match self {
                TrialState::Initial(reaction_network) => {
                    // get possible reactions then determine if reaction_network is unstable, semi stable, or stable.  
                    let possible_reactions = reaction_network.get_possible_reactions();

                    if possible_reactions.is_empty() {
                        *self = TrialState::Stable(reaction_network.clone());
                    }
                    else if possible_reactions.is_subset(&reaction_network.get_null_adjacent_reactions()) {
                        *self = TrialState::SemiStable(reaction_network.clone(), 0);
                    }
                    else {
                        *self = TrialState::Unstable(reaction_network.clone());
                    }
                } 
                TrialState::Unstable(reaction_network) => {
                    // get possible reactions then determine if reaction_netowrk is unstabel, semi stable, or stable. if the reaction netowrk is unstable or semi stable get_next_reaction() from possible_reactions and react() it 
                    let possible_reactions = reaction_network.get_possible_reactions();

                    if possible_reactions.is_empty() {
                        *self = TrialState::Stable(reaction_network.clone());
                    }
                    else if possible_reactions.is_subset(&reaction_network.get_null_adjacent_reactions()) {
                        if let Some(next_reaction) = reaction_network.get_next_reaction(&possible_reactions)  {
                            reaction_network.react(next_reaction);
                        }
                        *self = TrialState::SemiStable(reaction_network.clone(), 0);
                    }
                    else {
                        if let Some(next_reaction) = reaction_network.get_next_reaction(&possible_reactions)  {
                            reaction_network.react(next_reaction);
                        }
                        *self = TrialState::Unstable(reaction_network.clone());
                    }
                }
                TrialState::SemiStable(reaction_network, count) => {
                    // get possible reactions then determine if reaction network is unstable, semi stable, or stable. if the reaction network is semi stable and count is less than 100 and get_next_reaction() from possible_reactions and react() it, if count is greater than 100 set self to Stable variant
                    let possible_reactions = reaction_network.get_possible_reactions();

                    if possible_reactions.is_empty() {
                        *self = TrialState::Stable(reaction_network.clone());
                    }
                    else if possible_reactions.is_subset(&reaction_network.get_null_adjacent_reactions()) {
                        if *count < 100 {
                            if let Some(next_reaction) = reaction_network.get_next_reaction(&possible_reactions)  {
                                reaction_network.react(next_reaction);
                            }
                            *self = TrialState::SemiStable(reaction_network.clone(), *count + 1);
                        }
                        else {
                            if let Some(next_reaction) = reaction_network.get_next_reaction(&possible_reactions)  {
                                reaction_network.react(next_reaction);
                            }
                            *self = TrialState::Stable(reaction_network.clone());
                        }
                    }
                    else {
                        *self = TrialState::Unstable(reaction_network.clone());
                    }
                }
                TrialState::Stable(reaction_network) => {
                    //get solution from reaction network and return it. 
                    return reaction_network.get_solution().clone();
                }
            }

        }
    }

}

