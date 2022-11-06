#![allow(dead_code)]
mod asm {
    use core::arch::asm;
    /**
     * Reads a byte from port.
     */
    #[inline(always)]
    pub unsafe fn inb(port: u16) -> u8 {
        let mut v: u8;
        asm!("in al,dx", out("al") v, in("dx") port);
        v
    }
    /**
     * Writes a byte to a port.
     */
    #[inline(always)]
    pub unsafe fn outb(val: u8, port: u16) {
        asm!("out dx,al",in("dx") port, in("al") val);
    }
}

mod constants;
use self::constants::*;

pub fn speaker_off() {
    unsafe {
        let status = asm::inb(pit::SPEAKER_PORT);
        let status_mask = !(speaker_port::TIMER_2 | speaker_port::DATA);

        asm::outb(status & status_mask, pit::SPEAKER_PORT);
    }
}

pub fn speaker_on(mut pitch: u16) {
    pitch = match pitch {
        //clamp pitch
        pitch if pitch < 20 => 20,
        pitch if pitch > 20_000 => 20_000,
        _ => pitch,
    };

    let c = PIT_FREQ / pitch as u32;

    unsafe {
        asm::outb(
            control::CHANNEL_2
                | control::ACCESS_BOTH
                | control::MODE_SQUAREWAVE
                | control::BCD_BINARY,
            pit::CONTROL,
        );

        // the access mode both requires us to first write the low byte and then the high byte
        asm::outb((c & 0xFF) as u8, pit::COUNTER_2); //low
        asm::outb((c >> 8) as u8, pit::COUNTER_2); //high

        let status = asm::inb(pit::SPEAKER_PORT);

        asm::outb(
            status | speaker_port::TIMER_2 | speaker_port::DATA,
            pit::SPEAKER_PORT,
        );
    }
}