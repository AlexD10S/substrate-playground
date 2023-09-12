# Useful Substrate Scripts

Build:
```shell
cargo build
```

### Pay fees with no-native asset.
Run locally your Substrate network. In this folder there is an example to set up an environment to do the test for a Asset Hub parachain, in zombienet.

For Zombienet we specify the networks we want to run in the .toml file: `zombienet/testing-network.toml`.

For that we need to create some binaries and add them in the `zombienet/bin folder`:
- `polkadot` (which you can download from [the releases](https://github.com/paritytech/polkadot/releases))
- `polkadot-parachain` (which you will build from [cumulus](https://github.com/paritytech/cumulus))

You have to install Zombienet using  the instructions from the repo: https://github.com/paritytech/zombienet/ 
```sh
cd src/zombineet
chmod +x zombienet-macos
```

In the repo zmobienet we have the .toml file with the networks we will run with zombienet, run it:
```sh
./zombienet-macos spawn testing-network.toml -p native
```

Generate the metadata
```shell
subxt metadata --url http://127.0.0.1:9944 -f bytes > src/metadata/local_metadata.scale   
```

Run the script
```shell
cargo run
```
