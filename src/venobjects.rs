use std::process::exit;

use crate::insts::Instructions;

#[derive(Debug, Clone, PartialEq)]
pub enum VenObjects {
    Int(i64),
    Float(f64),
    Str(String),
    Class(String, Vec<Instructions>),
    Function(String, Vec<Instructions>),
}
impl VenObjects {
    pub fn get_int(&self) -> Option<i64> {
        match self {
            VenObjects::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn get_float(&self) -> Option<f64> {
        match self {
            VenObjects::Float(float) => Some(*float),
            _ => None,
        }
    }
    pub fn get_str(&self) -> String {
        match self {
            VenObjects::Str(name) => name.clone(),
            a => {
                eprintln!("Incorrect string: {:?}", a);
                exit(69);
            }
        }
    }
}
