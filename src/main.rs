use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;
use veneno_engine::cpu;
use veneno_engine::venobjects::VenObjects;

fn main() {
    let mut arg = env::args().skip(1);
    let mut cpu = cpu::CPU {
        registers: vec![],
        acc: VenObjects::Int(0),
        blocks: HashMap::new(),
        aliases: HashMap::new(),
        tokens: vec![],
    };
    cpu.init(100);
    let data = fs::read_to_string(arg.next().unwrap().clone());
    if let Ok(data) = data {
        cpu.parse_instructions(data);
        cpu.exec(None);
    } else {
        eprintln!("Cannot read from file: {:?}", data);
        exit(69);
    }
}
