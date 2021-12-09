use std::collections::HashMap;

fn main() {
    let instructions: Vec<Instruction> = include_str!("input.txt")
        .split('\n')
        .map(map_line)
        .collect();

    let mut registers: HashMap<char, i64> = HashMap::new();
    registers.insert('a', 0);
    registers.insert('b', 0);

    let final_register = execute_instructions(&registers, &instructions);

    let part_1 = final_register.get(&'b').unwrap();
    dbg!(part_1);

    let mut registers: HashMap<char, i64> = HashMap::new();
    registers.insert('a', 1);
    registers.insert('b', 0);

    let final_register = execute_instructions(&registers, &instructions);

    let part_2 = final_register.get(&'b').unwrap();
    dbg!(part_2);
}

fn execute_instructions(
    register: &HashMap<char, i64>,
    instructions: &[Instruction],
) -> HashMap<char, i64> {
    let mut registers = register.clone();

    let mut pointer: i64 = 0;

    while pointer >= 0 && pointer < instructions.len() as i64 {
        match &instructions.get(pointer as usize).unwrap() {
            Instruction::Jump(p) => {
                pointer += p;
            }
            Instruction::JumpIfEven(r, p) => {
                if registers.get(r).unwrap() % 2 == 0 {
                    pointer += p;
                } else {
                    pointer += 1;
                }
            }
            Instruction::JumpIfOne(r, p) => {
                if registers.get(r).unwrap() == &1 {
                    pointer += p;
                } else {
                    pointer += 1;
                }
            }
            Instruction::Half(r) => {
                if let Some(x) = registers.get_mut(r) {
                    *x >>= 1;
                    pointer += 1;
                }
            }
            Instruction::Triple(r) => {
                if let Some(x) = registers.get_mut(r) {
                    *x *= 3;
                    pointer += 1;
                }
            }
            Instruction::Increment(r) => {
                if let Some(x) = registers.get_mut(r) {
                    *x += 1;
                    pointer += 1;
                }
            }
        }
    }

    registers
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Jump(i64),
    JumpIfEven(char, i64),
    JumpIfOne(char, i64),
    Half(char),
    Triple(char),
    Increment(char),
}

// hlf r sets register r to half its current value, then continues with the next instruction.
// tpl r sets register r to triple its current value, then continues with the next instruction.
// inc r increments register r, adding 1 to it, then continues with the next instruction.
// jmp offset is a jump; it continues with the instruction offset away relative to itself.
// jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
// jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).

fn map_line(line: &str) -> Instruction {
    let split_line: Vec<&str> = line.split(" ").collect();

    match *split_line.get(0).unwrap() {
        "jmp" => {
            let jmp: i64 = split_line.get(1).unwrap().parse::<i64>().unwrap();
            Instruction::Jump(jmp)
        }
        "jie" => {
            let register: char = split_line.get(1).unwrap().chars().next().unwrap();
            let jmp: i64 = split_line.get(2).unwrap().parse::<i64>().unwrap();
            Instruction::JumpIfEven(register, jmp)
        }
        "jio" => {
            let register: char = split_line.get(1).unwrap().chars().next().unwrap();
            let jmp: i64 = split_line.get(2).unwrap().parse::<i64>().unwrap();
            Instruction::JumpIfOne(register, jmp)
        }
        "tpl" => {
            let register: char = split_line.get(1).unwrap().chars().next().unwrap();
            Instruction::Triple(register)
        }
        "hlf" => {
            let register: char = split_line.get(1).unwrap().chars().next().unwrap();
            Instruction::Half(register)
        }
        "inc" => {
            let register: char = split_line.get(1).unwrap().chars().next().unwrap();
            Instruction::Increment(register)
        }
        _ => panic!("Unkown value"),
    }
}
mod tests {
    use super::*;

    #[test]
    fn test_number_sum() {
        assert_eq!(map_line("jmp +23"), Instruction::Jump(23));
        assert_eq!(map_line("jmp -23"), Instruction::Jump(-23));
        assert_eq!(map_line("jio a, -23"), Instruction::JumpIfOne('a', -23));
        assert_eq!(map_line("jie a, -23"), Instruction::JumpIfEven('a', -23));
        assert_eq!(map_line("hlf a"), Instruction::Half('a'));
        assert_eq!(map_line("tpl a"), Instruction::Triple('a'));
        assert_eq!(map_line("inc a"), Instruction::Increment('a'));
    }

    #[test]
    fn test_execute_instructions() {
        let mut map = HashMap::new();
        map.insert('a', 0);
        map.insert('b', 0);

        let instructions = vec![
            Instruction::Increment('a'),
            Instruction::JumpIfOne('a', 2),
            Instruction::Triple('a'),
            Instruction::Increment('a'),
        ];

        let mut expected_map = HashMap::new();
        expected_map.insert('a', 2);
        expected_map.insert('b', 0);

        assert_eq!(execute_instructions(&map, &instructions), expected_map);
    }
}
