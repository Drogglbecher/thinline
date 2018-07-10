pip install 'travis-cargo<0.2' --user --verbose 

export PATH=$HOME/.local/bin:$PATH
export PATH=$HOME/Library/Python/2.7/bin:$PATH

if [ $(cargo install --list | grep cargo-kcov) ]; then
    cargo install cargo-kcov
fi
