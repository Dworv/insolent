use std::iter;

use crate::interpreter::{next_instruction, Instruction};

#[test]
fn next_instruction_increment() {
    assert_eq!(
        next_instruction(&mut iter::once('+')),
        Some((Instruction::Increment, 0, 1))
    )
}

#[test]
fn next_instruction_decrement() {
    assert_eq!(
        next_instruction(&mut iter::once('-')),
        Some((Instruction::Decrement, 0, 1))
    )
}

#[test]
fn next_instruction_moveright() {
    assert_eq!(
        next_instruction(&mut iter::once('>')),
        Some((Instruction::MoveRight, 0, 1))
    )
}

#[test]
fn next_instruction_moveleft() {
    assert_eq!(
        next_instruction(&mut iter::once('<')),
        Some((Instruction::MoveLeft, 0, 1))
    )
}

#[test]
fn next_instruction_openloop() {
    assert_eq!(
        next_instruction(&mut iter::once('[')),
        Some((Instruction::OpenLoop, 0, 1))
    )
}

#[test]
fn next_instruction_closeloop() {
    assert_eq!(
        next_instruction(&mut iter::once(']')),
        Some((Instruction::CloseLoop, 0, 1))
    )
}

#[test]
fn next_instruction_input() {
    assert_eq!(
        next_instruction(&mut iter::once(',')),
        Some((Instruction::Input, 0, 1))
    )
}

#[test]
fn next_instruction_output() {
    assert_eq!(
        next_instruction(&mut iter::once('.')),
        Some((Instruction::Output, 0, 1))
    )
}
