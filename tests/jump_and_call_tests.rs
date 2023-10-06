use rust6502::{instructions::*, *};

mod common;

#[test]
fn jsr_can_jump_to_subroutine_and_return() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    cpu.program_counter = 0xFF00;
    memory.data[0xFF00] = INSTRUCTION_JSR;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0xFF03] = INSTRUCTION_LDA_IMM;
    memory.data[0xFF04] = 0x42;
    memory.data[0x8000] = INSTRUCTION_RTS;
    let cycles_used = cpu.execute(14, &mut memory);
    assert_eq!(cycles_used, 14);
    assert_eq!(cpu.accumulator, 0x42);
}

#[test]
fn jsr_does_not_affect_processor_status() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    cpu.program_counter = 0xFF00;
    memory.data[0xFF00] = INSTRUCTION_JSR;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    let cycles_used = cpu.execute(6, &mut memory);
    assert_eq!(cycles_used, 6);
    assert_eq!(cpu.processor_status, cpu_copy.processor_status);
    assert_eq!(cpu.stack_pointer, cpu_copy.stack_pointer);
    assert_eq!(cpu.program_counter, 0x8000);
}

#[test]
fn rts_does_not_affect_processor_status() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    cpu.program_counter = 0xFF00;
    memory.data[0xFF00] = INSTRUCTION_JSR;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0x8000] = INSTRUCTION_RTS;
    let cycles_used = cpu.execute(12, &mut memory);
    assert_eq!(cycles_used, 12);
    assert_eq!(cpu.processor_status, cpu_copy.processor_status);
    assert_eq!(cpu.program_counter, 0x8000);
}

#[test]
fn jmp_absolute_can_jump_to_location() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    cpu.program_counter = 0xFF00;
    memory.data[0xFF00] = INSTRUCTION_JMP_ABS;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    let cycles_used = cpu.execute(3, &mut memory);
    assert_eq!(cycles_used, 3);
    assert_eq!(cpu.processor_status, cpu_copy.processor_status);
    assert_eq!(cpu.stack_pointer, cpu_copy.stack_pointer);
    assert_eq!(cpu.program_counter, 0x8000);
}

#[test]
fn jmp_indirect_can_jump_to_location() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    cpu.program_counter = 0xFF00;
    memory.data[0xFF00] = INSTRUCTION_JMP_INDR;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0x8000] = 0x00;
    memory.data[0x8001] = 0x90;
    let cycles_used = cpu.execute(5, &mut memory);
    assert_eq!(cycles_used, 5);
    assert_eq!(cpu.processor_status, cpu_copy.processor_status);
    assert_eq!(cpu.stack_pointer, cpu_copy.stack_pointer);
    assert_eq!(cpu.program_counter, 0x9000);
}
