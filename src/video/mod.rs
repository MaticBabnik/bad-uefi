#![allow(dead_code)]
use log::info;

use uefi::proto::console::gop::{BltOp, BltPixel, GraphicsOutput};
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::{BootServices, ScopedProtocol};
use uefi::prelude::*;

fn get_image(bs: &BootServices) -> ScopedProtocol<LoadedImage> {
    let image_handle = bs.image_handle();

    return bs
        .open_protocol_exclusive::<LoadedImage>(image_handle)
        .unwrap();
}

fn get_dev_sfs(bs: &BootServices, dev_hnd: Handle) -> ScopedProtocol<SimpleFileSystem> {
    return bs
        .open_protocol_exclusive::<SimpleFileSystem>(dev_hnd)
        .unwrap();
}

pub type Gop<'a> = ScopedProtocol<'a, GraphicsOutput<'a>>;

pub fn open_graphics<'lt>(bs: &'lt BootServices) -> Gop<'lt> {
    return bs
        .open_protocol_exclusive::<GraphicsOutput>(
            bs.get_handle_for_protocol::<GraphicsOutput>().unwrap(),
        )
        .unwrap();
}

pub fn clear_hex(go: &mut Gop, c: u32) {
    let r = (c >> 16 & 0xff) as u8;
    let g = (c >> 8 & 0xff) as u8;
    let b = (c & 0xff) as u8;

    clear(go, r, g, b);
}

pub fn clear(go: &mut Gop, r: u8, g: u8, b: u8) {
    let dims = go.current_mode_info().resolution();

    let bo = BltOp::VideoFill {
        color: BltPixel::new(r, g, b),
        dest: (0, 0),
        dims,
    };

    go.blt(bo).unwrap();
}

pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

pub fn draw_rect(go: &mut Gop, rect: Rect, c: BltPixel) {
    let bo = BltOp::VideoFill {
        color: c,
        dest: (rect.x, rect.y),
        dims: (rect.w, rect.h),
    };

    go.blt(bo).unwrap();
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

pub struct VideoRect {
    pub origin: Vec2,
    pub pixel_size: usize,
}

pub fn video_rect(g: &mut Gop, in_size: Vec2) -> VideoRect {
    let (screen_w, screen_h) = g.current_mode_info().resolution();

    if in_size.x > screen_w || in_size.y > screen_h {
        panic!("Screen too small");
    }

    let x_ratio = screen_w / in_size.x;
    let y_ratio = screen_h / in_size.y;

    let pixel_size = if x_ratio < y_ratio { x_ratio } else { y_ratio };
    info!(
        "x_ratio: {}, y_ratio: {}, ratio: {}",
        x_ratio, y_ratio, pixel_size
    );

    let video_w = in_size.x * pixel_size;
    let video_h = in_size.y * pixel_size;

    let x = (screen_w - video_w) / 2;
    let y = (screen_h - video_h) / 2;

    VideoRect {
        origin: Vec2 { x, y },
        pixel_size
    }
}
