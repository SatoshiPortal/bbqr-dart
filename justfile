gen:
    flutter pub get
    ./compile.all.sh
    
clean:
    flutter clean
    cd rust && cargo clean

test:
    ./compile.native.sh
    flutter test
