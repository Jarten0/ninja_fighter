use std::ops::Add;

use crate::bean::Bean;
use crate::bean_types::protag::Protag;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Cup {
    pub beans: Vec<Box<dyn Bean>>,
    pub beans_and_dependencies: Vec<Box<dyn Bean>>,
}

impl Cup {
    pub fn pour_beans(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.beans
    }

    pub fn pour_beans_and_dependencies(&mut self) -> &mut Vec<Box<dyn Bean>> {
        &mut self.beans_and_dependencies
    }
}

pub struct BeanGrinder;

impl BeanGrinder {
    pub fn brew_new_cup(container: Container) -> Cup {
        Cup {
            beans: container.beans,
            beans_and_dependencies: Vec::new(),
        }
    }

    pub fn brew_default_cup() -> Cup {
        let mut cup = Cup {
            beans: Vec::new(),
            beans_and_dependencies: Vec::new(),
        };

        let protag: Box<Protag> = Box::new(Protag::new());
        cup.beans.push(protag);

        cup
    }

    pub fn package_cup(cup: Cup, container: &mut Container, file_name: String) {
        container.beans = cup.beans;
        container.path = String::from("assets/saveData/").add(&file_name);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub beans: Vec<Box<dyn Bean>>,
    pub path: String,
}
