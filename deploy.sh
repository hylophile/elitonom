cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/elitonom.wasm
wasm-opt -Oz out/elitonom_bg.wasm -o out/elitonom_bg.wasm
rclone copy -LP ./out mve:web/web/monotile/game
