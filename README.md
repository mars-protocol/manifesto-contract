# manifesto-contract

- This repository contains the source code and the deployment script for the smart contract implementing Mars Manifesto on the Terra blockchain.

Martian Date follows the [Darian Calender](https://en.wikipedia.org/wiki/Darian_calendar) and the calculations for the Martian Time has been referenced from https://marsclock.com/

## Development

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
cd manifesto
cargo unit-test
cargo integration-test
```

### Compiling & Production

Execute the following commands to build the contract and generate .wasm file in the `\artifacts` folder

```
cd manifesto
cargo build
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.11.3
```

## Deployment

You can execute the deployment script available in the `/scripts` folder to deploy the contract on Localterra / tequila-0004 testnet.

### Deploying on LocalTerra

- make sure you have the LocalTerra running
- execute the following commands

```
cd script
npm install
node localterra_script.js
```

### Deploying on Tequila testnet

- execute the following command

```
cd script
npm install
node testnet_script.js
```

# Astroport Periphery

This repo contains the contracts which facilitates ASTRO tokens airdrop, lockdrop and LP Bootstrapping via auction during the intital protocol launch.

## Development

### Dependencies

- Rust v1.44.1+
- `wasm32-unknown-unknown` target
- Docker
- [LocalTerra](https://github.com/terra-project/LocalTerra)
- Node.js v16

### Envrionment Setup

1. Install `rustup` via https://rustup.rs/

2. Add `wasm32-unknown-unknown` target

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Install Node libraries required for testing:

```bash
cd scripts
npm install
```

### Compile

Make sure the current working directory is set to the root directory of this repository, then

```bash
cargo build
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.11.3
```

## License

TBD
