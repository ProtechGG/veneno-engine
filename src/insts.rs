#![allow(non_camel_case_types, dead_code)]

use std::{collections::HashMap, i64, process::exit};

use crate::venobjects::VenObjects;

#[derive(Debug, Clone, PartialEq)]
pub enum Instructions {
    ADD,
    SUB,
    DIV,
    MUL,
    POW,
    ROOT,
    AND,
    OR,
    XOR,
    PRINT,
    MOV,
    DECLARE,
    KEYWORD(String),
    BLOCK(String, Vec<Instructions>),
    RUN,
    PRINTLN,
    TIMES,
    IF,
    ELSE,
    TRUE,
    FALSE,
    EQ,
    GT,
    LT,
    NOT,
    // REGISTERS
    REG(usize),
    ACC,
    EOL,
    END,
    // TYPES
    DATA(VenObjects),
}

impl Instructions {
    pub fn build_from_str(stri: &str) -> Instructions {
        match stri {
            "end" => Instructions::END,
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
            "times" => Instructions::TIMES,
            "run" => Instructions::RUN,
            "else" => Instructions::ELSE,
            "if" => Instructions::IF,
            "println" => Instructions::PRINTLN,
            "and" => Instructions::AND,
            "or" => Instructions::OR,
            "xor" => Instructions::XOR,
            "true" => Instructions::TRUE,
            "false" => Instructions::FALSE,
            "eq" => Instructions::EQ,
            "gt" => Instructions::GT,
            "lt" => Instructions::LT,
            "not" => Instructions::NOT,
            a => {
                if let Some(int) = a.strip_prefix('r') {
                    Instructions::REG(int.parse().expect("Error: invalid character: {a}"))
                } else if a.starts_with('\"') && a.ends_with('\"') {
                    Self::DATA(VenObjects::Str(a[1..(a.len() - 1)].to_string()))
                } else if let Ok(num) = a.parse::<i64>() {
                    Self::DATA(VenObjects::Int(num))
                } else if let Ok(num) = a.parse::<f64>() {
                    Self::DATA(VenObjects::Float(num))
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
    pub fn get_val_or_reg_val(
        &self,
        regs: &[VenObjects],
        acc: &VenObjects,
        aliases: &HashMap<String, usize>,
    ) -> VenObjects {
        match self {
            Self::REG(rid) => regs[*rid].clone(),
            Self::DATA(VenObjects::Str(alias)) => {
                if let Some(&rid) = aliases.get(alias.as_str()) {
                    regs[rid].clone()
                } else {
                    VenObjects::Str(alias.to_string())
                }
            }
            Self::DATA(data) => data.clone(),
            Self::ACC => acc.clone(),
            Self::TRUE => VenObjects::Bool(true),
            Self::FALSE => VenObjects::Bool(false),
            a => {
                eprintln!("Incorrect register...: {:?}", a);
                exit(69);
            }
        }
    }
}
