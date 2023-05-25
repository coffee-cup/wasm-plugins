mkdir -p handlers

# cargo build --package hello-handler --target wasm32-wasi
# wasm-tools component new ./target/wasm32-wasi/debug/hello_handler.wasm \
#     -o handlers/hello-handler.wasm --adapt ./wasi_snapshot_preview1.wasm

cargo build --package hello-handler --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/debug/hello_handler.wasm -o handlers/hello-handler.wasm

echo "Created wasm handlers"
