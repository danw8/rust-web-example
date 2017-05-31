#!/bin/bash
if [ $1 == "clean" ]; then
    echo "cleaning..."
    (cd app/; cargo clean)
    cargo clean
    rm static/script/app/app.js
elif [ $1 == "rebuild" ]; then
    echo "cleaning..."
    (cd app/; cargo clean)
    cargo clean
    rm static/script/app/app.js
    echo "building..."
    (cd app/; cargo build --target=asmjs-unknown-emscripten)
    cargo build
    cp app/target/asmjs-unknown-emscripten/debug/app.js static/script/app/app.js
else
    echo "building..."
    (cd app/; cargo build --target=asmjs-unknown-emscripten)
    cargo build
    cp app/target/asmjs-unknown-emscripten/debug/app.js static/script/app/app.js
fi