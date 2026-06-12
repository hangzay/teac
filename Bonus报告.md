# Bonus 完成情况报告

## 1. 总体说明

本报告汇总各实验 Bonus 的完成情况。各项 Bonus 均围绕编译器本身的可维护性、兼容性、健壮性和测试稳定性展开，并随完整代码仓库一同提交。

实现位置覆盖 `src/` 与 `tests/`：实验三 Bonus 位于 IR 生成相关源码中，实验一和实验四 Bonus 主要位于测试驱动 `tests/tests.rs` 中。

## 2. 实验一 Bonus：AST 测试自动化与测评分流

实验一 Bonus 选择“AST 测试自动化与测评分流改进”。该任务改进 Parser 阶段的验收机制，使 AST 测试能发现“解析成功但 AST 丢节点”的问题。

实现位置：

- `tests/tests.rs:423`：`extract_fn_names` 使用正则从 `.tea` 源码自动提取所有 `fn name(`，包括 `impl` 块内的方法。
- `tests/tests.rs:446`：`test_ast_parse` 运行 `teac --emit ast`，检查退出码、stderr、AST 非空，并确认源码中每个函数名都出现在 AST 输出中。
- `tests/tests.rs:871`：`asmt_tests!` 将新增特性测试统一分流到 AST、IR、ASM 三个阶段。

改进意义：原始 parse-only 测试只要命令成功就可能通过，无法确认函数定义、方法定义是否真正进入 AST。自动提取函数名后，新增和重命名测试函数不需要手工维护期望列表，测试会跟随源码变化，并能在 Parser 阶段定位 AST 缺失问题。

验证命令：

```bash
cargo test --features float,for-loop,multi-dim-array,struct-method,asmt-tests-ast -- --test-threads=1
```

## 3. 实验二 Bonus：无

实验二文档没有设置 Bonus 项。因此实验二只完成主任务：在 `src/experimental/return_infer.rs` 中实现函数返回类型推断，支持跨函数调用链、自递归和返回类型冲突诊断。

验证命令：

```bash
cargo test --features return-type-inference -- --test-threads=1
```

## 4. 实验三 Bonus：IR 兼容性与深表达式健壮性

实验三 Bonus 选择“IR 兼容性与深表达式健壮性改进”，提升 IR 生成阶段在真实环境和复杂输入下的可靠性。

实现位置：

- `src/ir/types.rs:46`：`Dtype::Pointer` 打印为 typed pointer 形式，例如 `float*`、`i32*`，兼容旧版 `clang`。本地 `clang` 10.0 不支持 LLVM opaque pointer 的 `ptr` 语法，typed pointer 能保证 IR 路径可执行。
- `src/ir/gen/type_infer.rs:454`：`type_of_arith_expr` 使用显式栈遍历算术表达式，避免深层左结合表达式在类型推导阶段递归过深。
- `src/ir/gen/function_gen.rs:831`：`handle_arith_expr` 同样使用显式栈遍历，在 IR lowering 阶段逐步生成操作数和二元运算，避免代码生成阶段栈溢出。

改进意义：`long_code2` 等测试包含很长的左结合算术表达式，递归遍历容易造成栈深度风险。显式栈遍历保持语义不变，但把递归调用转换为循环处理，提高了编译器健壮性。typed pointer 输出则提升了 IR 与不同 LLVM/clang 版本的兼容性。

验证命令：

```bash
cargo test --features float,for-loop,asmt-tests-ir -- --test-threads=1
cargo test long_code2 -- --test-threads=1
```

## 5. 实验四 Bonus：端到端测评与合并稳定性

实验四 Bonus 选择“端到端测评与合并回归稳定性改进”，完善 AArch64 端到端测试链路，降低工具链缺失、平台差异和合并回归造成的定位成本。

实现位置：

- `tests/tests.rs:53`：`ensure_cross_tools` 按平台检查 `cc`、`docker`、`aarch64-linux-gnu-gcc`、`qemu-aarch64`，缺失时给出明确安装提示。
- `tests/tests.rs:170`：`ensure_std` 使用 `Once` 保证每个测试进程最多构建一次运行时对象文件，并根据 `std.c` 修改时间决定是否重编译。
- `tests/tests.rs:305`：`link_and_run_in_docker` 将 Intel macOS 下的 Linux AArch64 测试拆分为 Docker 链接阶段和运行阶段。
- `tests/tests.rs:871`：`asmt_tests!` 统一 AST、IR、ASM 三阶段测试入口，保证实验四默认路径能先完成 AST 验证，再进入 AArch64 汇编、链接和运行。

改进意义：实验四依赖交叉编译器、QEMU、Docker 或本机 AArch64 环境。提前检查工具链并统一测试分流，可以把“环境错误”和“编译器错误”区分开，使后端回归测试更稳定。该改进也帮助验证实验三特性合并到实验四后没有破坏端到端执行。

验证命令：

```bash
cargo test --features float -- --test-threads=1
cargo test --features for-loop,struct-method,multi-dim-array -- --test-threads=1
```

## 6. 结果汇总

| 实验 | Bonus 状态 | 开放任务 | 主要位置 |
|---|---|---|---|
| 实验一 | 已完成 | AST 测试自动化与测评分流 | `tests/tests.rs` |
| 实验二 | 无 Bonus | 不适用 | 不适用 |
| 实验三 | 已完成 | IR 兼容性与深表达式健壮性 | `src/ir/types.rs`、`src/ir/gen/type_infer.rs`、`src/ir/gen/function_gen.rs` |
| 实验四 | 已完成 | 端到端测评与合并稳定性 | `tests/tests.rs` |

总体结论：实验一、三、四均完成了符合开放任务定义的 Bonus；实验二无 Bonus 要求。相关工作分别聚焦于测试体系、IR 兼容性、编译器健壮性和端到端验证稳定性。
