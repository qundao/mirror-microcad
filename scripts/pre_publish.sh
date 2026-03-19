#!/bin/bash

rustup update
cargo update
cargo clippy
source ./scripts/delete_test_results.sh

cargo test

source ./scripts/update_books.sh

echo "Pre-publish script run successfully. Do not forgot to run 'git tag' before publish!"
