#!/usr/bin/env bash

cargo build --release --bin bad

while true
do
	./target/release/bad
done
