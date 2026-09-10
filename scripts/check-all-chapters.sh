#!/usr/bin/env bash
set -euo pipefail

chapters=(
    "03:chapter03-common-programming-concepts"
    "04:chapter04-understanding-ownership"
    "05:chapter05-using-structs-to-structure-related-data"
    "06:chapter06-enums-and-pattern-matching"
    "07:chapter07-managing-growing-projects-with-packages-crates-and-modules"
    "08:chapter08-common-collections"
    "09:chapter09-error-handling"
    "10:chapter10-generic-types-traits-and-lifetimes"
    "11:chapter11-writing-automated-tests"
    "12:chapter12-an-i-o-project-building-a-command-line-program"
    "13:chapter13-functional-language-features-iterators-and-closures"
    "14:chapter14-more-about-cargo-and-crates-io"
    "15:chapter15-smart-pointers"
    "16:chapter16-fearless-concurrency"
    "17:chapter17-fundamentals-of-asynchronous-programming"
    "18:chapter18-object-oriented-programming-features-of-rust"
    "19:chapter19-patterns-and-matching"
)

for chapter in "${chapters[@]}"; do
    number=${chapter%%:*}
    package=${chapter#*:}
    output=$(cargo run --quiet --locked -p "$package")
    heading="Chapter ${number}:"

    if [[ "$output" != "$heading"* ]]; then
        printf 'ERROR: %s output does not start with %q\n' "$package" "$heading" >&2
        exit 1
    fi
done

printf 'All 17 chapter binaries exited successfully with the expected heading.\n'
