# Compile rust
cargo build --release
cp ./target/wasm32-unknown-unknown/release/arduino.wasm ./temp/app.wasm

# Optimize wasm (optional)
wasm-opt -O3 ./temp/app.wasm -o ./temp/app.wasm
wasm-strip ./temp/app.wasm

# Convert to C header, cd into temp dir so that the dir name is not part of the variables in app.wasm.h
cd temp
xxd -i app.wasm > app.wasm.h
cd ..

# Compile arduino
arduino-cli compile --fqbn arduino:sam:arduino_due_x  arduino 

# Upload to arduino
arduino-cli upload -p $1 --fqbn arduino:sam:arduino_due_x  arduino
