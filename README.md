<div align="center">

# force-ansi

English | [简体中文](README-CN.md)

A command-line wrapper program that can force ANSI code to be enabled in the Windows console

[Website](https://opensound.run) | [crates.io](https://crates.io/crates/force-ansi)

Original Author: [@czy-29](https://github.com/czy-29)

Latest version: [v1.0.0](https://github.com/opensound-org/force-ansi/releases/tag/v1.0.0)

![Crates.io Total Downloads](https://img.shields.io/crates/d/force-ansi)
![GitHub Repo stars](https://img.shields.io/github/stars/opensound-org/force-ansi)

[![dependency status (version)](https://deps.rs/crate/force-ansi/1.0.0/status.svg?subject=v1.0.0-deps)](https://deps.rs/crate/force-ansi/1.0.0)
[![dependency status (git)](https://deps.rs/repo/github/opensound-org/force-ansi/status.svg?subject=git-deps)](https://deps.rs/repo/github/opensound-org/force-ansi)

[![Static Badge](https://img.shields.io/badge/build_with-Rust_1.84.0-dca282)](https://blog.rust-lang.org/2024/11/28/Rust-1.84.0.html)

</div>

## What
A command-line wrapper program that can force ANSI code to be enabled in the Windows console.

## Why
In the Windows console (whether it's `cmd` or `PowerShell`), if you want to use ANSI code (such as ANSI colors) properly, you need to call some Windows APIs to manually enable it.

However, some command-line programs on Windows use ANSI code but do not enable it correctly. This project, as a wrapper program, aims to force ANSI support for you on Windows. For programs that were previously unable to display ANSI code correctly, after wrapping them with this program, ANSI code will be displayed normally (command-line arguments will be passed through to the target program).

Reference links:
1. [nu_ansi_term::enable_ansi_support()](https://docs.rs/nu-ansi-term/latest/x86_64-pc-windows-msvc/nu_ansi_term/fn.enable_ansi_support.html)
2. [Console Virtual Terminal Sequences](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences)
3. [When `ANSI` is enabled, `nu_ansi_term::enable_ansi_support()` should be called on `Windows`](https://github.com/tokio-rs/tracing/issues/3068)
4. [Bug: Enabling `ANSI` colors for `tracing` logs on `Windows` requires calling `nu_ansi_term::enable_ansi_support()`](https://github.com/surrealdb/surrealdb/issues/5224)

## How
This is a pure binary project. You can use the following command to install:
```
cargo install force-ansi
```
Note that this will install two programs: `force-ansi.exe` and `abnormal-ansi.exe`.

`force-ansi.exe` is our main program, which you can use on any target program.

`abnormal-ansi.exe` is an example target program we provide.

You can first run `abnormal-ansi` in `cmd` or `PowerShell` to see if the console will output garbled characters.

Then run:
```
force-ansi abnormal-ansi
```
to see if the console can render ANSI colors normally now.

The usage for other target programs is consistent, and command-line arguments are simply passed transparently.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=opensound-org/force-ansi&type=Date)](https://star-history.com/#opensound-org/force-ansi&Date)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

[Github](https://github.com/opensound-org/force-ansi) is our [single source of truth](https://en.wikipedia.org/wiki/Single_source_of_truth), where we welcome all issues and pull requests.

We also have two downstream read-only mirrors that are [automatically pushed](.github/workflows/mirror.yml):
- [Gitea](https://gitea.29bot.com/opensound-org/force-ansi)
- [Gitee](https://gitee.com/opensound-org/force-ansi)

As they are read-only mirrors, please do not initiate any merge or pull requests on these two platforms.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `force-ansi` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
