/// Contains a Name, Count, Or Threshold type for a species
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Species {
    Name(String),
    Count(u64),
    Threshold(Option<Threshold>),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Threshold {
    LessThan(u64),
    LessThanOrEqual(u64),
    GreaterThan(u64),
    GreaterThanOrEqual(u64),
}