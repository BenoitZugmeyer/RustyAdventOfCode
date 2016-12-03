#!/bin/bash

set -euo pipefail

ROOT_DIR=$(dirname $0)

for input_file in "$ROOT_DIR"/tests/*/input; do
    name=$(basename $(dirname "$input_file"))
    output_file=$(dirname "$input_file")/output

    echo -n "Running $name "
    if [[ -f "$ROOT_DIR/target/release/$name" ]]; then
        echo -n "using release target"
        bin="$ROOT_DIR/target/release/$name"
    elif [[ -f "$ROOT_DIR/target/debug/$name" ]]; then
        echo -n "using debug target"
        bin="$ROOT_DIR/target/debug/$name"
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
