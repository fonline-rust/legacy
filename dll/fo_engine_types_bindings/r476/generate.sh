#!/usr/bin/env sh
set -euo pipefail
export CPLUS_INCLUDE_PATH="D:\\Program Files (x86)\\Microsoft Visual Studio 10.0\\VC\\include"
fo_bindgen_generate
cargo test --features=client,server
