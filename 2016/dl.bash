#!/bin/bash

set -euo pipefail

dayname=$(printf day%02d $2)
mkdir -p tests/$dayname
curl "http://adventofcode.com/2016/day/$2/input" -H "Cookie: session=$1" > tests/$dayname/input
touch tests/$dayname/output
git add tests/$dayname/input tests/$dayname/output
