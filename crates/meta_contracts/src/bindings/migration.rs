pub use migration::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod migration {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"last_completed_migration\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"completed\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCompleted\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static MIGRATION_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        25,
        22,
        51,
        23,
        144,
        85,
        52,
        128,
        21,
        97,
        0,
        34,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        1,
        112,
        128,
        97,
        0,
        50,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        65,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        68,
        93,
        240,
        172,
        20,
        97,
        0,
        70,
        87,
        128,
        99,
        141,
        165,
        203,
        91,
        20,
        97,
        0,
        98,
        87,
        128,
        99,
        253,
        172,
        213,
        118,
        20,
        97,
        0,
        141,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        79,
        96,
        1,
        84,
        129,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        84,
        97,
        0,
        117,
        144,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        129,
        86,
        91,
        96,
        64,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        144,
        145,
        22,
        129,
        82,
        96,
        32,
        1,
        97,
        0,
        89,
        86,
        91,
        97,
        0,
        160,
        97,
        0,
        155,
        54,
        96,
        4,
        97,
        1,
        33,
        86,
        91,
        97,
        0,
        162,
        86,
        91,
        0,
        91,
        96,
        0,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        51,
        20,
        97,
        1,
        28,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        51,
        96,
        36,
        130,
        1,
        82,
        127,
        84,
        104,
        105,
        115,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        32,
        105,
        115,
        32,
        114,
        101,
        115,
        116,
        114,
        105,
        99,
        116,
        101,
        100,
        32,
        116,
        111,
        32,
        116,
        96,
        68,
        130,
        1,
        82,
        114,
        52,
        50,
        144,
        49,
        183,
        183,
        58,
        57,
        48,
        177,
        186,
        19,
        185,
        144,
        55,
        187,
        183,
        50,
        185,
        96,
        105,
        27,
        96,
        100,
        130,
        1,
        82,
        96,
        132,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        253,
        91,
        96,
        1,
        85,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        1,
        51,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        63,
        42,
        118,
        249,
        28,
        85,
        176,
        41,
        44,
        2,
        56,
        198,
        91,
        239,
        65,
        245,
        147,
        145,
        127,
        48,
        141,
        243,
        196,
        78,
        178,
        183,
        175,
        35,
        146,
        148,
        239,
        148,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static MIGRATION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        65,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        68,
        93,
        240,
        172,
        20,
        97,
        0,
        70,
        87,
        128,
        99,
        141,
        165,
        203,
        91,
        20,
        97,
        0,
        98,
        87,
        128,
        99,
        253,
        172,
        213,
        118,
        20,
        97,
        0,
        141,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        79,
        96,
        1,
        84,
        129,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        91,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        0,
        84,
        97,
        0,
        117,
        144,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        129,
        86,
        91,
        96,
        64,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        144,
        145,
        22,
        129,
        82,
        96,
        32,
        1,
        97,
        0,
        89,
        86,
        91,
        97,
        0,
        160,
        97,
        0,
        155,
        54,
        96,
        4,
        97,
        1,
        33,
        86,
        91,
        97,
        0,
        162,
        86,
        91,
        0,
        91,
        96,
        0,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        51,
        20,
        97,
        1,
        28,
        87,
        96,
        64,
        81,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        51,
        96,
        36,
        130,
        1,
        82,
        127,
        84,
        104,
        105,
        115,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        32,
        105,
        115,
        32,
        114,
        101,
        115,
        116,
        114,
        105,
        99,
        116,
        101,
        100,
        32,
        116,
        111,
        32,
        116,
        96,
        68,
        130,
        1,
        82,
        114,
        52,
        50,
        144,
        49,
        183,
        183,
        58,
        57,
        48,
        177,
        186,
        19,
        185,
        144,
        55,
        187,
        183,
        50,
        185,
        96,
        105,
        27,
        96,
        100,
        130,
        1,
        82,
        96,
        132,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        253,
        91,
        96,
        1,
        85,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        1,
        51,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        63,
        42,
        118,
        249,
        28,
        85,
        176,
        41,
        44,
        2,
        56,
        198,
        91,
        239,
        65,
        245,
        147,
        145,
        127,
        48,
        141,
        243,
        196,
        78,
        178,
        183,
        175,
        35,
        146,
        148,
        239,
        148,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static MIGRATION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Migration<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Migration<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Migration<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Migration<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Migration<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Migration)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Migration<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MIGRATION_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MIGRATION_ABI.clone(),
                MIGRATION_BYTECODE.clone(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `last_completed_migration` (0x445df0ac) function
        pub fn last_completed_migration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([68, 93, 240, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCompleted` (0xfdacd576) function
        pub fn set_completed(
            &self,
            completed: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 172, 213, 118], completed)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Migration<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `last_completed_migration` function with signature `last_completed_migration()` and selector `0x445df0ac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "last_completed_migration", abi = "last_completed_migration()")]
    pub struct LastCompletedMigrationCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `setCompleted` function with signature `setCompleted(uint256)` and selector `0xfdacd576`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setCompleted", abi = "setCompleted(uint256)")]
    pub struct SetCompletedCall {
        pub completed: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MigrationCalls {
        LastCompletedMigration(LastCompletedMigrationCall),
        Owner(OwnerCall),
        SetCompleted(SetCompletedCall),
    }
    impl ::ethers::core::abi::AbiDecode for MigrationCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <LastCompletedMigrationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LastCompletedMigration(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <SetCompletedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetCompleted(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MigrationCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::LastCompletedMigration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetCompleted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MigrationCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LastCompletedMigration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCompleted(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LastCompletedMigrationCall> for MigrationCalls {
        fn from(value: LastCompletedMigrationCall) -> Self {
            Self::LastCompletedMigration(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MigrationCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<SetCompletedCall> for MigrationCalls {
        fn from(value: SetCompletedCall) -> Self {
            Self::SetCompleted(value)
        }
    }
    ///Container type for all return fields from the `last_completed_migration` function with signature `last_completed_migration()` and selector `0x445df0ac`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LastCompletedMigrationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
