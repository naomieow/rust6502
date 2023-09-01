#[allow(dead_code)]
use crate::*;

pub fn load_register_immediate(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x84;
    let cycles_used = cpu.execute(2, &mut memory);
    assert_eq!(cycles_used, 2);
    assert_eq!(*cpu.get_register(&register_type), 0x84);
    assert!(!cpu.processor_status.zero());
    assert!(cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

pub fn load_register_zeropage(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x42;
    memory.data[0x0042] = 0x37;
    let cycles_used = cpu.execute(3, &mut memory);
    assert_eq!(cycles_used, 3);
    assert_eq!(*cpu.get_register(&register_type), 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

pub fn load_register_zeropage_x(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.register_x = 5;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x42;
    memory.data[0x0047] = 0x37;
    let cycles_used = cpu.execute(4, &mut memory);
    assert_eq!(cycles_used, 4);
    assert_eq!(*cpu.get_register(&register_type), 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

pub fn load_register_zeropage_y(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.register_y = 5;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x42;
    memory.data[0x0047] = 0x37;
    let cycles_used = cpu.execute(4, &mut memory);
    assert_eq!(cycles_used, 4);
    assert_eq!(*cpu.get_register(&register_type), 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

pub fn load_register_absolute(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.processor_status.set_zero(true);
    cpu.processor_status.set_negative(true);
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0xFFFE] = 0x44;
    memory.data[0x4480] = 0x37;
    let cycles_used = cpu.execute(4, &mut memory);
    assert_eq!(cycles_used, 4);
    assert_eq!(*cpu.get_register(&register_type), 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

pub fn load_register_absolute_x(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.processor_status.set_zero(true);
    cpu.processor_status.set_negative(true);
    cpu.register_x = 1;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0xFFFE] = 0x44;
    memory.data[0x4481] = 0x37;
    let cycles_used = cpu.execute(4, &mut memory);
    assert_eq!(cycles_used, 4);
    assert_eq!(*cpu.get_register(&register_type), 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

pub fn load_register_absolute_y(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.processor_status.set_zero(true);
    cpu.processor_status.set_negative(true);
    cpu.register_y = 1;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0xFFFE] = 0x44;
    memory.data[0x4481] = 0x37;
    let cycles_used = cpu.execute(4, &mut memory);
    assert_eq!(cycles_used, 4);
    assert_eq!(*cpu.get_register(&register_type), 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

pub fn load_register_absolute_x_with_page_cross(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.register_x = 0x1;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0xFF;
    memory.data[0xFFFE] = 0x44;
    memory.data[0x4500] = 0x37;
    let cycles_used = cpu.execute(5, &mut memory);
    assert_eq!(cycles_used, 5);
    assert_eq!(*cpu.get_register(&register_type), 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

pub fn load_register_absolute_y_with_page_cross(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy: CPU = cpu.clone();
    cpu.register_y = 0x1;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0xFF;
    memory.data[0xFFFE] = 0x44;
    memory.data[0x4500] = 0x37;
    let cycles_used = cpu.execute(5, &mut memory);
    assert_eq!(cycles_used, 5);
    assert_eq!(*cpu.get_register(&register_type), 0x37);
    assert!(!cpu.processor_status.zero());
    assert!(!cpu.processor_status.negative());
    verify_unmodified_pstatus_ld(&cpu, &cpu_copy);
}

pub fn verify_unmodified_pstatus_ld(cpu: &CPU, cpu_copy: &CPU) {
    assert_eq!(
        cpu.processor_status.carry(),
        cpu_copy.processor_status.carry()
    );
    assert_eq!(
        cpu.processor_status.r#break(),
        cpu_copy.processor_status.r#break()
    );
    assert_eq!(
        cpu.processor_status.decimal(),
        cpu_copy.processor_status.decimal()
    );
    assert_eq!(
        cpu.processor_status.interrupt(),
        cpu_copy.processor_status.interrupt()
    );
    assert_eq!(
        cpu.processor_status.overflow(),
        cpu_copy.processor_status.overflow()
    );
}
