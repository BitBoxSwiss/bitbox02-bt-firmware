TOOLS=arm-none-eabi-
SIZE=${TOOLS}size
OBJCOPY=${TOOLS}objcopy
TARGET=thumbv6m-none-eabi
CARGO=cargo

# Compile smaller verison of core/alloc
CARGO_FLAGS_DEBUG=\
	-Zbuild-std=core,alloc \
	-Zbuild-std-features=optimize_for_size \
	--config .cargo/runner-debug.toml
CARGO_FLAGS_RELEASE=\
	--release \
	-Zbuild-std=core,alloc \
	-Zbuild-std-features=panic_immediate_abort,optimize_for_size \
	--config .cargo/runner-release.toml

# For debugging linking / compiler flag issues:
# * pass -vv to cargo to see all cargo invocations and the output of all build.rs scripts.
# * use CC_ENABLE_DEBUG_OUTPUT=1 to make the rust crate `cc` print all its
#   invocations. (combine with -vv to actually see output)

.PHONY: build
build:
	@${CARGO} build ${CARGO_FLAGS_DEBUG}
	@${SIZE} target/${TARGET}/debug/bitbox02-bt
	@${OBJCOPY} -Obinary target/${TARGET}/debug/bitbox02-bt target/${TARGET}/debug/bitbox02-bt.bin

.PHONY: build-release
build-release:
	@${CARGO} build ${CARGO_FLAGS_RELEASE}
	@${SIZE} target/${TARGET}/release/bitbox02-bt
	@${OBJCOPY} -Obinary target/${TARGET}/release/bitbox02-bt target/${TARGET}/release/bitbox02-bt.bin

.PHONY: run
run:
	@${CARGO} run ${CARGO_FLAGS_DEBUG}

.PHONY: run-release
run-release:
	@${CARGO} run ${CARGO_FLAGS_RELEASE}

.PHONY: clean
clean:
	@${CARGO} clean

.PHONY: doc
doc:
	@${CARGO} doc --document-private-items --open
