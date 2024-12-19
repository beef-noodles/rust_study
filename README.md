

## How to

```sh
rustup target add wasm32-wasi

# Install wasmtime
brew install wasmtime
```

### Build

```sh
cargo build --target wasm32-wasi  --release
```

### Run

```sh
wasmtime target/wasm32-wasi/release/wasi_metrics.wasm 
```

## Ref

- [使用Rust和WASI对应用程序进行监控](https://mp.weixin.qq.com/s/tW02Fibx06_g6S0CBCc8Pg)