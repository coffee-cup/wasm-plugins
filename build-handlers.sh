mkdir -p handlers

# TODO: Get wasi components working
# cargo build --package hello-handler --target wasm32-wasi
# wasm-tools component new ./target/wasm32-wasi/debug/hello_handler.wasm \
#     -o handlers/hello-handler.wasm --adapt ./wasi_snapshot_preview1.wasm

echo "Compiling handlers..."
cargo build --package hello-handler --target wasm32-unknown-unknown

echo "Creating WASM components..."
wasm-tools component new ./target/wasm32-unknown-unknown/debug/hello_handler.wasm -o handlers/hello-handler.wasm

echo "Done!"
