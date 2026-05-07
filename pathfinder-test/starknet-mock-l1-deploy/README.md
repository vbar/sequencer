This is a tool to get a "bootstrapped" L1 state that has a mock Starknet
contract deployed.

When running `cargo run` the tool starts up
[Anvil](https://getfoundry.sh/anvil/overview/), deploys a mock Starknet
contract then saves Anvil state into a file and exits. This state file can then
be used to start up Anvil with a state that can be used for bootstrapping
Apollo.

Example usage:

```sh
cargo run
gzip -dc ./anvil_state.bin >anvil_state.txt
anvil --load-state ./anvil_state.txt
```

The tool requires Anvil to be installed.

