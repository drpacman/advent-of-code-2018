use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};
use std::fmt;
use regex::Regex;

struct Instruction {
    op_code : OpCode,
    a : usize,
    b : usize,
    c : usize
}

struct Programme {
    instructions : Vec<Instruction>,
    ip_reg : usize
}

impl Programme {
    fn execute(&self, registers: &mut [usize; 6]) -> bool {
        // get the instruction to execute
        let ip = registers[ self.ip_reg ];
        let instr_id = registers[self.ip_reg];
        let inst = &self.instructions[instr_id];
        (inst.op_code)( registers, inst.a, inst.b, inst.c );
        // update the instruction pointer
        let next_ip = registers[ self.ip_reg ] + 1;   
        if next_ip >= self.instructions.len() {
            return false;
        } 
        registers[ self.ip_reg ] = next_ip;
        true
    }
}

fn parse_file(filename : &str) -> Result<Programme, std::io::Error> {
    let mut op_codes = HashMap::<&str, OpCode>::new();
    op_codes.insert("addr", addr);
    op_codes.insert("addi", addi);
    op_codes.insert("mulr", mulr);
    op_codes.insert("muli", muli);
    op_codes.insert("banr", banr);
    op_codes.insert("bani", bani);
    op_codes.insert("borr", borr);
    op_codes.insert("bori", bori);
    op_codes.insert("setr", setr);
    op_codes.insert("seti", seti);
    op_codes.insert("gtir", gtir);
    op_codes.insert("gtri", gtri);
    op_codes.insert("gtrr", gtrr);
    op_codes.insert("eqir", eqir);
    op_codes.insert("eqri", eqri);
    op_codes.insert("eqrr", eqrr);
    
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut lines_iter = contents.trim().split("\n").into_iter();
    let ip_re = Regex::new(r"#ip ([0-5])").unwrap();
    let instr_re = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();
    let mut instructions = Vec::<Instruction>::new();
    let mut ip_reg = 0;
    
    let first_line = lines_iter.next().unwrap();
    if let Some(cap) = ip_re.captures_iter(first_line).nth(0){
        ip_reg = cap[1].parse::<usize>().unwrap()            
    } else {
        panic!("Missing expected first line of programme")
    }

    while let Some(line) = lines_iter.next() {
        if let Some(cap) = instr_re.captures_iter(line).nth(0) {
            let op_id = cap[1].to_string();
            instructions.push( Instruction {
                op_code : *op_codes.get(op_id.as_str()).unwrap(),
                a : cap[2].parse::<usize>().unwrap(),
                b : cap[3].parse::<usize>().unwrap(),
                c : cap[4].parse::<usize>().unwrap() 
            });
        } else {
            panic!("Unexpected line {}", line);
        }
    }
    Ok(Programme { instructions : instructions, ip_reg: ip_reg })
}

fn addr(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { registers[c] = registers[a] + registers[b]; }
fn addi(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { registers[c] = registers[a] + b; }
fn mulr(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { registers[c] = registers[a] * registers[b]; }
fn muli(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { registers[c] = registers[a] * b; }
fn banr(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { registers[c] = registers[a] & registers[b]; }
fn bani(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { registers[c] = registers[a] & b; }
fn borr(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { registers[c] = registers[a] | registers[b]; }
fn bori(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { registers[c] = registers[a] | b; }
fn setr(registers: &mut [usize; 6], a : usize, _b: usize, c: usize) { registers[c] = registers[a]; }
fn seti(registers: &mut [usize; 6], a : usize, _b: usize, c: usize) { registers[c] = a; }
fn gtir(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { if a > registers[b] { registers[c] = 1 } else { registers[c] = 0 }; }
fn gtri(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { if registers[a] > b { registers[c] = 1 } else { registers[c] = 0 }; }
fn gtrr(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { if registers[a] > registers[b] { registers[c] = 1 } else { registers[c] = 0 }; }
fn eqir(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { if a == registers[b] { registers[c] = 1 } else { registers[c] = 0 }; }
fn eqri(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { if registers[a] == b { registers[c] = 1 } else { registers[c] = 0 }; }
fn eqrr(registers: &mut [usize; 6], a : usize, b: usize, c: usize) { if registers[a] == registers[b] { registers[c] = 1 } else { registers[c] = 0 }; }

type OpCode = fn(&mut[usize;6], usize, usize, usize) -> ();

fn part1() {
    let mut programme = parse_file("input.txt").unwrap();
    let mut registers : [usize; 6] = [0; 6];
    while programme.execute(&mut registers) {}
    println!("Part1 - {:?}", registers);
}

fn part2() {
    let mut result : u64= 0;
    let mut y = 1;
    while y <= 10551260 {
        let mut x = 1;
        while x <= 10551260 {
            if x*y as u64 == 10551260 { 
                result = result + y 
            } else if (x*y >10551260) {
                break;
            }
            x=x+1
        }
        y=y+1
    }
    println!("Part2 -{:?}", result);
}
fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_programme() {
        let programme = parse_file("example.txt").unwrap();
        assert_eq!(programme.ip_reg, 0);
        assert_eq!(programme.instructions.len(), 7);
    }

    #[test]
    fn test_execute() {
        let programme = parse_file("example.txt").unwrap();
        let mut registers : [usize; 6] = [0; 6];
        while programme.execute(&mut registers) {
            println!("{:?}", registers);
        }
        assert_eq!(registers[0], 6);
    }

}
