## Manifesto Contract

Execute the following command to build the contract and generate .wasm file in the `\artifacts` folder

````
cargo build
````

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.10.2
```


### Deployment instructions
`scripts` folder contains the scripts to deploy the contract on localterra / tequila testnet