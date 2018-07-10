if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    if [[ "$TRAVIS_RUST_VERSION" == "stable" ]]; then
        travis-cargo doc-upload;
    elif [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
        cargo kcov --print-install-kcov-sh | sh;
        cargo kcov --coveralls --kcov ./kcov-33/build/src/kcov;
    fi
fi
