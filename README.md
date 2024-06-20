# osu-lazer-manager

![Crates.io Version](https://img.shields.io/crates/v/osu-lazer-manager)
![License](https://img.shields.io/crates/l/osu-lazer-manager)
[![Tests](https://github.com/nanashi-1/osu-lazer-manager/actions/workflows/test.yml/badge.svg)](https://github.com/nanashi-1/osu-lazer-manager/actions/workflows/test.yml)

osu!lazer Management Tool for Linux

This tool simplifies managing osu!lazer on your Linux system. It lets you:

- Install and Update osu!lazer: Easily install new versions or update existing ones.
- Manage Multiple Versions: Keep different osu!lazer versions handy and switch between them effortlessly.
- Launch Directly from CLI: No need for extra steps, launch osu!lazer straight from the command line.
- Desktop Entry Management: Create and update desktop entries for a seamless user experience.

## What's New in v1.0.0:

This major update brings significant improvements and lays the foundation for future features:

- Simplified osu!lazer Launch: Launch osu!lazer directly from the command line with a single command.
- Multi-Version Support: Manage and switch between multiple osu!lazer versions effortlessly.
- Code Refactoring: The codebase is now more modular, maintainable, and efficient. It utilizes an internal API, allowing for potential future expansion (like a GUI!).
- Enhanced Error Handling: Improved error handling with the anyhow crate provides clearer and more informative messages.
- Flexible Terminal Colors: Leverages the colorize crate for customizable terminal colors, independent of the program itself.
- Dynamic Desktop Entry Updates: Desktop entries and icons automatically update when changes occur in osu!lazer, eliminating the need for manual tool updates.

## Installation

The recommended way to install the osu!lazer manager is using Cargo. This offers the benefits of package management and ensures a secure installation process.

### Pre-compiled Binaries

**with scripts**
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nanashi-1/osu-lazer-manager/releases/latest/download/osu-lazer-manager-installer.sh | sh
```

**with cargo-binstall**
```bash
cargo binstall osu-lazer-manager
```

### Compile Current Development Version

If you want to try the latest development version before it's published on crates.io, you can install it directly from your local Git repository or a remote one using Cargo's `install` command with the `--git` flag:

```bash
cargo install --git https://github.com/nanashi-1/osu-lazer-manager
```

## License

This project is licensed under the MIT License. You can find a copy of the license in the [LICENSE](LICENSE) file.
