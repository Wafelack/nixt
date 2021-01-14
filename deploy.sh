#!/usr/bin/env bash
set -euo pipefail

cargo build --release
cp ./target/release/nixt .
tar -czf nixt.tar.gz nixt std/
rm nixt

