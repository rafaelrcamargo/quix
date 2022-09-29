<div align="center">

![logo.png](./assets/logo.png)

# Quix

[![Built with rust][rust-badge]][rust] [![License][license-badge]][license]

Quix is a **CLI**, created aiming for **the best developer experience**, _lets make VTEX IO quick again._

[Getting started](#getting-started) • [Commands](#commands) • [Installation](#installation) • [Configuration](#configuration)

</div>

# ⚡️ Getting started

First, to clarify the purpose of this project, as developers, we know the importance of a fast and reliable developer experience. Time to get started!

# ⚙️ Commands

As we know, this CLI is under development, so we have implemented a few commands to help you get started. The following are the available commands:

## ⛓️ Link

> Links the project to the current workspace.

This command will synchronize the local project with the remote VTEX account you are logged in. _(At the moment, the login and use commands are not implemented, use the [VTEX IO CLI](toolbelt) instead.)_

### Usage

```bash
qx link <FLAGS>
```

### Flags

| Flag | Description                                                |
| :--: | :--------------------------------------------------------- |
| `-c` | Clean the project cache before linking.                    |
| `-q` | Link the project **quicker**. (By ignoring some steps. 👀) |

# 🥇 Contributing

Feel free to contribute to this project, if you have any suggestions or improvements, please open an issue or pull request.

# 🧮 Improvements

One of the biggest improvements we have made is the performance, and thats the foundation of this project.

But no one ever knows for sure what is the best way to improve the performance of a project in every situation, so we have implemented and documented some benchmarks to help in our conclusions.

This list of benchmarks is not exhaustive, but it is a good starting point to understand how to improve the performance of this project, and to this moment we've documented the following:

## 📈 Benchmarks

- [🛑 Minifier](/benchmarks/minifier/results.md)
  - The idea behind this benchmark is to compare the performance of the minifier crate, and the VTEX IO Link endpoint. Comparing the performance of the raw files, and the minified files in the first and subsequent `quix link` commands.

# ⚠️ License

_This code is licensed under the [MIT]("https://github.com/RafaelRCamargo/from-reddit-to-shorts/blob/master/LICENSE") license._

[rust-badge]: https://img.shields.io/badge/builtwith-rust-B7410E?style=flat-square
[rust]: https://www.rust-lang.org/pt-BR
[license-badge]: https://img.shields.io/github/license/rafaelrcamargo/quix?color=lightgray&style=flat-square
[license]: https://github.com/rafaelrcamargo/quix/blob/main/LICENSE
