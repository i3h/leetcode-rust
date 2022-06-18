#!/bin/bash
cargo test test_$1 --lib -- --nocapture
