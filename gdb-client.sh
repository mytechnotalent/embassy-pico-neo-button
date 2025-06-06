#!/bin/bash

arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/rust_embassy_pico_project

# target remote :1337
# break rust_embassy_pico_project::__cortex_m_rt_main