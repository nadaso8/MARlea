use crate::simulation::reaction_network::reaction::Reaction;
use std::collections::HashSet;

#[derive(Clone)]
pub enum NetworkStability<'reacting> {
    Initial, 
    Unstable(HashSet<&'reacting Reaction<'reacting>>),
    SemiStable(HashSet<&'reacting Reaction<'reacting>>, u32),
    Stable,
}

impl<'reacting> NetworkStability<'reacting> {

    pub fn update<'update> (&'update mut self, possible_reactions: &'update HashSet<&'reacting Reaction<'reacting>>) {
        match self {
            NetworkStability::Initial => {
                for reaction in possible_reactions {
                    self.insert(reaction);
                }
                self.check_stability();
            }
            NetworkStability::Unstable(set) => {
                set.clear();
                for reaction in possible_reactions {
                    self.insert(reaction)
                }
                self.check_stability()
            }
            NetworkStability::SemiStable(set, _cycle_count) => {
                set.clear();
                for reaction in possible_reactions {
                    self.insert(reaction)
                }
                self.check_stability()
            }
            NetworkStability::Stable => {panic!("update called on stable variant of NetworkStability")}
        }
    }

    fn check_stability (&self) {
        match self {
            NetworkStability::Unstable(set) | NetworkStability::SemiStable(set, _) => {
                todo!("check over all currently possible reactions, and if they're SemiStable set self to be semi stable variant with cycle count 0. if therea are no reactions set self to stable");
            }
            _ => {panic!("check_stability called on variant of NetworkStability other than Unstable, or SemiStable")}
        }
    }
    
    fn from(&mut self, reaction: &'reacting Reaction<'reacting>) {
        *self = NetworkStability::Unstable(HashSet::from([reaction],));
    }

    // inserts a 
    fn insert<'inserting>(&'inserting mut self, reaction: &'reacting Reaction<'reacting>) {
        match self {
            NetworkStability::Initial => {
                self.from(reaction)
            }
            NetworkStability::Unstable(set) | NetworkStability::SemiStable(set, _) => {
                set.insert(reaction);
            }
            NetworkStability::Stable => {panic!("cannot insert reaction to stable variant")}
        }
    }

}

