#![allow(non_upper_case_globals)]

use std::{collections::HashMap, process::exit};

use crate::{
    insts::Instructions,
    venobjects::{Float, VenObjects},
};

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
    pub fn parse_instructions(&mut self, insts: String) {
        let mut current_token = String::new();
        let mut tokens = vec![];
        let mut current_block = vec![];
        let mut is_block = false;
        let mut block_name: String = "".into();
        for i in insts.chars() {
            if current_token == "block" {
                is_block = true;
            }
            if (i == ' ' || i == ',') && is_block && !current_token.is_empty() {
                current_block.push(Instructions::build_from_str(
                    current_token.trim().to_lowercase().as_str(),
                ));
                current_token.clear();
            } else if (i == '\n' || i == '\t') && is_block {
                if !current_token.trim().is_empty() {
                    current_block.push(Instructions::build_from_str(
                        current_token.trim().to_lowercase().as_str(),
                    ));
                }
                current_token.clear();
            } else if i == ':' {
                if !is_block {
                    eprintln!("Invalid block syntax",);
                    exit(69);
                } else {
                    block_name = current_token.clone();
                    current_token.clear();
                }
            } else {
                current_token.push(i);
            }
            if current_token == "end" {
                is_block = false;
                tokens.push(Instructions::BLOCK(
                    block_name.clone(),
                    current_block.clone(),
                ));
                self.blocks
                    .insert(block_name.clone(), current_block.clone());
                current_block.clear();
                current_token.clear();
            }
            current_token = current_token.trim().to_string();
        }
        self.exec(tokens);
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
                    let val = val.get_val_or_reg_val(self.registers.clone(), self.acc.clone());

                    match tokens[i + 1] {
                        Instructions::REG(id) => self.registers[id] = val,
                        Instructions::ACC => self.acc = val,
                        _ => {
                            eprintln!("Invalid value for mov");
                            exit(69)
                        }
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
                    let text =
                        tokens[i + 1].get_val_or_reg_val(self.registers.clone(), self.acc.clone());
                    match text {
                        VenObjects::Int(num) => println!("{}", num),
                        VenObjects::Str(stri) => println!("{}", stri),
                        VenObjects::Float(float) => println!("{}", float.to_primitive()),
                        VenObjects::Class(name, insts) => println!("{}: {:?}", name, insts),
                        VenObjects::Function(name, body) => println!("{}: {:?}", name, body),
                    }
                    i += 2;
                }
                Instructions::RUN => {
                    let block_name = &tokens[i + 1];
                    match block_name {
                        Instructions::BLOCKNAME(block_name) => {
                            self.exec(
                                self.blocks
                                    .get(block_name)
                                    .expect("Error: invalid run block command")
                                    .clone(),
                            );
                        }
                        a => {
                            eprintln!("Invalid block name after run command: {:?}", a);
                            exit(69);
                        }
                    }
                    i += 2;
                }
                Instructions::BLOCKNAME(_) => i += 1,
                aas => println!("Unimplemented: {:?}", aas),
            }
        }
    }
    fn get_reg(&self, token: Instructions) -> VenObjects {
        match token {
            Instructions::REG(rid) => self.registers[rid].clone(),
            Instructions::ACC => self.acc.clone(),
            Instructions::DATA(reg) => reg.clone(),
            _ => {
                eprintln!("Error invalid register: {:?}", token);
                exit(68);
            }
        }
    }

    fn operate<F: Fn(f64, f64) -> f64>(&mut self, i: usize, tokens: Vec<Instructions>, f: F) {
        let to = self.get_reg(tokens[i + 1].clone()).get_int();
        let from = self.get_reg(tokens[i + 2].clone()).get_int();
        if let Some(to) = to {
            if let Some(from) = from {
                self.acc = VenObjects::Int(f(to as f64, from as f64).round() as i64);
            } else {
                let from = tokens[i + 2]
                    .get_val_or_reg_val(self.registers.clone(), self.acc.clone())
                    .get_float()
                    .unwrap();
                self.acc = VenObjects::Float(Float::build(f(to as f64, from)));
            }
        } else {
            let to = tokens[i + 1]
                .get_val_or_reg_val(self.registers.clone(), self.acc.clone())
                .get_float()
                .unwrap();
            if let Some(from) = from {
                self.acc = VenObjects::Float(Float::build(f(to, from as f64)));
            } else {
                let from = tokens[i + 2]
                    .get_val_or_reg_val(self.registers.clone(), self.acc.clone())
                    .get_float()
                    .unwrap();

                self.acc = VenObjects::Float(Float::build(f(to, from)));
            }
        }
    }
}
