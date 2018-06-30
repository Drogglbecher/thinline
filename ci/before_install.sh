set -e

function llvm_download() {
    # Get the sources
    wget http://releases.llvm.org/${LLVM_VERSION}/llvm-${LLVM_VERSION}.tar.xz
    mkdir llvm-${LLVM_VERSION}
    tar xf llvm-${LLVM_VERSION}.tar.xz -C llvm-${LLVM_VERSION} --strip-components=1

    export LLVM_CONFIG_PATH=$HOME/llvm-${LLVM_VERSION}/bin/llvm-config
}

if [ "${TRAVIS_OS_NAME}" == "linux" ]; then
    llvm_download $LLVM_VERSION
    export LD_LIBRARY_PATH=$HOME/llvm-${LLVM_VERSION}/lib:$LD_LIBRARY_PATH
fi
