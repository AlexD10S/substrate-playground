# Examples Substrate Scripts

Collection of different Substrate examples/scripts I have build to showcase/test different Substrate functionalities.

Examples:
- 1. Pay fees with no-native asset in a Substrate based chain. [How to run it](#fees_asset_substrate)
- 2. Pay fees with no-native asset in a AssetHub chain. [How to run it](#fees_asset_asset_hub)
- 3. SCALE Encode a particular type in Rococo, useful for Rococo sudo tasks [How to run it](#encode_value_rococo)

Build:
```shell
cargo build
```

## Pay fees with no-native asset.
Two examples of showcase how to pay fees with a no-native asset, it depends on the configuration of the `pallet_asset_conversion`.
The first example uses the following configuration: https://github.com/paritytech/polkadot-sdk/blob/master/substrate/bin/node/runtime/src/lib.rs#L1622

While the second example uses the following: https://github.com/paritytech/polkadot-sdk/blob/master/cumulus/parachains/runtimes/assets/asset-hub-westend/src/lib.rs#L304

See below the explanation of how to run both examples:

<a id="fees_asset_substrate"></a>
### Pay fees with no-native asset in a Substrate chain.
Run locally your Substrate network. In this folder there is an example to set up an environment to do the test for a Substrate solo chain.

```sh
./target/release/substrate-node --dev 
```

Generate the metadata
```shell
subxt metadata --url http://127.0.0.1:9944 -f bytes > src/metadata/local_substrate_metadata.scale   
```

Run the script
```shell
cargo run fees-asset-substrate
```


<a id="fees_asset_asset_hub"></a>
### Pay fees with no-native asset in a AssetHub chain.
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
subxt metadata --url http://127.0.0.1:9944 -f bytes > src/metadata/asset_hub_metadata.scale   
```

Run the script
```shell
cargo run fees-asset-asset-hub
```


<a id="encode_value_rococo"></a>
### Example of how to Scale Encode a type for Rococo.
This is useful for me for Rococo sudo tasks, it can be useful for you for some other tasks or jsut for learning purposes. 
It encodes a specific type, but you can change it for the type you want to encode.

Generate the metadata
```shell
subxt metadata --url https://rococo-rpc.polkadot.io:443 -f bytes >  src/metadata/rococo_metadata.scale
```
Change the metadata if you want to use other networks

Run the script
```shell
cargo run encode-value-rococo
```