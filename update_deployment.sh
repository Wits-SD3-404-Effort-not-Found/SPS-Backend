#!/bin/bash

git fetch
git checkout dev
git pull

cargo build --release

sudo systemctl restart spsbe
