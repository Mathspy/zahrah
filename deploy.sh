#!/bin/sh

# Clone into said directory and go there
cd "/root/zahrah"
git fetch origin master
git reset --hard origin/master

# Build Zahrah in release mode and move to /opt
cargo build --release
cp target/release/zahrah /opt/zahrah

# Go to /opt, prepare env and run Zahrah in a subshell
cd /opt/zahrah
(export $(grep -v '^#' .env | xargs) && ./zahrah)
