#[macro_export]
macro_rules! asm_helper {
    (
        $cpu:ident, $memory:ident, $cycles:ident,
        jsr $addr:expr; $($rest:tt)*
    ) => {
        {
            println!("PC: {:#06x}", $cpu.program_counter);
            $memory.data[$cpu.program_counter as usize] = INSTRUCTION_JSR;
            $memory.data[$cpu.program_counter.wrapping_add(1) as usize] = ($addr >> 8) as u8;
            $memory.data[$cpu.program_counter.wrapping_add(2) as usize] = ($addr & 0xFF) as u8;
            $cycles += 6;
            println!("Cycles: {}", $cycles);
        }
        asm::asm_helper!($cpu, $memory, $cycles, $($rest)*);
    };

    (
        $cpu:ident, $memory:ident, $cycles:ident,
        lda #$imm:expr; $($rest:tt)*
    ) => {
        {
            println!("PC: {:#06x}", $cpu.program_counter);
            $memory.data[$cpu.program_counter as usize] = INSTRUCTION_LDA_IMM;
            $memory.data[$cpu.program_counter.wrapping_add(1) as usize] = $imm;
            $cycles += 2;
            println!("Cycles: {}", $cycles);
        }
        asm::asm_helper!($cpu, $memory, $cycles, $($rest)*);
    };

    (
        $cpu:ident, $memory:ident, $cycles:ident,
        lda $addr:expr; $($rest:tt)*
    ) => {
        {
            println!("PC: {:#06x}", $cpu.program_counter);
            if $addr <= 0xFF {
                println!("Zero");
                $memory.data[$cpu.program_counter as usize] = INSTRUCTION_LDA_ZERO;
                $memory.data[$cpu.program_counter.wrapping_add(1) as usize] = $addr;
                $cycles += 3;
            } else {
                println!("Abs");
                $memory.data[$cpu.program_counter as usize] = INSTRUCTION_LDA_ABS;
                $memory.data[$cpu.program_counter.wrapping_add(1) as usize] = ($addr >> 8) as u8;
                $memory.data[$cpu.program_counter.wrapping_add(2) as usize] = ($addr & 0xFF) as u8;
                $cycles += 4;
            }
            println!("Cycles: {}", $cycles);

        }
        asm::asm_helper!($cpu, $memory, $cycles, $($rest)*);
    };

    (
        $cpu:ident, $memory:ident, $cycles:ident,
    ) => {
        println!("Total Cycles: {}", $cycles);
        $cpu.execute($cycles, &mut $memory)
    };
}

#[macro_export]
macro_rules! asm {
    ($($tokens:tt)*) => {
        let mut cpu: CPU = CPU::reset();
        let mut memory: Memory = Memory::reset();
        let mut cycles = 0;
        asm::asm_helper!(cpu, memory, cycles, $($tokens)*);
        println!("Acc: {:#010x}", cpu.accumulator);
    };
}
