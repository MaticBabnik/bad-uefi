#![allow(dead_code)]
use alloc::vec::Vec;
use uefi::proto::console::gop::BltPixel;

use crate::video::{draw_rect, Gop, Rect, VideoRect};

const HEADER: [u8; 3] = [85, 69, 86]; //UEV

#[derive(Clone, Copy)]
pub enum ColorMode {
    T4 = 0x0,
    G8M = 0x1,
    G8C = 0x2,
    G24 = 0x3,
}

pub fn color_mode_bpp(mode: &ColorMode) -> usize {
    match mode {
        ColorMode::T4 => 1,
        ColorMode::G8M => 1,
        ColorMode::G8C => 1,
        ColorMode::G24 => 3,
    }
}

pub struct UEFIVideo<'lt> {
    pub width: u16,
    pub height: u16,
    pub color_mode: ColorMode,
    pub frame_duration: usize,
    pub frame_size: usize,
    pub frame_count: usize,
    buffer: &'lt Vec<u8>,
    _priv: (), //prevent manual construction
}

#[derive(Debug)]
pub enum UEFIVideoError {
    InvalidHeader,
    InvalidColorMode,
    InvalidBodySize,
    ColorFormatNotImplemented,
    FrameIndexOutOfBounds,
}

impl<'lt> UEFIVideo<'lt> {
    pub fn from(buf: &'lt Vec<u8>) -> Result<UEFIVideo, UEFIVideoError> {
        if buf.len() < 8 {
            return Err(UEFIVideoError::InvalidHeader);
        }

        if buf[0..3] != HEADER {
            return Err(UEFIVideoError::InvalidHeader);
        }

        let width = (buf[3] as u16) << 8 | buf[4] as u16;
        let height = (buf[5] as u16) << 8 | buf[6] as u16;

        let mode = buf[7];

        let color_mode = match mode & 0xF {
            0x0 => ColorMode::T4,
            0x1 => ColorMode::G8M,
            0x2 => ColorMode::G8C,
            0x3 => ColorMode::G24,
            _ => return Err(UEFIVideoError::InvalidHeader),
        };

        let framerate = match (mode >> 4) & 0xF {
            0x0 => 1,
            x => x * 2,
        };

        let frame_duration = 1_000_000 / framerate as usize;

        let frame_size = width as usize * height as usize * color_mode_bpp(&color_mode);

        if (buf.len() - 8) % frame_size != 0 {
            return Err(UEFIVideoError::InvalidBodySize);
        }

        let frame_count = (buf.len() - 8) / frame_size;

        Ok(UEFIVideo {
            width,
            height,
            color_mode,
            frame_duration,
            frame_size,
            frame_count,
            buffer: buf,
            _priv: (),
        })
    }

    pub fn get_frame_slice(&self, i: usize) -> &[u8] {
        if i >= self.frame_count {
            panic!("Some retard tried to render out of bounds");
        }
        let start = self.frame_size * i + 8;
        let end = start + self.frame_size;

        &self.buffer[start..end]
    }

    fn render_g8c(&self, index: usize, go: &mut Gop, r: &VideoRect) -> usize {
        let frame = self.get_frame_slice(index);
        let mut i = 0;

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let px = frame[i];

                draw_rect(
                    go,
                    Rect {
                        x: r.origin.x + x * r.pixel_size,
                        y: r.origin.y + y * r.pixel_size,
                        w: r.pixel_size,
                        h: r.pixel_size,
                    },
                    BltPixel::new(px & 0xE0, (px & 0x1C) << 3, (px & 0x3) << 6),
                );

                i += 1;
            }
        }
        i
    }

    fn render_g8m(&self, index: usize, go: &mut Gop, r: &VideoRect) -> usize {
        let frame = self.get_frame_slice(index);
        let mut i = 0;

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let px = frame[i];

                draw_rect(
                    go,
                    Rect {
                        x: r.origin.x + x * r.pixel_size,
                        y: r.origin.y + y * r.pixel_size,
                        w: r.pixel_size,
                        h: r.pixel_size,
                    },
                    BltPixel::new(px, px, px),
                );

                i += 1;
            }
        }
        i
    }

    pub fn render_frame(
        &self,
        index: usize,
        go: &mut Gop,
        r: &VideoRect,
    ) -> Result<usize, UEFIVideoError> {
        if index >= self.frame_count {
            return Err(UEFIVideoError::FrameIndexOutOfBounds);
        }

        match self.color_mode {
            ColorMode::G8C => Ok(self.render_g8c(index, go, r)),
            ColorMode::G8M => Ok(self.render_g8m(index, go, r)),
            _ => Err(UEFIVideoError::ColorFormatNotImplemented), //TODO: implement other color modes
        }
    }
}
