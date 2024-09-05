# HTTP Server powered by axum

## Build

```bash
cargo build --release
```

## Run

```bash
wasmedge ./target/wasm32-wasip1/release/test-axum.wasm
```

## Test

- `/help` endpoints

  ```bash
  curl --location 'http://localhost:8080/echo' \
    --header 'Content-Type: application/json' \
    --data '"wasmedge"'
  ```
