use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::{HashSet};
use std::hash::{Hash, Hasher};
use std::fmt;

fn parse_file(filename : &str) -> Result<(Vec<[usize; 4]>,Vec<[usize; 4]>), std::io::Error> {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut lines_iter = contents.split("\n").into_iter();
    let before_re = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let instr_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    let after_re = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let mut state = 0;
    let mut part1 = Vec::<[usize; 4]>::new();

    while let Some(line) = lines_iter.next() {
        match state {
            0 => {
                if let Some(cap) = before_re.captures_iter(line).nth(0) {
                    part1.push( [ cap[1].parse::<usize>().unwrap(), 
                                  cap[2].parse::<usize>().unwrap(),
                                  cap[3].parse::<usize>().unwrap(),
                                  cap[4].parse::<usize>().unwrap() ]);
                } else {
                    break;
                }
                state = 1;
            },
            1 => {
                if let Some(cap) = instr_re.captures_iter(line).nth(0) {
                    part1.push( [ cap[1].parse::<usize>().unwrap(), 
                                  cap[2].parse::<usize>().unwrap(),
                                  cap[3].parse::<usize>().unwrap(),
                                  cap[4].parse::<usize>().unwrap() ]);
                } else {
                    panic!("Missing instruction");
                }
                state = 2;
            },
            2 => {
                if let Some(cap) = after_re.captures_iter(line).nth(0) {
                    part1.push( [ cap[1].parse::<usize>().unwrap(), 
                                  cap[2].parse::<usize>().unwrap(),
                                  cap[3].parse::<usize>().unwrap(),
                                  cap[4].parse::<usize>().unwrap() ]);
                } else {
                    panic!("Missing After");
                }
                state = 3;
            },
            3 => {
                // skip line
                state = 0
            },
            _ => ()
        }
    }
    let mut part2 : Vec<[usize; 4]>= vec![];
    while let Some(line) = lines_iter.next() {
        if let Some(cap) = instr_re.captures_iter(line).nth(0) {
            part2.push( [ cap[1].parse::<usize>().unwrap(), 
                          cap[2].parse::<usize>().unwrap(),
                          cap[3].parse::<usize>().unwrap(),
                          cap[4].parse::<usize>().unwrap() ]);
        } else if line != "" {
            panic!("Unexpected line {}", line);
        }
    }
    Ok((part1,part2))
}

fn addr(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { registers[c] = registers[a] + registers[b]; }
fn addi(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { registers[c] = registers[a] + b; }
fn mulr(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { registers[c] = registers[a] * registers[b]; }
fn muli(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { registers[c] = registers[a] * b; }
fn banr(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { registers[c] = registers[a] & registers[b]; }
fn bani(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { registers[c] = registers[a] & b; }
fn borr(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { registers[c] = registers[a] | registers[b]; }
fn bori(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { registers[c] = registers[a] | b; }
fn setr(registers: &mut [usize; 4], a : usize, _b: usize, c: usize) { registers[c] = registers[a]; }
fn seti(registers: &mut [usize; 4], a : usize, _b: usize, c: usize) { registers[c] = a; }
fn gtir(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { if a > registers[b] { registers[c] = 1 } else { registers[c] = 0 }; }
fn gtri(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { if registers[a] > b { registers[c] = 1 } else { registers[c] = 0 }; }
fn gtrr(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { if registers[a] > registers[b] { registers[c] = 1 } else { registers[c] = 0 }; }
fn eqir(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { if a == registers[b] { registers[c] = 1 } else { registers[c] = 0 }; }
fn eqri(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { if registers[a] == b { registers[c] = 1 } else { registers[c] = 0 }; }
fn eqrr(registers: &mut [usize; 4], a : usize, b: usize, c: usize) { if registers[a] == registers[b] { registers[c] = 1 } else { registers[c] = 0 }; }

type OpCode = fn(&mut[usize;4], usize, usize, usize) -> ();

#[derive(Clone, Copy)]
struct Op {
    name : &'static str,
    op_code : OpCode
}

impl Hash for Op {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl PartialEq for Op {
    fn eq(&self, other: &Op) -> bool {
        self.name == other.name
    }
}
impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Op {{ name: {} }}", self.name)
    }
}
impl Eq for Op {}

fn matching_op_codes(before: [usize;4], after : [usize;4], instr : [usize;4]) -> Vec<Op> {
    let mut op_codes = vec![ Op { name: "Addr", op_code: addr }, 
                            Op { name: "Addi", op_code: addi },
                            Op { name: "Mulr", op_code: mulr },
                            Op { name: "Muli", op_code: muli }, 
                            Op { name: "Banr", op_code: banr }, 
                            Op { name: "Bani", op_code: bani }, 
                            Op { name: "Borr", op_code: borr }, 
                            Op { name: "Bori", op_code: bori }, 
                            Op { name: "Setr", op_code: setr }, 
                            Op { name: "Seti", op_code: seti },
                            Op { name: "Grir", op_code: gtir }, 
                            Op { name: "Gtri", op_code: gtri }, 
                            Op { name: "Gtrr", op_code: gtrr }, 
                            Op { name: "Eqir", op_code: eqir }, 
                            Op { name: "Eqri", op_code: eqri }, 
                            Op { name: "Eqrr", op_code: eqrr }];
    op_codes.retain(|o| {
        let mut b : [usize;4] = [0; 4];
        b.copy_from_slice(&before);
        (o.op_code)(&mut b, instr[1], instr[2], instr[3]);
        b == after
    });
    op_codes
}

fn map_op_codes(part1 : Vec<[usize;4]>) -> [Op; 16] {
    let mut result : [Op; 16] = [Op { name: "Addr", op_code: addr }; 16];
    let mut known_op_codes = HashSet::<Op>::new();
    
    let entries_iter = part1.chunks(3);
    for e in entries_iter {
        let matching = matching_op_codes(e[0], e[2], e[1]);
        let candidate_opcodes : Vec<Op>= matching.iter().filter(|op| !known_op_codes.contains(&op)).map(|op| *op).collect();
        if candidate_opcodes.len() == 1 {
            let identified_op = candidate_opcodes.iter().nth(0).unwrap();
            result[e[1][0]] = *identified_op;
            known_op_codes.insert(*identified_op);
        }
    }
    result
}

fn part2() -> usize {
    let (part1, part2) = parse_file("input.txt").unwrap();
    let op_lookup = map_op_codes(part1);
    let mut registers : [usize; 4] = [0; 4];
    for instr in part2 {
        let op = op_lookup[instr[0]];
        (op.op_code)(&mut registers, instr[1], instr[2], instr[3]);
    }    
    registers[0]
}

fn part1() -> usize {
    let (part1, _) = parse_file("input.txt").unwrap();
    let entries_iter = part1.chunks(3);
    let entries_satisfying_criteria : Vec<bool>= entries_iter.filter(|e| {
        matching_op_codes( e[0], e[2], e[1]).len() >= 3
    }).map(|_| true).collect();
    entries_satisfying_criteria.len()
}

fn main() {
    println!("Part1 - {}", part1());
    println!("Part2 - {}", part2());
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matching_op_codes() {
        let result = matching_op_codes([3, 2, 1, 1], [3, 2, 2, 1], [9, 2, 1, 2]);
        assert_eq!(result.len(), 3);
    }
    
    #[test]
    fn test_parse_file() {
        let (part1, part2) = parse_file("input.txt").unwrap();
        assert_eq!( part1.len() % 3, 0 );
        assert_eq!( part2.len(), 938 );
    }
}
