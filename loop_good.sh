#!/usr/bin/env bash

cargo build --release --bin good

while true
do
	./target/release/good
done
