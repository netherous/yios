#!/bin/bash

cargo build

cargo bootimage

qemu-system-x86_64 -drive format=raw,file=target/x86_64-custom/debug/bootimage-yios.bin
