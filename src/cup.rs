use crate::bean::Bean;
use crate::bean_types::protag::Protag;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Cup {
    pub beans: Vec<Box<dyn Bean>>,
}

impl Cup {
    pub fn pour_beans(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.beans
    }
}

pub struct BeanGrinder;

impl BeanGrinder {
    pub fn brew_new_cup(container: Container) -> Cup {
        Cup {
            beans: container.beans,
        }
    }

    pub fn brew_default_cup() -> Cup {
        let mut cup = Cup { beans: Vec::new() };

        let protag: Box<Protag> = Box::new(Protag::default());
        cup.beans.push(protag);

        cup
    }

    pub fn package_cup(cup: Cup, container: &mut Container) {
        container.beans = cup.beans;
    }

    
}

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub beans: Vec<Box<dyn Bean>>,
    pub path: String,
}
