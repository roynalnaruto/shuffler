pub use beaconcontract_mod::*;
mod beaconcontract_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::JsonRpcClient,
        signers::{Client, Signer},
    };
    #[doc = "BeaconContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BEACONCONTRACT_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ( "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"verifierAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"n_iters\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"randomness\",\"type\":\"bytes32\"}],\"name\":\"LogNewRandomness\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"getLatestRandomness\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"}],\"name\":\"getRandomness\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"n_iterations\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256[5]\",\"name\":\"proofPublicInput\",\"type\":\"uint256[5]\"}],\"name\":\"registerNewRandomness\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]" ) . expect ( "invalid abi" )
    });
    #[derive(Clone)]
    pub struct BeaconContract<P, S>(Contract<P, S>);
    impl<P, S> std::ops::Deref for BeaconContract<P, S> {
        type Target = Contract<P, S>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<P: JsonRpcClient, S: Signer> std::fmt::Debug for BeaconContract<P, S> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BeaconContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, P: JsonRpcClient, S: Signer> BeaconContract<P, S> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>, C: Into<Arc<Client<P, S>>>>(address: T, client: C) -> Self {
            let contract = Contract::new(address.into(), BEACONCONTRACT_ABI.clone(), client.into());
            Self(contract)
        }
        #[doc = "Calls the contract's `n_iterations` (0xc06d9475) function"]
        pub fn n_iterations(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([192, 109, 148, 117], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(&self) -> ContractCall<P, S, Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerNewRandomness` (0xd92f1280) function"]
        pub fn register_new_randomness(
            &self,
            block_number: U256,
            block_hash: [u8; 32],
            proof_public_input: [U256; 5],
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash(
                    [217, 47, 18, 128],
                    (block_number, block_hash, proof_public_input),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRandomness` (0x453f4f62) function"]
        pub fn get_randomness(&self, block_number: U256) -> ContractCall<P, S, [u8; 32]> {
            self.0
                .method_hash([69, 63, 79, 98], block_number)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLatestRandomness` (0xf4bebd7a) function"]
        pub fn get_latest_randomness(&self) -> ContractCall<P, S, (U256, [u8; 32])> {
            self.0
                .method_hash([244, 190, 189, 122], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `LogNewRandomness` event"]
        pub fn log_new_randomness_filter(&self) -> Event<P, LogNewRandomnessFilter> {
            self.0
                .event("LogNewRandomness")
                .expect("event not found (this should never happen)")
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct LogNewRandomnessFilter {
        pub block_number: U256,
        pub randomness: [u8; 32],
    }
    impl LogNewRandomnessFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                114, 218, 216, 185, 70, 79, 136, 162, 151, 44, 142, 74, 5, 74, 69, 137, 77, 54,
                244, 69, 51, 9, 116, 252, 40, 29, 61, 90, 20, 154, 206, 70,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`LogNewRandomness(uint256,bytes32)`"]
        pub const fn abi_signature() -> &'static str {
            "LogNewRandomness(uint256,bytes32)"
        }
    }
    impl Detokenize for LogNewRandomnessFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 2 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    2,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let block_number =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let randomness =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(LogNewRandomnessFilter {
                block_number,
                randomness,
            })
        }
    }
}
