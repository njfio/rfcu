#!/bin/bash

FILE_PATH="$1"
# Run the rfcu command and extract structures
rfcu --file_path "$FILE_PATH" get_structure
