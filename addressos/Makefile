KERNEL_NAME := addressos-kernel

TARGET := riscv64gc-unknown-none-elf
MODE := debug
BUILD_DIR := target/$(TARGET)/$(MODE)
KERNEL_ELF := $(BUILD_DIR)/$(KERNEL_NAME)

BOOTLOADER := bootloader/riscv/rustsbi-qemu.bin

KERNEL_ENTRY_PA := 0x80200000

#Shell
SHELL := /bin/bash

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Rust
CARGO := cargo

# QEMU
QEMU := qemu-system-riscv64
QEMU_FLAGS := -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_ELF),addr=$(KERNEL_ENTRY_PA)

# LLDB
LLDB := rust-lldb

ifeq ($(MODE), debug)
	BUILD_CMD := @$(CARGO) build --target $(TARGET)
else
	BUILD_CMD := $(CARGO) build --target $(TARGET) --$(MODE)
endif

build:
	# @$(SHELL) build_bin.sh
	@$(BUILD_CMD)

run: build
	@$(QEMU) $(QEMU_FLAGS)

lldbserver: build
	@$(QEMU) $(QEMU_FLAGS) -s -S

lldbclient:
	@$(LLDB) --file $(KERNEL_ELF) -o 'gdb-remote localhost:1234'

clean:
	@$(CARGO) clean

.PHONY: all clean run
