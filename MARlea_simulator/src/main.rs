use std::{collections::{HashMap}};


struct Reaction {
    reactants: HashMap<String, u8>,
    products: HashMap<String, u8>,
    reaction_rate: u32,
}

/// simulates the effects of a reaction occuring
/// <reaction> - specifies a reaction struct containing the reactants to be removed and products to be added to memory
/// <memory> - specifies the memory datastructure to subtract and add values to
fn react(reaction: Reaction, mut memory: HashMap<String,u64>) {
    
    
    // subtracts reactant values from memory of strings in solution
    for reactant in reaction.reactants {
        memory.entry(reactant.0)
        .and_modify(|memory_value| *memory_value -= reactant.1 as u64);
    }
    
    // adds product values to memory of strings in solution or adds a new value if one was not already present
    for product in reaction.products {
        memory.entry(product.0)
        .and_modify(|memory_value| *memory_value += product.1 as u64)
        .or_insert(product.1 as u64);
    }
}

fn main () {

}