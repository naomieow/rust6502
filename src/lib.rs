use bitfield::bitfield;

pub mod instructions;

pub type Byte = u8;
pub type Word = u16;

const MAX_MEM: u32 = 1024 * 64;

impl CPU {
    pub fn execute(&mut self, mut cycles: i32, memory: &mut Memory) -> i32 {
        let cycles_requested: i32 = cycles.clone();
        while cycles > 0 {
            let instruction: Byte = self.fetch_byte(&mut cycles, &memory);
            match instruction {
                //
                // Load Registers
                //

                // LDA
                instructions::INSTRUCTION_LDA_IMM => {
                    self.accumulator = self.fetch_byte(&mut cycles, &memory);
                    self.load_register_set_status(&RegisterType::Accumulator);
                }
                instructions::INSTRUCTION_LDA_ZERO => {
                    let address: Word = self.get_zero_page_addr(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::Accumulator, &memory);
                }
                instructions::INSTRUCTION_LDA_ZERO_X => {
                    let address: Word = self.get_zero_page_addr_x(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::Accumulator, &memory);
                }
                instructions::INSTRUCTION_LDA_ABS => {
                    let address: Word = self.get_absolute_addr(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::Accumulator, &memory);
                }
                instructions::INSTRUCTION_LDA_ABS_X => {
                    let address: Word = self.get_absolute_addr_x(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::Accumulator, &memory);
                }
                instructions::INSTRUCTION_LDA_ABS_Y => {
                    let address: Word = self.get_absolute_addr_y(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::Accumulator, &memory);
                }
                instructions::INSTRUCTION_LDA_INDR_X => {
                    let mut zero_page_addr: Byte = self.fetch_byte(&mut cycles, &memory);
                    zero_page_addr += self.register_x;
                    cycles -= 1;
                    let effective_address: Word =
                        self.read_word(&mut cycles, zero_page_addr as Word, &memory);
                    self.load_register(
                        &mut cycles,
                        effective_address,
                        &RegisterType::Accumulator,
                        &memory,
                    );
                }
                instructions::INSTRUCTION_LDA_INDR_Y => {
                    let zero_page_addr: Byte = self.fetch_byte(&mut cycles, &memory);
                    let effective_address: Word =
                        self.read_word(&mut cycles, zero_page_addr as Word, &memory);
                    let effective_address_plus_y = effective_address + self.register_y as Word;
                    if (effective_address_plus_y & 0xFF00) != (effective_address & 0xFF00) {
                        cycles -= 1;
                    }
                    self.load_register(
                        &mut cycles,
                        effective_address_plus_y,
                        &RegisterType::Accumulator,
                        &memory,
                    );
                }

                // LDX
                instructions::INSTRUCTION_LDX_IMM => {
                    self.register_x = self.fetch_byte(&mut cycles, &memory);
                    self.load_register_set_status(&RegisterType::RegisterX);
                }
                instructions::INSTRUCTION_LDX_ZERO => {
                    let address: Word = self.get_zero_page_addr(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::RegisterX, &memory);
                }
                instructions::INSTRUCTION_LDX_ZERO_Y => {
                    let address: Word = self.get_zero_page_addr_y(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::RegisterX, &memory);
                }
                instructions::INSTRUCTION_LDX_ABS => {
                    let address: Word = self.get_absolute_addr(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::RegisterX, &memory);
                }
                instructions::INSTRUCTION_LDX_ABS_Y => {
                    let address: Word = self.get_absolute_addr_y(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::RegisterX, &memory);
                }

                // LDY
                instructions::INSTRUCTION_LDY_IMM => {
                    self.register_y = self.fetch_byte(&mut cycles, &memory);
                    self.load_register_set_status(&RegisterType::RegisterY);
                }
                instructions::INSTRUCTION_LDY_ZERO => {
                    let address: Word = self.get_zero_page_addr(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::RegisterY, &memory);
                }
                instructions::INSTRUCTION_LDY_ZERO_X => {
                    let address: Word = self.get_zero_page_addr_x(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::RegisterY, &memory);
                }
                instructions::INSTRUCTION_LDY_ABS => {
                    let address: Word = self.get_absolute_addr(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::RegisterY, &memory);
                }
                instructions::INSTRUCTION_LDY_ABS_X => {
                    let address: Word = self.get_absolute_addr_x(&mut cycles, &memory);
                    self.load_register(&mut cycles, address, &RegisterType::RegisterY, &memory);
                }

                //
                instructions::INSTRUCTION_JSR => {
                    let subroutine_addr: Word = self.fetch_word(&mut cycles, &memory);
                    memory.write_word(
                        &mut cycles,
                        subroutine_addr,
                        self.program_counter.wrapping_sub(1) as u32,
                    );
                    self.program_counter = subroutine_addr;
                    cycles -= 1;
                }
                _ => {
                    panic!(
                        "Unknown instruction: {:#010x} at address {:#010x}",
                        instruction,
                        self.program_counter.wrapping_sub(1)
                    );
                }
            }
        }
        cycles_requested - cycles
    }
    pub fn reset() -> Self {
        Self {
            program_counter: 0xFFFC,
            stack_pointer: 0x0100,
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            processor_status: ProcessorStatus(0x00000000),
        }
    }

    fn load_register(
        &mut self,
        cycles: &mut i32,
        address: Word,
        register_type: &RegisterType,
        memory: &Memory,
    ) {
        let data: Byte = self.read_byte(cycles, address, &memory);
        *self.get_register(register_type) = data;
        self.load_register_set_status(register_type);
    }

    pub fn get_register(&mut self, register_type: &RegisterType) -> &mut Byte {
        match register_type {
            RegisterType::Accumulator => &mut self.accumulator,
            RegisterType::RegisterX => &mut self.register_x,
            RegisterType::RegisterY => &mut self.register_y,
        }
    }

    fn read_byte(&mut self, cycles: &mut i32, address: Word, memory: &Memory) -> Byte {
        let data: Byte = memory.data[address as usize];
        *cycles -= 1;
        data
    }

    fn read_word(&mut self, cycles: &mut i32, address: Word, memory: &Memory) -> Word {
        let low: Byte = self.read_byte(cycles, address, &memory);
        let high: Byte = self.read_byte(cycles, address + 1, &memory);
        low as Word | ((high as Word) << 8)
    }

    fn fetch_byte(&mut self, cycles: &mut i32, memory: &Memory) -> Byte {
        let data: Byte = memory.data[self.program_counter as usize];
        self.program_counter = self.program_counter.wrapping_add(1);
        *cycles -= 1;
        data
    }

    fn fetch_word(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let mut data: Word = memory.data[self.program_counter as usize] as u16;
        self.program_counter = self.program_counter.wrapping_add(1);
        data |= (memory.data[self.program_counter as usize] as Word) << 8;
        self.program_counter = self.program_counter.wrapping_add(1);
        *cycles -= 2;
        data
    }

    fn load_register_set_status(&mut self, register_type: &RegisterType) {
        // println!("Acc: {:#010x}", self.accumulator);
        // println!("ReX: {:#010x}", self.register_x);
        // println!("ReY: {:#010x}", self.register_y);
        let register = *self.get_register(register_type);
        self.processor_status.set_zero(register == 0x00000000);
        self.processor_status
            .set_negative(register & 0b10000000 > 0);
    }

    fn get_zero_page_addr(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        self.fetch_byte(cycles, &memory) as Word
    }

    fn get_zero_page_addr_x(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let mut zero_page_address: Byte = self.fetch_byte(cycles, &memory);
        zero_page_address = zero_page_address.wrapping_add(self.register_x);
        *cycles -= 1;
        zero_page_address as Word
    }

    fn get_zero_page_addr_y(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let mut zero_page_address: Byte = self.fetch_byte(cycles, &memory);
        zero_page_address = zero_page_address.wrapping_add(self.register_y);
        *cycles -= 1;
        zero_page_address as Word
    }

    fn get_absolute_addr(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        self.fetch_word(cycles, &memory)
    }

    fn get_absolute_addr_x(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let absolute_addr: Word = self.fetch_word(cycles, &memory);
        let absolute_addr_plus_x = absolute_addr + self.register_x as Word;
        if (absolute_addr_plus_x & 0xFF00) != (absolute_addr & 0xFF00) {
            *cycles -= 1;
        }
        absolute_addr_plus_x
    }

    fn get_absolute_addr_y(&mut self, cycles: &mut i32, memory: &Memory) -> Word {
        let absolute_addr: Word = self.fetch_word(cycles, &memory);
        let absolute_addr_plus_y = absolute_addr + self.register_y as Word;
        if (absolute_addr_plus_y & 0xFF00) != (absolute_addr & 0xFF00) {
            *cycles -= 1;
        }
        absolute_addr_plus_y
    }
}

#[derive(Clone, Copy)]
pub struct CPU {
    pub processor_status: ProcessorStatus,
    pub program_counter: Word,
    pub stack_pointer: Word,
    pub accumulator: Byte,
    pub register_x: Byte,
    pub register_y: Byte,
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct ProcessorStatus(Byte);
    Byte;
    pub carry, _: 1;
    pub zero, set_zero: 2;
    pub interrupt, _: 3;
    pub decimal, _: 4;
    pub r#break, _: 5;
    pub overflow, _: 6;
    pub negative, set_negative: 7;
}

pub struct Memory {
    pub data: [Byte; MAX_MEM as usize],
}

impl Memory {
    pub fn reset() -> Self {
        Self {
            data: [0x00000000; MAX_MEM as usize],
        }
    }

    pub fn write_word(&mut self, cycles: &mut i32, word: Word, address: u32) {
        self.data[address as usize] = (word & 0xFF) as Byte;
        self.data[(address + 1) as usize] = (word >> 8) as Byte;
        *cycles -= 2;
    }
}

pub enum RegisterType {
    Accumulator,
    RegisterX,
    RegisterY,
}
