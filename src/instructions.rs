use crate::Byte;

// STA
pub const INSTRUCTION_STA_ZERO: Byte = 0x85;
pub const INSTRUCTION_STA_ZERO_X: Byte = 0x95;
pub const INSTRUCTION_STA_ABS: Byte = 0x8D;
pub const INSTRUCTION_STA_ABS_X: Byte = 0x9D;
pub const INSTRUCTION_STA_ABS_Y: Byte = 0x99;
pub const INSTRUCTION_STA_INDR_X: Byte = 0x81;
pub const INSTRUCTION_STA_INDR_Y: Byte = 0x91;

// STX
pub const INSTRUCTION_STX_ZERO: Byte = 0x86;
pub const INSTRUCTION_STX_ZERO_Y: Byte = 0x96;
pub const INSTRUCTION_STX_ABS: Byte = 0x8E;

// STY
pub const INSTRUCTION_STY_ZERO: Byte = 0x84;
pub const INSTRUCTION_STY_ZERO_X: Byte = 0x94;
pub const INSTRUCTION_STY_ABS: Byte = 0x8C;

// LDA
pub const INSTRUCTION_LDA_IMM: Byte = 0xA9;
pub const INSTRUCTION_LDA_ZERO: Byte = 0xA5;
pub const INSTRUCTION_LDA_ZERO_X: Byte = 0xB5;
pub const INSTRUCTION_LDA_ABS: Byte = 0xAD;
pub const INSTRUCTION_LDA_ABS_X: Byte = 0xBD;
pub const INSTRUCTION_LDA_ABS_Y: Byte = 0xB9;
pub const INSTRUCTION_LDA_INDR_X: Byte = 0xA1;
pub const INSTRUCTION_LDA_INDR_Y: Byte = 0xB1;

// LDX
pub const INSTRUCTION_LDX_IMM: Byte = 0xA2;
pub const INSTRUCTION_LDX_ZERO: Byte = 0xA6;
pub const INSTRUCTION_LDX_ZERO_Y: Byte = 0xB6;
pub const INSTRUCTION_LDX_ABS: Byte = 0xAE;
pub const INSTRUCTION_LDX_ABS_Y: Byte = 0xBE;

// LDY
pub const INSTRUCTION_LDY_IMM: Byte = 0xA0;
pub const INSTRUCTION_LDY_ZERO: Byte = 0xA4;
pub const INSTRUCTION_LDY_ZERO_X: Byte = 0xB4;
pub const INSTRUCTION_LDY_ABS: Byte = 0xAC;
pub const INSTRUCTION_LDY_ABS_X: Byte = 0xBC;

pub const INSTRUCTION_JSR: Byte = 0x20;
pub const INSTRUCTION_RTS: Byte = 0x60;
