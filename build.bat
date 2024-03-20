@Echo off
del /s /q dist
mkdir dist
cargo build --release
move target\wasm32-unknown-unknown\release\simulation.wasm dist\simulation.wasm
wasm-bindgen dist\simulation.wasm --out-dir dist --target web
copy index.html dist\index.html
copy index.js dist\index.js
copy dist\simulation.js simulation.js
cd dist
ws