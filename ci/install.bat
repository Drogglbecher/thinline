curl -sSf https://static.rust-lang.org/dist/rust-nightly-x86_64-pc-windows-gnu.exe -o rust.exe
rust.exe /VERYSILENT /NORESTART /DIR="C:\rust"
set PATH=%PATH%;C:\rust\bin

curl -sSf http://releases.llvm.org/%LLVM_VERSION%/LLVM-%LLVM_VERSION%-win64.exe -o LLVM.exe
7z x LLVM.exe -oC:\LLVM
set PATH=%PATH%;C:\LLVM\bin
set LIBCLANG_PATH=C:\LLVM\bin

set PATH=%PATH%;C:\MinGW\msys\1.0\bin;C:\MinGW\bin;C:\tools\MinGW\bin;
