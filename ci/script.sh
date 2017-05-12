if [ "${TRAVIS_OS_NAME}" == "osx" ]; then
    rvm get head || true
fi

export CARGO_TARGET_DIR=`pwd`/target

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]] && [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    CLANG_VERSION="$CLANG_VERSION clippy"
fi

RUST_BACKTRACE=1 cargo test --verbose --features $CLANG_VERSION -- --nocapture

if [ "${CLANG_VERSION}" \< "clang_3_7" ]; then
    RUST_BACKTRACE=1 cargo test --verbose --features "$CLANG_VERSION static" -- --nocapture
fi

RUST_BACKTRACE=1 cargo test --verbose --features "$CLANG_VERSION runtime" -- --nocapture

cargo doc --no-deps
