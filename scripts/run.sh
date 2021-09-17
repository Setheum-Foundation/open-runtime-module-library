#!/usr/bin/env bash

set -e

COMMAND=$1
shift

set -x

for file in **/Cargo.toml; do
	fi
	cargo $COMMAND --manifest-path "$file" $@;
done

