<div align="center">

# force-ansi

[English](README.md) | 简体中文

一个可以在Windows控制台中强制启用ANSI代码的命令行wrapper程序

[官网](https://opensound.run) | [crates.io](https://crates.io/crates/force-ansi)

原始作者：[@czy-29](https://github.com/czy-29)

最新版本：[v1.0.0](https://github.com/opensound-org/force-ansi/releases/tag/v1.0.0)

![Crates.io Total Downloads](https://img.shields.io/crates/d/force-ansi)
![GitHub Repo stars](https://img.shields.io/github/stars/opensound-org/force-ansi)

[![dependency status (version)](https://deps.rs/crate/force-ansi/1.0.0/status.svg?subject=v1.0.0-deps)](https://deps.rs/crate/force-ansi/1.0.0)
[![dependency status (git)](https://deps.rs/repo/github/opensound-org/force-ansi/status.svg?subject=git-deps)](https://deps.rs/repo/github/opensound-org/force-ansi)

[![Static Badge](https://img.shields.io/badge/build_with-Rust_1.83.0-dca282)](https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html)

</div>

## 是什么
一个可以在Windows控制台中强制启用ANSI代码的命令行wrapper程序。

## 为什么
在Windows控制台（无论是 `cmd` 还是 `PowerShell` ）中，如果想正常使用ANSI代码（如ANSI颜色），需要调用一些Windows API手动启用。

然而有一些命令行程序在Windows上使用了ANSI代码，却没有正确启用它。本项目作为一个Wrapper程序，旨在Windows上为您强制启用ANSI支持。对于之前无法正确显示ANSI代码的程序，使用本程序包裹一下之后，ANSI代码将可以正常显示（命令行参数会透传至目标程序）。

参考链接：
1. [nu_ansi_term::enable_ansi_support()](https://docs.rs/nu-ansi-term/latest/x86_64-pc-windows-msvc/nu_ansi_term/fn.enable_ansi_support.html)
2. [Console Virtual Terminal Sequences](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences)
3. [When `ANSI` is enabled, `nu_ansi_term::enable_ansi_support()` should be called on `Windows`](https://github.com/tokio-rs/tracing/issues/3068)
4. [Bug: Enabling `ANSI` colors for `tracing` logs on `Windows` requires calling `nu_ansi_term::enable_ansi_support()`](https://github.com/surrealdb/surrealdb/issues/5224)

## 怎么用
这是一个纯二进制项目。您可以使用以下命令安装：
```
cargo install force-ansi
```
注意，这将安装两个程序：`force-ansi.exe` 和 `abnormal-ansi.exe`。

`force-ansi.exe` 是我们的主程序，您可以对任意目标程序使用它。

`abnormal-ansi.exe` 是我们提供的一个示例目标程序。

您可以首先在 `cmd` 或 `PowerShell` 中运行 `abnormal-ansi`，看一下是不是控制台中会输出乱码。

然后您再运行：
```
force-ansi abnormal-ansi
```
看看现在是不是控制台可以正常渲染ANSI颜色了。

对于其它目标程序的用法是一致的，命令行参数就是简单地透传。

## Star历史

[![Star History Chart](https://api.star-history.com/svg?repos=opensound-org/force-ansi&type=Date)](https://star-history.com/#opensound-org/force-ansi&Date)

# 许可证

本项目使用以下两种许可之一

 * Apache协议，2.0版本，([LICENSE-APACHE](LICENSE-APACHE) 或
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT协议 ([LICENSE-MIT](LICENSE-MIT) 或
   http://opensource.org/licenses/MIT)

由您选择。

## 贡献

[Github](https://github.com/opensound-org/force-ansi)是我们的[单一信源](https://en.wikipedia.org/wiki/Single_source_of_truth)，这里我们欢迎所有的issue和pull request。

我们另有两个[自动推送](.github/workflows/mirror.yml)的下游只读镜像：
- [GitLab](https://gitlab.com/opensound-org/force-ansi)
- [Gitee](https://gitee.com/opensound-org/force-ansi)

由于它们是只读镜像，因此请不要在这两个平台上发起任何合并请求或pull request。

除非您另有明确说明，否则您有意提交的
包含在 `force-ansi` 中的任何贡献（如 Apache-2.0 许可证中所定义）均应
获得上述双重许可，无需任何附加条款或条件。
