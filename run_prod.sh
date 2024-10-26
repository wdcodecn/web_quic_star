#!/usr/bin/env bash
nohup cargo run --release --package web_quick_start --bin example_app > output.log 2> error.log &