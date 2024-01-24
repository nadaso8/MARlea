use std::sync::mpsc::Receiver;

use marlea_engine::{
    trial::reaction_network::solution::{
        Count, 
        Name, 
        Solution
    }, 
    MarleaResult,
    Point,
};

pub enum MarleaSaveObjError {
    InitFailed(),
    WriteFailed(),
}

#[derive(Debug)]
pub struct MarleaSaveObj {
    name: &'static str,
    save_file_handles: Vec<std::fs::File>,
}

impl MarleaSaveObj {
    /// spawn new save_obj with empty name
    pub fn new() -> Self {
        todo!("implement constructor")
    }

    /// set name and return self
    pub fn save_as(&mut self, name: &'static str) -> &mut Self {
        todo!("set name")
    }

    /// write a result file
    pub fn write_result(result_channel: MarleaResult) -> Result<(), MarleaSaveObjError> {
        todo!("implement")
    }

    /// write a timeline file 
    pub fn write_timeline(timeline: Vec<marlea_engine::Point>) -> Result<(), MarleaSaveObjError> {
        todo!("implement")
    }
}