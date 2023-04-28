PREFIX:=/usr
BUILD_FLAGS=--release

all: build

build:
	cargo build $(BUILD_FLAGS)

check:
	cargo fmt
	cargo check $(BUILD_FLAGS)
	cargo clippy $(BUILD_FLAGS)

test:
	cargo test $(BUILD_FLAGS)

clean:
	cargo clean
	$(RM) -rf */*.o
	$(RM) -f ./src/qbe/mod.rs
	$(RM) -rf ./deps/ziglibc/zig-linux*
	$(RM) -rf ./deps/musl-libc/build

.PHONY: all build clean check test
