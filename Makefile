# To replace docker with podman set `DOCKER=podman` in your environment
DOCKER ?= docker

.PHONY: dockerdev
dockerdev:
	@./scripts/dockerenv.sh

.PHONY: dockerinit
dockerinit:
	${DOCKER} build -t shiftcrypto/da14531 .

.PHONY: dockerpull
dockerpull:
	${DOCKER} pull shiftcrypto/da14531

.PHONY: dockerpush
dockerpush:
	${DOCKER} build --push --platform linux/amd64,linux/arm64 -t shiftcrypto/da14531 .

.PHONY: dockerstop
dockerstop:
	@./scripts/dockerenv.sh stop

.PHONY: gdb-server
gdb-server:
	JLinkGDBServer -device da14531 -if SWD -ir

.PHONY: rtt-client
rtt-client:
	telnet localhost 19021

.PHONY: build
build:
	${MAKE} -C bitbox02-bt build

.PHONY: run
run:
	${MAKE} -C bitbox02-bt run

.PHONY: clean
clean:
	${MAKE} -C bitbox02-bt clean

.PHONY: test
test:
	(cd u2fframing; cargo test test)
