set -e

export CARGO_TARGET_DIR=`pwd`/target

RUST_BACKTRACE=1 cargo test --verbose -- --nocapture

cargo doc --no-deps
