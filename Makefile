.PHONY: all build clean install

PROGRAM_NAME=kcswitch

all: build

build:
	cargo build --release

clean:
	cargo clean

install:
	mkdir -p $(HOME)/.local/bin
	cp target/release/$(PROGRAM_NAME) $(HOME)/.local/bin/$(PROGRAM_NAME)
