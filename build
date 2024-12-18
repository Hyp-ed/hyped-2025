#!/bin/bash

# Run cargo build in the root directory
echo "Building main workspace"
cargo build

# Find first layer of directories in /boards and run cargo build in each
for dir in boards/*/
do
    dir=${dir%*/}
    echo "Building $dir"
    cd $dir && cargo build && cd ../..
done
