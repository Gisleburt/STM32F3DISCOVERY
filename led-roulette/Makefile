.PHONY: setup build deploy
BINARY_FILE:=target/thumbv7em-none-eabihf/debug/led-roulette
TARGET:=thumbv7em-none-eabihf

setup:
	@which rustup || (echo rustup not found && false)
	@which cargo || (echo cargo not found && false)
	@which openocd || (echo openocd not found && false)
	@which arm-none-eabi-gdb || (echo arm-none-eabi-gdb not found && false)
	rustup override add nightly
	rustup target add $(TARGET)
	rustup component add llvm-tools-preview
	cargo install cargo-binutils

$(BINARY_FILE): src
	cargo build

build: $(BINARY_FILE)

deploy: build
	(cd /tmp &&  openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg & )
	arm-none-eabi-gdb -q $(BINARY_FILE) -ex "target remote :3333" -ex "load" -ex "monitor reset halt"
