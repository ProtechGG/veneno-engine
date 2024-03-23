#![allow(non_upper_case_globals, clippy::expect_fun_call)]

use crate::{error::Error, insts::Instructions, venobjects::VenObjects};
use std::{collections::HashMap, process::exit};

#[derive(Debug)]
pub struct CPU {
    pub registers: Vec<VenObjects>,
    pub acc: VenObjects,
    pub blocks: HashMap<String, Vec<Instructions>>,
    pub aliases: HashMap<String, usize>,
    pub tokens: Vec<Instructions>,
}

impl CPU {
    pub fn init(&mut self, no_of_regs: usize) {
        for _ in 0..no_of_regs {
            self.registers.push(VenObjects::Int(0));
        }
    }
    pub fn exec(&mut self, tokens: Option<&Vec<Instructions>>) {
        let insts;
        match tokens {
            Some(tok) => insts = tok,
            None => {
                let temp = self.tokens.clone();
                self.exec(Some(&temp));
                return;
            }
        }
        let tokens = insts;
        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].clone() {
                Instructions::BLOCK(name, insts) => {
                    let mut insts = insts;
                    insts.remove(0);
                    if name == "main" {
                        self.exec(Some(&insts));
                        exit(0);
                    }
                }
                Instructions::DECLARE => {
                    if tokens[i + 1] == Instructions::ACC {
                        Error::throw(
                            Error::CANNOT_DECLARE_ACC,
                            Some("Cannot declare acc, acc is a special value"),
                        );
                    }
                    let reg = self
                        .get_reg_id(&tokens[i + 1])
                        .expect(&Error::INVALID_REGISTER_OR_VALUE.extract());
                    let alias = &tokens[i + 2]
                        .extract_value()
                        .expect(&Error::INVALID_DECLARATION.extract());
                    self.aliases.insert(alias.get_str(), reg);
                    i += 3;
                }
                Instructions::MOV => {
                    let val = tokens[i + 2].clone();
                    let val = val.get_val_or_reg_val(&self.registers, &self.acc, &self.aliases);

                    match tokens[i + 1].clone() {
                        Instructions::REG(id) => self.registers[id] = val,
                        Instructions::ACC => self.acc = val,
                        Instructions::DATA(VenObjects::Str(alias)) => {
                            if let Some(&reg_id) = self.aliases.get(&alias) {
                                self.registers[reg_id] = val;
                            } else {
                                Error::throw(
                                    Error::INVALID_VALUE_FOR_MOVE,
                                    Some(
                                        format!("{:?}", Instructions::DATA(VenObjects::Str(alias)))
                                            .as_str(),
                                    ),
                                );
                            }
                        }
                        a => Error::throw(
                            Error::INVALID_VALUE_FOR_MOVE,
                            Some(format!("{:?}", a).as_str()),
                        ),
                    }
                    i += 3;
                }
                Instructions::AND => {
                    self.operate_bool(i, &tokens, |x, y| x && y);
                    i += 3;
                }
                Instructions::OR => {
                    self.operate_bool(i, &tokens, |x, y| x || y);
                    i += 3;
                }
                Instructions::XOR => {
                    self.operate_bool(i, &tokens, |x, y| x ^ y);
                    i += 3;
                }
                Instructions::ROOT => {
                    self.operate_int(i, &tokens, |x, y| x.powf(1.0 / y));
                    i += 3;
                }
                Instructions::ADD => {
                    self.operate_int(i, &tokens, |x, y| x + y);
                    i += 3;
                }
                Instructions::SUB => {
                    self.operate_int(i, &tokens, |x, y| x - y);
                    i += 3;
                }
                Instructions::DIV => {
                    self.operate_int(i, &tokens, |x, y| x / y);
                    i += 3;
                }
                Instructions::MUL => {
                    self.operate_int(i, &tokens, |x, y| x * y);
                    i += 3;
                }
                Instructions::POW => {
                    self.operate_int(i, &tokens, |x, y| x.powf(y));
                    i += 3;
                }
                Instructions::LT => {
                    self.operate_int_to_bool(i, &tokens, |x, y| x < y);
                    i += 3;
                }
                Instructions::GT => {
                    self.operate_int_to_bool(i, &tokens, |x, y| x > y);
                    i += 3;
                }
                Instructions::EQ => {
                    self.operate_int_to_bool(i, &tokens, |x, y| x == y);
                    i += 3;
                }
                Instructions::NOT => {
                    let bool_cond =
                        tokens[i + 1].get_val_or_reg_val(&self.registers, &self.acc, &self.aliases);
                    self.acc = VenObjects::Bool(
                        !bool_cond
                            .get_bool()
                            .expect(&Error::INVALID_BOOL_OPERAND.extract()),
                    );
                    let bool_id = self.get_reg_id(&tokens[i + 1]);
                    match bool_id {
                        Some(bool_id) => self.registers[bool_id] = self.acc.clone(),
                        None => {}
                    }
                    i += 2;
                }

                Instructions::PRINT => {
                    let text =
                        tokens[i + 1].get_val_or_reg_val(&self.registers, &self.acc, &self.aliases);
                    match text {
                        VenObjects::Int(num) => print!("{}", num),
                        VenObjects::Str(stri) => print!("{}", stri),
                        VenObjects::Float(float) => print!("{}", float),
                        VenObjects::Bool(bol) => print!("{}", bol),
                        VenObjects::Class(name, insts) => print!("{}: {:?}", name, insts),
                        VenObjects::Function(name, body) => print!("{}: {:?}", name, body),
                    }
                    i += 2;
                }
                Instructions::PRINTLN => {
                    let text =
                        tokens[i + 1].get_val_or_reg_val(&self.registers, &self.acc, &self.aliases);
                    match text {
                        VenObjects::Int(num) => println!("{}", num),
                        VenObjects::Str(stri) => println!("{}", stri),
                        VenObjects::Float(float) => println!("{}", float),
                        VenObjects::Bool(bol) => println!("{}", bol),
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
                            self.exec(Some(
                                &self
                                    .blocks
                                    .get(&block_name)
                                    .expect("Error: invalid run block command")
                                    .clone(),
                            ));
                        }
                        a => Error::throw(
                            Error::INVALID_RUN_BLOCK_SYNTAX,
                            Some(format!("{:?}", a).as_str()),
                        ),
                    }
                    i += 2;
                }
                Instructions::IF => {
                    let condition = &tokens[i + 1].get_val_or_reg_val(
                        &self.registers,
                        &self.acc,
                        &self.aliases,
                    );
                    let mut inst = vec![];
                    let mut is_else = vec![];

                    for i in tokens.iter().skip(i + 1).enumerate() {
                        if *i.1 == Instructions::EOL {
                            if tokens[i.0 + 1] == Instructions::ELSE {
                                for i in tokens.iter().skip(i.0 + 1) {
                                    if *i == Instructions::EOL {
                                        break;
                                    }
                                    is_else.push(i);
                                }
                            }
                            break;
                        } else {
                            inst.push(i.1.clone());
                        }
                    }
                    if condition
                        .get_bool()
                        .expect(&Error::INVALID_BOOL_OPERAND.extract())
                    {
                        self.exec(Some(&inst));
                    } else if !is_else.is_empty() {
                        self.exec(Some(&inst));
                        i += 4;
                    }
                    i += 2 + 3;
                }
                Instructions::TIMES => {
                    let mut insts = vec![];
                    let times = tokens[i + 1]
                        .get_val_or_reg_val(&self.registers, &self.acc, &self.aliases)
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
                        self.exec(Some(&insts));
                        id += 1;
                    }
                    i += insts.len() + 3;
                }
                Instructions::REG(_) => {
                    i += 1;
                }
                Instructions::ACC => {
                    i += 1;
                }
                Instructions::EOL => {
                    i += 1;
                }
                Instructions::DATA(_) => {
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
    }
    fn get_reg(&self, token: &Instructions) -> VenObjects {
        match token {
            Instructions::REG(rid) => self.registers[*rid].clone(),
            Instructions::ACC => self.acc.clone(),
            Instructions::DATA(VenObjects::Str(alias)) => {
                if let Some(&rid) = self.aliases.get(alias.as_str()) {
                    self.registers[rid].clone()
                } else {
                    VenObjects::Str(alias.to_string())
                }
            }
            Instructions::TRUE => VenObjects::Bool(true),
            Instructions::FALSE => VenObjects::Bool(false),
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
            Instructions::DATA(VenObjects::Str(alias)) => {
                if let Some(&rid) = self.aliases.get(alias) {
                    Some(rid)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn operate_int_to_bool<F: Fn(f64, f64) -> bool>(
        &mut self,
        i: usize,
        tokens: &[Instructions],
        f: F,
    ) {
        let to = tokens[i + 1].get_val_or_reg_val(&self.registers, &self.acc, &self.aliases);
        let from = tokens[i + 2].get_val_or_reg_val(&self.registers, &self.acc, &self.aliases);
        let mut can_be_float = false;
        if to.get_int() == None || from.get_int() == None {
            can_be_float = true;
        }
        if !can_be_float {
            let to = to.get_int().unwrap();
            let from = from.get_int().unwrap();

            self.acc = VenObjects::Bool(f(to as f64, from as f64));
        }
        if can_be_float {
            let to = to.get_float().expect(&Error::INVALID_INT_OPERAND.extract());
            let from = from
                .get_float()
                .expect(&Error::INVALID_INT_OPERAND.extract());
            self.acc = VenObjects::Bool(f(to, from));
        }
    }

    fn operate_int<F: Fn(f64, f64) -> f64>(&mut self, i: usize, tokens: &[Instructions], f: F) {
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
                    .get_val_or_reg_val(&self.registers, &self.acc, &self.aliases)
                    .get_float()
                    .unwrap();
                self.acc = VenObjects::Float(f(to as f64, from));
                if let Some(rid) = self.get_reg_id(&tokens[i + 1]) {
                    self.registers[rid] = self.acc.clone();
                }
            }
        } else {
            let to = tokens[i + 1]
                .get_val_or_reg_val(&self.registers, &self.acc, &self.aliases)
                .get_float()
                .unwrap();
            let from = tokens[i + 2]
                .get_val_or_reg_val(&self.registers, &self.acc, &self.aliases)
                .get_float()
                .unwrap();

            self.acc = VenObjects::Float(f(to, from));
            if let Some(rid) = self.get_reg_id(&tokens[i + 1]) {
                self.registers[rid] = self.acc.clone();
            }
        }
    }

    fn operate_bool<F: Fn(bool, bool) -> bool>(&mut self, i: usize, tokens: &[Instructions], f: F) {
        let to = self
            .get_reg(&tokens[i + 1])
            .get_bool()
            .expect(&Error::INVALID_BOOL_OPERAND.extract());
        let from = self
            .get_reg(&tokens[i + 2])
            .get_bool()
            .expect(&Error::INVALID_BOOL_OPERAND.extract());
        self.acc = VenObjects::Bool(f(to, from));
    }
}
