<div align="center">

<img src="./assets/logo.png" alt="logo" width="150" height="150">

# Quix - qx

[![Built with rust][rust-badge]][rust] [![License][license-badge]][license]

Quix is a **CLI**, created aiming for **the best developer experience**, _lets make VTEX IO quick again._

[Getting started](#getting-started) • [Commands](#commands) • [Installation](#installation) • [Configuration](#configuration)

</div>

# ⚡️ Getting started

First, to clarify the purpose of this project, as developers, we know the importance of a fast and reliable developer experience. Time to get started!

# ⚙️ Commands

As we know, this CLI is under development, so we have implemented a few commands to help you get started. The following are the available commands:

## ⛓️ Link:

> Links the project to the current workspace.

This command will synchronize the local project with the remote VTEX account you are logged in. (At the moment, the login and use commands are not implemented, use the [VTEX IO CLI](toolbelt) instead.)

### 📝 Usage:

```bash
qx link <FLAGS>
```

### 🎏 Flags:

| Flag | Description                                                |
| :--: | :--------------------------------------------------------- |
| `-c` | Clean the project cache before linking.                    |
| `-q` | Link the project **quicker**. (By ignoring some steps. 👀) |

## 🥇 Contributing

- Feel free to contribute to this project, if you have any suggestions or improvements, please open an issue or pull request.

> ## Thanks! 😄

---

## ⚠️ License

_This code is licensed under the [MIT]("https://github.com/RafaelRCamargo/from-reddit-to-shorts/blob/master/LICENSE") license._

[toolbelt]: https://github.com/vtex/toolbelt
[rust-badge]: https://img.shields.io/badge/builtwith-rust-B7410E?style=flat-square
[rust]: https://www.rust-lang.org/pt-BR
[license-badge]: https://img.shields.io/github/license/rafaelrcamargo/quix?color=lightgray&style=flat-square
[license]: https://github.com/rafaelrcamargo/quix/blob/main/LICENSE
[releases]: https://github.com/rafaelrcamargo/quix/releases
