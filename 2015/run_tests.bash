#!/bin/bash

set -euo pipefail

ROOT_DIR=$(dirname $0)

for input_file in "$ROOT_DIR"/tests/*.input; do
    file_without_extension="${input_file%.*}"
    output_file="$file_without_extension.output"
    name="$(basename "$file_without_extension")"
    dayname="${name%%.*}"

    echo -n "Running $name "
    if [[ -f "$ROOT_DIR/target/release/$dayname" ]]; then
        echo -n "using release target"
        bin="$ROOT_DIR/target/release/$dayname"
    elif [[ -f "$ROOT_DIR/target/debug/$dayname" ]]; then
        echo -n "using debug target"
        bin="$ROOT_DIR/target/debug/$dayname"
    else
        echo "no binary, skipping"
        continue
    fi

    echo -n "... "

    output=$("$bin" < "$input_file")
    diff_output=$(diff -u "$output_file" <(echo "$output") 2>&1 || true)
    if [[ -n $diff_output ]]; then
        echo "failed:"
        echo "$diff_output"
        echo
    else
        echo "succeed"
    fi
done
