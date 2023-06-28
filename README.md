<div align="center">

![logo.png](./assets/logo.png)

# Quix

[![Built with rust][rust-badge]][rust] [![License][license-badge]][license]

Quix is a **CLI**, created aiming for **the best developer experience** with the [VTEX IO](https://vtex.io/) platform.

[Getting started](#getting-started) • [Commands](#commands) • [Installation](#installation) • [Contributing](#contributing) • [Known issues](#known-issues) • [LICENSE](#license)

</div>

<a name="getting-started">

# ⚡️ Getting started

</a>

First, to clarify the purpose of this project. As developers, we know the importance of a fast and reliable developer experience. And that's what we are aiming for, to make the VTEX IO developer experience as pleasant as possible.

<a name="commands">

# ⚙️ Commands

</a>

> **Note**: This CLI is a work in progress, and only a few commands are currently available.

The following commands are available:

## ⛓️ Link

> Links the project to the current workspace.

This command will synchronize the local project with the remote VTEX account you are logged in. _(At the moment, the `login` and `use` commands are not implemented, use the [VTEX IO CLI](toolbelt) instead.)_

### Usage

```bash
qx link <FLAGS>
```

#### Flags

| Flag | Description                                   |
| :--: | :-------------------------------------------: |
| `-c` | Cleans the project cache before linking.      |
| `-q` | Enables **quick** linking, skipping steps. 👀 |

<a name="installation">

# 📦 Installation

</a>

1. Check the [releases]("https://github.com/rafaelrcamargo/quix/releases") page to download the latest version of the CLI.

2. Then we need to setup the CLI, for an easier setup we have created a script to add the CLI to your `PATH` environment variable. The setup process goes as follows:

<details>
<summary>🖥️ Windows</summary>

> **Warn**: Tested on version `0`.0.0`, newer versions are expected to work, but not covered.

```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/rafaelrcamargo/quix/main/release/add_to_path.ps1?token=GHSAT0AAAAAABQRQIGNWZVRGJSTIAH46OLAYZYP4VQ -OutFile install.ps1; .\install.ps1
```

</details>

<details>
<summary>🍎 MacOS</summary>

```bash
curl -s https://raw.githubusercontent.com/rafaelrcamargo/quix/main/release/add_to_path.sh?token=GHSAT0AAAAAABQRQIGMIQULFXTTFIS76DDQYZYP5GA | bash
```

</details>

<details>
<summary>🐧 Linux</summary>

> **Warn**: Tested on version `0`.0.0`, newer versions are expected to work, but not covered.

```bash
curl -s https://raw.githubusercontent.com/rafaelrcamargo/quix/main/release/add_to_path.sh?token=GHSAT0AAAAAABQRQIGMIQULFXTTFIS76DDQYZYP5GA | bash
```

</details>

## 🧮 Improvements

One of the main focuses of this project is performance. We have implemented and documented several benchmarks to help improve the performance of Quix. The following benchmark is available:

<details open>

<summary>

### 📈 Benchmarks

</summary>

<br>

- [🛑 Minifier](/benchmarks/minifier/results.md)
  - Compares the performance of the minifier crate and the VTEX IO Link endpoint, analyzing the performance of raw files and minified files in the initial and subsequent `quix link` commands.

</details>

<a name="known-issues">

# 🕵️ Known issues

</a>

<details>
<summary>VS Code - Terminal not rendering properly?</summary>
<br>
That's a known issue, and it's related to the way VS Code handles the terminal. To fix this, just open the settings and add the following line:

```json
{
  ...
  "terminal.integrated.gpuAcceleration": "on"
}
```

This will enable the GPU acceleration for the terminal, and it will fix the rendering issue.

> For now this solves the issue, but can get kinda weird with some appearance settings.

</details>

<a name="contributing">

# 🥇 Contributing

</a>

Contributions to this project are welcome! If you have any suggestions or improvements, please open an issue or pull request.

<a name="license">

# ⚠️ License

</a>

This code is licensed under the [Apache 2.0](LICENSE) license.

[rust-badge]: https://img.shields.io/badge/builtwith-rust-B7410E?style=flat-square
[rust]: https://www.rust-lang.org/pt-BR
[license-badge]: https://img.shields.io/github/license/rafaelrcamargo/quix?color=lightgray&style=flat-square
[license]: https://github.com/rafaelrcamargo/quix/blob/main/LICENSE
