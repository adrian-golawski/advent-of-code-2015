use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let mut instructions: Vec<Instruction> = include_str!("input.txt")
        .lines()
        .map(parse_instruction)
        .collect();

    let mut memory = Memory::new();

    memory.execute_instructions(&instructions);

    let signal_a = memory.read_register(&Register("a".to_string())).unwrap();

    println!("[Part one]: {}", signal_a);

    let mut memory = Memory::new();

    match instructions.iter_mut().find(|x| match x {
        Instruction::ASSIGN(_, Register(x)) => *x == "b".to_string(),
        _ => false,
    }) {
        Some(Instruction::ASSIGN(a, _)) => *a = Address::Value(Value(signal_a)),
        _ => {}
    }

    memory.execute_instructions(&instructions);

    println!(
        "[Part two]: {}",
        memory.read_register(&Register("a".to_string())).unwrap()
    );
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Register(String);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Value(u16);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Address {
    Register(Register),
    Value(Value),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    ASSIGN(Address, Register),
    OR(Address, Address, Register),
    AND(Address, Address, Register),
    LSHIFT(Value, Register, Register),
    RSHIFT(Value, Register, Register),
    NOT(Address, Register),
}

fn parse_instruction(input: &str) -> Instruction {
    peg::parser! {
      grammar instruction_parser() for str {
        rule register() -> Register = a:$(['a'..='z']+) {
          Register (a.to_string())
        }

        rule value() -> Value = s:$(['0'..='9']+) { Value(s.parse().unwrap()) }

        rule address() -> Address = a:register() { Address::Register(a) } / a:value() { Address::Value(a) }

        rule or() -> Instruction = a:address()" OR " b:address() " -> " c:register() {
          Instruction::OR(a, b, c)
        }

        rule and() -> Instruction = a:address()" AND " b:address() " -> " c:register() {
          Instruction::AND(a, b, c)
        }

        rule lshift() -> Instruction = a:register() " LSHIFT " shift:value() " -> " b:register() {
          Instruction::LSHIFT(shift, a, b)
        }

        rule rshift() -> Instruction = a:register() " RSHIFT " shift:value() " -> " b:register() {
          Instruction::RSHIFT(shift, a, b)
        }

        rule assign() -> Instruction = assign:address() " -> " a:register() {
          Instruction::ASSIGN(assign, a)
        }

        rule not() -> Instruction = "NOT " a:address() " -> " b:register() {
          Instruction::NOT(a, b)
        }

        pub(crate) rule parse() -> Instruction = or() / and() / lshift() / rshift() / assign() / not()
      }
    }
    return instruction_parser::parse(input).unwrap();
}

struct Memory(HashMap<Register, u16>);

impl Memory {
    fn new() -> Memory {
        return Memory(HashMap::new());
    }

    fn read_address(&self, source: &Address) -> Option<u16> {
        match source {
            Address::Register(r) => self.read_register(&r),
            Address::Value(value) => Some(value.0),
        }
    }

    fn read_register(&self, source: &Register) -> Option<u16> {
        match self.0.get(&source) {
            Some(&x) => Some(x.clone()),
            None => None,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> anyhow::Result<(), ()> {
        match instruction {
            Instruction::ASSIGN(source, target) => {
                let value = match self.read_address(&source) {
                    Some(x) => x,
                    None => return Err(()),
                };
                self.0.insert(target, value);
                Ok(())
            }
            Instruction::OR(source_a, source_b, target) => {
                let value_a = match self.read_address(&source_a) {
                    Some(x) => x,
                    None => return Err(()),
                };
                let value_b = match self.read_address(&source_b) {
                    Some(x) => x,
                    None => return Err(()),
                };
                self.0.insert(target, value_a | value_b);
                Ok(())
            }
            Instruction::AND(source_a, source_b, target) => {
                let value_a = match self.read_address(&source_a) {
                    Some(x) => x,
                    None => return Err(()),
                };
                let value_b = match self.read_address(&source_b) {
                    Some(x) => x,
                    None => return Err(()),
                };
                self.0.insert(target, value_a & value_b);
                Ok(())
            }
            Instruction::LSHIFT(shift, source, target) => {
                let shift_value = shift.0;
                let shift_source = match self.read_register(&source) {
                    Some(x) => x,
                    None => return Err(()),
                };
                self.0.insert(target, shift_source << shift_value);
                Ok(())
            }
            Instruction::RSHIFT(shift, source, target) => {
                let shift_value = shift.0;
                let shift_source = match self.read_register(&source) {
                    Some(x) => x,
                    None => return Err(()),
                };
                self.0.insert(target, shift_source >> shift_value);
                Ok(())
            }
            Instruction::NOT(source, target) => {
                let source = match self.read_address(&source) {
                    Some(x) => x,
                    None => return Err(()),
                };
                self.0.insert(target, !source);
                Ok(())
            }
        }
    }

    fn execute_instructions(&mut self, instructions: &Vec<Instruction>) {
        let mut retry_instructions = Vec::new();
        for instruction in instructions {
            match self.execute_instruction(instruction.clone()) {
                Ok(()) => {}
                Err(()) => {
                    retry_instructions.push(instruction.clone());
                }
            };
        }

        while retry_instructions.len() != 0 {
            let mut failed_instructions = Vec::new();

            for instruction in retry_instructions.clone() {
                match self.execute_instruction(instruction.clone()) {
                    Ok(()) => {}
                    Err(()) => {
                        failed_instructions.push(instruction.clone());
                    }
                };
            }

            retry_instructions = failed_instructions;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let tests = [
            (
                "x OR yy -> ee",
                Instruction::OR(
                    Address::Register(Register("x".to_string())),
                    Address::Register(Register("yy".to_string())),
                    Register("ee".to_string()),
                ),
            ),
            (
                "xx AND yy -> ee",
                Instruction::AND(
                    Address::Register(Register("xx".to_string())),
                    Address::Register(Register("yy".to_string())),
                    Register("ee".to_string()),
                ),
            ),
            (
                "pp LSHIFT 2 -> qq",
                Instruction::LSHIFT(
                    Value(2),
                    Register("pp".to_string()),
                    Register("qq".to_string()),
                ),
            ),
            (
                "pp RSHIFT 2 -> qq",
                Instruction::RSHIFT(
                    Value(2),
                    Register("pp".to_string()),
                    Register("qq".to_string()),
                ),
            ),
            (
                "NOT dq -> kh",
                Instruction::NOT(
                    Address::Register(Register("dq".to_string())),
                    Register("kh".to_string()),
                ),
            ),
            (
                "100 -> bb",
                Instruction::ASSIGN(Address::Value(Value(100)), Register("bb".to_string())),
            ),
        ];

        for (expected, result) in tests.iter() {
            assert_eq!(parse_instruction(&expected), *result);
        }
    }

    #[test]
    fn test_register() {
        let mut register = Memory::new();

        register
            .execute_instruction(Instruction::ASSIGN(
                Address::Value(Value(1)),
                Register("x".to_string()),
            ))
            .unwrap();

        assert_eq!(
            register.read_register(&Register("x".to_string())).unwrap(),
            0b0001
        );

        register
            .execute_instruction(Instruction::OR(
                Address::Value(Value(0b1010)),
                Address::Value(Value(0b1100)),
                Register("y".to_string()),
            ))
            .unwrap();
        assert_eq!(
            register.read_register(&Register("y".to_string())).unwrap(),
            0b1110
        );
        register
            .execute_instruction(Instruction::AND(
                Address::Value(Value(0b1010)),
                Address::Value(Value(0b1100)),
                Register("y".to_string()),
            ))
            .unwrap();
        assert_eq!(
            register.read_register(&Register("y".to_string())).unwrap(),
            0b1000
        );

        register
            .execute_instruction(Instruction::LSHIFT(
                Value(0b1),
                Register("y".to_string()),
                Register("y".to_string()),
            ))
            .unwrap();

        assert_eq!(
            register.read_register(&Register("y".to_string())).unwrap(),
            0b10000
        );

        register
            .execute_instruction(Instruction::RSHIFT(
                Value(0b1),
                Register("y".to_string()),
                Register("y".to_string()),
            ))
            .unwrap();

        assert_eq!(
            register.read_register(&Register("y".to_string())).unwrap(),
            0b1000
        );

        register
            .execute_instruction(Instruction::NOT(
                Address::Value(Value(0b1111111111111111)),
                Register("y".to_string()),
            ))
            .unwrap();

        assert_eq!(
            register.read_register(&Register("y".to_string())).unwrap(),
            0b0
        );
    }

    #[test]
    fn test_together() {
        let s: Vec<Instruction> = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"
            .lines()
            .map(parse_instruction)
            .collect();

        let mut memory = Memory::new();
        for instruction in s {
            memory.execute_instruction(instruction).unwrap();
        }

        let expected: [(&str, u16); 8] = [
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];

        for (register, result) in expected.iter() {
            assert_eq!(
                memory
                    .read_register(&Register(register.to_string()))
                    .unwrap(),
                *result
            );
        }

        dbg!(&memory.0);
    }
}

// --- Day 7: Some Assembly Required ---

// This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates! Unfortunately,
// little Bobby is a little under the recommended age range, and he needs help assembling the circuit.

// Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal (a number from 0 to 65535).
// A signal is provided to each wire by a gate, another wire, or some specific value.
// Each wire can only get a signal from one source, but can provide its signal to multiple destinations.
// A gate provides no signal until all of its inputs have a signal.

// The included instructions booklet describes how to connect the parts together:
// x AND y -> z means to connect wires x and y to an AND gate,
// and then connect its output to wire z.

// For example:

//     123 -> x means that the signal 123 is provided to wire x.
//     x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
//     p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
//     NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.

// Other possible gates include OR (bitwise OR) and RSHIFT (right-shift).
// If, for some reason, you'd like to emulate the circuit instead,
// almost all programming languages (for example, C, JavaScript, or Python) provide operators for these gates.

// For example, here is a simple circuit:

// 123 -> x
// 456 -> y
// x AND y -> d
// x OR y -> e
// x LSHIFT 2 -> f
// y RSHIFT 2 -> g
// NOT x -> h
// NOT y -> i

// After it is run, these are the signals on the wires:

// d: 72
// e: 507
// f: 492
// g: 114
// h: 65412
// i: 65079
// x: 123
// y: 456

// In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?

// --- Part Two ---

// Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires (including wire a). What new signal is ultimately provided to wire a?
