#!/bin/bash

wget https://github.com/nanashi-1/osu-lazer-manager/releases/latest/download/osu-lazer-manager.tar.xz
tar -xvf osu-lazer-manager.tar.xz
mkdir -p ~/.local/bin/
mv osu-lazer-manager ~/.local/bin/osu-lazer-manager
rm osu-lazer-manager.tar.xz
