#! /bin/bash

# * Convert the .png files to .svg files

for i in *.png; do
    printf "$i\n"
    modified=${i::-4}
    convert "$i" "svg/${modified}.svg"
done