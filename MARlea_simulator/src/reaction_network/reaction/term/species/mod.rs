/// Contains a Name and value representing the count of some named DNA string
#[derive(Hash, Eq, PartialEq,Clone)]
pub struct Species {
    pub name: String,
    pub count: u64,
}
