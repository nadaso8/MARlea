use super::*;


#[derive(Eq, PartialEq, Clone)]
pub enum TrialResult {
    StableSolution(Solution, i32), 
    TimelineEntry(Solution, usize),
}