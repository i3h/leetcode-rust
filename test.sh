#!/bin/bash
RUSTFLAGS=-Awarnings cargo test test_$1 --lib -- --nocapture
