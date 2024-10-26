#!/usr/bin/env bash
nohup cargo run --package web_quick_start --bin example_app --features dev > output.log 2> error.log &