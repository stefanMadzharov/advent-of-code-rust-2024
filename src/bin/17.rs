advent_of_code::solution!(17);
use std::collections::HashMap;
use Instruction::*;
use Operand::*;

type Register = u64;
#[derive(Debug)]
enum Operand {
    LITERAL(u8),
    COMBO(u8),
}

impl Operand {
    fn get_value(&self, computer: &Computer) -> u64 {
        match *self {
            LITERAL(value) => value as u64,
            COMBO(value) => match value {
                0 | 1 | 2 | 3 => value as u64,
                4 => computer.register_a,
                5 => computer.register_b,
                6 => computer.register_c,
                7 => {
                    unreachable!("Combo operand 7 is reserved")
                }
                _ => {
                    unreachable!("The operands are only until 7")
                }
            },
        }
    }
}

#[derive(Debug)]
enum Instruction {
    ADV(Operand), // OPCODE 0
    BXL(Operand), // OPCODE 1
    BST(Operand), // OPCODE 2
    JNZ(Operand), // OPCODE 3
    BXC(()),      // OPCODE 4
    OUT(Operand), // OPCODE 5
    BDV(Operand), // OPCODE 6
    CDV(Operand), // OPCODE 7
}

struct Computer {
    register_a: Register,
    register_b: Register,
    register_c: Register,
    instruction_pointer: usize,
    output: Vec<u64>,
}

impl Computer {
    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            ADV(operand) => self.register_a /= 2_u64.pow(operand.get_value(&self) as u32),
            BXL(operand) => {
                self.register_b ^= operand.get_value(&self);
            }
            BST(operand) => {
                self.register_b = operand.get_value(&self) % 8;
            }
            JNZ(operand) => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand.get_value(&self) as usize / 2;
                }
            }
            BXC(()) => {
                self.register_b ^= self.register_c;
            }
            OUT(operand) => {
                self.output.push(operand.get_value(&self) % 8);
            }
            BDV(operand) => {
                self.register_b = self.register_a / 2_u64.pow(operand.get_value(&self) as u32)
            }
            CDV(operand) => {
                self.register_c = self.register_a / 2_u64.pow(operand.get_value(&self) as u32)
            }
        }
    }

    fn execute_program(&mut self, program: &Vec<Instruction>) {
        while let Some(instruction) = program.get(self.instruction_pointer) {
            self.execute_instruction(instruction);
            match instruction {
                JNZ(_) => {
                    if self.register_a == 0 {
                        break;
                    } else {
                        continue;
                    }
                }
                _ => self.instruction_pointer += 1,
            }
        }
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|int| format!("{}", int))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

fn parse_program(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| line.contains("Program:"))
        .flat_map(|line| line.split(' '))
        .skip(1)
        .take(1)
        .flat_map(|str| str.split(',').map(|number| number.parse::<u8>().unwrap()))
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(|chunk| {
            let (opcode, operand) = (chunk[0], chunk[1]);
            match opcode {
                0 => ADV(COMBO(operand)),
                1 => BXL(LITERAL(operand)),
                2 => BST(COMBO(operand)),
                3 => JNZ(LITERAL(operand)),
                4 => BXC(()),
                5 => OUT(COMBO(operand)),
                6 => BDV(COMBO(operand)),
                7 => CDV(COMBO(operand)),
                _ => unreachable!("Opcode can only go up to 7"),
            }
        })
        .collect()
}

fn calculate_start_range(exact_locations: &HashMap<usize, Vec<usize>>) -> Option<usize> {
    let j = exact_locations.len();
    let mut range_start = 0;
    for k in 0..j {
        let mut possible_values = exact_locations.get(&k)?.clone();
        possible_values.sort();
        range_start += possible_values[0] * 8_usize.pow((exact_locations.len() - k) as u32);
    }
    Some(range_start)
}

fn backtrack(
    i: &mut usize,
    exact_locations: &mut HashMap<usize, Vec<usize>>,
    computer_output: &mut Vec<Vec<u64>>,
) {
    for key in (0..*i).rev() {
        match exact_locations.get_mut(&key) {
            Some(possible_ops) => match possible_ops.len() {
                0 => unreachable!("This should not be empty"),
                1 => {
                    exact_locations.remove(&key);
                    break;
                }
                _ => {
                    possible_ops.sort();
                    possible_ops.remove(0);
                    computer_output.remove(*i);
                    break;
                }
            },
            None => {
                *i -= 1;
            }
        }
    }
}

fn get_quine(program: &Vec<Instruction>, quine: &Vec<u64>) -> u64 {
    let mut computer_output: Vec<Vec<u64>> = vec![]; // debug information for me
    let mut exact_locations: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut i: usize = 0;
    while quine.len() > i {
        let digit = quine[quine.len() - 1 - i];
        computer_output.push(vec![]);
        let mut pushed = false;
        match calculate_start_range(&exact_locations) {
            Some(range_start) => {
                for register_a in range_start..range_start + 8 {
                    let mut computer = Computer {
                        register_a: register_a as u64,
                        register_b: 0,
                        register_c: 0,
                        instruction_pointer: 0,
                        output: vec![],
                    };
                    computer.execute_program(program);
                    computer_output[i].push(computer.output[0]);
                    let new_output = computer.output[0];
                    if new_output == digit {
                        let exact_location = register_a - range_start;
                        let entry = exact_locations.entry(i).or_insert(vec![exact_location]);
                        if !entry.contains(&exact_location) {
                            entry.push(exact_location)
                        }
                        pushed = true;
                    }
                    if computer.output == vec![2, 4, 1, 1, 7, 5, 1, 4, 0, 3, 4, 5, 5, 5, 3, 0] {
                        return register_a as u64;
                    }
                }
                if !pushed {
                    backtrack(&mut i, &mut exact_locations, &mut computer_output);
                } else {
                    i += 1;
                }
            }
            None => {
                backtrack(&mut i, &mut exact_locations, &mut computer_output);
            }
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer {
        register_a: 65804993,
        register_b: 0,
        register_c: 0,
        instruction_pointer: 0,
        output: vec![],
    };
    let program = parse_program(input);
    computer.execute_program(&program);
    let output = computer.get_output();
    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let program = parse_program(input);
    Some(get_quine(
        &program,
        &vec![2, 4, 1, 1, 7, 5, 1, 4, 0, 3, 4, 5, 5, 5, 3, 0],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let binding = advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(&binding);
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
