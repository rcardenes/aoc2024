use std::io::{stdin, BufRead, BufReader, Read};

#[derive(Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => panic!("Expected a value from 0 .. 7, not {}!", value)
        }
    }
}

#[derive(Debug)]
struct CPU {
    register: Vec<i64>,
    program: Vec<u8>,
    trace: bool
}

impl CPU {
    fn new() -> Self {
        CPU {
            register: vec![0; 3],
            program: vec![],
            trace: false,
        }
    }

    #[allow(dead_code)]
    fn set_trace(&mut self, status: bool) {
        self.trace = status
    }

    fn initialize_regs(&mut self, regs: Vec<i64>) {
        assert!(regs.len() == self.register.len());

        self.register = regs
    }

    fn load_program(&mut self, program: Vec<u8>) {
        self.program = program
    }

    fn get_combo(&self, lit: u8) -> i64 {
        match lit {
            0..4 => lit as i64,
            4..7 => self.register[(lit - 4) as usize],
            7 => panic!("Reserved combo literal cannot show in legal programs"),
            _ => panic!("Illegal combo literal {lit}")
        }
    }

    fn print_trace(&self, left: &str, ip: usize) {
        if self.trace {
            eprintln!("{left:-24} | IP: {ip:4} | A: {:-10} | B: {:-10} | C: {:-10}",
                self.register[0],
                self.register[1],
                self.register[2]);
        }
    }

    fn run_program(&mut self, shortcircuit: bool) -> Vec<u8> {
        let mem_limit: usize = self.program.len() - 1;
        let mut output = vec![];
        let mut ip = 0usize;
        let mut program_copy = self.program.clone().into_iter();

        self.print_trace("START", 0);
        loop {
            if ip >= mem_limit {
                // HALT, we'll read past the end of the program
                break;
            }

            let opcode = Opcode::from(self.program[ip]);
            let operand = self.program[ip + 1];

            ip = ip + 2;
            match opcode {
                Opcode::Adv => {
                    // A = A / (2^combo)
                    let combo = self.get_combo(operand);
                    let denominator = 1i64 << combo;
                    self.register[0] = self.register[0] / denominator;
                    self.print_trace(format!("adv {combo}({operand})").as_str(), ip);
                }
                Opcode::Bxl => {
                    self.register[1] = self.register[1] ^ operand as i64;
                    self.print_trace(format!("bxl {operand}").as_str(), ip);
                },
                Opcode::Bst => {
                    let combo = self.get_combo(operand);
                    self.register[1] = combo % 8;
                    self.print_trace(format!("bst {combo}({operand})").as_str(), ip);
                }
                Opcode::Jnz => {
                    if self.register[0] != 0 {
                        ip = operand as usize;
                    }
                    self.print_trace(format!("jnz {operand}").as_str(), ip);
                }
                Opcode::Bxc => {
                    self.register[1] = self.register[1] ^ self.register[2];
                    self.print_trace("bxc", ip);
                }
                Opcode::Out => {
                    let out = (self.get_combo(operand) % 8) as u8;
                    if shortcircuit {
                        if let Some(value) = program_copy.next() {
                            if out != value {
//                                eprintln!("{output:?}");
                                return vec![]
                            }
                        } else {
                            return output
                        }
                    }
                    output.push(out);
                    self.print_trace(format!("out {out}({operand})").as_str(), ip);
                }
                Opcode::Bdv => {
                    // B = A / (2^combo)
                    let combo = self.get_combo(operand);
                    let denominator = 1i64 << combo;
                    self.register[1] = self.register[0] / denominator;
                    self.print_trace(format!("bdv {combo}({operand})").as_str(), ip);
                },
                Opcode::Cdv => {
                    // C = A / (2^combo)
                    let combo = self.get_combo(operand);
                    let denominator = 1i64 << combo;
                    self.register[2] = self.register[0] / denominator;
                    self.print_trace(format!("cdv {combo}({operand})").as_str(), ip);
                },
            }
        }

        output
    }
}

fn read_input<R>(stream: BufReader<R>) -> (CPU, String)
where 
    R: Read
{
    let mut cpu = CPU::new();
    let mut registers = vec![0i64; 3];
    let mut raw_string = String::new();

    for line in stream.lines() {
        let line = line.unwrap();

        if line.trim_end().len() != 0 {
            let (entry, value) = line.split_once(": ").unwrap();

            if entry.starts_with("Register") {
                let reg_no = match entry.chars().last() {
                    Some('A') => 0,
                    Some('B') => 1,
                    Some('C') => 2,
                    _ => panic!("Unrecognized register!")
                };

                registers[reg_no] = value.parse::<i64>().unwrap();
            } else if entry == "Program" {
                raw_string = value.to_string();
                let program = value
                    .split(",")
                    .map(|c| c.parse::<u8>().unwrap())
                    .collect::<Vec<_>>();
                cpu.load_program(program);
            } else {
                eprintln!("Ignoring entry: {entry}");
            }
        }
    }

    cpu.initialize_regs(registers);
    (cpu, raw_string)
}

fn main() {
    let (mut cpu, prog) = read_input(BufReader::new(stdin()));

    cpu.set_trace(false);

    let output = cpu.run_program(false)
        .into_iter()
        .map(|n| char::from(n + 48).to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("First half: {output}");

    let mut candidates = vec![0i64];
    let mut solutions = vec![];

    while let Some(p) = candidates.pop() {
        let prefix: i64 = p << 3;
        for i in 0..8 {
            let value = prefix | i;
            cpu.initialize_regs(vec![value, 0, 0]);
            let new_output = cpu.run_program(false)
                .into_iter()
                .map(|n| char::from(n + 48).to_string())
                .collect::<Vec<_>>()
                .join(",");

            if new_output.len() > prog.len() {
                break;
            }

            if new_output == prog {
                solutions.push(value);
            }
            else if prog.ends_with(new_output.as_str()) {
                candidates.push(value);
            }
        }
    }

    solutions.sort();
    println!("Second half: {}", solutions[0]);

}
