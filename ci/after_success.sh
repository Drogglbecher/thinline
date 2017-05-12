if [[ "$TRAVIS_RUST_VERSION" == "stable" ]] && [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    travis-cargo doc-upload;
    cargo kcov --print-install-kcov-sh | sh;
    cargo kcov --coveralls --kcov ./kcov-33/build/src/kcov;
fi
