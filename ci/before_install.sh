set -e

if [ "${TRAVIS_OS_NAME}" == "osx" ]; then
    rvm get head || true
fi

function llvm_download() {
    export LLVM_VERSION_TRIPLE="6.0.0"
    export LLVM=clang+llvm-${LLVM_VERSION_TRIPLE}-x86_64-$1
    export LLVM_DIRECTORY="$HOME/.llvm/${LLVM}"

    if [ -d "${LLVM_DIRECTORY}" ]; then
        echo "Using cached LLVM download for ${LLVM}..."
    else
        wget http://llvm.org/releases/${LLVM_VERSION_TRIPLE}/${LLVM}.tar.xz
        mkdir llvm
        tar xf ${LLVM}.tar.xz -C "${LLVM_DIRECTORY}" --strip-components=1
    fi

    export LLVM_CONFIG_PATH="${LLVM_DIRECTORY}/bin/llvm-config"
}

if [ "${TRAVIS_OS_NAME}" == "linux" ]; then
    llvm_download linux-gnu-ubuntu-14.04
    export LD_LIBRARY_PATH="${LLVM_DIRECTORY}/lib":$LD_LIBRARY_PATH
else
    llvm_download x86_64-apple-darwin
    export DYLD_LIBRARY_PATH="${LLVM_DIRECTORY}/lib":$DYLD_LIBRARY_PATH
fi

