gen:
    flutter pub get
    flutter_rust_bridge_codegen generate
    
clean:
    flutter clean
    cd rust && cargo clean

test:
    ./compile.native.sh
    flutter test
