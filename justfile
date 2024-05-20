gen:
    export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev)/include"
    flutter pub get
    ./codegen.sh

clean:
    flutter clean
    cd rust && cargo clean

test:
    ./compile.native.sh
    flutter test
