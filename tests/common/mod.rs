use rust6502::{Memory, CPU};

pub mod load_common;
pub mod store_common;

pub fn setup() -> (CPU, Memory) {
    let cpu: CPU = CPU::reset();
    let memory: Memory = Memory::reset();
    (cpu, memory)
}
