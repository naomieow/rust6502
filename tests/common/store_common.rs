use crate::*;

pub fn store_register_zeropage(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    *cpu.get_register(&register_type) = 0x2F;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0x0080] = 0x00;
    let cycles_used = cpu.execute(3, &mut memory);
    assert_eq!(cycles_used, 3);
    assert_eq!(memory.data[0x0080], 0x2F);
    verify_unmodified_pstatus_st(&cpu, &cpu_copy);
}

pub fn store_register_zeropage_x(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    *cpu.get_register(&register_type) = 0x42;
    cpu.register_x = 0x0F;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0x008F] = 0x00;
    let cycles_used = cpu.execute(4, &mut memory);
    assert_eq!(cycles_used, 4);
    assert_eq!(memory.data[0x008F], 0x42);
    verify_unmodified_pstatus_st(&cpu, &cpu_copy);
}

pub fn store_register_zeropage_y(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    *cpu.get_register(&register_type) = 0x42;
    cpu.register_y = 0x0F;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0x008F] = 0x00;
    let cycles_used = cpu.execute(4, &mut memory);
    assert_eq!(cycles_used, 4);
    assert_eq!(memory.data[0x008F], 0x42);
    verify_unmodified_pstatus_st(&cpu, &cpu_copy);
}

pub fn store_register_absolute(opcode: Byte, register_type: RegisterType) {
    let (mut cpu, mut memory): (CPU, Memory) = common::setup();
    let cpu_copy = cpu.clone();
    *cpu.get_register(&register_type) = 0x2F;
    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x00;
    memory.data[0xFFFE] = 0x80;
    memory.data[0x8000] = 0x00;
    let cycles_used = cpu.execute(4, &mut memory);
    assert_eq!(cycles_used, 4);
    assert_eq!(memory.data[0x8000], 0x2F);
    verify_unmodified_pstatus_st(&cpu, &cpu_copy);
}

pub fn verify_unmodified_pstatus_st(cpu: &CPU, cpu_copy: &CPU) {
    assert_eq!(
        cpu.processor_status.zero(),
        cpu_copy.processor_status.zero()
    );
    assert_eq!(
        cpu.processor_status.negative(),
        cpu_copy.processor_status.negative()
    );
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
