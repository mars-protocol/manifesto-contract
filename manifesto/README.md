## MANIFESTO : CONTRACT

- This Folder contains the source code for the smart contract implementing Mars Manifesto on the Terra blockchain.


### Environment Setup
- Rust v1.44.1+
- wasm32-unknown-unknown target
- Docker

1. Install rustup via https://rustup.rs/
2. Run the following:

```
rustup default stable
rustup target add wasm32-unknown-unknown
Make sure Docker is installed
```

### Unit / Integration Tests
You can run:
```
cargo unit-test
cargo integration-test
```


### Compiling & Production
Execute the following commands to build the contract and generate .wasm file in the `\artifacts` folder
```
cargo build
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.10.2
```