SHELL := bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:

TARGET := riscv64gc-unknown-none-elf
PROFILE := release

CARGO ?= cargo
OBJCOPY := rust-objcopy
OBJDUMP := rust-objdump
QEMU ?= qemu-system-riscv64

USER_APP_SOURCES := $(sort $(wildcard user/src/bin/*.rs))
USER_APP_NAMES := $(basename $(notdir $(USER_APP_SOURCES)))
USER_TARGET_DIR := user/target/$(TARGET)/$(PROFILE)
USER_ELFS := $(addprefix $(USER_TARGET_DIR)/,$(USER_APP_NAMES))
USER_BINS := $(addsuffix .bin,$(USER_ELFS))
USER_INPUTS := user/Cargo.toml user/Cargo.lock user/build.rs user/.cargo/config.toml \
	$(shell find user/src -type f | sort)
USER_APP_MANIFEST := .build/user-apps.list

OS_TARGET_DIR := os/target/$(TARGET)/$(PROFILE)
KERNEL_ELF := $(OS_TARGET_DIR)/os
KERNEL_BIN := $(KERNEL_ELF).bin
KERNEL_ENTRY_PA := 0x80200000
BOOTLOADER := bootloader/rustsbi-qemu.bin
OS_INPUTS := os/Cargo.toml os/Cargo.lock os/build.rs os/.cargo/config.toml \
	$(shell find os/src -type f | sort)

.PHONY: build user run debug disasm user-disasm clean help FORCE

build: $(KERNEL_BIN)

user: $(USER_BINS)

# 仅当应用的有序名称列表变化时更新时间戳。这样添加或删除应用也会触发重新链接。
$(USER_APP_MANIFEST): FORCE
	@mkdir -p $(@D)
	@tmp="$@.tmp"; \
		{ for app in $(USER_APP_NAMES); do printf '%s\n' "$$app"; done; } > "$$tmp"; \
		if test -r "$@" && cmp -s "$$tmp" "$@"; then \
			rm -f "$$tmp"; \
		else \
			mv "$$tmp" "$@"; \
		fi

FORCE:

# 一次 Cargo 调用生成所有用户 ELF；GNU Make 的 grouped target 防止并行重复执行。
$(USER_ELFS) &: $(USER_INPUTS) $(USER_APP_MANIFEST)
	@cd user && $(CARGO) build --release --bins

$(USER_TARGET_DIR)/%.bin: $(USER_TARGET_DIR)/%
	@$(OBJCOPY) --binary-architecture=riscv64 $< --strip-all -O binary $@

$(KERNEL_ELF): $(OS_INPUTS) $(USER_BINS) $(USER_APP_MANIFEST)
	@cd os && $(CARGO) build --release

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY) --binary-architecture=riscv64 $< --strip-all -O binary $@

run: $(KERNEL_BIN)
	@$(QEMU) \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

debug: $(KERNEL_BIN)
	@tmux new-session -d \
		"$(QEMU) -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S" && \
		tmux split-window -h \
		"riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

disasm: $(KERNEL_ELF)
	@$(OBJDUMP) --arch-name=riscv64 -x $(KERNEL_ELF) | less

user-disasm: user
	@if [[ -z "$(APP)" ]]; then \
		echo "usage: make user-disasm APP=<name>" >&2; \
		echo "available apps: $(USER_APP_NAMES)" >&2; \
		exit 2; \
	fi; \
	app_elf="$(USER_TARGET_DIR)/$(APP)"; \
	if [[ ! -f "$$app_elf" ]]; then \
		echo "error: unknown application '$(APP)'" >&2; \
		exit 1; \
	fi; \
	$(OBJDUMP) --arch-name=riscv64 --all-headers --disassemble --demangle "$$app_elf" | less

clean:
	@cd user && $(CARGO) clean
	@cd os && $(CARGO) clean
	@$(RM) -r -- .build

help:
	@echo "make build                 Build user programs and the kernel"
	@echo "make run                   Build and run the kernel in QEMU"
	@echo "make debug                 Start QEMU and GDB in tmux"
	@echo "make disasm                Disassemble the kernel"
	@echo "make user-disasm APP=name  Disassemble one user program"
	@echo "make clean                 Remove all build artifacts"
