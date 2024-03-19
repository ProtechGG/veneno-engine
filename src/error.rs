#![allow(non_camel_case_types)]
use std::process::exit;

#[derive(Debug)]
pub enum Error {
    INVALID_REGISTER_OR_VALUE,
    INVALID_BLOCK_SYNTAX,
    CANNOT_DO_OPERATION,
    INVALID_VALUE_FOR_MOVE,
    INVALID_RUN_BLOCK_SYNTAX,
    INVALID_TIMES_LOOP_SYNTAX,
}

impl Error {
    pub fn extract(&self) -> String {
        match self {
            Self::INVALID_REGISTER_OR_VALUE => "ERROR: INVALID REGISTER OR VALUE".into(),
            Self::INVALID_BLOCK_SYNTAX => "INVALID BLOCK SYNTAX".into(),
            Self::CANNOT_DO_OPERATION => "ERROR: CANNOT DO OPERATION".into(),
            Self::INVALID_VALUE_FOR_MOVE => "CANNOT MOV VALUE: INVALID VALUE OR REGISTER".into(),
            Self::INVALID_RUN_BLOCK_SYNTAX => "INVALID `run block` SYNTAX".into(),
            Self::INVALID_TIMES_LOOP_SYNTAX => "INVALID `times loop` SYNTAX".into(),
        }
    }
    pub fn throw(error: Self, errormsg: Option<&str>) {
        match errormsg {
            Some(msg) => {
                eprintln!("{:?}: {:?}", error, msg);
                exit(69);
            }
            None => {
                eprintln!("{:?}", error);
                exit(69);
            }
        }
    }
}
