#![allow(dead_code)]

use alloc::vec;
use alloc::vec::Vec;

use uefi::{prelude::*, CStr16};
use uefi::proto::loaded_image::LoadedImage;

use uefi::proto::media::file::{File, FileAttribute, FileInfo, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::{BootServices, ScopedProtocol};


fn get_dev_sfs(bs: &BootServices, dev_hnd: Handle) -> ScopedProtocol<SimpleFileSystem> {
    return bs
        .open_protocol_exclusive::<SimpleFileSystem>(dev_hnd)
        .unwrap();
}


fn get_image(bs: &BootServices) -> ScopedProtocol<LoadedImage> {
    let image_handle = bs.image_handle();

    return bs
        .open_protocol_exclusive::<LoadedImage>(image_handle)
        .unwrap();
}

pub fn get_file_content(stb: &mut SystemTable<Boot>, path: &CStr16) -> Vec<u8> {
    let dev_hnd: Handle;
    { //get current fs handle
        let image = &mut get_image(stb.boot_services());
        dev_hnd = image.device()
    }
    let mut file: RegularFile;
    {
        let fs = &mut get_dev_sfs(stb.boot_services(), dev_hnd); //open FS
        let root = &mut fs.open_volume().expect("Failed to open volume");

        file = root
            .open(
                path,
                uefi::proto::media::file::FileMode::Read,
                FileAttribute::empty(),
            )
            .expect("Failed to open file")
            .into_regular_file()
            .expect("Not a regular file");
    }
    let mut file_info_buffer = [0u8; 128];
    
    let file_length = file
        .get_info::<FileInfo>(&mut file_info_buffer)
        .expect("Could not get file info")
        .file_size() as usize;


    let mut file_buffer = vec![0; file_length];

    file.read(&mut file_buffer).expect("Couldn't read file");

    file_buffer
}