# 用户程序与内核子项目的 justfile。
user_justfile := "user/justfile"
os_justfile := "os/justfile"

# 默认按“用户程序 -> 内核”的顺序完成整个系统构建。
default: build

# 将所有用户程序构建为供内核嵌入的裸二进制文件。
user:
    @just --justfile {{ user_justfile }} build

# 先构建用户程序，再构建内核镜像。
build: user
    @just --justfile {{ os_justfile }} build

# 构建完整系统并通过 QEMU 启动。
run: user
    @just --justfile {{ os_justfile }} run

# 构建完整系统并启动 QEMU/GDB 调试环境。
debug: user
    @just --justfile {{ os_justfile }} debug

# 构建完整系统并查看内核反汇编。
disasm: user
    @just --justfile {{ os_justfile }} disasm

# 清理用户程序与内核的 Cargo 构建产物。
clean:
    @cargo clean --manifest-path user/Cargo.toml
    @cargo clean --manifest-path os/Cargo.toml
