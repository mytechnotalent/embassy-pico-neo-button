#!/bin/bash

probe-rs gdb \
    target/thumbv6m-none-eabi/debug/rust_embassy_pico_project \
    --chip RP2040
