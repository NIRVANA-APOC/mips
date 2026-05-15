# MIPS Emulator (TEMU)

一个教育级 MIPS-I 指令集模拟器，用 Rust 从零实现。支持 40+ 条 MIPS 指令、交互式调试器、DDR3 内存模拟，以及通过 `syscall` 输出字符串到控制台。

[![Rust](https://img.shields.io/badge/rust-2021-orange)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-49%20passing-brightgreen)]()
[![Clippy](https://img.shields.io/badge/clippy-clean-blue)]()

---

## 功能特性

- **指令集**：支持算术、逻辑、移位、乘除、加载存储、分支跳转等 40+ 条 MIPS 指令
- **内存系统**：512MB DRAM，模拟真实 DDR3 的 Rank/Bank/Row/Column 结构与 Row Buffer 机制
- **交互式调试器**：单步执行、寄存器查看、调试模式、程序重启
- **系统调用**：支持 `print_string` (syscall 4) 和 `exit` (syscall 10)
- **零 unsafe**：彻底消除 `static mut`，全部状态封装在安全的 `Emulator` 结构体中
- **测试覆盖**：49 个单元测试覆盖所有主要指令

---

## 快速开始

### 编译

```bash
cargo build --release
```

### 运行 Hello World

项目已内置 Hello World 示例：

```bash
cargo run --release
```

在 `(Azathoth)>>>` 提示符下输入 `c`（continue），你将看到：

```
Hello World.
load ./bin/inst.bin
load ./bin/data.bin
(Azathoth)>>> 1fc00000:   20 04 00 00  => addi  $a0,   $zero,   0x0000
1fc00004:   20 02 00 04  => addi  $v0,   $zero,   0x0004
Hello World
1fc00008:   00 00 00 0c  => syscall
...
```

然后输入 `q` 退出。

---

## 项目结构

```
src/
├── main.rs                 # 程序入口
└── emu/
    ├── mod.rs
    ├── cpu/
    │   ├── exec.rs         # Emulator 核心 + 指令分发
    │   ├── reg.rs          # 通用寄存器 (Gpr) / CPU 状态 (CpuRegs)
    │   ├── operand.rs      # 操作数解码结构
    │   ├── helper.rs       # 指令编码辅助函数
    │   ├── r_type.rs       # R-type 指令实现
    │   ├── i_type.rs       # I-type 指令实现
    │   └── j_type.rs       # J-type 指令实现
    ├── memory/
    │   └── dram.rs         # DDR3 DRAM 模拟器
    └── monitor/
        ├── system.rs         # 程序加载 / 重启逻辑
        └── ui.rs             # 交互式命令行调试器

bin/
├── inst.bin                # 指令二进制（加载到 0xBFC00000）
└── data.bin                # 数据二进制（加载到 0x00000000）
```

---

## 支持的 MIPS 指令

### 算术运算
`add` `addu` `sub` `subu` `addi` `addiu`

### 比较与置位
`slt` `sltu` `slti` `sltiu`

### 乘除法
`mult` `multu` `div` `divu`

### 逻辑与位运算
`and` `or` `xor` `nor` `andi` `ori` `xori`

### 移位
`sll` `srl` `sra` `sllv` `srlv` `srav`

### 加载与存储
`lb` `lh` `lw` `lbu` `lhu` `sb` `sh` `sw` `lui`

### 分支与跳转
`beq` `bne` `blez` `bgtz` `bltz` `bgez` `bltzal` `bgezal`  
`j` `jal` `jr` `jalr`

### 系统指令
`syscall` `break`

### 异常处理
- **算术溢出**：`add` / `addi` / `sub` 溢出时终止执行
- **除零**：`div` / `divu` 除零时终止执行
- **地址对齐**：`lw`/`sw` 要求 4 字节对齐，`lh`/`sh` 要求 2 字节对齐
- **无效指令**：未定义 opcode 触发异常终止

---

## 交互式调试器

启动后进入 `(Azathoth)>>>` 命令行：

| 命令 | 简写 | 功能 |
|---|---|---|
| `help` | `h` | 显示帮助 |
| `continue` | `c` | 持续执行直到程序结束 |
| `single` | `s` | 单步执行一条指令 |
| `reg` | `r` | 查看所有寄存器；`r $t0` 查看单个寄存器 |
| `debug` | `dbg` | 开关调试输出（打印指令解码过程） |
| `restart` | `re` | 重置 CPU 和内存，重新加载 bin 文件 |
| `quit` | `q` | 退出模拟器 |

每条指令执行后会自动打印跟踪信息：
```
1fc00000:   20 02 00 04  => addi  $v0,   $zero,   0x0004
```
格式：`物理地址: 原始字节 => 反汇编`

---

## 编写自定义程序

### 加载机制

| 文件 | 加载位置 | 用途 |
|---|---|---|
| `bin/inst.bin` | 虚拟 `0xBFC00000`（物理 `0x1FC00000`） | 指令 |
| `bin/data.bin` | 物理 `0x00000000` | 数据 |

### Syscall 约定

| `$v0` | 功能 | 参数 |
|---|---|---|
| `4` | `print_string` | `$a0` = 字符串首地址（以 `\0` 结尾） |
| `10` | `exit` | 结束程序 |

### 示例：输出字符串

```python
import struct

# addi $a0, $zero, 0      ; $a0 = 字符串地址
# addi $v0, $zero, 4      ; $v0 = 4 (print_string)
# syscall
# addi $v0, $zero, 10     ; $v0 = 10 (exit)
# syscall
instructions = [
    0x20040000,   # addi $a0, $zero, 0
    0x20020004,   # addi $v0, $zero, 4
    0x0000000C,   # syscall
    0x2002000A,   # addi $v0, $zero, 10
    0x0000000C,   # syscall
]

with open('bin/inst.bin', 'wb') as f:
    for inst in instructions:
        f.write(struct.pack('<I', inst))

with open('bin/data.bin', 'wb') as f:
    f.write(b"Your Message Here\n\0")
```

---

## 运行测试

```bash
cargo test
```

49 个单元测试覆盖所有主要指令，包括溢出、除零、对齐异常等边界情况。

---

## 技术亮点

- **零 `static mut`**：全部状态封装在 `Emulator` 结构体中，彻底消除数据竞争风险
- **安全内存访问**：DDR3 模拟器使用安全的 `Vec<u8>` 和 slice 操作，替代原始指针
- **类型安全寄存器**：`$zero` 寄存器恒为 0，写入被静默忽略（符合 MIPS 规范）
- **零 Clippy 警告**：代码通过 `cargo clippy` 严格检查

---

## 许可

MIT License
