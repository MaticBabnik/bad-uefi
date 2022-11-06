#!/bin/bash
set -e

cargo build --release
cp target/x86_64-unknown-uefi/release/bad-uefi.efi esp/efi/boot/bootx64.efi

qemu-system-x86_64 -enable-kvm -m 4096 \
    -audiodev pa,id=0 -machine pcspk-audiodev=0 \
    -drive if=pflash,format=raw,readonly=on,file=files/OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=files/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp