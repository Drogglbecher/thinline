set -e
pushd ~

if [ "${TRAVIS_OS_NAME}" == "osx" ]; then
    rvm get head || true
fi

function llvm_linux_target() {
    if [ "$1" == "5.0" ]; then
        echo "linux-x86_64-ubuntu14.04"
    else
        echo "x86_64-linux-gnu-ubuntu-14.04"
    fi
}

function llvm_download() {
    export LLVM=clang+llvm-${LLVM_VERSION}.0-$1
    export LLVM_DIRECTORY="$HOME/.llvm/${LLVM}"

    if [ -d "${LLVM_DIRECTORY}" ]; then
        echo "Using cached LLVM download for ${LLVM}..."
    else
        wget http://llvm.org/releases/${LLVM_VERSION}.0/${LLVM}.tar.xz --debug --verbose
        mkdir -p "${LLVM_DIRECTORY}"
        tar xf ${LLVM}.tar.xz -C "${LLVM_DIRECTORY}" --strip-components=1
    fi

    export LLVM_CONFIG_PATH="${LLVM_DIRECTORY}/bin/llvm-config"
}

if [ "${TRAVIS_OS_NAME}" == "linux" ]; then
    llvm_download `llvm_linux_target ${LLVM_VERSION}`
    export LD_LIBRARY_PATH="${LLVM_DIRECTORY}/lib":$LD_LIBRARY_PATH
else
    llvm_download x86_64-apple-darwin
    cp "${LLVM_DIRECTORY}/lib/libclang.dylib" /usr/local/lib/libclang.dylib
    export DYLD_LIBRARY_PATH="${LLVM_DIRECTORY}/lib":$DYLD_LIBRARY_PATH
fi

popd
set +e
