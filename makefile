RUST_TOOLCHAIN := stable

TARGET := aarch64-apple-darwin

RUST_BUILD := cargo build

RUST_TEST := cargo test

LOG_FILE := tests/sample_access_2.log

.PHONY: all build test clean

all: build

build:
	cargo build

test:
	cargo test

clean:
	rm -rf target/

run:
	target/debug/dedupfeed < $(LOG_FILE)

docker:
	cargo build --release
	docker build .