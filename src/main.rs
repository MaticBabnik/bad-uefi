#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate log;
extern crate uefi;

mod audio;
mod fs;
mod uefiv;
mod va;
mod video;

use audio::{speaker_off, speaker_on};
use core::fmt::Write;
use fs::get_file_content;
use log::info;
use uefi::{prelude::*, proto::console::text::Color};
use uefiv::UEFIVideo;
use video::{clear, open_graphics, video_rect, Vec2};

#[entry]
fn main(hnd: Handle, mut stb: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut stb).expect("Failed to initialize UEFI services");
    {
        _ = stb.stdout().reset(false);
        _ = stb.stdout().set_color(Color::LightCyan, Color::Black);
        _ = stb.stdout().write_str("bad-uefi\n");
        _ = stb.stdout().set_color(Color::White, Color::Black);
        _ = stb.stdout().write_str("Matic Babnik 2022\n\n");

        _ = stb.stdout().set_color(Color::LightGray, Color::Black);

        speaker_on(440);
        stb.boot_services().stall(100_000);
        speaker_off();
        stb.boot_services().stall(100_000);
    }

    info!("Loading audio...");
    let audio_events = va::read_audio_event_list(&mut stb, cstr16!("audio.uefia"));
    info!("Done");
    info!("Loading video...");

    let f = get_file_content(&mut stb, cstr16!("video.uefiv"));
    let video = UEFIVideo::from(&f).expect("Fuck");

    info!(
        "Done, w={} h={} mode={:x} n={} delay={}",
        video.width, video.height, video.color_mode as u8, video.frame_count, video.frame_duration
    );

    let video_events = va::make_video_event_list(&video);
    let events = va::merge_events(&audio_events, &video_events);

    info!(
        "Loaded and merged {} audio and {} video events",
        audio_events.len(),
        video_events.len()
    );

    stb.boot_services().stall(1_000_000);

    let mut g = open_graphics(stb.boot_services());
    clear(&mut g, 0, 0, 0);

    let v_rect = video_rect(
        &mut g,
        Vec2 {
            x: video.width as usize,
            y: video.height as usize,
        },
    );

    let mut time = 0;

    for e in &events {
        info!(
            "time = {}, next ev = {}, sleep={}",
            time,
            e.time,
            e.time - time
        );
        stb.boot_services().stall(e.time - time);
        time = e.time;

        match e.kind {
            va::EventKind::Sound => {
                if e.param == 0 {
                    audio::speaker_off()
                } else {
                    audio::speaker_on(e.param as u16);
                }
            }
            va::EventKind::Frame => {
                video
                    .render_frame(e.param, &mut g, &v_rect)
                    .expect("Failed to render frame");
            }
        }
    }

    speaker_off();

    Status::SUCCESS
}
