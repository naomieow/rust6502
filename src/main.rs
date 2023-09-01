use rust6502::{instructions::*, *};

fn main() {
    asm::asm! {
        lda #0x84;
        jsr 0x4242;
    };
}
