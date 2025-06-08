#!/bin/bash

cargo build 

g++ -std=c++23 -I target/include  src/main.cpp -L target/debug -l rust_interop -o target/debug/app

export LD_LIBRARY_PATH=target/debug
target/debug/app