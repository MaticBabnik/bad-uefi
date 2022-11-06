# Bad UEFI
Another day, another Bad Apple project.

Video and audio are loaded from `\video.uefiv` and `\audio.uefia` respectively. (when running in QEMU `esp/` is the root directory)

Note: You need to download and convert the video yourself. An example audio track is included (`sample-audio.txt`) but you still need to convert it. 

## How to use the encoder
The encoder is written in NodeJS and can be found in the `encoder` directory.
First install the dependencies with `npm i`. 

To convert a video run:
```
node encoder encode <input file> <output file> -w <out width> -h <out height>
```
A sensible default mode is `g8c` (graphic mode, 8 bit depth, color).
For more info on the modes and framerate see the [UEFIV readme](encoder/uefiv.md).

To convert an audio track run:
```
node encoder audio <input file> <output file> -d <delay> -b <bpm>
```

## Building / running

The UEFI source is written in Rust and can be found in the `src` directory.

### On real hardware

Run `cargo build --release` then copy the .efi file to `/efi/boot/bootx64.efi` on a FAT32 partition and boot it.

### QEMU
You can also run it inside QEMU.

1. Install QEMU, KVM and OVMF

2. Copy `/usr/share/OVMF/x64/OVMF_CODE.fd` and `/usr/share/OVMF/x64/OVMF_VARS.fd` to the `files` directory

3. Run `./run-qemu.sh`

4. ~~Profit???~~

## TODO
- [ ] Implement `t4` color mode in the encoder
- [ ] Implement `t4` and `g24` color modes in the "bootloader"
- [ ] Document the audio format
