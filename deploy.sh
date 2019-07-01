#!/bin/sh

# Clone into said directory and go there
cd "~/zahrah"
git fetch origin master
git reset --hard origin/master

# Prepare env and build, run Zahrah in a subshell
(export $(grep -v '^#' .env | xargs) && screen -s zahrah -d -m "cargo run --release")
