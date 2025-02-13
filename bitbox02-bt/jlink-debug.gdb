# `target extended-remote :2331` is enough if you run on the same machine as
# where you run the jlink gdb server
target extended-remote :2331

set print asm-demangle on

monitor halt

# detect unhandled exceptions, hard faults and panics
break Default_Handler
break HardFault_Handler
#break app.rs:114

load

# We need to remap SysRAM to 0x00000000 for interrupts to work
set {uint16_t}0x50000012 = *(uint16_t*)0x50000012 | 0x0002
monitor reset

# Since we are running in RAM the probe might have difficulty finding the
# correct RTT Block. We specify the block address here.
eval "monitor exec SetRTTAddr %p", &_SEGGER_RTT

# Start process but immediately halt the processor
stepi
