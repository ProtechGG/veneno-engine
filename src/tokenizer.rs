use crate::{cpu::CPU, insts::Instructions};
use std::process::exit;

impl CPU {
    pub fn parse_instructions(&mut self, insts: String) {
        let mut current_token = String::new();
        let mut tokens = vec![];
        let mut current_block = vec![];
        let mut is_block = false;
        let mut block_name: String = "".into();
        let mut is_string = false;
        for i in insts.chars() {
            if current_token == "block" && !is_string {
                is_block = true;
            }
            if current_token.starts_with('#') && current_token.ends_with('!') {
                current_token.remove(0);
                current_token.remove(current_token.len() - 1);
                match current_token.remove(0) {
                    'r' => self.init(
                        current_token
                            .parse::<usize>()
                            .expect("Cannot initialize registers"),
                    ),
                    a => {
                        eprintln!("Invalid command: {}", a)
                    }
                }
                current_token.clear();
            }
            if (i == ' ' || i == ',') && is_block && !current_token.is_empty() && !is_string {
                current_block.push(Instructions::build_from_str(
                    current_token.trim().to_lowercase().as_str(),
                ));
                current_token.clear();
            } else if i == '"' {
                is_string = !is_string;
            } else if (i == '\n' || i == '\t') && is_block && !is_string {
                if !current_token.trim().is_empty() {
                    current_block.push(Instructions::build_from_str(
                        current_token.trim().to_lowercase().as_str(),
                    ));
                }
                current_token.clear();
            } else if i == ';' && is_block && !is_string {
                if !current_token.trim().is_empty() {
                    current_block.push(Instructions::build_from_str(
                        current_token.trim().to_lowercase().as_str(),
                    ));
                }
                current_token.clear();
                current_block.push(Instructions::EOL);
            } else if i == ':' && !is_string {
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
            if current_token == "end" && !is_string {
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
            if !is_string {
                current_token = current_token.trim().to_string();
            }
        }
        self.tokens = tokens;
    }
}
