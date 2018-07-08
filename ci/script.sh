set -e

export CARGO_TARGET_DIR=`pwd`/target

RUST_BACKTRACE=1 cargo test --verbose --features clang_6_0 -- --nocapture

cargo doc --no-deps
