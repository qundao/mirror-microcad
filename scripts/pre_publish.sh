#!/bin/bash

rustup update
cargo update
source ./scripts/delete_test_results.sh

source ./scripts/update_copyright.sh
source ./scripts/update_md_banner.sh
cargo test

source ./scripts/update_books.sh

echo "Pre-publish script run successfully. Do not forgot to run 'git tag' before publish!"
