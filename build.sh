rm -rf dist
mkdir dist
cargo build --release --target=wasm32-unknown-unknown
mv target/wasm32-unknown-unknown/release/simulation.wasm dist/simulation.wasm
wasm-bindgen dist/simulation.wasm --out-dir dist --target=web
cp index.html dist/index.html
cp index.js dist/index.js
cp index.css dist/styles.css
cp dist/simulation.js simulation.js
cd dist
ws
