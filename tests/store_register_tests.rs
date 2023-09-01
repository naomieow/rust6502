use rust6502::{instructions::*, *};

mod common;

use common::store_common::*;

#[test]
fn sta_zeropage_can_store_accumulator_into_memory() {
    store_register_zeropage(INSTRUCTION_STA_ZERO, RegisterType::Accumulator);
}

#[test]
fn sta_zeropage_with_offset_x_can_store_accumulator_into_memory() {
    store_register_zeropage_x(INSTRUCTION_STA_ZERO_X, RegisterType::Accumulator);
}

#[test]
fn sta_absolute_can_store_accumulator_into_memory() {
    store_register_absolute(INSTRUCTION_STA_ABS, RegisterType::Accumulator);
}

#[test]
fn sta_absolute_with_offset_x_can_store_accumulator_into_memory() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    cpu.accumulator = 0x42;
    cpu.register_x = 0x0F;
    memory.data[0xFFFC] = INSTRUCTION_STA_ABS_X;
    memory.data[0xFFFD] = 0x00;
    memory.data[0xFFFE] = 0x80;
    let cycles_used = cpu.execute(5, &mut memory);
    assert_eq!(cycles_used, 5);
    assert_eq!(memory.data[0x800F], 0x42);
    common::store_common::verify_unmodified_pstatus_st(&cpu, &cpu_copy);
}

#[test]
fn sta_absolute_with_offset_y_can_store_accumulator_into_memory() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    cpu.accumulator = 0x42;
    cpu.register_y = 0x0F;
    memory.data[0xFFFC] = INSTRUCTION_STA_ABS_Y;
    memory.data[0xFFFD] = 0x00;
    memory.data[0xFFFE] = 0x80;
    let cycles_used = cpu.execute(5, &mut memory);
    assert_eq!(cycles_used, 5);
    assert_eq!(memory.data[0x800F], 0x42);
    common::store_common::verify_unmodified_pstatus_st(&cpu, &cpu_copy);
}

#[test]
fn sta_indirect_x_can_store_accumulator_into_memory() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    cpu.accumulator = 0x42;
    cpu.register_x = 0x0F;
    memory.data[0xFFFC] = INSTRUCTION_STA_INDR_X;
    memory.data[0xFFFD] = 0x20;
    memory.data[0xFFFE] = 0x00;
    memory.data[0x002F] = 0x00;
    memory.data[0x0030] = 0x80;
    memory.data[0x8000] = 0x00;
    let cycles_used = cpu.execute(6, &mut memory);
    assert_eq!(cycles_used, 6);
    assert_eq!(memory.data[0x8000], 0x42);
    common::store_common::verify_unmodified_pstatus_st(&cpu, &cpu_copy);
}

#[test]
fn sta_indirect_y_can_store_accumulator_into_memory() {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    cpu.accumulator = 0x42;
    cpu.register_y = 0x0F;
    memory.data[0xFFFC] = INSTRUCTION_STA_INDR_Y;
    memory.data[0xFFFD] = 0x20;
    memory.data[0xFFFE] = 0x00;
    memory.data[0x0020] = 0x00;
    memory.data[0x0021] = 0x80;
    memory.data[0x8000 + 0x0F] = 0x00;
    let cycles_used = cpu.execute(6, &mut memory);
    assert_eq!(cycles_used, 6);
    assert_eq!(memory.data[0x8000 + 0x0F], 0x42);
    common::store_common::verify_unmodified_pstatus_st(&cpu, &cpu_copy);
}

#[test]
fn stx_zeropage_can_store_register_x_into_memory() {
    store_register_zeropage(INSTRUCTION_STX_ZERO, RegisterType::RegisterX);
}

#[test]
fn stx_zeropage_with_offset_y_can_store_register_x_into_memory() {
    store_register_zeropage_y(INSTRUCTION_STX_ZERO_Y, RegisterType::RegisterX);
}

#[test]
fn stx_absolute_can_store_register_x_into_memory() {
    store_register_absolute(INSTRUCTION_STX_ABS, RegisterType::RegisterX);
}

#[test]
fn sty_zeropage_can_store_register_y_into_memory() {
    store_register_zeropage(INSTRUCTION_STY_ZERO, RegisterType::RegisterY);
}

#[test]
fn sty_zeropage_with_offset_x_can_store_register_y_into_memory() {
    store_register_zeropage_x(INSTRUCTION_STY_ZERO_X, RegisterType::RegisterY);
}

#[test]
fn sty_absolute_can_store_register_y_into_memory() {
    store_register_absolute(INSTRUCTION_STY_ABS, RegisterType::RegisterY);
}
