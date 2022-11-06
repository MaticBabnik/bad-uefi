use alloc::vec::Vec;
use uefi::{
    table::{Boot, SystemTable},
    CStr16,
};
use uefiv::UEFIVideo;

use crate::fs::get_file_content;

#[derive(Debug, Clone, Copy)]
pub enum EventKind {
    Frame,
    Sound,
}

#[derive(Debug, Clone, Copy)]
pub struct VAEvent {
    pub time: usize,
    pub kind: EventKind,
    pub param: usize,
}

pub fn make_video_event_list(video: &UEFIVideo) -> Vec<VAEvent> {
    let mut events: Vec<VAEvent> = Vec::new();

    for i in 0..video.frame_count {
        events.push(VAEvent {
            time: i * video.frame_duration,
            kind: EventKind::Frame,
            param: i,
        });
    }

    events
}

pub fn read_audio_event_list(stb: &mut SystemTable<Boot>, file: &CStr16) -> Vec<VAEvent> {
    let bytes = get_file_content(stb, file);

    let size = bytes.len() / 4;
    let mut time = 0;

    let mut out: Vec<VAEvent> = Vec::new();

    for i in 0..size {
        let delay = ((bytes[i * 4] as usize) << 8 | (bytes[i * 4 + 1] as usize)) * 1000;
        let param = (bytes[i * 4 + 2] as usize) << 8 | (bytes[i * 4 + 3] as usize);

        out.push(VAEvent {
            kind: EventKind::Sound,
            time,
            param,
        });

        time += delay;
    }

    out
}

pub fn merge_events(a: &Vec<VAEvent>, b: &Vec<VAEvent>) -> Vec<VAEvent> {
    let mut out: Vec<VAEvent> = Vec::new();

    let mut ai = 0;
    let mut bi = 0;

    while ai < a.len() && bi < b.len() {
        if a[ai].time < b[bi].time {
            out.push(a[ai]);
            ai += 1;
        } else {
            out.push(b[bi]);
            bi += 1;
        }
    }
    while ai < a.len() {
        out.push(a[ai]);
        ai += 1;
    }
    while bi < b.len() {
        out.push(b[bi]);
        bi += 1;
    }

    out
}
