# YANE
Yet Another Nes Emulator or YANE is a NES Emulator made in rust for learning purposes and to have fun doing it

## Running

### Run on desktop
```
cargo run
```
### Compile to web assembly

You'll need wasm-pack for this.

#### Install wasm-pack
```bash
cargo install wasm-pack
```

#### Compile and run server

```bash
wasm-pack build yane-web --target web
# I'm using sfz here, but you can use whatever static file serving program you want
sfz yane-web
```
