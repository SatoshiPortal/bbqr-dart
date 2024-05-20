gen:
    flutter pub get
    ./codegen.sh

clean:
    flutter clean
    cd rust && cargo clean

test:
    ./compile.native.sh
    flutter test
