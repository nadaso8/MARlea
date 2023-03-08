pub mod reaction_network;

use std::collections::HashMap;
use reaction_network::ReactionNetwork;

use self::reaction_network::reaction::term::species::Species;

pub struct Trial<'trial> {
    reaction_network: ReactionNetwork<'trial>,
    stability: Stability,
}


impl<'trial> Trial<'trial> {

    pub fn from() {

    }

    pub fn simulate(&mut self) -> &HashMap<&Species,Species> {
        loop{
            self.step();
            if let Stability::Stable = self.stability {
                return self.reaction_network.get_solution();
            }
        }
    }

    fn  step (&mut self) {
        match self.stability {
            Stability::Initial => {
                 
                // if the network's possible reactions set is empty,
                // then mark the network as stable
                if self.reaction_network.get_possible_reactions().is_empty() {
                    self.stability = Stability::Stable;
                }
                // If possible reactions set is subset of null adjacent reactions set,
                // then mark it semi-stable
                else if self.reaction_network.get_possible_reactions().is_subset(&self.reaction_network.get_null_adjacent_reactions()) {
                    self.stability = Stability::SemiStable(0);
                } else {
                    // otherwise, leave it unstable, since there is more to be processed
                    self.stability = Stability::Unstable;
                }
            } 
            Stability::Unstable => {
                // get possible reactions then determine if reaction_network is unstabel, semi stable, or stable. if the reaction netowrk is unstable or semi stable get_next_reaction() from possible_reactions and react() it 

                if self.reaction_network.get_possible_reactions().is_empty() {
                    self.stability = Stability::Stable;
                }
                else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) {
                    self.reaction_network.react();
                    self.stability = Stability::SemiStable(0);
                }
                else {
                    self.reaction_network.react();
                    self.stability = Stability::Unstable;
                }
            }
            Stability::SemiStable(count) => {
                // get possible reactions then determine if reaction network is unstable, semi stable, or stable. if the reaction network is semi stable and count is less than 100 and get_next_reaction() from possible_reactions and react() it, if count is greater than 100 set self to Stable variant

                if self.reaction_network.get_possible_reactions().is_empty() {
                    self.stability = Stability::Stable;
                }
                else if self.reaction_network.get_possible_reactions().is_subset(self.reaction_network.get_null_adjacent_reactions()) {
                    if count < 99 {
                        self.reaction_network.react();
                        self.stability = Stability::SemiStable(count + 1);
                    }
                    else {
                        self.reaction_network.react();
                        self.stability = Stability::Stable;
                    }
                }
                else {
                    self.stability = Stability::Unstable;
                }
            }
            Stability::Stable => {
                //return once a stable state is reached.  
                self.stability = Stability::Stable;
            }
        }
    }


}

enum Stability {
    Initial, 
    Unstable,
    SemiStable(i32),
    Stable,
}