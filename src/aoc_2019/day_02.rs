
type Code = i32;
const INTCODE_HALT: Code = 99;
const INTCODE_ADD: Code = 1;
const INTCODE_MUL: Code = 2;

pub fn main(puzzle_input: &String) {
    let code = parse_code(puzzle_input);
    println!("Part 1: {}", run_computer(&code, 12, 2));
    let (noun, verb) = gravity_assist(&code, 19690720);
    println!("Part 2: {}", noun * 100 + verb);
}

fn run_computer(code: &Vec<Code>, noun: Code, verb: Code) -> Code {
    let mut computer = IntCode::new(code);
    computer.code[1] = noun;
    computer.code[2] = verb;
    computer.run();
    computer.code[0]
}

fn gravity_assist(code: &Vec<Code>, target: Code) -> (Code, Code) {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_computer(code, noun, verb) == target {
                return (noun, verb)
            }
        }
    }
    return (-1, -1)
}

struct IntCode {
    pub code: Vec<Code>,
    pointer: usize,
}

fn parse_code(contents: &String) -> Vec<Code> {
    let mut code = Vec::new();
    for line in contents.split(",") {
        if let Ok(n) = line.trim().parse() {
            code.push(n);
        }
    }
    code
}

impl IntCode {
    fn new(code: &Vec<Code>) -> IntCode {
        let code = code.clone();
        IntCode { code, pointer: 0 }
    }

    fn is_over(&self) -> bool {
        self.code[self.pointer] == INTCODE_HALT
    }

    fn run_add(&mut self) {
        let a = self.code[self.pointer + 1] as usize;
        let b = self.code[self.pointer + 2] as usize;
        let c = self.code[self.pointer + 3] as usize;
        self.code[c] = self.code[a] + self.code[b];
        self.pointer += 4;
    }

    fn run_mul(&mut self) {
        let a = self.code[self.pointer + 1] as usize;
        let b = self.code[self.pointer + 2] as usize;
        let c = self.code[self.pointer + 3] as usize;
        self.code[c] = self.code[a] * self.code[b];
        self.pointer += 4;
    }

    fn run(&mut self) {
        while ! self.is_over() {
            match self.code[self.pointer] {
                INTCODE_ADD => self.run_add(),
                INTCODE_MUL => self.run_mul(),
                _ => panic!("Unknown intcode instruction"),
            }
        }
    }
}
