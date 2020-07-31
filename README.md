# Shuffler
Shuffler uses StarkWare's [VeeDo](https://github.com/starkware-libs/veedo) VDF to seed a seedable RNG, and shuffle a randomisable list of items.

# Beacon Contract
We use the [Beacon Contract](https://ropsten.etherscan.io/address/0x79474439753C7c70011C3b00e06e559378bAD040) deployed on Ethereum's Rinkeby testnet to pull randomness seeds from.

# Get Started
* Environment variables (file `.env`)
```
export PROVIDER_WSS_URL=wss://ropsten.infura.io/ws/v3/xyz
export PRIVATE_KEY=abc
```
* Compile, source the secrets and run
```
$ cargo build
$ source .env
$ ./target/debug/shuffler
```
