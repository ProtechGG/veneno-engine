#![allow(non_camel_case_types, dead_code)]

use std::{i64, process::exit};

use crate::venobjects::{Float, VenObjects};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instructions {
    ADD,
    SUB,
    DIV,
    MUL,
    POW,
    ROOT,
    PRINT,
    MOV,
    DECLARE,
    KEYWORD(String),
    BLOCK(String, Vec<Instructions>),
    RUN,
    TIMES,
    // REGISTERS
    REG(usize),
    ACC,
    EOL,
    // TYPES
    DATA(VenObjects),
}

impl Instructions {
    pub fn build_from_str(stri: &str) -> Instructions {
        match stri {
            "add" => Instructions::ADD,
            "sub" => Instructions::SUB,
            "div" => Instructions::DIV,
            "mul" => Instructions::MUL,
            "pow" => Instructions::POW,
            "root" => Instructions::ROOT,
            "print" => Instructions::PRINT,
            "mov" => Instructions::MOV,
            "declare" => Instructions::DECLARE,
            "acc" => Instructions::ACC,
            "run" => Instructions::RUN,
            "times" => Instructions::TIMES,
            a => {
                if let Some(int) = a.strip_prefix('r') {
                    Instructions::REG(int.parse().expect("Error: invalid character: {a}"))
                } else if a.starts_with('\"') && a.ends_with('\"') {
                    Self::DATA(VenObjects::Str(a[1..(a.len() - 1)].to_string()))
                } else if let Ok(num) = a.parse::<i64>() {
                    Self::DATA(VenObjects::Int(num))
                } else if let Ok(num) = a.parse::<f64>() {
                    Self::DATA(VenObjects::Float(Float::build(num)))
                } else {
                    Instructions::DATA(VenObjects::Str(stri.to_string()))
                }
            }
        }
    }
    pub fn extract_value(&self) -> Option<VenObjects> {
        match self {
            Self::DATA(data) => Some(data.clone()),
            _ => None,
        }
    }
    pub fn get_val_or_reg_val(&self, regs: Vec<VenObjects>, acc: VenObjects) -> VenObjects {
        match self {
            Self::REG(rid) => regs[*rid].clone(),
            Self::DATA(data) => data.clone(),
            Self::ACC => acc,
            a => {
                eprintln!("Incorrect register...: {:?}", a);
                exit(69);
            }
        }
    }
}
