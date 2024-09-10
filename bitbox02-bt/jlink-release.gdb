# `target extended-remote :2331` is enough if you run on the same machine as
# where you run the jlink gdb server
target extended-remote host.docker.internal:2331

set print asm-demangle on

monitor reset

# detect unhandled exceptions, hard faults and panics
break Default_Handler
break HardFault_Handler

load

# Since we are running in RAM the probe might have difficulty finding the
# correct RTT Block. We specify the block address here.
eval "monitor exec SetRTTAddr %p", &_SEGGER_RTT

# Start process but immediately halt the processor
stepi
