///Module containing a contract's types and functions.
/**

```solidity
library BLS {
    struct G1Point { bytes32 x_a; bytes32 x_b; bytes32 y_a; bytes32 y_b; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BLS {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct G1Point { bytes32 x_a; bytes32 x_b; bytes32 y_a; bytes32 y_b; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        #[allow(missing_docs)]
        pub x_a: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub x_b: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub y_a: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub y_b: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.x_a, value.x_b, value.y_a, value.y_b)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    x_a: tuple.0,
                    x_b: tuple.1,
                    y_a: tuple.2,
                    y_b: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.x_a),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.x_b),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.y_a),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.y_b),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for G1Point {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "G1Point(bytes32 x_a,bytes32 x_b,bytes32 y_a,bytes32 y_b)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x_a)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x_b)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y_a)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y_b)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.x_a)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.x_b)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.y_a)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.y_b)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.x_a, out);
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.x_b, out);
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.y_a, out);
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.y_b, out);
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

    See the [wrapper's documentation](`BLSInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> BLSInstance<P, N> {
        BLSInstance::<P, N>::new(address, __provider)
    }
    /**A [`BLS`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BLS`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BLSInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BLSInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BLSInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BLSInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

        See the [wrapper's documentation](`BLSInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> BLSInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BLSInstance<P, N> {
            BLSInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BLSInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BLSInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library ISlasher {
    struct Commitment { uint64 commitmentType; bytes payload; bytes32 requestHash; address slasher; }
    struct Delegation { BLS.G1Point proposer; BLS.G1Point delegate; address committer; uint64 slot; bytes metadata; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISlasher {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct Commitment { uint64 commitmentType; bytes payload; bytes32 requestHash; address slasher; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Commitment {
        #[allow(missing_docs)]
        pub commitmentType: u64,
        #[allow(missing_docs)]
        pub payload: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub requestHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub slasher: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Commitment> for UnderlyingRustTuple<'_> {
            fn from(value: Commitment) -> Self {
                (
                    value.commitmentType,
                    value.payload,
                    value.requestHash,
                    value.slasher,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Commitment {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    commitmentType: tuple.0,
                    payload: tuple.1,
                    requestHash: tuple.2,
                    slasher: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Commitment {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Commitment {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.commitmentType),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.payload,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestHash),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.slasher,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Commitment {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Commitment {
            const NAME: &'static str = "Commitment";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Commitment(uint64 commitmentType,bytes payload,bytes32 requestHash,address slasher)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.commitmentType,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.payload,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.requestHash)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.slasher,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Commitment {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.commitmentType,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.payload,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestHash,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.slasher,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.commitmentType,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.payload,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestHash,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slasher,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct Delegation { BLS.G1Point proposer; BLS.G1Point delegate; address committer; uint64 slot; bytes metadata; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Delegation {
        #[allow(missing_docs)]
        pub proposer: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub delegate: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub committer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub slot: u64,
        #[allow(missing_docs)]
        pub metadata: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            BLS::G1Point,
            BLS::G1Point,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BLS::G1Point as alloy::sol_types::SolType>::RustType,
            <BLS::G1Point as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            u64,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Delegation> for UnderlyingRustTuple<'_> {
            fn from(value: Delegation) -> Self {
                (
                    value.proposer,
                    value.delegate,
                    value.committer,
                    value.slot,
                    value.metadata,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Delegation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    proposer: tuple.0,
                    delegate: tuple.1,
                    committer: tuple.2,
                    slot: tuple.3,
                    metadata: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Delegation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Delegation {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.proposer),
                    <BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.delegate),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.committer,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.slot,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.metadata,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Delegation {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Delegation {
            const NAME: &'static str = "Delegation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Delegation(G1Point proposer,G1Point delegate,address committer,uint64 slot,bytes metadata)",
                )
            }
            #[inline]
            fn eip712_components()
            -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components.push(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proposer,
                        )
                        .0,
                    <BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.committer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.slot)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.metadata,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Delegation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proposer,
                    )
                    + <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegate,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.committer,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.slot)
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.metadata,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proposer,
                    out,
                );
                <BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegate,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.committer,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slot,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.metadata,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISlasher`](self) contract instance.

    See the [wrapper's documentation](`ISlasherInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ISlasherInstance<P, N> {
        ISlasherInstance::<P, N>::new(address, __provider)
    }
    /**A [`ISlasher`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISlasher`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISlasherInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ISlasherInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISlasherInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ISlasherInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISlasher`](self) contract instance.

        See the [wrapper's documentation](`ISlasherInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> ISlasherInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISlasherInstance<P, N> {
            ISlasherInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ISlasherInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ISlasherInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library BLS {
    struct G1Point {
        bytes32 x_a;
        bytes32 x_b;
        bytes32 y_a;
        bytes32 y_b;
    }
}

library ISlasher {
    struct Commitment {
        uint64 commitmentType;
        bytes payload;
        bytes32 requestHash;
        address slasher;
    }
    struct Delegation {
        BLS.G1Point proposer;
        BLS.G1Point delegate;
        address committer;
        uint64 slot;
        bytes metadata;
    }
}

interface DummySlasher {
    function SLASH_AMOUNT_WEI() external view returns (uint256);
    function slash(ISlasher.Delegation memory delegation, ISlasher.Commitment memory commitment, address committer, bytes memory evidence, address challenger) external returns (uint256 slashAmountWei);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "SLASH_AMOUNT_WEI",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "slash",
    "inputs": [
      {
        "name": "delegation",
        "type": "tuple",
        "internalType": "struct ISlasher.Delegation",
        "components": [
          {
            "name": "proposer",
            "type": "tuple",
            "internalType": "struct BLS.G1Point",
            "components": [
              {
                "name": "x_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "x_b",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_b",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "delegate",
            "type": "tuple",
            "internalType": "struct BLS.G1Point",
            "components": [
              {
                "name": "x_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "x_b",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_b",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "committer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "slot",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "metadata",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "commitment",
        "type": "tuple",
        "internalType": "struct ISlasher.Commitment",
        "components": [
          {
            "name": "commitmentType",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "payload",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "requestHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "slasher",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "committer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "evidence",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "challenger",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "slashAmountWei",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod DummySlasher {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052670de0b6b3a76400005f553480156019575f80fd5b506101ba806100275f395ff3fe608060405234801561000f575f80fd5b5060043610610034575f3560e01c80634968f6c514610038578063c239ef4214610052575b5f80fd5b6100405f5481565b60405190815260200160405180910390f35b6100406100603660046100cc565b50505f54949350505050565b80356001600160a01b0381168114610082575f80fd5b919050565b5f8083601f840112610097575f80fd5b50813567ffffffffffffffff8111156100ae575f80fd5b6020830191508360208285010111156100c5575f80fd5b9250929050565b5f805f805f8060a087890312156100e1575f80fd5b863567ffffffffffffffff808211156100f8575f80fd5b90880190610160828b03121561010c575f80fd5b90965060208801359080821115610121575f80fd5b908801906080828b031215610134575f80fd5b81965061014360408a0161006c565b95506060890135915080821115610158575f80fd5b5061016589828a01610087565b909450925061017890506080880161006c565b9050929550929550929556fea26469706673582212201e32f9b1df90018e1bfbb86a9e3f00848040f488c64e2986d84b7f68a539e21864736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@Rg\r\xE0\xB6\xB3\xA7d\0\0_U4\x80\x15`\x19W_\x80\xFD[Pa\x01\xBA\x80a\0'_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cIh\xF6\xC5\x14a\08W\x80c\xC29\xEFB\x14a\0RW[_\x80\xFD[a\0@_T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0@a\0`6`\x04a\0\xCCV[PP_T\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x82W_\x80\xFD[\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12a\0\x97W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xAEW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\0\xC5W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a\0\xE1W_\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xF8W_\x80\xFD[\x90\x88\x01\x90a\x01`\x82\x8B\x03\x12\x15a\x01\x0CW_\x80\xFD[\x90\x96P` \x88\x015\x90\x80\x82\x11\x15a\x01!W_\x80\xFD[\x90\x88\x01\x90`\x80\x82\x8B\x03\x12\x15a\x014W_\x80\xFD[\x81\x96Pa\x01C`@\x8A\x01a\0lV[\x95P``\x89\x015\x91P\x80\x82\x11\x15a\x01XW_\x80\xFD[Pa\x01e\x89\x82\x8A\x01a\0\x87V[\x90\x94P\x92Pa\x01x\x90P`\x80\x88\x01a\0lV[\x90P\x92\x95P\x92\x95P\x92\x95V\xFE\xA2dipfsX\"\x12 \x1E2\xF9\xB1\xDF\x90\x01\x8E\x1B\xFB\xB8j\x9E?\0\x84\x80@\xF4\x88\xC6N)\x86\xD8K\x7Fh\xA59\xE2\x18dsolcC\0\x08\x19\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610034575f3560e01c80634968f6c514610038578063c239ef4214610052575b5f80fd5b6100405f5481565b60405190815260200160405180910390f35b6100406100603660046100cc565b50505f54949350505050565b80356001600160a01b0381168114610082575f80fd5b919050565b5f8083601f840112610097575f80fd5b50813567ffffffffffffffff8111156100ae575f80fd5b6020830191508360208285010111156100c5575f80fd5b9250929050565b5f805f805f8060a087890312156100e1575f80fd5b863567ffffffffffffffff808211156100f8575f80fd5b90880190610160828b03121561010c575f80fd5b90965060208801359080821115610121575f80fd5b908801906080828b031215610134575f80fd5b81965061014360408a0161006c565b95506060890135915080821115610158575f80fd5b5061016589828a01610087565b909450925061017890506080880161006c565b9050929550929550929556fea26469706673582212201e32f9b1df90018e1bfbb86a9e3f00848040f488c64e2986d84b7f68a539e21864736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cIh\xF6\xC5\x14a\08W\x80c\xC29\xEFB\x14a\0RW[_\x80\xFD[a\0@_T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0@a\0`6`\x04a\0\xCCV[PP_T\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x82W_\x80\xFD[\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12a\0\x97W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xAEW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\0\xC5W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a\0\xE1W_\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xF8W_\x80\xFD[\x90\x88\x01\x90a\x01`\x82\x8B\x03\x12\x15a\x01\x0CW_\x80\xFD[\x90\x96P` \x88\x015\x90\x80\x82\x11\x15a\x01!W_\x80\xFD[\x90\x88\x01\x90`\x80\x82\x8B\x03\x12\x15a\x014W_\x80\xFD[\x81\x96Pa\x01C`@\x8A\x01a\0lV[\x95P``\x89\x015\x91P\x80\x82\x11\x15a\x01XW_\x80\xFD[Pa\x01e\x89\x82\x8A\x01a\0\x87V[\x90\x94P\x92Pa\x01x\x90P`\x80\x88\x01a\0lV[\x90P\x92\x95P\x92\x95P\x92\x95V\xFE\xA2dipfsX\"\x12 \x1E2\xF9\xB1\xDF\x90\x01\x8E\x1B\xFB\xB8j\x9E?\0\x84\x80@\xF4\x88\xC6N)\x86\xD8K\x7Fh\xA59\xE2\x18dsolcC\0\x08\x19\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `SLASH_AMOUNT_WEI()` and selector `0x4968f6c5`.
    ```solidity
    function SLASH_AMOUNT_WEI() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASH_AMOUNT_WEICall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SLASH_AMOUNT_WEI()`](SLASH_AMOUNT_WEICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASH_AMOUNT_WEIReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SLASH_AMOUNT_WEICall> for UnderlyingRustTuple<'_> {
                fn from(value: SLASH_AMOUNT_WEICall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLASH_AMOUNT_WEICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SLASH_AMOUNT_WEIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: SLASH_AMOUNT_WEIReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLASH_AMOUNT_WEIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SLASH_AMOUNT_WEICall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SLASH_AMOUNT_WEI()";
            const SELECTOR: [u8; 4] = [73u8, 104u8, 246u8, 197u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: SLASH_AMOUNT_WEIReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: SLASH_AMOUNT_WEIReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `slash(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),(uint64,bytes,bytes32,address),address,bytes,address)` and selector `0xc239ef42`.
    ```solidity
    function slash(ISlasher.Delegation memory delegation, ISlasher.Commitment memory commitment, address committer, bytes memory evidence, address challenger) external returns (uint256 slashAmountWei);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashCall {
        #[allow(missing_docs)]
        pub delegation: <ISlasher::Delegation as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub commitment: <ISlasher::Commitment as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub committer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub evidence: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`slash(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),(uint64,bytes,bytes32,address),address,bytes,address)`](slashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashReturn {
        #[allow(missing_docs)]
        pub slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                ISlasher::Delegation,
                ISlasher::Commitment,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISlasher::Delegation as alloy::sol_types::SolType>::RustType,
                <ISlasher::Commitment as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slashCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashCall) -> Self {
                    (
                        value.delegation,
                        value.commitment,
                        value.committer,
                        value.evidence,
                        value.challenger,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        delegation: tuple.0,
                        commitment: tuple.1,
                        committer: tuple.2,
                        evidence: tuple.3,
                        challenger: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashReturn) -> Self {
                    (value.slashAmountWei,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        slashAmountWei: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashCall {
            type Parameters<'a> = (
                ISlasher::Delegation,
                ISlasher::Commitment,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slash(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),(uint64,bytes,bytes32,address),address,bytes,address)";
            const SELECTOR: [u8; 4] = [194u8, 57u8, 239u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISlasher::Delegation as alloy_sol_types::SolType>::tokenize(&self.delegation),
                    <ISlasher::Commitment as alloy_sol_types::SolType>::tokenize(&self.commitment),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.committer,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.evidence,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.challenger,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: slashReturn = r.into();
                        r.slashAmountWei
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: slashReturn = r.into();
                    r.slashAmountWei
                })
            }
        }
    };
    ///Container for all the [`DummySlasher`](self) function calls.
    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    pub enum DummySlasherCalls {
        #[allow(missing_docs)]
        SLASH_AMOUNT_WEI(SLASH_AMOUNT_WEICall),
        #[allow(missing_docs)]
        slash(slashCall),
    }
    impl DummySlasherCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] =
            &[[73u8, 104u8, 246u8, 197u8], [194u8, 57u8, 239u8, 66u8]];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(SLASH_AMOUNT_WEI),
            ::core::stringify!(slash),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::SIGNATURE,
            <slashCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(selector: [u8; 4usize]) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DummySlasherCalls {
        const NAME: &'static str = "DummySlasherCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 2usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::SLASH_AMOUNT_WEI(_) => {
                    <SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slash(_) => <slashCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<DummySlasherCalls>] = &[
                {
                    fn SLASH_AMOUNT_WEI(data: &[u8]) -> alloy_sol_types::Result<DummySlasherCalls> {
                        <SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DummySlasherCalls::SLASH_AMOUNT_WEI)
                    }
                    SLASH_AMOUNT_WEI
                },
                {
                    fn slash(data: &[u8]) -> alloy_sol_types::Result<DummySlasherCalls> {
                        <slashCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DummySlasherCalls::slash)
                    }
                    slash
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            )
                -> alloy_sol_types::Result<DummySlasherCalls>] = &[
                {
                    fn SLASH_AMOUNT_WEI(data: &[u8]) -> alloy_sol_types::Result<DummySlasherCalls> {
                        <SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(DummySlasherCalls::SLASH_AMOUNT_WEI)
                    }
                    SLASH_AMOUNT_WEI
                },
                {
                    fn slash(data: &[u8]) -> alloy_sol_types::Result<DummySlasherCalls> {
                        <slashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(DummySlasherCalls::slash)
                    }
                    slash
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::SLASH_AMOUNT_WEI(inner) => {
                    <SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::slash(inner) => {
                    <slashCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::SLASH_AMOUNT_WEI(inner) => {
                    <SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::slash(inner) => {
                    <slashCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DummySlasher`](self) contract instance.

    See the [wrapper's documentation](`DummySlasherInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> DummySlasherInstance<P, N> {
        DummySlasherInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        __provider: P,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<DummySlasherInstance<P, N>>>
    {
        DummySlasherInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        DummySlasherInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`DummySlasher`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`DummySlasher`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DummySlasherInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for DummySlasherInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DummySlasherInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        DummySlasherInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`DummySlasher`](self) contract instance.

        See the [wrapper's documentation](`DummySlasherInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(__provider: P) -> alloy_contract::Result<DummySlasherInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> DummySlasherInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DummySlasherInstance<P, N> {
            DummySlasherInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        DummySlasherInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`SLASH_AMOUNT_WEI`] function.
        pub fn SLASH_AMOUNT_WEI(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, SLASH_AMOUNT_WEICall, N> {
            self.call_builder(&SLASH_AMOUNT_WEICall)
        }
        ///Creates a new call builder for the [`slash`] function.
        pub fn slash(
            &self,
            delegation: <ISlasher::Delegation as alloy::sol_types::SolType>::RustType,
            commitment: <ISlasher::Commitment as alloy::sol_types::SolType>::RustType,
            committer: alloy::sol_types::private::Address,
            evidence: alloy::sol_types::private::Bytes,
            challenger: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, slashCall, N> {
            self.call_builder(&slashCall {
                delegation,
                commitment,
                committer,
                evidence,
                challenger,
            })
        }
    }
    /// Event filters.
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        DummySlasherInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
