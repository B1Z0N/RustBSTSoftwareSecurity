#!/bin/sh

cargo afl fuzz -i fuzz/seeds -o fuzz/output target/debug/fuzz
