#!/usr/bin/env sh
set -euo pipefail
toast "$@"
cargo test --features=client,server
