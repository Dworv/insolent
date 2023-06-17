use std::io::{ErrorKind::UnexpectedEof, Read, Write};

use insolent::{LanguageError, LanguageErrorKind};
type CellAddress = usize;

pub fn interpret<I: Iterator<Item = char>>(
    mut chars: I,
    input: Box<&mut dyn Read>,
    output: Box<&mut dyn Write>,
) -> Result<(), LanguageError> {
    let num_cells = 30_000usize;

    let mut cells: Vec<u8> = vec![0];
    let mut pointer: CellAddress = 0;

    let mut start = 0;
    let mut offset = 0;
    let mut cache: Vec<(Instruction, usize, usize)> = Vec::new();

    let mut loop_stack: Vec<LoopStart> = vec![];

    let mut line = 1;
    let mut col = 1;

    loop {
        if offset == start + cache.len() {
            if let Some(tup) = next_instruction(&mut chars) {
                cache.push(tup);
            } else {
                break;
            }
        }
        let (instruction, line_change, col_change) = &cache[offset - start];
        if line_change > &0 {
            line += line_change;
            col = *col_change;
        } else {
            col += col_change;
        }
        match instruction {
            Instruction::Increment => {
                cells[pointer] = cells[pointer].wrapping_add(1);
                println!("Incremented cell at {} to {}", pointer, cells[pointer]);
            }
            Instruction::Decrement => {
                cells[pointer] = cells[pointer].wrapping_sub(1);
                println!("Decremented cell at {} to {}", pointer, cells[pointer]);
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
                println!("Moved right to cell {}", pointer);
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
                println!("Moved left to cell {}", pointer);
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
                cells[pointer] = buf[0];
            }
            Instruction::OpenLoop => {
                if cells[pointer] == 0 {
                    let mut loop_depth = 1;
                    while loop_depth > 0 {
                        offset += 1;
                        if offset == start + cache.len() {
                            if let Some(tup) = next_instruction(&mut chars) {
                                cache.push(tup);
                            } else {
                                return Err(LanguageError {
                                    kind: LanguageErrorKind::Syntax,
                                    message: "Unmatched loop".to_string(),
                                    line,
                                    column: col,
                                });
                            }
                        }
                        let (instruction, line_change, col_change) = &cache[offset - start];
                        line += line_change;
                        col += col_change;
                        match instruction {
                            Instruction::OpenLoop => {
                                loop_depth += 1;
                            }
                            Instruction::CloseLoop => {
                                loop_depth -= 1;
                            }
                            _ => {}
                        }
                    }
                    println!("Skipped loop")
                } else {
                    loop_stack.push(LoopStart { offset, line, col });
                    println!("Entered loop")
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
                    let LoopStart {
                        offset: start_offset,
                        line: start_line,
                        col: start_col,
                    } = loop_stack[loop_stack.len() - 1];
                    offset = start_offset;
                    line = start_line;
                    col = start_col;
                } else {
                    loop_stack.pop();
                }
                println!("Exited loop")
            }
        }
        offset += 1;
        if loop_stack.len() == 0 {
            start = offset;
            cache.clear();
            cache.shrink_to_fit();
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Instruction {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Output,
    Input,
    OpenLoop,
    CloseLoop,
}

pub(crate) fn next_instruction<I: Iterator<Item = char>>(
    chars: &mut I,
) -> Option<(Instruction, usize, usize)> {
    let mut line = 0usize;
    let mut col = 1usize;
    for c in chars {
        return Some((
            match c {
                '+' => Instruction::Increment,
                '-' => Instruction::Decrement,
                '>' => Instruction::MoveRight,
                '<' => Instruction::MoveLeft,
                '.' => Instruction::Output,
                ',' => Instruction::Input,
                '[' => Instruction::OpenLoop,
                ']' => Instruction::CloseLoop,
                '\n' => {
                    line += 1;
                    col = 1;
                    continue;
                }
                _ => {
                    col += 1;
                    continue;
                }
            },
            line,
            col,
        ));
    }
    return None;
}

struct LoopStart {
    offset: usize,
    line: usize,
    col: usize,
}
