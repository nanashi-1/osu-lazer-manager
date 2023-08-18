# osu-lazer-installer

Installs and updates [osu!lazer](https://github.com/ppy/osu-resources) in Linux.

## Usage

### Install osu!lazer

    osu-lazer-installer install


If you've used this tool before and it failed, you may want to run:

    osu-lazer-installer install --force

### Update osu!lazer

    osu-lazer-installer update

If osu!lazer fails to run after update, run:

    osu-lazer-installer update --force

### Check installed version

    osu-lazer-installer version

### Check latest version

    osu-lazer-installer version --include-latest-version

*Note: This will also display the installed version.*

## Installation

### Quick Install

*Note: Not yet implemented*

### from Source

    cargo install --git https://github.com/nanashi-1/osu-lazer-installer

*Note: cargo must be installed in your system*