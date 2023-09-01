use rust6502::{instructions::*, *};

mod common;

use common::load_common::*;

#[test]
fn cpu_exhibits_no_behaviour_with_zero_cycles() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cycles_used: i32 = cpu.execute(0, &mut memory);
    assert_eq!(cycles_used, 0);
}

#[test]
fn cpu_can_execute_more_cycles_than_requested_if_required_by_instruction() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    memory.data[0xFFFC] = INSTRUCTION_LDA_IMM;
    memory.data[0xFFFD] = 0x84;
    let cycles_used = cpu.execute(1, &mut memory);
    assert_eq!(cycles_used, 2);
}

//
// LD Immediate Tests
//

// LDA

#[test]
fn lda_immediate_can_load_into_accumulator() {
    load_register_immediate(INSTRUCTION_LDA_IMM, RegisterType::Accumulator);
}

#[test]
fn lda_immediate_can_affect_zero_flag() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.accumulator = 0x44;
    memory.data[0xFFFC] = INSTRUCTION_LDA_IMM;
    memory.data[0xFFFD] = 0x0;
    cpu.execute(2, &mut memory);
    assert!(cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

// LDX

#[test]
fn ldx_immediate_can_load_into_register_x() {
    load_register_immediate(INSTRUCTION_LDX_IMM, RegisterType::RegisterX);
}

// LDY

#[test]
fn ldy_immediate_can_load_into_register_y() {
    load_register_immediate(INSTRUCTION_LDY_IMM, RegisterType::RegisterY);
}

//
// LD Zeropage Tests
//

// LDA

#[test]
fn lda_zeropage_can_load_into_accumulator() {
    load_register_zeropage(INSTRUCTION_LDA_ZERO, RegisterType::Accumulator);
}

#[test]
fn lda_zeropage_with_offset_x_can_load_into_accumulator() {
    load_register_zeropage_x(INSTRUCTION_LDA_ZERO_X, RegisterType::Accumulator);
}

#[test]
fn lda_zeropage_with_offset_x_can_load_into_accumulator_when_it_wraps() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.register_x = 0xFF;
    memory.data[0xFFFC] = INSTRUCTION_LDA_ZERO_X;
    memory.data[0xFFFD] = 0x80;
    memory.data[0x007F] = 0x37;
    let cycles_used = cpu.execute(4, &mut memory);
    assert_eq!(cycles_used, 4);
    assert_eq!(cpu.accumulator, 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

// LDX

#[test]
fn ldx_zeropage_can_load_into_register_x() {
    load_register_zeropage(INSTRUCTION_LDX_ZERO, RegisterType::RegisterX);
}

#[test]
fn ldx_zeropage_with_offset_y_can_load_into_register_x() {
    load_register_zeropage_y(INSTRUCTION_LDX_ZERO_Y, RegisterType::RegisterX);
}

// LDY

#[test]
fn ldy_zeropage_can_load_into_register_y() {
    load_register_zeropage(INSTRUCTION_LDY_ZERO, RegisterType::RegisterY);
}

#[test]
fn ldy_zeropage_with_offset_x_can_load_into_register_y() {
    load_register_zeropage_x(INSTRUCTION_LDY_ZERO_X, RegisterType::RegisterY);
}

//
// LD Absolute Tests
//

// LDA

#[test]
fn lda_absolute_can_load_into_accumulator() {
    load_register_absolute(INSTRUCTION_LDA_ABS, RegisterType::Accumulator);
}

#[test]
fn lda_absolute_with_offset_x_can_load_into_accumulator() {
    load_register_absolute_x(INSTRUCTION_LDA_ABS_X, RegisterType::Accumulator);
}

#[test]
fn lda_absolute_with_offset_y_can_load_into_accumulator() {
    load_register_absolute_y(INSTRUCTION_LDA_ABS_Y, RegisterType::Accumulator);
}

#[test]
fn lda_absolute_with_offset_x_can_load_into_accumulator_when_crosses_page_boundary() {
    load_register_absolute_x_with_page_cross(INSTRUCTION_LDA_ABS_X, RegisterType::Accumulator);
}

#[test]
fn lda_absolute_with_offset_y_can_load_into_accumulator_when_crosses_page_boundary() {
    load_register_absolute_y_with_page_cross(INSTRUCTION_LDA_ABS_Y, RegisterType::Accumulator);
}

// LDX

#[test]
fn ldx_absolute_with_offset_y_can_load_into_register_x() {
    load_register_absolute_y(INSTRUCTION_LDX_ABS_Y, RegisterType::RegisterX);
}

#[test]
fn ldx_absolute_with_offset_y_can_load_into_register_x_when_crosses_page_boundary() {
    load_register_absolute_y_with_page_cross(INSTRUCTION_LDX_ABS_Y, RegisterType::RegisterX);
}

#[test]
fn ldx_absolute_can_load_into_register_x() {
    load_register_absolute(INSTRUCTION_LDX_ABS, RegisterType::RegisterX);
}

// LDY

#[test]
fn ldy_absolute_with_offset_x_can_load_into_register_y() {
    load_register_absolute_x(INSTRUCTION_LDY_ABS_X, RegisterType::RegisterY);
}

#[test]
fn ldy_absolute_can_load_into_register_y() {
    load_register_absolute(INSTRUCTION_LDY_ABS, RegisterType::RegisterY);
}

#[test]
fn ldy_absolute_with_offset_x_can_load_into_register_y_when_crosses_page_boundary() {
    load_register_absolute_x_with_page_cross(INSTRUCTION_LDY_ABS_X, RegisterType::RegisterY);
}

//
// LDA Indirect Tests
//

#[test]
fn lda_indirect_x_can_load_into_accumulator() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.processor_status.set_zero(true);
    cpu.processor_status.set_negative(true);
    cpu.register_x = 0x04;
    memory.data[0xFFFC] = INSTRUCTION_LDA_INDR_X;
    memory.data[0xFFFD] = 0x02;
    memory.data[0x0006] = 0x00;
    memory.data[0x0007] = 0x80;
    memory.data[0x8000] = 0x37;
    let cycles_used = cpu.execute(6, &mut memory);
    assert_eq!(cycles_used, 6);
    assert_eq!(cpu.accumulator, 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

#[test]
fn lda_indirect_y_can_load_into_accumulator() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.processor_status.set_zero(true);
    cpu.processor_status.set_negative(true);
    cpu.register_y = 0x04;
    memory.data[0xFFFC] = INSTRUCTION_LDA_INDR_Y;
    memory.data[0xFFFD] = 0x02;
    memory.data[0x0002] = 0x00;
    memory.data[0x0003] = 0x80;
    memory.data[0x8004] = 0x37;
    let cycles_used = cpu.execute(5, &mut memory);
    assert_eq!(cycles_used, 5);
    assert_eq!(cpu.accumulator, 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

#[test]
fn lda_indirect_y_can_load_into_accumulator_when_crosses_page_boundary() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.register_y = 0x1;
    memory.data[0xFFFC] = INSTRUCTION_LDA_INDR_Y;
    memory.data[0xFFFD] = 0x05;
    memory.data[0x0005] = 0xFF;
    memory.data[0x0006] = 0x80;
    memory.data[0x8100] = 0x37;
    let cycles_used = cpu.execute(6, &mut memory);
    assert_eq!(cycles_used, 6);
    assert_eq!(cpu.accumulator, 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}
