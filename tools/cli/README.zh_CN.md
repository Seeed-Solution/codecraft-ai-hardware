[English](README.md)

# CodeCraft CLI

独立的 Rust 创作 CLI。运行时从所选仓库或项目读取数据，不把资料编译进二进制。
普通命令从不调用 Cargo。

在仓库根目录运行 `sh tools/cli/install.sh` 安装最新正式版 Release
（Windows 使用 `tools/cli/install.ps1`），然后使用 `codecraft --help`。先解析最新 Release 的具体版本，再下载该版本的二进制和校验和。可用 `--version X.Y.Z`（Windows：`-Version X.Y.Z`）选择其他已发布版本。
下载文件校验通过后才替换已有程序，失败时不会自动转为源码构建。

开发 CLI 时使用 `sh tools/cli/install.sh --from-source`
（Windows：`tools/cli/install.ps1 -FromSource`）。仅源码构建需要 Cargo 和原生链接器；
下载的二进制无需 Rust 或 Python。

当前工作树新增 `codecraft doctor --target <stack>/<device>`，只读检查已激活工具链。
在包含此命令的版本发布前需要源码安装；v0.5.0 尚不包含它。

使用 `cargo test --locked --package codecraft` 运行集成测试。测试归属
 tests/tools/cli/，平台实现归属 platforms/<stack>/native.rs。
参见[运行时与更新策略](../../docs/development/rust-cli.md)。
