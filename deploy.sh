cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/elitonom.wasm
rclone sync -LP ./out mve:web/monotile/
