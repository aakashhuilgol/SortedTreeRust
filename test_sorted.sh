#!/bin/bash

set -e

# Feel free to add more tests. Add files named dumpX.txt that do something
# and then finally dump the content of the tree.

for i in io-tests/dump*.txt; do
    echo "********** Testing $i **********"
    cargo run < $i | python3 io-tests/tool.py -s
done
