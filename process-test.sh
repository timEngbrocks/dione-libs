#!/bin/bash

cargo build --target=x86_64-pc-windows-msvc --package process
cargo build --target=x86_64-pc-windows-msvc --package process-test

export BINARY_PATH=./target/x86_64-pc-windows-msvc/debug/process-test.exe

./target/x86_64-pc-windows-msvc/debug/process-test.exe root

unset BINARY_PATH