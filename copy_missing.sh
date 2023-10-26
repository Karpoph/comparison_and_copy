#!/bin/bash

# Set the source and destination directories
source_dir="/home/watens/Desktop/pdfs_1"
dest_dir="/home/watens/Downloads/pdfs_2"

# Iterate through the files in the source directory
for file in "$source_dir"/*.pdf "$source_dir"/*.xml; do
    # Check if the file exists in the destination directory
    if [ ! -f "$dest_dir"/"${file##*/}" ]; then
        # Copy the missing file from source to destination
        cp "$file" "$dest_dir"
    fi
done
