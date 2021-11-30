# manifesto and MEDAL NFTs

- This repository contains the source code for the smart contracts implementing Mars Manifesto and Medal NFTs on the Terra blockchain.

This repo contains the following contracts -

- **Manifesto Contract** : Facilitates mainfesto signing which mints a Medal NFT to the signer.

- **MEDAL Token** : Token compatible with cw721 interface and following the opensea's metadata standards. MEDAL tokens are redeemable for physical Medal pins via the https://medal.marsprotocol.io/ interface.

- **R-MEDAL Token** : Token compatible with cw721 interface and following the opensea's metadata standards. Redemeed MEDAL tokens are minted when Medal tokens are redeemed for physical medals pins which burns the Medal tokens.

Martian Date follows the [Darian Calender](https://en.wikipedia.org/wiki/Darian_calendar) and the calculations for the Martian Time has been referenced from https://marsclock.com/

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

3. Install Node libraries required:

```bash
cd scripts
npm install
```

### Compile

Make sure the current working directory is set to the root directory of this repository, then execute the following commands to build the contract and generate .wasm file in the `\artifacts` folder

```bash
cargo build
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.11.3
```

## Deployment

You can execute the deployment script available in the `/scripts` folder to deploy the contract on bombay testnet.

- execute the following command

```
cd scripts
node --loader ts-node/esm deploy_script.ts
```

## License

[GNU General Public License v3.0](https://github.com/astroport-fi/astroport-periphery/blob/main/LICENSE)
