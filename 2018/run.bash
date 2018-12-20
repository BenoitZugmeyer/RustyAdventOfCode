#!/bin/bash

set -euo pipefail

ROOT_DIR=$(dirname $0)

exit_code=0
files=$(find $ROOT_DIR/src/bin -name "*.rs" -printf "%f\n" | sort -n)
echo $files

for src_file in $files; do
  name="${src_file%.*}"
  test_name="${name%-*}"

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

  output=$("$bin" < "tests/$test_name/input")
  diff_output=$(diff -u "tests/$test_name/output" <(echo "$output") 2>&1 || true)
  if [[ -n $diff_output ]]; then
      echo "failed:"
      echo "$diff_output"
      echo
      exit_code=1
  else
      echo "succeed"
  fi
done

exit $exit_code
