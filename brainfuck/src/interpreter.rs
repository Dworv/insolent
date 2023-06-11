use std::io::{Read, Write, ErrorKind::UnexpectedEof};

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
                if let Some(inst) = read_instruction(c, &mut line, &mut col) {
                    instructions.push(inst);
                } else {
                    continue;
                }
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
                (*output).write(&[cells[pointer] as u8]).unwrap();
            }
            Instruction::Input => {
                let mut buf = [0u8; 1];
                match (*input).read_exact(&mut buf) {
                    Ok(_) => {}
                    Err(err) => {
                        if err.kind() == UnexpectedEof {
                            buf = [0];
                        } else {
                            return Err(LanguageError {
                                kind: LanguageErrorKind::Runtime,
                                message: "Failed to read input".to_string(),
                                line,
                                column: col,
                            });
                        }
                    }
                };
                cells[pointer] = buf[0] as Cell;
            }
            Instruction::OpenLoop => {
                if cells[pointer] == 0 {
                    let mut loop_depth = 1;
                    while loop_depth > 0 {
                        if let Some(c) = chars.next() {
                            if let Some(inst) = read_instruction(c, &mut line, &mut col) {
                                match inst {
                                    Instruction::OpenLoop => loop_depth += 1,
                                    Instruction::CloseLoop => loop_depth -= 1,
                                    _ => {}
                                }
                            } else {
                                continue;
                            }
                        } else {
                            return Err(LanguageError {
                                kind: LanguageErrorKind::Syntax,
                                message: "Unmatched loop".to_string(),
                                line,
                                column: col,
                            });
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

fn read_instruction(c: char, line: &mut usize, col: &mut usize) -> Option<Instruction> {
    *col += 1;
    match c {
        '+' => Some(Instruction::Increment),
        '-' => Some(Instruction::Decrement),
        '>' => Some(Instruction::MoveRight),
        '<' => Some(Instruction::MoveLeft),
        '.' => Some(Instruction::Output),
        ',' => Some(Instruction::Input),
        '[' => Some(Instruction::OpenLoop),
        ']' => Some(Instruction::CloseLoop),
        _ => {
            if c == '\n' {
                *line += 1;
                *col = 1;
            }
            None
        }
    }
}