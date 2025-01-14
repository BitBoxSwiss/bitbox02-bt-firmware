import serial
import threading
import sys
import time

ser = serial.Serial(sys.argv[1], 115200, timeout=0.1)

def read_serial():
    while True:
        buf = b""
        while True:
            r = ser.read(64)
            if len(r) == 0:
                break
            buf += r
        if len(buf) > 0:
            print(buf.hex())


t = threading.Thread(target=read_serial, daemon=True)
t.start()

for line in sys.stdin:
    ser.write(bytes(range(64)))
