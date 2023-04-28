PREFIX:=/usr
BUILD_FLAGS=--release --target=x86_64-unknown-linux-musl

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

.PHONY: all build clean check test
