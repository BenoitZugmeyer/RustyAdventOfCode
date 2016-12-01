#!/bin/bash

set -euo pipefail

ROOT_DIR=$(dirname $0)

for bin_file in "$ROOT_DIR"/src/bin/*.rs; do
    file_without_extension="${bin_file%.*}"
    name="$(basename "$file_without_extension")"
    cargo clippy --bin "$name"
done

