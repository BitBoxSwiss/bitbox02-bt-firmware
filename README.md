# bitbox02 bluetooth firmware

## Dependencies

See Dockerfile for details

* ARM GCC Toolchain
* da14531 SDK

## Develop using a dev container

Run 

```
docker pull shiftcrypto/da14531
```

or fetch the SDK from renasys website and run

```
make dockerinit
```

To run the container you can run

```
make dockerdev
```

## Connect to device

Run Jlink gdb server

```
make gdb-server
```

Run telnet to see rtt output

```
make rtt-client
```

Run gdb, which will load firmware into RAM and start execution. The gdbinit
script assumes that you run from inside a container. If you run it on your host
machine you will need to change the address gdb should connect to.

```
cd bitbox02-bt
make run
```
