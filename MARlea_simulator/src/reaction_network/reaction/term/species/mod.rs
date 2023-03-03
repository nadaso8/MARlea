/// Contains a Name and value representing the count of some named DNA string
#[derive(Hash, Eq, PartialEq,Clone)]
pub struct Species {
    name: String,
    count: u64,
}

impl Species {

    /// returns a reference to the name String 
    pub fn get_name (&self) -> &String {
        return &self.name;
    }

    /// sets the value of count to be new_count
    pub fn set_count (&mut self, new_count: u64) {
        self.count = new_count;
        return;
    }

    /// returns value of count
    pub fn get_count (&self) -> u64 {
        return self.count;
    }
}
