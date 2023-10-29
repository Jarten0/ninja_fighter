use crate::bean::Bean;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Cup {
    pub beans: Vec<Box<dyn Bean>>,
}


// pub struct BeanGrinder {}

// impl BeanGrinder {
//     fn new_cup(container: Container) {
        
//     }
// }

pub struct Container {
    pub beans : Vec<Box<dyn Bean>>,
    pub path : String,
}