use serde::{self, Deserialize, Serialize};

use crate::bean_types::transform::Transform;

pub trait Script {
    fn init(&self) {}
    fn update(&self) {}
}

// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Renderer {
    
// }

// impl Script for Renderer {
//     fn init(&self) {
//         // self.image = graphics::Image::new(gpu, path)
//     }

//     fn update(&self) {
//         todo!()
//     }
// }



#[typetag::serde(tag = "type")]
pub trait Bean {

    /// Runs once when the bean enters the scope, will be called before all of it's scripts are finished
    fn init(&self) { }

    /// Runs once after the bean enters the scope, but unlike init(), 
    /// it will only be called after all the children have run their init and ready functions
    fn ready(&self) { }

    /// Will be called once per frame, starting from the bottom working up.
    fn update(&self) { }

    // fn serialize<S>(selff: &Self::Typ, serializer: S) -> Result<S::Ok, S::Error>
    // where
    //     S: serde::Serializer, Self: Sized 
    // {
    //     <Self::Typ as Serialize>::serialize(selff, serializer)
    // }

    // fn deserialize<D>(deserializer: D) -> Result<Self::Typ, D::Error> 
    // where 
    //     D: serde::Deserializer<'static>, Self: Sized 
    // {
    //     Self::Typ::deserialize(deserializer)
    // }
    
}


#[derive(Serialize, Deserialize)]
pub struct Protag {
    pub transform: Transform,
    // renderer: Renderer,
}

#[typetag::serde]
impl Bean for Protag {

    fn init(&self) {
        
    }

    fn update(&self) {
        println!("Haha! Hi!")
    }

}