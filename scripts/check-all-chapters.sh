#!/usr/bin/env bash
set -euo pipefail

for number in $(seq -w 3 19); do
    package="chapter${number}"
    output=$(cargo run --quiet --locked -p "$package")
    heading="Chapter ${number}:"

    if [[ "$output" != "$heading"* ]]; then
        printf 'ERROR: %s output does not start with %q\n' "$package" "$heading" >&2
        exit 1
    fi
done

printf 'All 17 chapter binaries exited successfully with the expected heading.\n'
