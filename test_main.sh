#!/bin/bash

# Feel free to add more tests. Write your inputs to io-tests/inputX.txt
# and the expected output to io-tests/expected_output_for_inputX.txt

set -e

for i in io-tests/input*.txt; do
    echo "********** Testing $i **********"
    a=$(dirname "$i")
    b=$(basename "$i")
    cargo run < $i | diff -q "$a/expected_output_for_$b" -
done