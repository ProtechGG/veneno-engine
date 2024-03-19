#![allow(non_upper_case_globals)]

use std::{collections::HashMap, process::exit};

use crate::{error::Error, insts::Instructions, venobjects::VenObjects};

#[derive(Debug)]
pub struct CPU {
    pub registers: Vec<VenObjects>,
    pub acc: VenObjects,
    pub blocks: HashMap<String, Vec<Instructions>>,
}

impl CPU {
    pub fn init(&mut self, no_of_regs: usize) {
        for _ in 0..no_of_regs {
            self.registers.push(VenObjects::Int(0));
        }
    }
    pub fn exec(&mut self, tokens: Vec<Instructions>) {
        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].clone() {
                Instructions::BLOCK(name, insts) => {
                    let mut insts = insts;
                    insts.remove(0);
                    if name == "main" {
                        self.exec(insts);
                        exit(0);
                    }
                }
                Instructions::MOV => {
                    let val = tokens[i + 2].clone();
                    let val = val.get_val_or_reg_val(&self.registers, &self.acc);

                    match tokens[i + 1].clone() {
                        Instructions::REG(id) => self.registers[id] = val,
                        Instructions::ACC => self.acc = val,
                        a => Error::throw(
                            Error::INVALID_VALUE_FOR_MOVE,
                            Some(format!("{:?}", a).as_str()),
                        ),
                    }
                    i += 3;
                }
                Instructions::ADD => {
                    self.operate(i, tokens.clone(), |x, y| x + y);
                    i += 3;
                }
                Instructions::SUB => {
                    self.operate(i, tokens.clone(), |x, y| x - y);
                    i += 3;
                }
                Instructions::DIV => {
                    self.operate(i, tokens.clone(), |x, y| x / y);
                    i += 3;
                }
                Instructions::MUL => {
                    self.operate(i, tokens.clone(), |x, y| x * y);
                    i += 3;
                }
                Instructions::POW => {
                    self.operate(i, tokens.clone(), |x, y| x.powf(y));
                    i += 3;
                }
                Instructions::PRINT => {
                    let text = tokens[i + 1].get_val_or_reg_val(&self.registers, &self.acc);
                    match text {
                        VenObjects::Int(num) => println!("{}", num),
                        VenObjects::Str(stri) => println!("{}", stri),
                        VenObjects::Float(float) => println!("{}", float),
                        VenObjects::Class(name, insts) => println!("{}: {:?}", name, insts),
                        VenObjects::Function(name, body) => println!("{}: {:?}", name, body),
                    }
                    i += 2;
                }
                Instructions::RUN => {
                    let block_name = &tokens[i + 1];
                    match block_name {
                        Instructions::DATA(block_name) => {
                            let block_name = block_name.get_str();
                            self.exec(
                                self.blocks
                                    .get(&block_name)
                                    .expect("Error: invalid run block command")
                                    .clone(),
                            );
                        }
                        a => Error::throw(
                            Error::INVALID_RUN_BLOCK_SYNTAX,
                            Some(format!("{:?}", a).as_str()),
                        ),
                    }
                    i += 2;
                }
                Instructions::TIMES => {
                    let mut insts = vec![];
                    let times = tokens[i + 1]
                        .get_val_or_reg_val(&self.registers, &self.acc)
                        .get_int()
                        .unwrap_or_else(|| panic!("{}", Error::INVALID_RUN_BLOCK_SYNTAX.extract()));
                    for i in tokens.iter().skip(i + 2) {
                        if *i == Instructions::EOL {
                            break;
                        } else {
                            insts.push(i.clone());
                        }
                    }
                    let mut id = 0;
                    while times > id {
                        self.exec(insts.clone());
                        id += 1;
                    }
                    i += insts.len() + 3;
                }
                Instructions::DATA(_) => {
                    i += 1;
                }
                aas => {
                    println!("Unimplemented: {:?}", aas);
                    i += 1;
                }
            }
        }
    }
    fn get_reg(&self, token: &Instructions) -> VenObjects {
        match token {
            Instructions::REG(rid) => self.registers[*rid].clone(),
            Instructions::ACC => self.acc.clone(),
            Instructions::DATA(reg) => reg.clone(),
            a => {
                Error::throw(
                    Error::INVALID_REGISTER_OR_VALUE,
                    Some(format!("{:?}", a).as_str()),
                );
                VenObjects::Int(0)
            }
        }
    }
    fn get_reg_id(&self, token: &Instructions) -> Option<usize> {
        match token {
            Instructions::REG(num) => Some(*num),
            _ => None,
        }
    }

    fn operate<F: Fn(f64, f64) -> f64>(&mut self, i: usize, tokens: Vec<Instructions>, f: F) {
        let to = self.get_reg(&tokens[i + 1]);
        let from = self.get_reg(&tokens[i + 2]).get_int();
        if let Some(to) = to.get_int() {
            if let Some(from) = from {
                self.acc = VenObjects::Int(f(to as f64, from as f64).round() as i64);
                if let Some(rid) = self.get_reg_id(&tokens[i + 1]) {
                    self.registers[rid] = self.acc.clone();
                }
            } else {
                let from = tokens[i + 2]
                    .get_val_or_reg_val(&self.registers, &self.acc)
                    .get_float()
                    .unwrap();
                self.acc = VenObjects::Float(f(to as f64, from));
                if let Some(rid) = self.get_reg_id(&tokens[i + 1]) {
                    self.registers[rid] = self.acc.clone();
                }
            }
        } else {
            let to = tokens[i + 1]
                .get_val_or_reg_val(&self.registers, &self.acc)
                .get_float()
                .unwrap();
            let from = tokens[i + 2]
                .get_val_or_reg_val(&self.registers, &self.acc)
                .get_float()
                .unwrap();

            self.acc = VenObjects::Float(f(to, from));
            if let Some(rid) = self.get_reg_id(&tokens[i + 1]) {
                self.registers[rid] = self.acc.clone();
            }
        }
    }
}
