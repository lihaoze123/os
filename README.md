一个 [rCore](https://rcore-os.cn/rCore-Tutorial-Book-v3) 学习项目。

## 构建

项目使用 GNU Make 4.3 或更高版本编排用户程序、内核镜像和 QEMU，Cargo 继续负责 Rust 编译与依赖管理。

```bash
make build
make run
```

常用目标：

```text
make build                 构建用户程序和内核
make run                   在 QEMU 中运行
make debug                 在 tmux 中启动 QEMU 和 GDB
make disasm                查看内核反汇编
make user-disasm APP=name  查看指定用户程序的反汇编
make clean                 清理构建产物
make help                  查看帮助
```

推荐通过 Nix 开发环境获得交叉编译工具链：

```bash
nix develop
make build
```
