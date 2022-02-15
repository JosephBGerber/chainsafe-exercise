## ChainSafe Exercise

This is an ethereum contract and associated Rust program that allows users to declare 
an icon that describes themselves. There is no limitation on what users can set as their
icon. Getting a users icon will also open that icon using your system's default opener for the
given file (e.g. an icon that is a png will open in your default image viewer).

## Testing

### Starting an ethereum network and deploying the contract.
Start an instance of Ganache and ensure that the RPC server is running on localhost:7545.

Navigate to the ethereum directory and run `yarn truffle migrate`. This will build and deploy
the ethereum contract.

### Starting a IPFS daemon.
Start an instance of the ipfs daemon. The simplest method to do this is to run `ipfs daemon`
in a console and leave that running in the background.

### Building
Before this tool can be used, it has to be built. There are no special instructions for this.
Run `cargo build` to build the tool.

### Unit Testing
Run `cargo test` to check that all unit tests pass.

Now we can start testing the application. In this section, I will be using `cargo run ...` to
run my instance of the application.

### Setting an icon.
Running `cargo run -- set <ethereum address> <path to icon>` will upload your icon to IPFS and then
store that icon's CID within an ethereum contract.

### Getting an icon.
Running `cargo run -- get <your ethereum address> <target's ethereum address>` will open your 
target's icon. Omitting the target field, `cargo run -- get <your etherum address>`, will
open your icon.



## Examples

### Setting my icon to rats.gif.

`cargo run -- set 0xb40bCa35AE4FB0518ED4fd88853a08d4Dd98023C rats.gif`


### Getting my icon.

`cargo run -- get 0xb40bCa35AE4FB0518ED4fd88853a08d4Dd98023C`
