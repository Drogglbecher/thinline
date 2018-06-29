set -e

export CARGO_TARGET_DIR=`pwd`/target

RUST_BACKTRACE=1 cargo test --verbose --features $CLANG_VERSION -- --nocapture
RUST_BACKTRACE=1 cargo test --verbose --features "$CLANG_VERSION runtime" -- --nocapture

cargo doc --no-deps
