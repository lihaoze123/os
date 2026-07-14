# RISC-V 裸机目标平台。
target := "riscv64gc-unknown-none-elf"
# Release 模式生成的内核 ELF 文件。
kernel_elf := "target/" + target + "/release/os"
# 从 ELF 文件剥离出的、供 QEMU 直接加载的内核二进制文件。
kernel_bin := kernel_elf + ".bin"
# QEMU 使用的 RustSBI 引导程序。
bootloader := "../bootloader/rustsbi-qemu.bin"
# 内核在物理内存中的加载与入口地址。
kernel_entry_pa := "0x80200000"

# 未指定 recipe 时默认构建内核。
default: build

# 以 Release 模式编译内核，并转换为裸二进制镜像。
build:
    @cargo build --release
    @rust-objcopy --binary-architecture=riscv64 {{ kernel_elf }} --strip-all -O binary {{ kernel_bin }}

# 构建内核并通过 QEMU 启动。
run: build
    @qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios {{ bootloader }} \
        -device loader,file={{ kernel_bin }},addr={{ kernel_entry_pa }}

# 在 tmux 中分别启动等待 GDB 的 QEMU 和调试器。
debug: build
    @tmux new-session -d \
        "qemu-system-riscv64 -machine virt -nographic -bios {{ bootloader }} -device loader,file={{ kernel_bin }},addr={{ kernel_entry_pa }} -s -S" && \
        tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file {{ kernel_elf }}' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
        tmux -2 attach-session -d

# 查看内核 ELF 的文件头、段信息和反汇编结果。
disasm: build
    @rust-objdump --arch-name=riscv64 -x {{ kernel_elf }} | less

# 删除 Cargo 生成的构建产物。
clean:
    @cargo clean
