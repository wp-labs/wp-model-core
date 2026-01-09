# wp-open-api

![CI](https://github.com/wp-labs/wp-open-api/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/wp-labs/wp-open-api/graph/badge.svg?token=6SVCXBHB6B)](https://codecov.io/gh/wp-labs/wp-open-api)
![License](https://img.shields.io/badge/License-Elastic%202.0-green.svg)
![Rust](https://img.shields.io/badge/rust-stable%2Bbeta-orange.svg)

`wp-open-api` maintains the shared APIs for warp-parse: sink/source runtime traits, parser interfaces, and the core data model. All crates here are reusable Rust libraries consumed by other services.

## Structure

- `wp-connector-api/`: sink/source runtime traits plus helper types.
- `wp-model-core/`: fields, records, value types, and formatting utilities.
- `wp-parse-api/`: RawData, parser-facing traits, and error types for the ingestion pipeline.

## Build & Test

```bash
# inside a crate
cargo build
cargo test

# from the workspace root targeting a package
cargo build -p wp-parse-api
cargo test  -p wp-model-core

# lint & format
cargo fmt --all
cargo clippy --all-targets --all-features -D warnings
```

## Maintainers & Attribution

- Default copyright/maintainer signature: `WarpParse Dev Team`. If publishing under a personal name, update `[workspace.package].authors` and the copyright line in `LICENSE`.
- Contributions are accepted under Apache 2.0; submitting code implies agreeing to the same license.
- For commercial licensing or branding questions, open an issue and tag the WarpParse Dev Team.

---

# wp-open-api（中文）

`wp-open-api` 汇集了 warp-parse 在数据接入、解析与核心模型上的公共 API，全部以 Rust library 形式供上层服务复用。

## 仓库结构

- `wp-connector-api/`：Sink/Source 运行时接口与辅助 Trait。
- `wp-model-core/`：字段、记录、值类型以及格式化工具。
- `wp-parse-api/`：RawData、解析接口与错误定义。

## 构建与测试

```bash
# 进入某个 crate 后执行
cargo build
cargo test

# 在工作区根按包名触发
cargo build -p wp-parse-api
cargo test  -p wp-model-core

# 常用检查
cargo fmt --all
cargo clippy --all-targets --all-features -D warnings
```

## 开源许可

本仓库使用 [ELv2](LICENSE)。如需二次分发，请保留 License/NOTICE，并在修改时注明。

## 开发者声明
- 默认署名：`WarpParse Dev Team`。如需个人发布，可修改 `[workspace.package].authors` 及 `LICENSE` 尾部版权信息。
欢迎通过 issue/PR 扩展解析模型、改进 API 或补充文档。
