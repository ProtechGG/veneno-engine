#![allow(non_camel_case_types)]
use std::process::exit;

#[derive(Debug)]
pub enum Error {
    INVALID_REGISTER_OR_VALUE,
    INVALID_BLOCK_SYNTAX,
    INVALID_INT_OPERAND,
    INVALID_VALUE_FOR_MOVE,
    INVALID_RUN_BLOCK_SYNTAX,
    INVALID_TIMES_LOOP_SYNTAX,
    INVALID_BOOL_OPERAND,
}

impl Error {
    pub fn extract(&self) -> String {
        match self {
            Self::INVALID_REGISTER_OR_VALUE => "INVALID REGISTER OR VALUE".into(),
            Self::INVALID_BLOCK_SYNTAX => "INVALID BLOCK SYNTAX".into(),
            Self::INVALID_INT_OPERAND => "INVALID INT OPERANDS".into(),
            Self::INVALID_VALUE_FOR_MOVE => "CANNOT MOV VALUE: INVALID VALUE OR REGISTER".into(),
            Self::INVALID_RUN_BLOCK_SYNTAX => "INVALID `run block` SYNTAX".into(),
            Self::INVALID_TIMES_LOOP_SYNTAX => "INVALID `times loop` SYNTAX".into(),
            Self::INVALID_BOOL_OPERAND => "INVALID BOOL OPERANDS".into(),
        }
    }
    pub fn throw(error: Self, errormsg: Option<&str>) {
        match errormsg {
            Some(msg) => {
                eprintln!("{}: {}", error.extract(), msg);
                exit(69);
            }
            None => {
                eprintln!("{}", error.extract());
                exit(69);
            }
        }
    }
}
