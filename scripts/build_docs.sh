#!/bin/sh

# Extract version automatically from Cargo.toml
VERSION="v$(awk -F '"' '/^version/ {print $2; exit}' Cargo.toml)"

SOURCE_DIR=./books
DEST_DIR=`pwd`/target/docs/

# 1. Provide a safe default fallback if empty
DEST_DIR="${DEST_DIR:-./dist}"

# 2. Safety Check: Halt completely if the variable is blank or just a slash
if [ -z "${DEST_DIR}" ] || [ "${DEST_DIR}" = "/" ] || [ "${DEST_DIR}" = "." ]; then
    echo "❌ ERROR: DEST_DIR is empty, invalid, or pointing to system roots. Aborting wipe!" >&2
    exit 1
fi

# 3. Double-quoted target deletion
if [ -d "${DEST_DIR}" ]; then
    echo "🧹 Safely clearing previous build assets in ${DEST_DIR}..."
    rm -rf "${DEST_DIR:?}"/*
fi

VERSION_DIR="${DEST_DIR}/${VERSION}"
LATEST_DIR="${DEST_DIR}/latest"

echo "Building µcad documentation for ${VERSION} in ${DEST_DIR}"

echo "⚙️ Processing landing page..."
mkdir -p "$DEST_DIR"

# 2. Copy index.html to destination and swap out the placeholder
# (Works on both Linux and macOS)
sed "s/{{VERSION}}/${VERSION}/g" ${SOURCE_DIR}/index.html > "${DEST_DIR}/index.html"

# Copy logo
cp ${SOURCE_DIR}/logo.png ${DEST_DIR}/

# Build books
mdbook build ${SOURCE_DIR}/language -d ${VERSION_DIR}/language
mdbook build ${SOURCE_DIR}/tests -d ${VERSION_DIR}/tests
mdbook build ${SOURCE_DIR}/tutorials -d ${VERSION_DIR}/tutorials
mdbook build ${SOURCE_DIR}/examples -d ${VERSION_DIR}/examples

MDBOOK_DIR=./target/mdbook

## Generate MD book for builtin library
cargo run -- doc -g mdbook --output ${MDBOOK_DIR}/builtin __builtin
mdbook build ${MDBOOK_DIR}/builtin -d ${VERSION_DIR}/builtin

# Generate books for standard library
cargo run -- doc -g mdbook --output ${MDBOOK_DIR}/std --no-std crates/std/lib/std
mdbook build ${MDBOOK_DIR}/std -d ${VERSION_DIR}/std

# 1. Clean up any existing file, directory, or symlink named 'latest'
if [ -L "${LATEST_DIR}" ] || [ -e "${LATEST_DIR}" ]; then
    rm -rf "${LATEST_DIR}"
fi

# 2. Create the relative symlink
# Syntax: ln -s <TARGET> <LINK_NAME>
ln -s "./${VERSION}" "${LATEST_DIR}"