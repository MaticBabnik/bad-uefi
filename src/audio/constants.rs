#![allow(dead_code)]
pub const PIT_FREQ: u32 = 1_193_181;

pub mod pit {
    pub const COUNTER_0: u16 = 0x40;
    pub const COUNTER_1: u16 = 0x41;
    pub const COUNTER_2: u16 = 0x42;

    pub const CONTROL: u16 = 0x43;

    pub const SPEAKER_PORT: u16 = 0x61;
}

pub mod speaker_port {
    pub const TIMER_2: u8 = 0x01;
    pub const DATA: u8 = 0x02;
    pub const TIMER_2_LATCH: u8 = 0x20;
}

/**
 * PIT control byte
 *  _______________________________________________
 * |  7      6 | 5      4 | 3      2      1 | 0   |
 * |  Channel  |  Access  |       Mode      | BCD |
 * ------------------------------------------------
 */
pub mod control {
    pub const CHANNEL_0: u8 = 0x00;
    pub const CHANNEL_1: u8 = 0x40;
    pub const CHANNEL_2: u8 = 0x80;
    pub const CHANNEL_MASK: u8 = 0xC0;

    pub const ACCESS_LATCH: u8 = 0x00;
    pub const ACCESS_LOW: u8 = 0x10;
    pub const ACCESS_HIGH: u8 = 0x20;
    pub const ACCESS_BOTH: u8 = 0x30;
    pub const ACCESS_MASK: u8 = 0x30;

    pub const MODE_HARDSTROBE: u8 = 0xA;
    pub const MODE_SOFTSTROBE: u8 = 0x8;
    pub const MODE_SQUAREWAVE: u8 = 0x6;
    pub const MODE_RATE_GEN: u8 = 0x4;
    pub const MODE_ONESHOT: u8 = 0x2;
    pub const MODE_INTR_ON_TERM: u8 = 0x0;
    pub const MODE_MASK: u8 = 0xE;

    pub const BCD_BINARY: u8 = 0x0;
    pub const BCD_BCD: u8 = 0x1;
    pub const BCD_MASK: u8 = 0x1;
}
