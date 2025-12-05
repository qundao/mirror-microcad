#!/bin/bash

source ./scripts/delete_test_results.sh

source ./scripts/update_copyright.sh
source ./scripts/update_md_banner.sh
cargo test

source ./scripts/update_books.sh

