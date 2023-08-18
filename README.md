# osu-lazer-installer

Installs and updates [osu!lazer](https://github.com/ppy/osu-resources) in Linux.

## Usage

### Install osu!lazer

    osu-lazer-manager install


If you've used this tool before and it failed, you may want to run:

    osu-lazer-manager install --force

### Update osu!lazer

    osu-lazer-manager update

If osu!lazer fails to run after update, run:

    osu-lazer-manager update --force

### Check installed version

    osu-lazer-manager version

### Check latest version

    osu-lazer-manager version --include-latest-version

*Note: This will also display the installed version.*

### Uninstall osu!lazer

    osu-lazer-manager uninstall

## Installation

### Quick Install

*Note: Not yet implemented*

### from Source

    cargo install --git https://github.com/nanashi-1/osu-lazer-manager

*Note: cargo must be installed in your system*

## License

This repository is licensed under the MIT License.
