use std::io::{Read, Write};

use insolent::{LanguageError, LanguageErrorKind};

type Cell = u32;
type CellAddress = usize;

pub fn interpret<I: Iterator<Item = char>>(
    mut chars: I,
    input: Box<&mut dyn Read>,
    output: Box<&mut dyn Write>,
) -> Result<(), LanguageError> {
    let num_cells = 10_000usize;
    let cell_max = u8::MAX as Cell;

    let mut cells: Vec<Cell> = vec![0];
    let mut pointer: CellAddress = 0;

    let mut instruction_offset = 0;
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut current_instruction = 0;

    let mut loop_stack: Vec<usize> = vec![];

    let mut line = 1;
    let mut col = 1;

    loop {
        if current_instruction == instruction_offset + instructions.len() {
            if let Some(c) = chars.next() {
                let new_inst = match c {
                    '+' => Instruction::Increment,
                    '-' => Instruction::Decrement,
                    '>' => Instruction::MoveRight,
                    '<' => Instruction::MoveLeft,
                    '.' => Instruction::Output,
                    ',' => Instruction::Input,
                    '[' => Instruction::OpenLoop,
                    ']' => Instruction::CloseLoop,
                    _ => {
                        if c == '\n' {
                            line += 1;
                            col = 1;
                        } else {
                            col += 1;
                        }
                        continue;
                    }
                };
                instructions.push(new_inst);
            } else {
                break;
            }
        }
        match &instructions[current_instruction - instruction_offset] {
            Instruction::Increment => {
                cells[pointer] = (cells[pointer] + 1) % cell_max;
            }
            Instruction::Decrement => {
                cells[pointer] = (cells[pointer] - 1) % cell_max;
            }
            Instruction::MoveRight => {
                pointer += 1;
                if pointer >= num_cells {
                    return Err(LanguageError {
                        kind: LanguageErrorKind::Runtime,
                        message: format!("Pointer out of bounds: {}", pointer),
                        line,
                        column: col,
                    });
                }
                if pointer >= cells.len() {
                    cells.push(0);
                }
            }
            Instruction::MoveLeft => {
                if pointer == 0 {
                    return Err(LanguageError {
                        kind: LanguageErrorKind::Runtime,
                        message: format!("Pointer out of bounds: {}", pointer),
                        line,
                        column: col,
                    });
                }
                pointer -= 1;
            }
            Instruction::Output => {
                (*output).write(&[cells[pointer] as u8]).unwrap(); //TODO: encode
            }
            Instruction::Input => {
                // TODO: decode
                let mut buf = [0u8; 1];
                (*input).read_exact(&mut buf).unwrap();
                cells[pointer] = buf[0] as Cell;
            }
            Instruction::OpenLoop => {
                if cells[pointer] == 0 {
                    let mut loop_depth = 1;
                    while loop_depth > 0 {
                        current_instruction += 1;
                        if current_instruction == instruction_offset + instructions.len() {
                            if let Some(c) = chars.next() {
                                let new_inst = match c {
                                    '[' => Instruction::OpenLoop,
                                    ']' => Instruction::CloseLoop,
                                    _ => continue,
                                };
                                instructions.push(new_inst);
                            } else {
                                return Err(LanguageError {
                                    kind: LanguageErrorKind::Syntax,
                                    message: "Unmatched loop".to_string(),
                                    line,
                                    column: col,
                                });
                            }
                        }
                        match &instructions[current_instruction - instruction_offset] {
                            Instruction::OpenLoop => {
                                loop_depth += 1;
                            }
                            Instruction::CloseLoop => {
                                loop_depth -= 1;
                            }
                            _ => {}
                        }
                    }
                } else {
                    loop_stack.push(current_instruction);
                }
            }
            Instruction::CloseLoop => {
                if loop_stack.len() == 0 {
                    return Err(LanguageError {
                        kind: LanguageErrorKind::Syntax,
                        message: "Unmatched loop".to_string(),
                        line,
                        column: col,
                    });
                }
                if cells[pointer] != 0 {
                    current_instruction = loop_stack[loop_stack.len() - 1];
                } else {
                    loop_stack.pop();
                }
            }
        }
        current_instruction += 1;
        if loop_stack.len() == 0 {
            instruction_offset = current_instruction;
            instructions.clear();
            instructions.shrink_to_fit();
        }
    }

    Ok(())
}

enum Instruction {
    // instructions
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Output,
    Input,
    OpenLoop,
    CloseLoop,
}
