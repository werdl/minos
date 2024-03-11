#!/bin/sh

img="disk.img"
path="/tmp/minos"

# pip install fusepy
mkdir -p $path
echo "Mounting $img in $path"
python run/minos-fuse.py $img $path
