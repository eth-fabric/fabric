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
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
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
				Self { x_a: tuple.0, x_b: tuple.1, y_a: tuple.2, y_b: tuple.3 }
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
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_b),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_b),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for G1Point {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
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
				alloy_sol_types::private::Cow::Borrowed("G1Point(bytes32 x_a,bytes32 x_b,bytes32 y_a,bytes32 y_b)")
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				alloy_sol_types::private::Vec::new()
			}
			#[inline]
			fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
				<Self as alloy_sol_types::SolStruct>::eip712_root_type()
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_b,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_b,
					)
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
					+ <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.x_a,
					) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.x_b,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_a,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_b,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_a, out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_b, out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_a, out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_b, out,
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
	/**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

	See the [wrapper's documentation](`BLSInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
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
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> BLSInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

		See the [wrapper's documentation](`BLSInstance`) for more details.*/
		#[inline]
		pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
			Self { address, provider: __provider, _network: ::core::marker::PhantomData }
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
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> BLSInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> BLSInstance<P, N> {
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
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
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<Commitment> for UnderlyingRustTuple<'_> {
			fn from(value: Commitment) -> Self {
				(value.commitmentType, value.payload, value.requestHash, value.slasher)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for Commitment {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { commitmentType: tuple.0, payload: tuple.1, requestHash: tuple.2, slasher: tuple.3 }
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
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.commitmentType),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.payload),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.requestHash,
					),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.slasher),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for Commitment {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
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
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				alloy_sol_types::private::Vec::new()
			}
			#[inline]
			fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
				<Self as alloy_sol_types::SolStruct>::eip712_root_type()
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(
						&self.commitmentType,
					)
					.0,
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(&self.payload).0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.requestHash,
					)
					.0,
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(&self.slasher)
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
					+ <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.commitmentType,
					) + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.payload,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.requestHash,
				) + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.slasher,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.commitmentType,
					out,
				);
				<alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.payload,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
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
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<Delegation> for UnderlyingRustTuple<'_> {
			fn from(value: Delegation) -> Self {
				(value.proposer, value.delegate, value.committer, value.slot, value.metadata)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for Delegation {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { proposer: tuple.0, delegate: tuple.1, committer: tuple.2, slot: tuple.3, metadata: tuple.4 }
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
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.committer),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.slot),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.metadata),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for Delegation {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
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
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(2);
				components.push(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components());
				components.push(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.proposer).0,
					<BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.delegate).0,
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
						&self.committer,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.slot).0,
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(&self.metadata).0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for Delegation {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.proposer)
					+ <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.delegate)
					+ <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.committer,
					) + <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.slot,
				) + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.metadata,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.proposer, out);
				<BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.delegate, out);
				<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.committer,
					out,
				);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.slot, out,
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
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
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
			f.debug_tuple("ISlasherInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> ISlasherInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`ISlasher`](self) contract instance.

		See the [wrapper's documentation](`ISlasherInstance`) for more details.*/
		#[inline]
		pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
			Self { address, provider: __provider, _network: ::core::marker::PhantomData }
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
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> ISlasherInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> ISlasherInstance<P, N> {
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
			alloy_contract::Event::new_sol(&self.provider, &self.address)
		}
	}
}
///Module containing a contract's types and functions.
/**

```solidity
library PreconfStructs {
	struct BlockHeaderData { bytes32 parentHash; bytes32 stateRoot; bytes32 txRoot; uint256 blockNumber; uint256 timestamp; uint256 baseFee; }
}
```*/
#[allow(
	non_camel_case_types,
	non_snake_case,
	clippy::pub_underscore_fields,
	clippy::style,
	clippy::empty_structs_with_brackets
)]
pub mod PreconfStructs {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct BlockHeaderData { bytes32 parentHash; bytes32 stateRoot; bytes32 txRoot; uint256 blockNumber; uint256 timestamp; uint256 baseFee; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct BlockHeaderData {
		#[allow(missing_docs)]
		pub parentHash: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub stateRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub txRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub timestamp: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub baseFee: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::Uint<256>,
			alloy::sol_types::sol_data::Uint<256>,
			alloy::sol_types::sol_data::Uint<256>,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::primitives::aliases::U256,
			alloy::sol_types::private::primitives::aliases::U256,
			alloy::sol_types::private::primitives::aliases::U256,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<BlockHeaderData> for UnderlyingRustTuple<'_> {
			fn from(value: BlockHeaderData) -> Self {
				(value.parentHash, value.stateRoot, value.txRoot, value.blockNumber, value.timestamp, value.baseFee)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for BlockHeaderData {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self {
					parentHash: tuple.0,
					stateRoot: tuple.1,
					txRoot: tuple.2,
					blockNumber: tuple.3,
					timestamp: tuple.4,
					baseFee: tuple.5,
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for BlockHeaderData {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for BlockHeaderData {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.parentHash,
					),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.stateRoot),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.txRoot),
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.timestamp),
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.baseFee),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for BlockHeaderData {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
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
		impl alloy_sol_types::SolStruct for BlockHeaderData {
			const NAME: &'static str = "BlockHeaderData";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"BlockHeaderData(bytes32 parentHash,bytes32 stateRoot,bytes32 txRoot,uint256 blockNumber,uint256 timestamp,uint256 baseFee)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				alloy_sol_types::private::Vec::new()
			}
			#[inline]
			fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
				<Self as alloy_sol_types::SolStruct>::eip712_root_type()
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.parentHash,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.stateRoot,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.txRoot,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(
						&self.blockNumber,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(
						&self.timestamp,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(
						&self.baseFee,
					)
					.0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for BlockHeaderData {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.parentHash,
					) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.stateRoot,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.txRoot,
				) + <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.blockNumber,
				) + <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.timestamp,
				) + <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.baseFee,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.parentHash,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.stateRoot,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.txRoot,
					out,
				);
				<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.blockNumber,
					out,
				);
				<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.timestamp,
					out,
				);
				<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.baseFee,
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
	/**Creates a new wrapper around an on-chain [`PreconfStructs`](self) contract instance.

	See the [wrapper's documentation](`PreconfStructsInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> PreconfStructsInstance<P, N> {
		PreconfStructsInstance::<P, N>::new(address, __provider)
	}
	/**A [`PreconfStructs`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`PreconfStructs`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct PreconfStructsInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for PreconfStructsInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("PreconfStructsInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> PreconfStructsInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`PreconfStructs`](self) contract instance.

		See the [wrapper's documentation](`PreconfStructsInstance`) for more details.*/
		#[inline]
		pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
			Self { address, provider: __provider, _network: ::core::marker::PhantomData }
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
	impl<P: ::core::clone::Clone, N> PreconfStructsInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> PreconfStructsInstance<P, N> {
			PreconfStructsInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> PreconfStructsInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> PreconfStructsInstance<P, N> {
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
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

library PreconfStructs {
	struct BlockHeaderData {
		bytes32 parentHash;
		bytes32 stateRoot;
		bytes32 txRoot;
		uint256 blockNumber;
		uint256 timestamp;
		uint256 baseFee;
	}
}

interface StateLockSlasher {
	error BeaconRootNotFound();
	error BlockIsNotFinalized();
	error BlockIsTooOld();
	error ChallengeAlreadyExists();
	error ChallengeDoesNotExist();
	error DelegationExpired();
	error ECDSAInvalidSignature();
	error ECDSAInvalidSignatureLength(uint256 length);
	error ECDSAInvalidSignatureS(bytes32 s);
	error EthTransferFailed();
	error FraudProofWindowActive();
	error IncorrectChallengeBond();
	error InvalidBlockHash();
	error InvalidBlockNumber();
	error InvalidFieldCount();
	error InvalidParentBlockHash();
	error InvalidSignatureLength();
	error NoSignature();
	error NotURC();
	error TransactionExcluded();
	error UnexpectedSigner();
	error UnsupportedTxType();
	error WrongChallengerAddress();
	error WrongTransactionHashProof();

	constructor(uint256 _slashAmountWei);

	function BEACON_ROOTS_CONTRACT() external view returns (address);
	function BLOCKHASH_EVM_LOOKBACK() external view returns (uint256);
	function EIP4788_WINDOW() external view returns (uint256);
	function ETH2_GENESIS_TIMESTAMP() external view returns (uint256);
	function JUSTIFICATION_DELAY() external view returns (uint256);
	function SLASH_AMOUNT_WEI() external view returns (uint256);
	function SLOT_TIME() external view returns (uint256);
	function _decodeBlockHeaderRLP(bytes memory headerRLP) external pure returns (PreconfStructs.BlockHeaderData memory blockHeader);
	function _getCurrentSlot() external view returns (uint256);
	function _getSlotFromTimestamp(uint256 _timestamp) external view returns (uint256);
	function _getTimestampFromSlot(uint256 _slot) external view returns (uint256);
	function slash(ISlasher.Delegation memory delegation, ISlasher.Commitment memory commitment, address committer, bytes memory evidence, address challenger) external returns (uint256 slashAmountWei);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
	"type": "constructor",
	"inputs": [
	  {
		"name": "_slashAmountWei",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "BEACON_ROOTS_CONTRACT",
	"inputs": [],
	"outputs": [
	  {
		"name": "",
		"type": "address",
		"internalType": "address"
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "BLOCKHASH_EVM_LOOKBACK",
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
	"name": "EIP4788_WINDOW",
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
	"name": "ETH2_GENESIS_TIMESTAMP",
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
	"name": "JUSTIFICATION_DELAY",
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
	"name": "SLOT_TIME",
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
	"name": "_decodeBlockHeaderRLP",
	"inputs": [
	  {
		"name": "headerRLP",
		"type": "bytes",
		"internalType": "bytes"
	  }
	],
	"outputs": [
	  {
		"name": "blockHeader",
		"type": "tuple",
		"internalType": "struct PreconfStructs.BlockHeaderData",
		"components": [
		  {
			"name": "parentHash",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "stateRoot",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "txRoot",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "blockNumber",
			"type": "uint256",
			"internalType": "uint256"
		  },
		  {
			"name": "timestamp",
			"type": "uint256",
			"internalType": "uint256"
		  },
		  {
			"name": "baseFee",
			"type": "uint256",
			"internalType": "uint256"
		  }
		]
	  }
	],
	"stateMutability": "pure"
  },
  {
	"type": "function",
	"name": "_getCurrentSlot",
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
	"name": "_getSlotFromTimestamp",
	"inputs": [
	  {
		"name": "_timestamp",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
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
	"name": "_getTimestampFromSlot",
	"inputs": [
	  {
		"name": "_slot",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
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
  },
  {
	"type": "error",
	"name": "BeaconRootNotFound",
	"inputs": []
  },
  {
	"type": "error",
	"name": "BlockIsNotFinalized",
	"inputs": []
  },
  {
	"type": "error",
	"name": "BlockIsTooOld",
	"inputs": []
  },
  {
	"type": "error",
	"name": "ChallengeAlreadyExists",
	"inputs": []
  },
  {
	"type": "error",
	"name": "ChallengeDoesNotExist",
	"inputs": []
  },
  {
	"type": "error",
	"name": "DelegationExpired",
	"inputs": []
  },
  {
	"type": "error",
	"name": "ECDSAInvalidSignature",
	"inputs": []
  },
  {
	"type": "error",
	"name": "ECDSAInvalidSignatureLength",
	"inputs": [
	  {
		"name": "length",
		"type": "uint256",
		"internalType": "uint256"
	  }
	]
  },
  {
	"type": "error",
	"name": "ECDSAInvalidSignatureS",
	"inputs": [
	  {
		"name": "s",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	]
  },
  {
	"type": "error",
	"name": "EthTransferFailed",
	"inputs": []
  },
  {
	"type": "error",
	"name": "FraudProofWindowActive",
	"inputs": []
  },
  {
	"type": "error",
	"name": "IncorrectChallengeBond",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InvalidBlockHash",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InvalidBlockNumber",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InvalidFieldCount",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InvalidParentBlockHash",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InvalidSignatureLength",
	"inputs": []
  },
  {
	"type": "error",
	"name": "NoSignature",
	"inputs": []
  },
  {
	"type": "error",
	"name": "NotURC",
	"inputs": []
  },
  {
	"type": "error",
	"name": "TransactionExcluded",
	"inputs": []
  },
  {
	"type": "error",
	"name": "UnexpectedSigner",
	"inputs": []
  },
  {
	"type": "error",
	"name": "UnsupportedTxType",
	"inputs": []
  },
  {
	"type": "error",
	"name": "WrongChallengerAddress",
	"inputs": []
  },
  {
	"type": "error",
	"name": "WrongTransactionHashProof",
	"inputs": []
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
pub mod StateLockSlasher {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	/// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f80fd5b50604051614208380380614208833981016040819052602b91606f565b5f81905546614268036043576365156ac0600155606a565b46600103605657635fc63057600155606a565b466401a2140cff03606a576366755d6c6001555b506085565b5f60208284031215607e575f80fd5b5051919050565b614176806100925f395ff3fe608060405234801561000f575f80fd5b50600436106100b1575f3560e01c80639f329d0b1161006e5780639f329d0b14610178578063abeeb36614610181578063c239ef4214610194578063cac62dd9146101a7578063f45e8118146101b0578063f5beea8c146101b9575f80fd5b8063189cc9d0146100b55780632a04ff85146100d05780634968f6c5146100d857806356b4a92a146100e0578063743770291461013e57806386fb61ed14610146575b5f80fd5b6100bd6101cc565b6040519081526020015b60405180910390f35b6100bd600c81565b6100bd5f5481565b6100f36100ee366004613977565b6101db565b6040516100c791905f60c082019050825182526020830151602083015260408301516040830152606083015160608301526080830151608083015260a083015160a083015292915050565b6100bd602081565b610160720f3df6d732807ef1319fb7b8bb8522d0beac0281565b6040516001600160a01b0390911681526020016100c7565b6100bd60015481565b6100bd61018f3660046139b0565b6102ec565b6100bd6101a2366004613a21565b61030d565b6100bd611fff81565b6100bd61010081565b6100bd6101c73660046139b0565b6103aa565b5f6101d6426102ec565b905090565b6040805160c0810182525f80825260208201819052918101829052606081018290526080810182905260a081018290529061021d610218846103c3565b6103ef565b9050610241815f8151811061023457610234613ad8565b60200260200101516105da565b8252805161025c908290600390811061023457610234613ad8565b6020830152805161027a908290600490811061023457610234613ad8565b604083015280516102a5908290600890811061029857610298613ad8565b60200260200101516106d7565b606083015280516102c3908290600b90811061029857610298613ad8565b608083015280516102e1908290600f90811061029857610298613ad8565b60a083015250919050565b5f600c600154836102fd9190613b00565b6103079190613b27565b92915050565b5f8061031c6020880188613b3a565b8101906103299190613b92565b90505f61033885870187613d4b565b82519091506001600160401b03166103586101408b016101208c01613e48565b6001600160401b03161461037f576040516330d3ba0760e01b815260040160405180910390fd5b61039b82826103966101208d016101008e01613e61565b6106e1565b50505f54979650505050505050565b5f6103b6600c83613e7a565b6001546103079190613e91565b6040805180820182525f8082526020918201528151808301909252825182529182019181019190915290565b60605f806103fc846108ff565b9193509091506001905081600181111561041857610418613ea4565b1461046a5760405162461bcd60e51b815260206004820152601760248201527f496e76616c696420524c50206c6973742076616c75652e00000000000000000060448201526064015b60405180910390fd5b60408051602080825261042082019092525f91816020015b604080518082019091525f80825260208201528152602001906001900390816104825790505090505f835b86518110156105cf57602082106105195760405162461bcd60e51b815260206004820152602a60248201527f50726f766964656420524c50206c6973742065786365656473206d6178206c6960448201526939ba103632b733ba341760b11b6064820152608401610461565b5f806105546040518060400160405280858c5f01516105389190613b00565b8152602001858c6020015161054d9190613e91565b90526108ff565b5091509150604051806040016040528083836105709190613e91565b8152602001848b602001516105859190613e91565b81525085858151811061059a5761059a613ad8565b60209081029190910101526105b0600185613e91565b93506105bc8183613e91565b6105c69084613e91565b925050506104ad565b508152949350505050565b5f6021825f0151111561062f5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020627974657333322076616c75652e0000000000006044820152606401610461565b5f805f61063b856108ff565b919450925090505f81600181111561065557610655613ea4565b146106a25760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020627974657333322076616c75652e0000000000006044820152606401610461565b5f8386602001516106b39190613e91565b805190915060208410156106cd5760208490036101000a90045b9695505050505050565b5f610307826105da565b82516001600160401b031660206106f66101cc565b6107009190613b00565b8111156107205760405163b6144bff60e01b815260040160405180910390fd5b61010061072b6101cc565b6107359190613b00565b81101561075557604051630cdceb7960e21b815260040160405180910390fd5b82515f9061076590600190613b00565b90504381118061077f575061077c61010043613b00565b81105b1561079d57604051631391e11b60e21b815260040160405180910390fd5b83515f906107ad90600190613b00565b6020808701518051910120904091508082146107dc57604051631f03465b60e11b815260040160405180910390fd5b5f806107e789610bf9565b9250925050816001600160a01b0316876001600160a01b03161461081e5760405163aaaa914160e01b815260040160405180910390fd5b5f61082c89604001516101db565b80519091508414610850576040516320fa6c8b60e11b815260040160405180910390fd5b5f6108778a60a001515f8151811061086a5761086a613ad8565b6020026020010151610c81565b90505f806108a7838d608001515f8151811061089557610895613ad8565b60200260200101518660400151610c94565b91509150816108c95760405163094cec5f60e01b815260040160405180910390fd5b805160208201208551146108f057604051634462b38f60e11b815260040160405180910390fd5b50505050505050505050505050565b5f805f80845f0151116109545760405162461bcd60e51b815260206004820152601860248201527f524c50206974656d2063616e6e6f74206265206e756c6c2e00000000000000006044820152606401610461565b602084015180515f1a607f8111610976575f60015f9450945094505050610bf2565b60b781116109e5578551607f1982019081106109d45760405162461bcd60e51b815260206004820152601960248201527f496e76616c696420524c502073686f727420737472696e672e000000000000006044820152606401610461565b6001955093505f9250610bf2915050565b60bf8111610abc57855160b6198201908110610a435760405162461bcd60e51b815260206004820152601f60248201527f496e76616c696420524c50206c6f6e6720737472696e67206c656e6774682e006044820152606401610461565b5f816020036101000a6001850151049050808201885f015111610aa85760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420524c50206c6f6e6720737472696e672e00000000000000006044820152606401610461565b6001909101955093505f9250610bf2915050565b60f78111610b2b57855160bf198201908110610b1a5760405162461bcd60e51b815260206004820152601760248201527f496e76616c696420524c502073686f7274206c6973742e0000000000000000006044820152606401610461565b600195509350849250610bf2915050565b855160f6198201908110610b815760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420524c50206c6f6e67206c697374206c656e6774682e0000006044820152606401610461565b5f816020036101000a6001850151049050808201885f015111610bdf5760405162461bcd60e51b815260206004820152601660248201527524b73b30b634b210292628103637b733903634b9ba1760511b6044820152606401610461565b6001918201965094509250610bf2915050565b9193909250565b604080516060810182525f808252602082018190529181018290528190610c2c610c2285610d69565b8560200151610db0565b91505f610c3c8560400151610dd8565b9050610c4781610e92565b935060405180606001604052808660400151805190602001208152602001826060015181526020018260e001518152509150509193909250565b6060610307610c8f83610ead565b610fca565b5f60605f610ca185611038565b90505f805f610cb1848a89611123565b81519295509093509150158080610cc55750815b610d115760405162461bcd60e51b815260206004820152601a60248201527f50726f76696465642070726f6f6620697320696e76616c69642e0000000000006044820152606401610461565b5f81610d2b5760405180602001604052805f815250610d57565b610d5786610d3a600188613b00565b81518110610d4a57610d4a613ad8565b6020026020010151611528565b919b919a509098505050505050505050565b5f816040015180519060200120610d82835f0151611552565b604051602001610d93929190613ecf565b604051602081830303815290604052805190602001209050919050565b5f805f80610dbe86866115d2565b925092509250610dce828261161b565b5090949350505050565b610de0613811565b5f825f81518110610df357610df3613ad8565b01602001516001600160f81b0319169050607f60f81b8110610e1f57610e18836116d7565b9392505050565b6001600160f81b03198116600160f81b03610e3d57610e188361190d565b6001600160f81b03198116600160f91b03610e5b57610e1883611bcd565b6001600160f81b03198116600360f81b03610e7957610e1883611e52565b604051636fc3daa360e11b815260040160405180910390fd5b5f610307610e9f836121df565b610ea8846121f7565b610db0565b60605f82604051602001610ec391815260200190565b60405160208183030381529060405290505f5b6020811015610f0e57818181518110610ef157610ef1613ad8565b01602001516001600160f81b0319165f03610f0e57600101610ed6565b5f610f1a826020613b00565b6001600160401b03811115610f3157610f316138a0565b6040519080825280601f01601f191660200182016040528015610f5b576020820181803683370190505b5090505f5b8151811015610fc1578383610f7481613ee0565b945081518110610f8657610f86613ad8565b602001015160f81c60f81b828281518110610fa357610fa3613ad8565b60200101906001600160f81b03191690815f1a905350600101610f60565b50949350505050565b60608082516001148015610ff757506080835f81518110610fed57610fed613ad8565b016020015160f81c105b15611003575081610307565b61100f83516080612253565b83604051602001611021929190613ef8565b604051602081830303815290604052905092915050565b60605f611044836123fa565b90505f81516001600160401b03811115611060576110606138a0565b6040519080825280602002602001820160405280156110a557816020015b604080518082019091526060808252602082015281526020019060019003908161107e5790505b5090505f5b825181101561111b575f6110d68483815181106110c9576110c9613ad8565b6020026020010151612408565b905060405180604001604052808281526020016110f2836123fa565b81525083838151811061110757611107613ad8565b6020908102919091010152506001016110aa565b509392505050565b5f606081808061113287612495565b90505f8690505f80611157604051806040016040528060608152602001606081525090565b5f5b8c51811015611500578c818151811061117457611174613ad8565b60200260200101519150828461118a9190613e91565b9350611197600188613e91565b9650835f036111f1578151805160209091012085146111ec5760405162461bcd60e51b8152602060048201526011602482015270092dcecc2d8d2c840e4dedee840d0c2e6d607b1b6044820152606401610461565b6112ad565b815151602011611253578151805160209091012085146111ec5760405162461bcd60e51b815260206004820152601b60248201527f496e76616c6964206c6172676520696e7465726e616c206861736800000000006044820152606401610461565b84611260835f01516125a5565b146112ad5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420696e7465726e616c206e6f646520686173680000000000006044820152606401610461565b6112b960106001613e91565b826020015151036113295785518414611500575f8685815181106112df576112df613ad8565b602001015160f81c60f81b60f81c90505f83602001518260ff168151811061130957611309613ad8565b6020026020010151905061131c816125cc565b96506001945050506114f8565b6002826020015151036114b0575f61134083612600565b90505f815f8151811061135557611355613ad8565b016020015160f81c90505f61136b600283613f0c565b611376906002613f2d565b90505f611386848360ff16612623565b90505f6113938b8a612623565b90505f6113a08383612651565b905060ff8516600214806113b7575060ff85166003145b156113f1578083511480156113cc5750808251145b156113de576113db818b613e91565b99505b50600160ff1b9950611500945050505050565b60ff85161580611404575060ff85166001145b1561145957805f036114235750600160ff1b9950611500945050505050565b61144a886020015160018151811061143d5761143d613ad8565b60200260200101516125cc565b9a5097506114f8945050505050565b60405162461bcd60e51b815260206004820152602660248201527f52656365697665642061206e6f6465207769746820616e20756e6b6e6f776e206044820152650e0e4caccd2f60d31b6064820152608401610461565b60405162461bcd60e51b815260206004820152601c60248201527f526563656976656420616e20756e7061727361626c65206e6f64652e000000006044820152606401610461565b600101611159565b50600160ff1b8414866115138786612623565b909e909d50909b509950505050505050505050565b602081015180516060916103079161154290600190613b00565b815181106110c9576110c9613ad8565b6040805160088082528183019092526060915f91906020820181803683370190505090505f5b60088110156115cb5761158c816008613e7a565b846001600160401b0316901c60f81b8282815181106115ad576115ad613ad8565b60200101906001600160f81b03191690815f1a905350600101611578565b5092915050565b5f805f8351604103611609576020840151604085015160608601515f1a6115fb888285856126cc565b955095509550505050611614565b505081515f91506002905b9250925092565b5f82600381111561162e5761162e613ea4565b03611637575050565b600182600381111561164b5761164b613ea4565b036116695760405163f645eedf60e01b815260040160405180910390fd5b600282600381111561167d5761167d613ea4565b0361169e5760405163fce698f760e01b815260048101829052602401610461565b60038260038111156116b2576116b2613ea4565b036116d3576040516335e2f38360e21b815260048101829052602401610461565b5050565b6116df613811565b5f8082526116ef610218846103c3565b9050805160091415801561170557508051600614155b156117235760405163c2871a3760e01b815260040160405180910390fd5b611738815f8151811061029857610298613ad8565b60608301528051611756908290600190811061029857610298613ad8565b60808301528051611774908290600290811061029857610298613ad8565b60e0830152805161179f908290600390811061179257611792613ad8565b6020026020010151612794565b6001600160a01b031661010083015280516117c7908290600490811061029857610298613ad8565b61012083015280516117e690829060059081106110c9576110c9613ad8565b61014083015280516006036117fb5750919050565b5f6118128260068151811061029857610298613ad8565b90505f61182b8360078151811061029857610298613ad8565b90505f6118448460088151811061029857610298613ad8565b905081158015611852575080155b15611872576001600160401b038316602086015260016040860152611904565b6023836001600160401b0316106118ba576002611890602385613f46565b61189a9190613f66565b6001600160401b03908116602087015283166101e0860152600160408601525b5f6118c9600260018618613f8b565b6118d490601b613fb0565b6040519091506118ec90849084908490602001613fd0565b60408051601f198184030181529190526101c0870152505b50505050919050565b611915613811565b600180825282515f916119359161192d908290613b00565b859190612800565b90505f611944610218836103c3565b9050805160081415801561195a57508051600b14155b156119785760405163c2871a3760e01b815260040160405180910390fd5b61198d815f8151811061029857610298613ad8565b6001600160401b0316602084015280516119b4908290600190811061029857610298613ad8565b606084015280516119d2908290600290811061029857610298613ad8565b608084015280516119f0908290600390811061029857610298613ad8565b60e08401528051611a0e908290600490811061179257611792613ad8565b6001600160a01b03166101008401528051611a36908290600590811061029857610298613ad8565b6101208401528051611a5590829060069081106110c9576110c9613ad8565b8361014001819052505f611a8282600781518110611a7557611a75613ad8565b60200260200101516103ef565b905080516001600160401b03811115611a9d57611a9d6138a0565b604051908082528060200260200182016040528015611ad057816020015b6060815260200190600190039081611abb5790505b506101608501525f5b8151811015611b2e57611b04828281518110611af757611af7613ad8565b6020026020010151612940565b8561016001518281518110611b1b57611b1b613ad8565b6020908102919091010152600101611ad9565b508151600803611b4057505050919050565b5f611b578360088151811061029857610298613ad8565b611b6290601b613ff3565b90505f611b7b8460098151811061023457610234613ad8565b90505f611b9485600a8151811061023457610234613ad8565b9050818184604051602001611bab93929190613fd0565b60408051601f198184030181529190526101c088015250949695505050505050565b611bd5613811565b6002815281515f90611bef9060019061192d908290613b00565b90505f611bfe610218836103c3565b90508051600914158015611c1457508051600c14155b15611c325760405163c2871a3760e01b815260040160405180910390fd5b611c47815f8151811061029857610298613ad8565b6001600160401b031660208401528051611c6e908290600190811061029857610298613ad8565b60608401528051611c8c908290600290811061029857610298613ad8565b60a08401528051611caa908290600390811061029857610298613ad8565b60c08401528051611cc8908290600490811061029857610298613ad8565b60e08401528051611ce6908290600590811061179257611792613ad8565b6001600160a01b03166101008401528051611d0e908290600690811061029857610298613ad8565b6101208401528051611d2d90829060079081106110c9576110c9613ad8565b8361014001819052505f611d4d82600881518110611a7557611a75613ad8565b905080516001600160401b03811115611d6857611d686138a0565b604051908082528060200260200182016040528015611d9b57816020015b6060815260200190600190039081611d865790505b506101608501525f5b8151811015611dec57611dc2828281518110611af757611af7613ad8565b8561016001518281518110611dd957611dd9613ad8565b6020908102919091010152600101611da4565b508151600903611dfe57505050919050565b5f611e158360098151811061029857610298613ad8565b611e2090601b613ff3565b90505f611e3984600a8151811061023457610234613ad8565b90505f611b9485600b8151811061023457610234613ad8565b611e5a613811565b6003815281515f90611e749060019061192d908290613b00565b90505f611e83610218836103c3565b90508051600b14158015611e9957508051600e14155b15611eb75760405163c2871a3760e01b815260040160405180910390fd5b611ecc815f8151811061029857610298613ad8565b6001600160401b031660208401528051611ef3908290600190811061029857610298613ad8565b60608401528051611f11908290600290811061029857610298613ad8565b60a08401528051611f2f908290600390811061029857610298613ad8565b60c08401528051611f4d908290600490811061029857610298613ad8565b60e08401528051611f6b908290600590811061179257611792613ad8565b6001600160a01b03166101008401528051611f93908290600690811061029857610298613ad8565b6101208401528051611fb290829060079081106110c9576110c9613ad8565b8361014001819052505f611fd282600881518110611a7557611a75613ad8565b905080516001600160401b03811115611fed57611fed6138a0565b60405190808252806020026020018201604052801561202057816020015b606081526020019060019003908161200b5790505b506101608501525f5b815181101561207157612047828281518110611af757611af7613ad8565b856101600151828151811061205e5761205e613ad8565b6020908102919091010152600101612029565b506120888260098151811061029857610298613ad8565b846101800181815250505f6120a983600a81518110611a7557611a75613ad8565b905080516001600160401b038111156120c4576120c46138a0565b6040519080825280602002602001820160405280156120ed578160200160208202803683370190505b506101a08601525f5b815181101561213e5761211482828151811061023457610234613ad8565b866101a00151828151811061212b5761212b613ad8565b60209081029190910101526001016120f6565b508251600b036121515750505050919050565b5f61216884600b8151811061029857610298613ad8565b61217390601b613ff3565b90505f61218c85600c8151811061023457610234613ad8565b90505f6121a586600d8151811061023457610234613ad8565b90508181846040516020016121bc93929190613fd0565b60408051601f198184030181529190526101c08901525095979650505050505050565b5f6121e98261294b565b805190602001209050919050565b6060816101c00151515f0361221f5760405163d466bd1560e01b815260040160405180910390fd5b816101c001515160411461224657604051634be6321b60e01b815260040160405180910390fd5b506101c08101515b919050565b60608060388410156122b857604080516001808252818301909252906020820181803683370190505090506122888385613ff3565b60f81b815f8151811061229d5761229d613ad8565b60200101906001600160f81b03191690815f1a905350610e18565b5f60015b6122c68187613b27565b156122ec57816122d581613ee0565b92506122e5905061010082613e7a565b90506122bc565b6122f7826001613e91565b6001600160401b0381111561230e5761230e6138a0565b6040519080825280601f01601f191660200182016040528015612338576020820181803683370190505b5092506123458583613ff3565b612350906037613ff3565b60f81b835f8151811061236557612365613ad8565b60200101906001600160f81b03191690815f1a905350600190505b8181116123f1576101006123948284613b00565b6123a0906101006140ec565b6123aa9088613b27565b6123b491906140f7565b60f81b8382815181106123c9576123c9613ad8565b60200101906001600160f81b03191690815f1a905350806123e981613ee0565b915050612380565b50509392505050565b6060610307610218836103c3565b60605f805f612416856108ff565b919450925090505f81600181111561243057612430613ea4565b1461247d5760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420524c502062797465732076616c75652e00000000000000006044820152606401610461565b61248c856020015184846129d8565b95945050505050565b60605f82516002026001600160401b038111156124b4576124b46138a0565b6040519080825280601f01601f1916602001820160405280156124de576020820181803683370190505b5090505f5b83518110156115cb57600484828151811061250057612500613ad8565b602001015160f81c60f81b6001600160f81b031916901c82826002028151811061252c5761252c613ad8565b60200101906001600160f81b03191690815f1a905350601084828151811061255657612556613ad8565b016020015160f81c8161256b5761256b613b13565b0660f81b82826002026001018151811061258757612587613ad8565b60200101906001600160f81b03191690815f1a9053506001016124e3565b5f6020825110156125b857506020015190565b81806020019051810190610307919061410a565b5f60606020835f015110156125eb576125e483612940565b90506125f7565b6125f483612408565b90505b610e18816125a5565b606061030761261e83602001515f815181106110c9576110c9613ad8565b612495565b6060818351035f03612643575060408051602081019091525f8152610307565b610e18838384865103612800565b5f805b8084511180156126645750808351115b80156126b5575082818151811061267d5761267d613ad8565b602001015160f81c60f81b6001600160f81b0319168482815181106126a4576126a4613ad8565b01602001516001600160f81b031916145b15610e1857806126c481613ee0565b915050612654565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561270557505f9150600390508261278a565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015612756573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b03811661278157505f92506001915082905061278a565b92505f91508190505b9450945094915050565b80515f906001036127a657505f919050565b81516015146127f75760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020616464726573732076616c75652e0000000000006044820152606401610461565b610307826106d7565b60608182601f0110156128465760405162461bcd60e51b815260206004820152600e60248201526d736c6963655f6f766572666c6f7760901b6044820152606401610461565b8282840110156128895760405162461bcd60e51b815260206004820152600e60248201526d736c6963655f6f766572666c6f7760901b6044820152606401610461565b818301845110156128d05760405162461bcd60e51b8152602060048201526011602482015270736c6963655f6f75744f66426f756e647360781b6044820152606401610461565b6060821580156128ee5760405191505f825260208201604052610fc1565b6040519150601f8416801560200281840101858101878315602002848b0101015b8183101561292757805183526020928301920161290f565b5050858452601f01601f19166040525050949350505050565b606061030782612a85565b60605f8251600381111561296157612961613ea4565b0361296f5761030782612a99565b60018251600381111561298457612984613ea4565b036129925761030782612d85565b6002825160038111156129a7576129a7613ea4565b036129b55761030782612ff6565b6003825160038111156129ca576129ca613ea4565b03610e795761030782613264565b60605f826001600160401b038111156129f3576129f36138a0565b6040519080825280601f01601f191660200182016040528015612a1d576020820181803683370190505b50905080515f03612a2f579050610e18565b848401602082015f5b60208604811015612a59578251825260209283019290910190600101612a38565b505f6001602087066020036101000a039050808251168119845116178252839450505050509392505050565b606061030782602001515f845f01516129d8565b60605f82602001516001600160401b03165f14612abb57506020820151612afe565b6101c08301515115612afe576023836101e001516001600160401b031610612afe5760026023846101e00151612af19190613f46565b612afb9190613f66565b90505b5f8360400151612b0e575f612b11565b60035b612b1c906006613ff3565b60ff1690505f816001600160401b03811115612b3a57612b3a6138a0565b604051908082528060200260200182016040528015612b6d57816020015b6060815260200190600190039081612b585790505b509050612b7d8560600151610c81565b815f81518110612b8f57612b8f613ad8565b6020026020010181905250612ba78560800151610c81565b81600181518110612bba57612bba613ad8565b6020026020010181905250612bd28560e00151610c81565b81600281518110612be557612be5613ad8565b6020026020010181905250612bfe856101000151613607565b81600381518110612c1157612c11613ad8565b6020026020010181905250612c2a856101200151610c81565b81600481518110612c3d57612c3d613ad8565b6020026020010181905250612c56856101400151610fca565b81600581518110612c6957612c69613ad8565b6020026020010181905250846040015115612d7c5784602001516001600160401b03165f03612cd0576040515f602082015260210160405160208183030381529060405281600681518110612cc057612cc0613ad8565b6020026020010181905250612d01565b612ce2836001600160401b0316610c81565b81600681518110612cf557612cf5613ad8565b60200260200101819052505b612d365f5b6040519080825280601f01601f191660200182016040528015612d30576020820181803683370190505b50610fca565b81600781518110612d4957612d49613ad8565b6020908102919091010152612d5d5f612d06565b81600881518110612d7057612d70613ad8565b60200260200101819052505b61248c8161363f565b60408051600880825261012082019092526060915f9190816020015b6060815260200190600190039081612da1579050509050612dce83602001516001600160401b0316610c81565b815f81518110612de057612de0613ad8565b6020026020010181905250612df88360600151610c81565b81600181518110612e0b57612e0b613ad8565b6020026020010181905250612e238360800151610c81565b81600281518110612e3657612e36613ad8565b6020026020010181905250612e4e8360e00151610c81565b81600381518110612e6157612e61613ad8565b6020026020010181905250612e7a836101000151613607565b81600481518110612e8d57612e8d613ad8565b6020026020010181905250612ea6836101200151610c81565b81600581518110612eb957612eb9613ad8565b6020026020010181905250612ed2836101400151610fca565b81600681518110612ee557612ee5613ad8565b60200260200101819052505f836101600151516001600160401b03811115612f0f57612f0f6138a0565b604051908082528060200260200182016040528015612f4257816020015b6060815260200190600190039081612f2d5790505b5090505f5b84610160015151811015612f99578461016001518181518110612f6c57612f6c613ad8565b6020026020010151828281518110612f8657612f86613ad8565b6020908102919091010152600101612f47565b50612fa38161363f565b82600781518110612fb657612fb6613ad8565b602090810291909101015260015b612fcd8361363f565b604051602001612fde929190614121565b60405160208183030381529060405292505050919050565b60408051600980825261014082019092526060915f9190816020015b606081526020019060019003908161301257905050905061303f83602001516001600160401b0316610c81565b815f8151811061305157613051613ad8565b60200260200101819052506130698360600151610c81565b8160018151811061307c5761307c613ad8565b60200260200101819052506130948360a00151610c81565b816002815181106130a7576130a7613ad8565b60200260200101819052506130bf8360c00151610c81565b816003815181106130d2576130d2613ad8565b60200260200101819052506130ea8360e00151610c81565b816004815181106130fd576130fd613ad8565b6020026020010181905250613116836101000151613607565b8160058151811061312957613129613ad8565b6020026020010181905250613142836101200151610c81565b8160068151811061315557613155613ad8565b602002602001018190525061316e836101400151610fca565b8160078151811061318157613181613ad8565b60200260200101819052505f836101600151516001600160401b038111156131ab576131ab6138a0565b6040519080825280602002602001820160405280156131de57816020015b60608152602001906001900390816131c95790505b5090505f5b8461016001515181101561323557846101600151818151811061320857613208613ad8565b602002602001015182828151811061322257613222613ad8565b60209081029190910101526001016131e3565b5061323f8161363f565b8260088151811061325257613252613ad8565b60209081029190910101526002612fc4565b60408051600b80825261018082019092526060915f9190816020015b60608152602001906001900390816132805790505090506132ad83602001516001600160401b0316610c81565b815f815181106132bf576132bf613ad8565b60200260200101819052506132d78360600151610c81565b816001815181106132ea576132ea613ad8565b60200260200101819052506133028360a00151610c81565b8160028151811061331557613315613ad8565b602002602001018190525061332d8360c00151610c81565b8160038151811061334057613340613ad8565b60200260200101819052506133588360e00151610c81565b8160048151811061336b5761336b613ad8565b6020026020010181905250613384836101000151613607565b8160058151811061339757613397613ad8565b60200260200101819052506133b0836101200151610c81565b816006815181106133c3576133c3613ad8565b60200260200101819052506133dc836101400151610fca565b816007815181106133ef576133ef613ad8565b60200260200101819052505f836101600151516001600160401b03811115613419576134196138a0565b60405190808252806020026020018201604052801561344c57816020015b60608152602001906001900390816134375790505b5090505f5b846101600151518110156134a357846101600151818151811061347657613476613ad8565b602002602001015182828151811061349057613490613ad8565b6020908102919091010152600101613451565b506134ad8161363f565b826008815181106134c0576134c0613ad8565b60200260200101819052506134d9846101800151610c81565b826009815181106134ec576134ec613ad8565b60200260200101819052505f846101a00151516001600160401b03811115613516576135166138a0565b60405190808252806020026020018201604052801561354957816020015b60608152602001906001900390816135345790505b5090505f5b856101a00151518110156135aa57613585866101a00151828151811061357657613576613ad8565b60200260200101515f1c610c81565b82828151811061359757613597613ad8565b602090810291909101015260010161354e565b506135b48161363f565b83600a815181106135c7576135c7613ad8565b602090810291909101015260036135dd8461363f565b6040516020016135ee929190614121565b6040516020818303038152906040529350505050919050565b604051606082811b6bffffffffffffffffffffffff191660208301529061030790603401604051602081830303815290604052610fca565b60605f61364b83613682565b9050613659815160c0612253565b8160405160200161366b929190613ef8565b604051602081830303815290604052915050919050565b606081515f036136a157604080515f80825260208201909252906115cb565b5f805b83518110156136dd578381815181106136bf576136bf613ad8565b602002602001015151826136d39190613e91565b91506001016136a4565b5f826001600160401b038111156136f6576136f66138a0565b6040519080825280601f01601f191660200182016040528015613720576020820181803683370190505b505f92509050602081015b8551831015610fc1575f86848151811061374757613747613ad8565b602002602001015190505f6020820190506137648382845161379a565b87858151811061377657613776613ad8565b6020026020010151518361378a9190613e91565b600190950194925061372b915050565b8282825b602081106137d657815183526137b5602084613e91565b92506137c2602083613e91565b91506137cf602082613b00565b905061379e565b5f60016137e4836020613b00565b6137f0906101006140ec565b6137fa9190613b00565b925184518416931916929092179092525050505050565b604080516102008101909152805f81526020015f6001600160401b031681526020015f151581526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f815260200160608152602001606081526020015f815260200160608152602001606081526020015f6001600160401b031681525090565b634e487b7160e01b5f52604160045260245ffd5b60405160c081016001600160401b03811182821017156138d6576138d66138a0565b60405290565b604051601f8201601f191681016001600160401b0381118282101715613904576139046138a0565b604052919050565b5f82601f83011261391b575f80fd5b81356001600160401b03811115613934576139346138a0565b613947601f8201601f19166020016138dc565b81815284602083860101111561395b575f80fd5b816020850160208301375f918101602001919091529392505050565b5f60208284031215613987575f80fd5b81356001600160401b0381111561399c575f80fd5b6139a88482850161390c565b949350505050565b5f602082840312156139c0575f80fd5b5035919050565b80356001600160a01b038116811461224e575f80fd5b5f8083601f8401126139ed575f80fd5b5081356001600160401b03811115613a03575f80fd5b602083019150836020828501011115613a1a575f80fd5b9250929050565b5f805f805f8060a08789031215613a36575f80fd5b86356001600160401b0380821115613a4c575f80fd5b90880190610160828b031215613a60575f80fd5b90965060208801359080821115613a75575f80fd5b908801906080828b031215613a88575f80fd5b819650613a9760408a016139c7565b95506060890135915080821115613aac575f80fd5b50613ab989828a016139dd565b9094509250613acc9050608088016139c7565b90509295509295509295565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561030757610307613aec565b634e487b7160e01b5f52601260045260245ffd5b5f82613b3557613b35613b13565b500490565b5f808335601e19843603018112613b4f575f80fd5b8301803591506001600160401b03821115613b68575f80fd5b602001915036819003821315613a1a575f80fd5b80356001600160401b038116811461224e575f80fd5b5f60208284031215613ba2575f80fd5b81356001600160401b0380821115613bb8575f80fd5b9083019060608286031215613bcb575f80fd5b604051606081018181108382111715613be657613be66138a0565b604052613bf283613b7c565b8152602083013582811115613c05575f80fd5b613c118782860161390c565b602083015250604083013582811115613c28575f80fd5b613c348782860161390c565b60408301525095945050505050565b5f6001600160401b03821115613c5b57613c5b6138a0565b5060051b60200190565b5f82601f830112613c74575f80fd5b81356020613c89613c8483613c43565b6138dc565b82815260059290921b84018101918181019086841115613ca7575f80fd5b8286015b84811015613ce45780356001600160401b03811115613cc8575f80fd5b613cd68986838b010161390c565b845250918301918301613cab565b509695505050505050565b5f82601f830112613cfe575f80fd5b81356020613d0e613c8483613c43565b8083825260208201915060208460051b870101935086841115613d2f575f80fd5b602086015b84811015613ce45780358352918301918301613d34565b5f60208284031215613d5b575f80fd5b81356001600160401b0380821115613d71575f80fd5b9083019060c08286031215613d84575f80fd5b613d8c6138b4565b82358152602083013582811115613da1575f80fd5b613dad8782860161390c565b602083015250604083013582811115613dc4575f80fd5b613dd08782860161390c565b604083015250606083013582811115613de7575f80fd5b613df38782860161390c565b606083015250608083013582811115613e0a575f80fd5b613e1687828601613c65565b60808301525060a083013582811115613e2d575f80fd5b613e3987828601613cef565b60a08301525095945050505050565b5f60208284031215613e58575f80fd5b610e1882613b7c565b5f60208284031215613e71575f80fd5b610e18826139c7565b808202811582820484141761030757610307613aec565b8082018082111561030757610307613aec565b634e487b7160e01b5f52602160045260245ffd5b5f81518060208401855e5f93019283525090919050565b8281525f6139a86020830184613eb8565b5f60018201613ef157613ef1613aec565b5060010190565b5f6139a8613f068386613eb8565b84613eb8565b5f60ff831680613f1e57613f1e613b13565b8060ff84160691505092915050565b60ff828116828216039081111561030757610307613aec565b6001600160401b038281168282160390808211156115cb576115cb613aec565b5f6001600160401b0380841680613f7f57613f7f613b13565b92169190910492915050565b5f6001600160401b0380841680613fa457613fa4613b13565b92169190910692915050565b6001600160401b038181168382160190808211156115cb576115cb613aec565b928352602083019190915260f81b6001600160f81b031916604082015260410190565b60ff818116838216019081111561030757610307613aec565b600181815b8085111561404657815f190482111561402c5761402c613aec565b8085161561403957918102915b93841c9390800290614011565b509250929050565b5f8261405c57506001610307565b8161406857505f610307565b816001811461407e5760028114614088576140a4565b6001915050610307565b60ff84111561409957614099613aec565b50506001821b610307565b5060208310610133831016604e8410600b84101617156140c7575081810a610307565b6140d1838361400c565b805f19048211156140e4576140e4613aec565b029392505050565b5f610e18838361404e565b5f8261410557614105613b13565b500690565b5f6020828403121561411a575f80fd5b5051919050565b60f883901b6001600160f81b03191681525f6139a86001830184613eb856fea2646970667358221220d8cc604fa752454e198f785278ba72bd5dba4fa50e7e42a09d47a4db4b54c20564736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`@QaB\x088\x03\x80aB\x08\x839\x81\x01`@\x81\x90R`+\x91`oV[_\x81\x90UFaBh\x03`CWce\x15j\xC0`\x01U`jV[F`\x01\x03`VWc_\xC60W`\x01U`jV[Fd\x01\xA2\x14\x0C\xFF\x03`jWcfu]l`\x01U[P`\x85V[_` \x82\x84\x03\x12\x15`~W_\x80\xFD[PQ\x91\x90PV[aAv\x80a\0\x92_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80c\x9F2\x9D\x0B\x11a\0nW\x80c\x9F2\x9D\x0B\x14a\x01xW\x80c\xAB\xEE\xB3f\x14a\x01\x81W\x80c\xC29\xEFB\x14a\x01\x94W\x80c\xCA\xC6-\xD9\x14a\x01\xA7W\x80c\xF4^\x81\x18\x14a\x01\xB0W\x80c\xF5\xBE\xEA\x8C\x14a\x01\xB9W_\x80\xFD[\x80c\x18\x9C\xC9\xD0\x14a\0\xB5W\x80c*\x04\xFF\x85\x14a\0\xD0W\x80cIh\xF6\xC5\x14a\0\xD8W\x80cV\xB4\xA9*\x14a\0\xE0W\x80ct7p)\x14a\x01>W\x80c\x86\xFBa\xED\x14a\x01FW[_\x80\xFD[a\0\xBDa\x01\xCCV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBD`\x0C\x81V[a\0\xBD_T\x81V[a\0\xF3a\0\xEE6`\x04a9wV[a\x01\xDBV[`@Qa\0\xC7\x91\x90_`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[a\0\xBD` \x81V[a\x01`r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC7V[a\0\xBD`\x01T\x81V[a\0\xBDa\x01\x8F6`\x04a9\xB0V[a\x02\xECV[a\0\xBDa\x01\xA26`\x04a:!V[a\x03\rV[a\0\xBDa\x1F\xFF\x81V[a\0\xBDa\x01\0\x81V[a\0\xBDa\x01\xC76`\x04a9\xB0V[a\x03\xAAV[_a\x01\xD6Ba\x02\xECV[\x90P\x90V[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\x02\x1Da\x02\x18\x84a\x03\xC3V[a\x03\xEFV[\x90Pa\x02A\x81_\x81Q\x81\x10a\x024Wa\x024a:\xD8V[` \x02` \x01\x01Qa\x05\xDAV[\x82R\x80Qa\x02\\\x90\x82\x90`\x03\x90\x81\x10a\x024Wa\x024a:\xD8V[` \x83\x01R\x80Qa\x02z\x90\x82\x90`\x04\x90\x81\x10a\x024Wa\x024a:\xD8V[`@\x83\x01R\x80Qa\x02\xA5\x90\x82\x90`\x08\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[` \x02` \x01\x01Qa\x06\xD7V[``\x83\x01R\x80Qa\x02\xC3\x90\x82\x90`\x0B\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x80\x83\x01R\x80Qa\x02\xE1\x90\x82\x90`\x0F\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xA0\x83\x01RP\x91\x90PV[_`\x0C`\x01T\x83a\x02\xFD\x91\x90a;\0V[a\x03\x07\x91\x90a;'V[\x92\x91PPV[_\x80a\x03\x1C` \x88\x01\x88a;:V[\x81\x01\x90a\x03)\x91\x90a;\x92V[\x90P_a\x038\x85\x87\x01\x87a=KV[\x82Q\x90\x91P`\x01`\x01`@\x1B\x03\x16a\x03Xa\x01@\x8B\x01a\x01 \x8C\x01a>HV[`\x01`\x01`@\x1B\x03\x16\x14a\x03\x7FW`@Qc0\xD3\xBA\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x9B\x82\x82a\x03\x96a\x01 \x8D\x01a\x01\0\x8E\x01a>aV[a\x06\xE1V[PP_T\x97\x96PPPPPPPV[_a\x03\xB6`\x0C\x83a>zV[`\x01Ta\x03\x07\x91\x90a>\x91V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R\x82Q\x82R\x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[``_\x80a\x03\xFC\x84a\x08\xFFV[\x91\x93P\x90\x91P`\x01\x90P\x81`\x01\x81\x11\x15a\x04\x18Wa\x04\x18a>\xA4V[\x14a\x04jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid RLP list value.\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\x82W\x90PP\x90P_\x83[\x86Q\x81\x10\x15a\x05\xCFW` \x82\x10a\x05\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FProvided RLP list exceeds max li`D\x82\x01Ri9\xBA\x1062\xB73\xBA4\x17`\xB1\x1B`d\x82\x01R`\x84\x01a\x04aV[_\x80a\x05T`@Q\x80`@\x01`@R\x80\x85\x8C_\x01Qa\x058\x91\x90a;\0V[\x81R` \x01\x85\x8C` \x01Qa\x05M\x91\x90a>\x91V[\x90Ra\x08\xFFV[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83a\x05p\x91\x90a>\x91V[\x81R` \x01\x84\x8B` \x01Qa\x05\x85\x91\x90a>\x91V[\x81RP\x85\x85\x81Q\x81\x10a\x05\x9AWa\x05\x9Aa:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x05\xB0`\x01\x85a>\x91V[\x93Pa\x05\xBC\x81\x83a>\x91V[a\x05\xC6\x90\x84a>\x91V[\x92PPPa\x04\xADV[P\x81R\x94\x93PPPPV[_`!\x82_\x01Q\x11\x15a\x06/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP bytes32 value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[_\x80_a\x06;\x85a\x08\xFFV[\x91\x94P\x92P\x90P_\x81`\x01\x81\x11\x15a\x06UWa\x06Ua>\xA4V[\x14a\x06\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP bytes32 value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[_\x83\x86` \x01Qa\x06\xB3\x91\x90a>\x91V[\x80Q\x90\x91P` \x84\x10\x15a\x06\xCDW` \x84\x90\x03a\x01\0\n\x90\x04[\x96\x95PPPPPPV[_a\x03\x07\x82a\x05\xDAV[\x82Q`\x01`\x01`@\x1B\x03\x16` a\x06\xF6a\x01\xCCV[a\x07\0\x91\x90a;\0V[\x81\x11\x15a\x07 W`@Qc\xB6\x14K\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\0a\x07+a\x01\xCCV[a\x075\x91\x90a;\0V[\x81\x10\x15a\x07UW`@Qc\x0C\xDC\xEBy`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q_\x90a\x07e\x90`\x01\x90a;\0V[\x90PC\x81\x11\x80a\x07\x7FWPa\x07|a\x01\0Ca;\0V[\x81\x10[\x15a\x07\x9DW`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q_\x90a\x07\xAD\x90`\x01\x90a;\0V[` \x80\x87\x01Q\x80Q\x91\x01 \x90@\x91P\x80\x82\x14a\x07\xDCW`@Qc\x1F\x03F[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x07\xE7\x89a\x0B\xF9V[\x92P\x92PP\x81`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\x1EW`@Qc\xAA\xAA\x91A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08,\x89`@\x01Qa\x01\xDBV[\x80Q\x90\x91P\x84\x14a\x08PW`@Qc \xFAl\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08w\x8A`\xA0\x01Q_\x81Q\x81\x10a\x08jWa\x08ja:\xD8V[` \x02` \x01\x01Qa\x0C\x81V[\x90P_\x80a\x08\xA7\x83\x8D`\x80\x01Q_\x81Q\x81\x10a\x08\x95Wa\x08\x95a:\xD8V[` \x02` \x01\x01Q\x86`@\x01Qa\x0C\x94V[\x91P\x91P\x81a\x08\xC9W`@Qc\tL\xEC_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q` \x82\x01 \x85Q\x14a\x08\xF0W`@QcDb\xB3\x8F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPPPV[_\x80_\x80\x84_\x01Q\x11a\tTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FRLP item cannot be null.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[` \x84\x01Q\x80Q_\x1A`\x7F\x81\x11a\tvW_`\x01_\x94P\x94P\x94PPPa\x0B\xF2V[`\xB7\x81\x11a\t\xE5W\x85Q`\x7F\x19\x82\x01\x90\x81\x10a\t\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid RLP short string.\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[`\x01\x95P\x93P_\x92Pa\x0B\xF2\x91PPV[`\xBF\x81\x11a\n\xBCW\x85Q`\xB6\x19\x82\x01\x90\x81\x10a\nCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FInvalid RLP long string length.\0`D\x82\x01R`d\x01a\x04aV[_\x81` \x03a\x01\0\n`\x01\x85\x01Q\x04\x90P\x80\x82\x01\x88_\x01Q\x11a\n\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid RLP long string.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[`\x01\x90\x91\x01\x95P\x93P_\x92Pa\x0B\xF2\x91PPV[`\xF7\x81\x11a\x0B+W\x85Q`\xBF\x19\x82\x01\x90\x81\x10a\x0B\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid RLP short list.\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[`\x01\x95P\x93P\x84\x92Pa\x0B\xF2\x91PPV[\x85Q`\xF6\x19\x82\x01\x90\x81\x10a\x0B\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid RLP long list length.\0\0\0`D\x82\x01R`d\x01a\x04aV[_\x81` \x03a\x01\0\n`\x01\x85\x01Q\x04\x90P\x80\x82\x01\x88_\x01Q\x11a\x0B\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru$\xB7;0\xB64\xB2\x10)&(\x1067\xB73\x9064\xB9\xBA\x17`Q\x1B`D\x82\x01R`d\x01a\x04aV[`\x01\x91\x82\x01\x96P\x94P\x92Pa\x0B\xF2\x91PPV[\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x81\x90a\x0C,a\x0C\"\x85a\riV[\x85` \x01Qa\r\xB0V[\x91P_a\x0C<\x85`@\x01Qa\r\xD8V[\x90Pa\x0CG\x81a\x0E\x92V[\x93P`@Q\x80``\x01`@R\x80\x86`@\x01Q\x80Q\x90` \x01 \x81R` \x01\x82``\x01Q\x81R` \x01\x82`\xE0\x01Q\x81RP\x91PP\x91\x93\x90\x92PV[``a\x03\x07a\x0C\x8F\x83a\x0E\xADV[a\x0F\xCAV[_``_a\x0C\xA1\x85a\x108V[\x90P_\x80_a\x0C\xB1\x84\x8A\x89a\x11#V[\x81Q\x92\x95P\x90\x93P\x91P\x15\x80\x80a\x0C\xC5WP\x81[a\r\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FProvided proof is invalid.\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[_\x81a\r+W`@Q\x80` \x01`@R\x80_\x81RPa\rWV[a\rW\x86a\r:`\x01\x88a;\0V[\x81Q\x81\x10a\rJWa\rJa:\xD8V[` \x02` \x01\x01Qa\x15(V[\x91\x9B\x91\x9AP\x90\x98PPPPPPPPPV[_\x81`@\x01Q\x80Q\x90` \x01 a\r\x82\x83_\x01Qa\x15RV[`@Q` \x01a\r\x93\x92\x91\x90a>\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[_\x80_\x80a\r\xBE\x86\x86a\x15\xD2V[\x92P\x92P\x92Pa\r\xCE\x82\x82a\x16\x1BV[P\x90\x94\x93PPPPV[a\r\xE0a8\x11V[_\x82_\x81Q\x81\x10a\r\xF3Wa\r\xF3a:\xD8V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P`\x7F`\xF8\x1B\x81\x10a\x0E\x1FWa\x0E\x18\x83a\x16\xD7V[\x93\x92PPPV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x01`\xF8\x1B\x03a\x0E=Wa\x0E\x18\x83a\x19\rV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x01`\xF9\x1B\x03a\x0E[Wa\x0E\x18\x83a\x1B\xCDV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x03`\xF8\x1B\x03a\x0EyWa\x0E\x18\x83a\x1ERV[`@Qco\xC3\xDA\xA3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x03\x07a\x0E\x9F\x83a!\xDFV[a\x0E\xA8\x84a!\xF7V[a\r\xB0V[``_\x82`@Q` \x01a\x0E\xC3\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_[` \x81\x10\x15a\x0F\x0EW\x81\x81\x81Q\x81\x10a\x0E\xF1Wa\x0E\xF1a:\xD8V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16_\x03a\x0F\x0EW`\x01\x01a\x0E\xD6V[_a\x0F\x1A\x82` a;\0V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F1Wa\x0F1a8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0F[W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a\x0F\xC1W\x83\x83a\x0Ft\x81a>\xE0V[\x94P\x81Q\x81\x10a\x0F\x86Wa\x0F\x86a:\xD8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0F\xA3Wa\x0F\xA3a:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x0F`V[P\x94\x93PPPPV[``\x80\x82Q`\x01\x14\x80\x15a\x0F\xF7WP`\x80\x83_\x81Q\x81\x10a\x0F\xEDWa\x0F\xEDa:\xD8V[\x01` \x01Q`\xF8\x1C\x10[\x15a\x10\x03WP\x81a\x03\x07V[a\x10\x0F\x83Q`\x80a\"SV[\x83`@Q` \x01a\x10!\x92\x91\x90a>\xF8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[``_a\x10D\x83a#\xFAV[\x90P_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10`Wa\x10`a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xA5W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10~W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x11\x1BW_a\x10\xD6\x84\x83\x81Q\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[` \x02` \x01\x01Qa$\x08V[\x90P`@Q\x80`@\x01`@R\x80\x82\x81R` \x01a\x10\xF2\x83a#\xFAV[\x81RP\x83\x83\x81Q\x81\x10a\x11\x07Wa\x11\x07a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x10\xAAV[P\x93\x92PPPV[_``\x81\x80\x80a\x112\x87a$\x95V[\x90P_\x86\x90P_\x80a\x11W`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[_[\x8CQ\x81\x10\x15a\x15\0W\x8C\x81\x81Q\x81\x10a\x11tWa\x11ta:\xD8V[` \x02` \x01\x01Q\x91P\x82\x84a\x11\x8A\x91\x90a>\x91V[\x93Pa\x11\x97`\x01\x88a>\x91V[\x96P\x83_\x03a\x11\xF1W\x81Q\x80Q` \x90\x91\x01 \x85\x14a\x11\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\t-\xCE\xCC-\x8D,\x84\x0EM\xED\xEE\x84\r\x0C.m`{\x1B`D\x82\x01R`d\x01a\x04aV[a\x12\xADV[\x81QQ` \x11a\x12SW\x81Q\x80Q` \x90\x91\x01 \x85\x14a\x11\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FInvalid large internal hash\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[\x84a\x12`\x83_\x01Qa%\xA5V[\x14a\x12\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid internal node hash\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[a\x12\xB9`\x10`\x01a>\x91V[\x82` \x01QQ\x03a\x13)W\x85Q\x84\x14a\x15\0W_\x86\x85\x81Q\x81\x10a\x12\xDFWa\x12\xDFa:\xD8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_\x83` \x01Q\x82`\xFF\x16\x81Q\x81\x10a\x13\tWa\x13\ta:\xD8V[` \x02` \x01\x01Q\x90Pa\x13\x1C\x81a%\xCCV[\x96P`\x01\x94PPPa\x14\xF8V[`\x02\x82` \x01QQ\x03a\x14\xB0W_a\x13@\x83a&\0V[\x90P_\x81_\x81Q\x81\x10a\x13UWa\x13Ua:\xD8V[\x01` \x01Q`\xF8\x1C\x90P_a\x13k`\x02\x83a?\x0CV[a\x13v\x90`\x02a?-V[\x90P_a\x13\x86\x84\x83`\xFF\x16a&#V[\x90P_a\x13\x93\x8B\x8Aa&#V[\x90P_a\x13\xA0\x83\x83a&QV[\x90P`\xFF\x85\x16`\x02\x14\x80a\x13\xB7WP`\xFF\x85\x16`\x03\x14[\x15a\x13\xF1W\x80\x83Q\x14\x80\x15a\x13\xCCWP\x80\x82Q\x14[\x15a\x13\xDEWa\x13\xDB\x81\x8Ba>\x91V[\x99P[P`\x01`\xFF\x1B\x99Pa\x15\0\x94PPPPPV[`\xFF\x85\x16\x15\x80a\x14\x04WP`\xFF\x85\x16`\x01\x14[\x15a\x14YW\x80_\x03a\x14#WP`\x01`\xFF\x1B\x99Pa\x15\0\x94PPPPPV[a\x14J\x88` \x01Q`\x01\x81Q\x81\x10a\x14=Wa\x14=a:\xD8V[` \x02` \x01\x01Qa%\xCCV[\x9AP\x97Pa\x14\xF8\x94PPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FReceived a node with an unknown `D\x82\x01Re\x0E\x0EL\xAC\xCD/`\xD3\x1B`d\x82\x01R`\x84\x01a\x04aV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FReceived an unparsable node.\0\0\0\0`D\x82\x01R`d\x01a\x04aV[`\x01\x01a\x11YV[P`\x01`\xFF\x1B\x84\x14\x86a\x15\x13\x87\x86a&#V[\x90\x9E\x90\x9DP\x90\x9BP\x99PPPPPPPPPPV[` \x81\x01Q\x80Q``\x91a\x03\x07\x91a\x15B\x90`\x01\x90a;\0V[\x81Q\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[`@\x80Q`\x08\x80\x82R\x81\x83\x01\x90\x92R``\x91_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`\x08\x81\x10\x15a\x15\xCBWa\x15\x8C\x81`\x08a>zV[\x84`\x01`\x01`@\x1B\x03\x16\x90\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x15\xADWa\x15\xADa:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x15xV[P\x92\x91PPV[_\x80_\x83Q`A\x03a\x16\tW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa\x15\xFB\x88\x82\x85\x85a&\xCCV[\x95P\x95P\x95PPPPa\x16\x14V[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a\x16.Wa\x16.a>\xA4V[\x03a\x167WPPV[`\x01\x82`\x03\x81\x11\x15a\x16KWa\x16Ka>\xA4V[\x03a\x16iW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x16}Wa\x16}a>\xA4V[\x03a\x16\x9EW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04aV[`\x03\x82`\x03\x81\x11\x15a\x16\xB2Wa\x16\xB2a>\xA4V[\x03a\x16\xD3W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04aV[PPV[a\x16\xDFa8\x11V[_\x80\x82Ra\x16\xEFa\x02\x18\x84a\x03\xC3V[\x90P\x80Q`\t\x14\x15\x80\x15a\x17\x05WP\x80Q`\x06\x14\x15[\x15a\x17#W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x178\x81_\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[``\x83\x01R\x80Qa\x17V\x90\x82\x90`\x01\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x80\x83\x01R\x80Qa\x17t\x90\x82\x90`\x02\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xE0\x83\x01R\x80Qa\x17\x9F\x90\x82\x90`\x03\x90\x81\x10a\x17\x92Wa\x17\x92a:\xD8V[` \x02` \x01\x01Qa'\x94V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x83\x01R\x80Qa\x17\xC7\x90\x82\x90`\x04\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x01 \x83\x01R\x80Qa\x17\xE6\x90\x82\x90`\x05\x90\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[a\x01@\x83\x01R\x80Q`\x06\x03a\x17\xFBWP\x91\x90PV[_a\x18\x12\x82`\x06\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[\x90P_a\x18+\x83`\x07\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[\x90P_a\x18D\x84`\x08\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[\x90P\x81\x15\x80\x15a\x18RWP\x80\x15[\x15a\x18rW`\x01`\x01`@\x1B\x03\x83\x16` \x86\x01R`\x01`@\x86\x01Ra\x19\x04V[`#\x83`\x01`\x01`@\x1B\x03\x16\x10a\x18\xBAW`\x02a\x18\x90`#\x85a?FV[a\x18\x9A\x91\x90a?fV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x87\x01R\x83\x16a\x01\xE0\x86\x01R`\x01`@\x86\x01R[_a\x18\xC9`\x02`\x01\x86\x18a?\x8BV[a\x18\xD4\x90`\x1Ba?\xB0V[`@Q\x90\x91Pa\x18\xEC\x90\x84\x90\x84\x90\x84\x90` \x01a?\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x87\x01RP[PPPP\x91\x90PV[a\x19\x15a8\x11V[`\x01\x80\x82R\x82Q_\x91a\x195\x91a\x19-\x90\x82\x90a;\0V[\x85\x91\x90a(\0V[\x90P_a\x19Da\x02\x18\x83a\x03\xC3V[\x90P\x80Q`\x08\x14\x15\x80\x15a\x19ZWP\x80Q`\x0B\x14\x15[\x15a\x19xW`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\x8D\x81_\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa\x19\xB4\x90\x82\x90`\x01\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[``\x84\x01R\x80Qa\x19\xD2\x90\x82\x90`\x02\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x80\x84\x01R\x80Qa\x19\xF0\x90\x82\x90`\x03\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xE0\x84\x01R\x80Qa\x1A\x0E\x90\x82\x90`\x04\x90\x81\x10a\x17\x92Wa\x17\x92a:\xD8V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\x1A6\x90\x82\x90`\x05\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x01 \x84\x01R\x80Qa\x1AU\x90\x82\x90`\x06\x90\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[\x83a\x01@\x01\x81\x90RP_a\x1A\x82\x82`\x07\x81Q\x81\x10a\x1AuWa\x1Aua:\xD8V[` \x02` \x01\x01Qa\x03\xEFV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x9DWa\x1A\x9Da8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xD0W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A\xBBW\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a\x1B.Wa\x1B\x04\x82\x82\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a:\xD8V[` \x02` \x01\x01Qa)@V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a\x1B\x1BWa\x1B\x1Ba:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xD9V[P\x81Q`\x08\x03a\x1B@WPPP\x91\x90PV[_a\x1BW\x83`\x08\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x1Bb\x90`\x1Ba?\xF3V[\x90P_a\x1B{\x84`\t\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P_a\x1B\x94\x85`\n\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P\x81\x81\x84`@Q` \x01a\x1B\xAB\x93\x92\x91\x90a?\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x88\x01RP\x94\x96\x95PPPPPPV[a\x1B\xD5a8\x11V[`\x02\x81R\x81Q_\x90a\x1B\xEF\x90`\x01\x90a\x19-\x90\x82\x90a;\0V[\x90P_a\x1B\xFEa\x02\x18\x83a\x03\xC3V[\x90P\x80Q`\t\x14\x15\x80\x15a\x1C\x14WP\x80Q`\x0C\x14\x15[\x15a\x1C2W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1CG\x81_\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa\x1Cn\x90\x82\x90`\x01\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[``\x84\x01R\x80Qa\x1C\x8C\x90\x82\x90`\x02\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xA0\x84\x01R\x80Qa\x1C\xAA\x90\x82\x90`\x03\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xC0\x84\x01R\x80Qa\x1C\xC8\x90\x82\x90`\x04\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xE0\x84\x01R\x80Qa\x1C\xE6\x90\x82\x90`\x05\x90\x81\x10a\x17\x92Wa\x17\x92a:\xD8V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\x1D\x0E\x90\x82\x90`\x06\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x01 \x84\x01R\x80Qa\x1D-\x90\x82\x90`\x07\x90\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[\x83a\x01@\x01\x81\x90RP_a\x1DM\x82`\x08\x81Q\x81\x10a\x1AuWa\x1Aua:\xD8V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DhWa\x1Dha8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x9BW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1D\x86W\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a\x1D\xECWa\x1D\xC2\x82\x82\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a:\xD8V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a\x1D\xD9Wa\x1D\xD9a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1D\xA4V[P\x81Q`\t\x03a\x1D\xFEWPPP\x91\x90PV[_a\x1E\x15\x83`\t\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x1E \x90`\x1Ba?\xF3V[\x90P_a\x1E9\x84`\n\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P_a\x1B\x94\x85`\x0B\x81Q\x81\x10a\x024Wa\x024a:\xD8V[a\x1EZa8\x11V[`\x03\x81R\x81Q_\x90a\x1Et\x90`\x01\x90a\x19-\x90\x82\x90a;\0V[\x90P_a\x1E\x83a\x02\x18\x83a\x03\xC3V[\x90P\x80Q`\x0B\x14\x15\x80\x15a\x1E\x99WP\x80Q`\x0E\x14\x15[\x15a\x1E\xB7W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xCC\x81_\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa\x1E\xF3\x90\x82\x90`\x01\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[``\x84\x01R\x80Qa\x1F\x11\x90\x82\x90`\x02\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xA0\x84\x01R\x80Qa\x1F/\x90\x82\x90`\x03\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xC0\x84\x01R\x80Qa\x1FM\x90\x82\x90`\x04\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xE0\x84\x01R\x80Qa\x1Fk\x90\x82\x90`\x05\x90\x81\x10a\x17\x92Wa\x17\x92a:\xD8V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\x1F\x93\x90\x82\x90`\x06\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x01 \x84\x01R\x80Qa\x1F\xB2\x90\x82\x90`\x07\x90\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[\x83a\x01@\x01\x81\x90RP_a\x1F\xD2\x82`\x08\x81Q\x81\x10a\x1AuWa\x1Aua:\xD8V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xEDWa\x1F\xEDa8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a  W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a \x0BW\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a qWa G\x82\x82\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a:\xD8V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a ^Wa ^a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a )V[Pa \x88\x82`\t\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[\x84a\x01\x80\x01\x81\x81RPP_a \xA9\x83`\n\x81Q\x81\x10a\x1AuWa\x1Aua:\xD8V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a \xC4Wa \xC4a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xEDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[Pa\x01\xA0\x86\x01R_[\x81Q\x81\x10\x15a!>Wa!\x14\x82\x82\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x86a\x01\xA0\x01Q\x82\x81Q\x81\x10a!+Wa!+a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a \xF6V[P\x82Q`\x0B\x03a!QWPPPP\x91\x90PV[_a!h\x84`\x0B\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a!s\x90`\x1Ba?\xF3V[\x90P_a!\x8C\x85`\x0C\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P_a!\xA5\x86`\r\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P\x81\x81\x84`@Q` \x01a!\xBC\x93\x92\x91\x90a?\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x89\x01RP\x95\x97\x96PPPPPPPV[_a!\xE9\x82a)KV[\x80Q\x90` \x01 \x90P\x91\x90PV[``\x81a\x01\xC0\x01QQ_\x03a\"\x1FW`@Qc\xD4f\xBD\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x01\xC0\x01QQ`A\x14a\"FW`@QcK\xE62\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x01\xC0\x81\x01Q[\x91\x90PV[``\x80`8\x84\x10\x15a\"\xB8W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90Pa\"\x88\x83\x85a?\xF3V[`\xF8\x1B\x81_\x81Q\x81\x10a\"\x9DWa\"\x9Da:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa\x0E\x18V[_`\x01[a\"\xC6\x81\x87a;'V[\x15a\"\xECW\x81a\"\xD5\x81a>\xE0V[\x92Pa\"\xE5\x90Pa\x01\0\x82a>zV[\x90Pa\"\xBCV[a\"\xF7\x82`\x01a>\x91V[`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x0EWa#\x0Ea8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#8W` \x82\x01\x81\x806\x837\x01\x90P[P\x92Pa#E\x85\x83a?\xF3V[a#P\x90`7a?\xF3V[`\xF8\x1B\x83_\x81Q\x81\x10a#eWa#ea:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x90P[\x81\x81\x11a#\xF1Wa\x01\0a#\x94\x82\x84a;\0V[a#\xA0\x90a\x01\0a@\xECV[a#\xAA\x90\x88a;'V[a#\xB4\x91\x90a@\xF7V[`\xF8\x1B\x83\x82\x81Q\x81\x10a#\xC9Wa#\xC9a:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x80a#\xE9\x81a>\xE0V[\x91PPa#\x80V[PP\x93\x92PPPV[``a\x03\x07a\x02\x18\x83a\x03\xC3V[``_\x80_a$\x16\x85a\x08\xFFV[\x91\x94P\x92P\x90P_\x81`\x01\x81\x11\x15a$0Wa$0a>\xA4V[\x14a$}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid RLP bytes value.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[a$\x8C\x85` \x01Q\x84\x84a)\xD8V[\x95\x94PPPPPV[``_\x82Q`\x02\x02`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xB4Wa$\xB4a8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a$\xDEW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x15\xCBW`\x04\x84\x82\x81Q\x81\x10a%\0Wa%\0a:\xD8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x82\x82`\x02\x02\x81Q\x81\x10a%,Wa%,a:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x10\x84\x82\x81Q\x81\x10a%VWa%Va:\xD8V[\x01` \x01Q`\xF8\x1C\x81a%kWa%ka;\x13V[\x06`\xF8\x1B\x82\x82`\x02\x02`\x01\x01\x81Q\x81\x10a%\x87Wa%\x87a:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a$\xE3V[_` \x82Q\x10\x15a%\xB8WP` \x01Q\x90V[\x81\x80` \x01\x90Q\x81\x01\x90a\x03\x07\x91\x90aA\nV[_``` \x83_\x01Q\x10\x15a%\xEBWa%\xE4\x83a)@V[\x90Pa%\xF7V[a%\xF4\x83a$\x08V[\x90P[a\x0E\x18\x81a%\xA5V[``a\x03\x07a&\x1E\x83` \x01Q_\x81Q\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[a$\x95V[``\x81\x83Q\x03_\x03a&CWP`@\x80Q` \x81\x01\x90\x91R_\x81Ra\x03\x07V[a\x0E\x18\x83\x83\x84\x86Q\x03a(\0V[_\x80[\x80\x84Q\x11\x80\x15a&dWP\x80\x83Q\x11[\x80\x15a&\xB5WP\x82\x81\x81Q\x81\x10a&}Wa&}a:\xD8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a&\xA4Wa&\xA4a:\xD8V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14[\x15a\x0E\x18W\x80a&\xC4\x81a>\xE0V[\x91PPa&TV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a'\x05WP_\x91P`\x03\x90P\x82a'\x8AV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a'VW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a'\x81WP_\x92P`\x01\x91P\x82\x90Pa'\x8AV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[\x80Q_\x90`\x01\x03a'\xA6WP_\x91\x90PV[\x81Q`\x15\x14a'\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP address value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[a\x03\x07\x82a\x06\xD7V[``\x81\x82`\x1F\x01\x10\x15a(FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x04aV[\x82\x82\x84\x01\x10\x15a(\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x04aV[\x81\x83\x01\x84Q\x10\x15a(\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x04aV[``\x82\x15\x80\x15a(\xEEW`@Q\x91P_\x82R` \x82\x01`@Ra\x0F\xC1V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a)'W\x80Q\x83R` \x92\x83\x01\x92\x01a)\x0FV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RPP\x94\x93PPPPV[``a\x03\x07\x82a*\x85V[``_\x82Q`\x03\x81\x11\x15a)aWa)aa>\xA4V[\x03a)oWa\x03\x07\x82a*\x99V[`\x01\x82Q`\x03\x81\x11\x15a)\x84Wa)\x84a>\xA4V[\x03a)\x92Wa\x03\x07\x82a-\x85V[`\x02\x82Q`\x03\x81\x11\x15a)\xA7Wa)\xA7a>\xA4V[\x03a)\xB5Wa\x03\x07\x82a/\xF6V[`\x03\x82Q`\x03\x81\x11\x15a)\xCAWa)\xCAa>\xA4V[\x03a\x0EyWa\x03\x07\x82a2dV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xF3Wa)\xF3a8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a*\x1DW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x80Q_\x03a*/W\x90Pa\x0E\x18V[\x84\x84\x01` \x82\x01_[` \x86\x04\x81\x10\x15a*YW\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a*8V[P_`\x01` \x87\x06` \x03a\x01\0\n\x03\x90P\x80\x82Q\x16\x81\x19\x84Q\x16\x17\x82R\x83\x94PPPPP\x93\x92PPPV[``a\x03\x07\x82` \x01Q_\x84_\x01Qa)\xD8V[``_\x82` \x01Q`\x01`\x01`@\x1B\x03\x16_\x14a*\xBBWP` \x82\x01Qa*\xFEV[a\x01\xC0\x83\x01QQ\x15a*\xFEW`#\x83a\x01\xE0\x01Q`\x01`\x01`@\x1B\x03\x16\x10a*\xFEW`\x02`#\x84a\x01\xE0\x01Qa*\xF1\x91\x90a?FV[a*\xFB\x91\x90a?fV[\x90P[_\x83`@\x01Qa+\x0EW_a+\x11V[`\x03[a+\x1C\x90`\x06a?\xF3V[`\xFF\x16\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a+:Wa+:a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+mW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a+XW\x90P[P\x90Pa+}\x85``\x01Qa\x0C\x81V[\x81_\x81Q\x81\x10a+\x8FWa+\x8Fa:\xD8V[` \x02` \x01\x01\x81\x90RPa+\xA7\x85`\x80\x01Qa\x0C\x81V[\x81`\x01\x81Q\x81\x10a+\xBAWa+\xBAa:\xD8V[` \x02` \x01\x01\x81\x90RPa+\xD2\x85`\xE0\x01Qa\x0C\x81V[\x81`\x02\x81Q\x81\x10a+\xE5Wa+\xE5a:\xD8V[` \x02` \x01\x01\x81\x90RPa+\xFE\x85a\x01\0\x01Qa6\x07V[\x81`\x03\x81Q\x81\x10a,\x11Wa,\x11a:\xD8V[` \x02` \x01\x01\x81\x90RPa,*\x85a\x01 \x01Qa\x0C\x81V[\x81`\x04\x81Q\x81\x10a,=Wa,=a:\xD8V[` \x02` \x01\x01\x81\x90RPa,V\x85a\x01@\x01Qa\x0F\xCAV[\x81`\x05\x81Q\x81\x10a,iWa,ia:\xD8V[` \x02` \x01\x01\x81\x90RP\x84`@\x01Q\x15a-|W\x84` \x01Q`\x01`\x01`@\x1B\x03\x16_\x03a,\xD0W`@Q_` \x82\x01R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81`\x06\x81Q\x81\x10a,\xC0Wa,\xC0a:\xD8V[` \x02` \x01\x01\x81\x90RPa-\x01V[a,\xE2\x83`\x01`\x01`@\x1B\x03\x16a\x0C\x81V[\x81`\x06\x81Q\x81\x10a,\xF5Wa,\xF5a:\xD8V[` \x02` \x01\x01\x81\x90RP[a-6_[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a-0W` \x82\x01\x81\x806\x837\x01\x90P[Pa\x0F\xCAV[\x81`\x07\x81Q\x81\x10a-IWa-Ia:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra-]_a-\x06V[\x81`\x08\x81Q\x81\x10a-pWa-pa:\xD8V[` \x02` \x01\x01\x81\x90RP[a$\x8C\x81a6?V[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a-\xA1W\x90PP\x90Pa-\xCE\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x0C\x81V[\x81_\x81Q\x81\x10a-\xE0Wa-\xE0a:\xD8V[` \x02` \x01\x01\x81\x90RPa-\xF8\x83``\x01Qa\x0C\x81V[\x81`\x01\x81Q\x81\x10a.\x0BWa.\x0Ba:\xD8V[` \x02` \x01\x01\x81\x90RPa.#\x83`\x80\x01Qa\x0C\x81V[\x81`\x02\x81Q\x81\x10a.6Wa.6a:\xD8V[` \x02` \x01\x01\x81\x90RPa.N\x83`\xE0\x01Qa\x0C\x81V[\x81`\x03\x81Q\x81\x10a.aWa.aa:\xD8V[` \x02` \x01\x01\x81\x90RPa.z\x83a\x01\0\x01Qa6\x07V[\x81`\x04\x81Q\x81\x10a.\x8DWa.\x8Da:\xD8V[` \x02` \x01\x01\x81\x90RPa.\xA6\x83a\x01 \x01Qa\x0C\x81V[\x81`\x05\x81Q\x81\x10a.\xB9Wa.\xB9a:\xD8V[` \x02` \x01\x01\x81\x90RPa.\xD2\x83a\x01@\x01Qa\x0F\xCAV[\x81`\x06\x81Q\x81\x10a.\xE5Wa.\xE5a:\xD8V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x0FWa/\x0Fa8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a/BW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a/-W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a/\x99W\x84a\x01`\x01Q\x81\x81Q\x81\x10a/lWa/la:\xD8V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a/\x86Wa/\x86a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a/GV[Pa/\xA3\x81a6?V[\x82`\x07\x81Q\x81\x10a/\xB6Wa/\xB6a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01[a/\xCD\x83a6?V[`@Q` \x01a/\xDE\x92\x91\x90aA!V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`@\x80Q`\t\x80\x82Ra\x01@\x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a0\x12W\x90PP\x90Pa0?\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x0C\x81V[\x81_\x81Q\x81\x10a0QWa0Qa:\xD8V[` \x02` \x01\x01\x81\x90RPa0i\x83``\x01Qa\x0C\x81V[\x81`\x01\x81Q\x81\x10a0|Wa0|a:\xD8V[` \x02` \x01\x01\x81\x90RPa0\x94\x83`\xA0\x01Qa\x0C\x81V[\x81`\x02\x81Q\x81\x10a0\xA7Wa0\xA7a:\xD8V[` \x02` \x01\x01\x81\x90RPa0\xBF\x83`\xC0\x01Qa\x0C\x81V[\x81`\x03\x81Q\x81\x10a0\xD2Wa0\xD2a:\xD8V[` \x02` \x01\x01\x81\x90RPa0\xEA\x83`\xE0\x01Qa\x0C\x81V[\x81`\x04\x81Q\x81\x10a0\xFDWa0\xFDa:\xD8V[` \x02` \x01\x01\x81\x90RPa1\x16\x83a\x01\0\x01Qa6\x07V[\x81`\x05\x81Q\x81\x10a1)Wa1)a:\xD8V[` \x02` \x01\x01\x81\x90RPa1B\x83a\x01 \x01Qa\x0C\x81V[\x81`\x06\x81Q\x81\x10a1UWa1Ua:\xD8V[` \x02` \x01\x01\x81\x90RPa1n\x83a\x01@\x01Qa\x0F\xCAV[\x81`\x07\x81Q\x81\x10a1\x81Wa1\x81a:\xD8V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xABWa1\xABa8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a1\xDEW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a1\xC9W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a25W\x84a\x01`\x01Q\x81\x81Q\x81\x10a2\x08Wa2\x08a:\xD8V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a2\"Wa2\"a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a1\xE3V[Pa2?\x81a6?V[\x82`\x08\x81Q\x81\x10a2RWa2Ra:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x02a/\xC4V[`@\x80Q`\x0B\x80\x82Ra\x01\x80\x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a2\x80W\x90PP\x90Pa2\xAD\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x0C\x81V[\x81_\x81Q\x81\x10a2\xBFWa2\xBFa:\xD8V[` \x02` \x01\x01\x81\x90RPa2\xD7\x83``\x01Qa\x0C\x81V[\x81`\x01\x81Q\x81\x10a2\xEAWa2\xEAa:\xD8V[` \x02` \x01\x01\x81\x90RPa3\x02\x83`\xA0\x01Qa\x0C\x81V[\x81`\x02\x81Q\x81\x10a3\x15Wa3\x15a:\xD8V[` \x02` \x01\x01\x81\x90RPa3-\x83`\xC0\x01Qa\x0C\x81V[\x81`\x03\x81Q\x81\x10a3@Wa3@a:\xD8V[` \x02` \x01\x01\x81\x90RPa3X\x83`\xE0\x01Qa\x0C\x81V[\x81`\x04\x81Q\x81\x10a3kWa3ka:\xD8V[` \x02` \x01\x01\x81\x90RPa3\x84\x83a\x01\0\x01Qa6\x07V[\x81`\x05\x81Q\x81\x10a3\x97Wa3\x97a:\xD8V[` \x02` \x01\x01\x81\x90RPa3\xB0\x83a\x01 \x01Qa\x0C\x81V[\x81`\x06\x81Q\x81\x10a3\xC3Wa3\xC3a:\xD8V[` \x02` \x01\x01\x81\x90RPa3\xDC\x83a\x01@\x01Qa\x0F\xCAV[\x81`\x07\x81Q\x81\x10a3\xEFWa3\xEFa:\xD8V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a4\x19Wa4\x19a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a4LW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a47W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a4\xA3W\x84a\x01`\x01Q\x81\x81Q\x81\x10a4vWa4va:\xD8V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a4\x90Wa4\x90a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a4QV[Pa4\xAD\x81a6?V[\x82`\x08\x81Q\x81\x10a4\xC0Wa4\xC0a:\xD8V[` \x02` \x01\x01\x81\x90RPa4\xD9\x84a\x01\x80\x01Qa\x0C\x81V[\x82`\t\x81Q\x81\x10a4\xECWa4\xECa:\xD8V[` \x02` \x01\x01\x81\x90RP_\x84a\x01\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x16Wa5\x16a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5IW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a54W\x90P[P\x90P_[\x85a\x01\xA0\x01QQ\x81\x10\x15a5\xAAWa5\x85\x86a\x01\xA0\x01Q\x82\x81Q\x81\x10a5vWa5va:\xD8V[` \x02` \x01\x01Q_\x1Ca\x0C\x81V[\x82\x82\x81Q\x81\x10a5\x97Wa5\x97a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a5NV[Pa5\xB4\x81a6?V[\x83`\n\x81Q\x81\x10a5\xC7Wa5\xC7a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x03a5\xDD\x84a6?V[`@Q` \x01a5\xEE\x92\x91\x90aA!V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x91\x90PV[`@Q``\x82\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x83\x01R\x90a\x03\x07\x90`4\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0F\xCAV[``_a6K\x83a6\x82V[\x90Pa6Y\x81Q`\xC0a\"SV[\x81`@Q` \x01a6k\x92\x91\x90a>\xF8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[``\x81Q_\x03a6\xA1W`@\x80Q_\x80\x82R` \x82\x01\x90\x92R\x90a\x15\xCBV[_\x80[\x83Q\x81\x10\x15a6\xDDW\x83\x81\x81Q\x81\x10a6\xBFWa6\xBFa:\xD8V[` \x02` \x01\x01QQ\x82a6\xD3\x91\x90a>\x91V[\x91P`\x01\x01a6\xA4V[_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xF6Wa6\xF6a8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a7 W` \x82\x01\x81\x806\x837\x01\x90P[P_\x92P\x90P` \x81\x01[\x85Q\x83\x10\x15a\x0F\xC1W_\x86\x84\x81Q\x81\x10a7GWa7Ga:\xD8V[` \x02` \x01\x01Q\x90P_` \x82\x01\x90Pa7d\x83\x82\x84Qa7\x9AV[\x87\x85\x81Q\x81\x10a7vWa7va:\xD8V[` \x02` \x01\x01QQ\x83a7\x8A\x91\x90a>\x91V[`\x01\x90\x95\x01\x94\x92Pa7+\x91PPV[\x82\x82\x82[` \x81\x10a7\xD6W\x81Q\x83Ra7\xB5` \x84a>\x91V[\x92Pa7\xC2` \x83a>\x91V[\x91Pa7\xCF` \x82a;\0V[\x90Pa7\x9EV[_`\x01a7\xE4\x83` a;\0V[a7\xF0\x90a\x01\0a@\xECV[a7\xFA\x91\x90a;\0V[\x92Q\x84Q\x84\x16\x93\x19\x16\x92\x90\x92\x17\x90\x92RPPPPPV[`@\x80Qa\x02\0\x81\x01\x90\x91R\x80_\x81R` \x01_`\x01`\x01`@\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_`\x01`\x01`@\x1B\x03\x16\x81RP\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8\xD6Wa8\xD6a8\xA0V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a9\x04Wa9\x04a8\xA0V[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a9\x1BW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a94Wa94a8\xA0V[a9G`\x1F\x82\x01`\x1F\x19\x16` \x01a8\xDCV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a9[W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a9\x87W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x9CW_\x80\xFD[a9\xA8\x84\x82\x85\x01a9\x0CV[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a9\xC0W_\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"NW_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a9\xEDW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x03W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a:\x1AW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a:6W_\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:LW_\x80\xFD[\x90\x88\x01\x90a\x01`\x82\x8B\x03\x12\x15a:`W_\x80\xFD[\x90\x96P` \x88\x015\x90\x80\x82\x11\x15a:uW_\x80\xFD[\x90\x88\x01\x90`\x80\x82\x8B\x03\x12\x15a:\x88W_\x80\xFD[\x81\x96Pa:\x97`@\x8A\x01a9\xC7V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a:\xACW_\x80\xFD[Pa:\xB9\x89\x82\x8A\x01a9\xDDV[\x90\x94P\x92Pa:\xCC\x90P`\x80\x88\x01a9\xC7V[\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x07Wa\x03\x07a:\xECV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a;5Wa;5a;\x13V[P\x04\x90V[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a;OW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a;hW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a:\x1AW_\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\"NW_\x80\xFD[_` \x82\x84\x03\x12\x15a;\xA2W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\xB8W_\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a;\xCBW_\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a;\xE6Wa;\xE6a8\xA0V[`@Ra;\xF2\x83a;|V[\x81R` \x83\x015\x82\x81\x11\x15a<\x05W_\x80\xFD[a<\x11\x87\x82\x86\x01a9\x0CV[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a<(W_\x80\xFD[a<4\x87\x82\x86\x01a9\x0CV[`@\x83\x01RP\x95\x94PPPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a<[Wa<[a8\xA0V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a<tW_\x80\xFD[\x815` a<\x89a<\x84\x83a<CV[a8\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<\xA7W_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<\xE4W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xC8W_\x80\xFD[a<\xD6\x89\x86\x83\x8B\x01\x01a9\x0CV[\x84RP\x91\x83\x01\x91\x83\x01a<\xABV[P\x96\x95PPPPPPV[_\x82`\x1F\x83\x01\x12a<\xFEW_\x80\xFD[\x815` a=\x0Ea<\x84\x83a<CV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a=/W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15a<\xE4W\x805\x83R\x91\x83\x01\x91\x83\x01a=4V[_` \x82\x84\x03\x12\x15a=[W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=qW_\x80\xFD[\x90\x83\x01\x90`\xC0\x82\x86\x03\x12\x15a=\x84W_\x80\xFD[a=\x8Ca8\xB4V[\x825\x81R` \x83\x015\x82\x81\x11\x15a=\xA1W_\x80\xFD[a=\xAD\x87\x82\x86\x01a9\x0CV[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a=\xC4W_\x80\xFD[a=\xD0\x87\x82\x86\x01a9\x0CV[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a=\xE7W_\x80\xFD[a=\xF3\x87\x82\x86\x01a9\x0CV[``\x83\x01RP`\x80\x83\x015\x82\x81\x11\x15a>\nW_\x80\xFD[a>\x16\x87\x82\x86\x01a<eV[`\x80\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a>-W_\x80\xFD[a>9\x87\x82\x86\x01a<\xEFV[`\xA0\x83\x01RP\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a>XW_\x80\xFD[a\x0E\x18\x82a;|V[_` \x82\x84\x03\x12\x15a>qW_\x80\xFD[a\x0E\x18\x82a9\xC7V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x07Wa\x03\x07a:\xECV[\x80\x82\x01\x80\x82\x11\x15a\x03\x07Wa\x03\x07a:\xECV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x82\x81R_a9\xA8` \x83\x01\x84a>\xB8V[_`\x01\x82\x01a>\xF1Wa>\xF1a:\xECV[P`\x01\x01\x90V[_a9\xA8a?\x06\x83\x86a>\xB8V[\x84a>\xB8V[_`\xFF\x83\x16\x80a?\x1EWa?\x1Ea;\x13V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x03\x07Wa\x03\x07a:\xECV[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x15\xCBWa\x15\xCBa:\xECV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a?\x7FWa?\x7Fa;\x13V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a?\xA4Wa?\xA4a;\x13V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x15\xCBWa\x15\xCBa:\xECV[\x92\x83R` \x83\x01\x91\x90\x91R`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`@\x82\x01R`A\x01\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\x07Wa\x03\x07a:\xECV[`\x01\x81\x81[\x80\x85\x11\x15a@FW\x81_\x19\x04\x82\x11\x15a@,Wa@,a:\xECV[\x80\x85\x16\x15a@9W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a@\x11V[P\x92P\x92\x90PV[_\x82a@\\WP`\x01a\x03\x07V[\x81a@hWP_a\x03\x07V[\x81`\x01\x81\x14a@~W`\x02\x81\x14a@\x88Wa@\xA4V[`\x01\x91PPa\x03\x07V[`\xFF\x84\x11\x15a@\x99Wa@\x99a:\xECV[PP`\x01\x82\x1Ba\x03\x07V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a@\xC7WP\x81\x81\na\x03\x07V[a@\xD1\x83\x83a@\x0CV[\x80_\x19\x04\x82\x11\x15a@\xE4Wa@\xE4a:\xECV[\x02\x93\x92PPPV[_a\x0E\x18\x83\x83a@NV[_\x82aA\x05WaA\x05a;\x13V[P\x06\x90V[_` \x82\x84\x03\x12\x15aA\x1AW_\x80\xFD[PQ\x91\x90PV[`\xF8\x83\x90\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x81R_a9\xA8`\x01\x83\x01\x84a>\xB8V\xFE\xA2dipfsX\"\x12 \xD8\xCC`O\xA7REN\x19\x8FxRx\xBAr\xBD]\xBAO\xA5\x0E~B\xA0\x9DG\xA4\xDBKT\xC2\x05dsolcC\0\x08\x19\x003",
    );
	/// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50600436106100b1575f3560e01c80639f329d0b1161006e5780639f329d0b14610178578063abeeb36614610181578063c239ef4214610194578063cac62dd9146101a7578063f45e8118146101b0578063f5beea8c146101b9575f80fd5b8063189cc9d0146100b55780632a04ff85146100d05780634968f6c5146100d857806356b4a92a146100e0578063743770291461013e57806386fb61ed14610146575b5f80fd5b6100bd6101cc565b6040519081526020015b60405180910390f35b6100bd600c81565b6100bd5f5481565b6100f36100ee366004613977565b6101db565b6040516100c791905f60c082019050825182526020830151602083015260408301516040830152606083015160608301526080830151608083015260a083015160a083015292915050565b6100bd602081565b610160720f3df6d732807ef1319fb7b8bb8522d0beac0281565b6040516001600160a01b0390911681526020016100c7565b6100bd60015481565b6100bd61018f3660046139b0565b6102ec565b6100bd6101a2366004613a21565b61030d565b6100bd611fff81565b6100bd61010081565b6100bd6101c73660046139b0565b6103aa565b5f6101d6426102ec565b905090565b6040805160c0810182525f80825260208201819052918101829052606081018290526080810182905260a081018290529061021d610218846103c3565b6103ef565b9050610241815f8151811061023457610234613ad8565b60200260200101516105da565b8252805161025c908290600390811061023457610234613ad8565b6020830152805161027a908290600490811061023457610234613ad8565b604083015280516102a5908290600890811061029857610298613ad8565b60200260200101516106d7565b606083015280516102c3908290600b90811061029857610298613ad8565b608083015280516102e1908290600f90811061029857610298613ad8565b60a083015250919050565b5f600c600154836102fd9190613b00565b6103079190613b27565b92915050565b5f8061031c6020880188613b3a565b8101906103299190613b92565b90505f61033885870187613d4b565b82519091506001600160401b03166103586101408b016101208c01613e48565b6001600160401b03161461037f576040516330d3ba0760e01b815260040160405180910390fd5b61039b82826103966101208d016101008e01613e61565b6106e1565b50505f54979650505050505050565b5f6103b6600c83613e7a565b6001546103079190613e91565b6040805180820182525f8082526020918201528151808301909252825182529182019181019190915290565b60605f806103fc846108ff565b9193509091506001905081600181111561041857610418613ea4565b1461046a5760405162461bcd60e51b815260206004820152601760248201527f496e76616c696420524c50206c6973742076616c75652e00000000000000000060448201526064015b60405180910390fd5b60408051602080825261042082019092525f91816020015b604080518082019091525f80825260208201528152602001906001900390816104825790505090505f835b86518110156105cf57602082106105195760405162461bcd60e51b815260206004820152602a60248201527f50726f766964656420524c50206c6973742065786365656473206d6178206c6960448201526939ba103632b733ba341760b11b6064820152608401610461565b5f806105546040518060400160405280858c5f01516105389190613b00565b8152602001858c6020015161054d9190613e91565b90526108ff565b5091509150604051806040016040528083836105709190613e91565b8152602001848b602001516105859190613e91565b81525085858151811061059a5761059a613ad8565b60209081029190910101526105b0600185613e91565b93506105bc8183613e91565b6105c69084613e91565b925050506104ad565b508152949350505050565b5f6021825f0151111561062f5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020627974657333322076616c75652e0000000000006044820152606401610461565b5f805f61063b856108ff565b919450925090505f81600181111561065557610655613ea4565b146106a25760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020627974657333322076616c75652e0000000000006044820152606401610461565b5f8386602001516106b39190613e91565b805190915060208410156106cd5760208490036101000a90045b9695505050505050565b5f610307826105da565b82516001600160401b031660206106f66101cc565b6107009190613b00565b8111156107205760405163b6144bff60e01b815260040160405180910390fd5b61010061072b6101cc565b6107359190613b00565b81101561075557604051630cdceb7960e21b815260040160405180910390fd5b82515f9061076590600190613b00565b90504381118061077f575061077c61010043613b00565b81105b1561079d57604051631391e11b60e21b815260040160405180910390fd5b83515f906107ad90600190613b00565b6020808701518051910120904091508082146107dc57604051631f03465b60e11b815260040160405180910390fd5b5f806107e789610bf9565b9250925050816001600160a01b0316876001600160a01b03161461081e5760405163aaaa914160e01b815260040160405180910390fd5b5f61082c89604001516101db565b80519091508414610850576040516320fa6c8b60e11b815260040160405180910390fd5b5f6108778a60a001515f8151811061086a5761086a613ad8565b6020026020010151610c81565b90505f806108a7838d608001515f8151811061089557610895613ad8565b60200260200101518660400151610c94565b91509150816108c95760405163094cec5f60e01b815260040160405180910390fd5b805160208201208551146108f057604051634462b38f60e11b815260040160405180910390fd5b50505050505050505050505050565b5f805f80845f0151116109545760405162461bcd60e51b815260206004820152601860248201527f524c50206974656d2063616e6e6f74206265206e756c6c2e00000000000000006044820152606401610461565b602084015180515f1a607f8111610976575f60015f9450945094505050610bf2565b60b781116109e5578551607f1982019081106109d45760405162461bcd60e51b815260206004820152601960248201527f496e76616c696420524c502073686f727420737472696e672e000000000000006044820152606401610461565b6001955093505f9250610bf2915050565b60bf8111610abc57855160b6198201908110610a435760405162461bcd60e51b815260206004820152601f60248201527f496e76616c696420524c50206c6f6e6720737472696e67206c656e6774682e006044820152606401610461565b5f816020036101000a6001850151049050808201885f015111610aa85760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420524c50206c6f6e6720737472696e672e00000000000000006044820152606401610461565b6001909101955093505f9250610bf2915050565b60f78111610b2b57855160bf198201908110610b1a5760405162461bcd60e51b815260206004820152601760248201527f496e76616c696420524c502073686f7274206c6973742e0000000000000000006044820152606401610461565b600195509350849250610bf2915050565b855160f6198201908110610b815760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420524c50206c6f6e67206c697374206c656e6774682e0000006044820152606401610461565b5f816020036101000a6001850151049050808201885f015111610bdf5760405162461bcd60e51b815260206004820152601660248201527524b73b30b634b210292628103637b733903634b9ba1760511b6044820152606401610461565b6001918201965094509250610bf2915050565b9193909250565b604080516060810182525f808252602082018190529181018290528190610c2c610c2285610d69565b8560200151610db0565b91505f610c3c8560400151610dd8565b9050610c4781610e92565b935060405180606001604052808660400151805190602001208152602001826060015181526020018260e001518152509150509193909250565b6060610307610c8f83610ead565b610fca565b5f60605f610ca185611038565b90505f805f610cb1848a89611123565b81519295509093509150158080610cc55750815b610d115760405162461bcd60e51b815260206004820152601a60248201527f50726f76696465642070726f6f6620697320696e76616c69642e0000000000006044820152606401610461565b5f81610d2b5760405180602001604052805f815250610d57565b610d5786610d3a600188613b00565b81518110610d4a57610d4a613ad8565b6020026020010151611528565b919b919a509098505050505050505050565b5f816040015180519060200120610d82835f0151611552565b604051602001610d93929190613ecf565b604051602081830303815290604052805190602001209050919050565b5f805f80610dbe86866115d2565b925092509250610dce828261161b565b5090949350505050565b610de0613811565b5f825f81518110610df357610df3613ad8565b01602001516001600160f81b0319169050607f60f81b8110610e1f57610e18836116d7565b9392505050565b6001600160f81b03198116600160f81b03610e3d57610e188361190d565b6001600160f81b03198116600160f91b03610e5b57610e1883611bcd565b6001600160f81b03198116600360f81b03610e7957610e1883611e52565b604051636fc3daa360e11b815260040160405180910390fd5b5f610307610e9f836121df565b610ea8846121f7565b610db0565b60605f82604051602001610ec391815260200190565b60405160208183030381529060405290505f5b6020811015610f0e57818181518110610ef157610ef1613ad8565b01602001516001600160f81b0319165f03610f0e57600101610ed6565b5f610f1a826020613b00565b6001600160401b03811115610f3157610f316138a0565b6040519080825280601f01601f191660200182016040528015610f5b576020820181803683370190505b5090505f5b8151811015610fc1578383610f7481613ee0565b945081518110610f8657610f86613ad8565b602001015160f81c60f81b828281518110610fa357610fa3613ad8565b60200101906001600160f81b03191690815f1a905350600101610f60565b50949350505050565b60608082516001148015610ff757506080835f81518110610fed57610fed613ad8565b016020015160f81c105b15611003575081610307565b61100f83516080612253565b83604051602001611021929190613ef8565b604051602081830303815290604052905092915050565b60605f611044836123fa565b90505f81516001600160401b03811115611060576110606138a0565b6040519080825280602002602001820160405280156110a557816020015b604080518082019091526060808252602082015281526020019060019003908161107e5790505b5090505f5b825181101561111b575f6110d68483815181106110c9576110c9613ad8565b6020026020010151612408565b905060405180604001604052808281526020016110f2836123fa565b81525083838151811061110757611107613ad8565b6020908102919091010152506001016110aa565b509392505050565b5f606081808061113287612495565b90505f8690505f80611157604051806040016040528060608152602001606081525090565b5f5b8c51811015611500578c818151811061117457611174613ad8565b60200260200101519150828461118a9190613e91565b9350611197600188613e91565b9650835f036111f1578151805160209091012085146111ec5760405162461bcd60e51b8152602060048201526011602482015270092dcecc2d8d2c840e4dedee840d0c2e6d607b1b6044820152606401610461565b6112ad565b815151602011611253578151805160209091012085146111ec5760405162461bcd60e51b815260206004820152601b60248201527f496e76616c6964206c6172676520696e7465726e616c206861736800000000006044820152606401610461565b84611260835f01516125a5565b146112ad5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420696e7465726e616c206e6f646520686173680000000000006044820152606401610461565b6112b960106001613e91565b826020015151036113295785518414611500575f8685815181106112df576112df613ad8565b602001015160f81c60f81b60f81c90505f83602001518260ff168151811061130957611309613ad8565b6020026020010151905061131c816125cc565b96506001945050506114f8565b6002826020015151036114b0575f61134083612600565b90505f815f8151811061135557611355613ad8565b016020015160f81c90505f61136b600283613f0c565b611376906002613f2d565b90505f611386848360ff16612623565b90505f6113938b8a612623565b90505f6113a08383612651565b905060ff8516600214806113b7575060ff85166003145b156113f1578083511480156113cc5750808251145b156113de576113db818b613e91565b99505b50600160ff1b9950611500945050505050565b60ff85161580611404575060ff85166001145b1561145957805f036114235750600160ff1b9950611500945050505050565b61144a886020015160018151811061143d5761143d613ad8565b60200260200101516125cc565b9a5097506114f8945050505050565b60405162461bcd60e51b815260206004820152602660248201527f52656365697665642061206e6f6465207769746820616e20756e6b6e6f776e206044820152650e0e4caccd2f60d31b6064820152608401610461565b60405162461bcd60e51b815260206004820152601c60248201527f526563656976656420616e20756e7061727361626c65206e6f64652e000000006044820152606401610461565b600101611159565b50600160ff1b8414866115138786612623565b909e909d50909b509950505050505050505050565b602081015180516060916103079161154290600190613b00565b815181106110c9576110c9613ad8565b6040805160088082528183019092526060915f91906020820181803683370190505090505f5b60088110156115cb5761158c816008613e7a565b846001600160401b0316901c60f81b8282815181106115ad576115ad613ad8565b60200101906001600160f81b03191690815f1a905350600101611578565b5092915050565b5f805f8351604103611609576020840151604085015160608601515f1a6115fb888285856126cc565b955095509550505050611614565b505081515f91506002905b9250925092565b5f82600381111561162e5761162e613ea4565b03611637575050565b600182600381111561164b5761164b613ea4565b036116695760405163f645eedf60e01b815260040160405180910390fd5b600282600381111561167d5761167d613ea4565b0361169e5760405163fce698f760e01b815260048101829052602401610461565b60038260038111156116b2576116b2613ea4565b036116d3576040516335e2f38360e21b815260048101829052602401610461565b5050565b6116df613811565b5f8082526116ef610218846103c3565b9050805160091415801561170557508051600614155b156117235760405163c2871a3760e01b815260040160405180910390fd5b611738815f8151811061029857610298613ad8565b60608301528051611756908290600190811061029857610298613ad8565b60808301528051611774908290600290811061029857610298613ad8565b60e0830152805161179f908290600390811061179257611792613ad8565b6020026020010151612794565b6001600160a01b031661010083015280516117c7908290600490811061029857610298613ad8565b61012083015280516117e690829060059081106110c9576110c9613ad8565b61014083015280516006036117fb5750919050565b5f6118128260068151811061029857610298613ad8565b90505f61182b8360078151811061029857610298613ad8565b90505f6118448460088151811061029857610298613ad8565b905081158015611852575080155b15611872576001600160401b038316602086015260016040860152611904565b6023836001600160401b0316106118ba576002611890602385613f46565b61189a9190613f66565b6001600160401b03908116602087015283166101e0860152600160408601525b5f6118c9600260018618613f8b565b6118d490601b613fb0565b6040519091506118ec90849084908490602001613fd0565b60408051601f198184030181529190526101c0870152505b50505050919050565b611915613811565b600180825282515f916119359161192d908290613b00565b859190612800565b90505f611944610218836103c3565b9050805160081415801561195a57508051600b14155b156119785760405163c2871a3760e01b815260040160405180910390fd5b61198d815f8151811061029857610298613ad8565b6001600160401b0316602084015280516119b4908290600190811061029857610298613ad8565b606084015280516119d2908290600290811061029857610298613ad8565b608084015280516119f0908290600390811061029857610298613ad8565b60e08401528051611a0e908290600490811061179257611792613ad8565b6001600160a01b03166101008401528051611a36908290600590811061029857610298613ad8565b6101208401528051611a5590829060069081106110c9576110c9613ad8565b8361014001819052505f611a8282600781518110611a7557611a75613ad8565b60200260200101516103ef565b905080516001600160401b03811115611a9d57611a9d6138a0565b604051908082528060200260200182016040528015611ad057816020015b6060815260200190600190039081611abb5790505b506101608501525f5b8151811015611b2e57611b04828281518110611af757611af7613ad8565b6020026020010151612940565b8561016001518281518110611b1b57611b1b613ad8565b6020908102919091010152600101611ad9565b508151600803611b4057505050919050565b5f611b578360088151811061029857610298613ad8565b611b6290601b613ff3565b90505f611b7b8460098151811061023457610234613ad8565b90505f611b9485600a8151811061023457610234613ad8565b9050818184604051602001611bab93929190613fd0565b60408051601f198184030181529190526101c088015250949695505050505050565b611bd5613811565b6002815281515f90611bef9060019061192d908290613b00565b90505f611bfe610218836103c3565b90508051600914158015611c1457508051600c14155b15611c325760405163c2871a3760e01b815260040160405180910390fd5b611c47815f8151811061029857610298613ad8565b6001600160401b031660208401528051611c6e908290600190811061029857610298613ad8565b60608401528051611c8c908290600290811061029857610298613ad8565b60a08401528051611caa908290600390811061029857610298613ad8565b60c08401528051611cc8908290600490811061029857610298613ad8565b60e08401528051611ce6908290600590811061179257611792613ad8565b6001600160a01b03166101008401528051611d0e908290600690811061029857610298613ad8565b6101208401528051611d2d90829060079081106110c9576110c9613ad8565b8361014001819052505f611d4d82600881518110611a7557611a75613ad8565b905080516001600160401b03811115611d6857611d686138a0565b604051908082528060200260200182016040528015611d9b57816020015b6060815260200190600190039081611d865790505b506101608501525f5b8151811015611dec57611dc2828281518110611af757611af7613ad8565b8561016001518281518110611dd957611dd9613ad8565b6020908102919091010152600101611da4565b508151600903611dfe57505050919050565b5f611e158360098151811061029857610298613ad8565b611e2090601b613ff3565b90505f611e3984600a8151811061023457610234613ad8565b90505f611b9485600b8151811061023457610234613ad8565b611e5a613811565b6003815281515f90611e749060019061192d908290613b00565b90505f611e83610218836103c3565b90508051600b14158015611e9957508051600e14155b15611eb75760405163c2871a3760e01b815260040160405180910390fd5b611ecc815f8151811061029857610298613ad8565b6001600160401b031660208401528051611ef3908290600190811061029857610298613ad8565b60608401528051611f11908290600290811061029857610298613ad8565b60a08401528051611f2f908290600390811061029857610298613ad8565b60c08401528051611f4d908290600490811061029857610298613ad8565b60e08401528051611f6b908290600590811061179257611792613ad8565b6001600160a01b03166101008401528051611f93908290600690811061029857610298613ad8565b6101208401528051611fb290829060079081106110c9576110c9613ad8565b8361014001819052505f611fd282600881518110611a7557611a75613ad8565b905080516001600160401b03811115611fed57611fed6138a0565b60405190808252806020026020018201604052801561202057816020015b606081526020019060019003908161200b5790505b506101608501525f5b815181101561207157612047828281518110611af757611af7613ad8565b856101600151828151811061205e5761205e613ad8565b6020908102919091010152600101612029565b506120888260098151811061029857610298613ad8565b846101800181815250505f6120a983600a81518110611a7557611a75613ad8565b905080516001600160401b038111156120c4576120c46138a0565b6040519080825280602002602001820160405280156120ed578160200160208202803683370190505b506101a08601525f5b815181101561213e5761211482828151811061023457610234613ad8565b866101a00151828151811061212b5761212b613ad8565b60209081029190910101526001016120f6565b508251600b036121515750505050919050565b5f61216884600b8151811061029857610298613ad8565b61217390601b613ff3565b90505f61218c85600c8151811061023457610234613ad8565b90505f6121a586600d8151811061023457610234613ad8565b90508181846040516020016121bc93929190613fd0565b60408051601f198184030181529190526101c08901525095979650505050505050565b5f6121e98261294b565b805190602001209050919050565b6060816101c00151515f0361221f5760405163d466bd1560e01b815260040160405180910390fd5b816101c001515160411461224657604051634be6321b60e01b815260040160405180910390fd5b506101c08101515b919050565b60608060388410156122b857604080516001808252818301909252906020820181803683370190505090506122888385613ff3565b60f81b815f8151811061229d5761229d613ad8565b60200101906001600160f81b03191690815f1a905350610e18565b5f60015b6122c68187613b27565b156122ec57816122d581613ee0565b92506122e5905061010082613e7a565b90506122bc565b6122f7826001613e91565b6001600160401b0381111561230e5761230e6138a0565b6040519080825280601f01601f191660200182016040528015612338576020820181803683370190505b5092506123458583613ff3565b612350906037613ff3565b60f81b835f8151811061236557612365613ad8565b60200101906001600160f81b03191690815f1a905350600190505b8181116123f1576101006123948284613b00565b6123a0906101006140ec565b6123aa9088613b27565b6123b491906140f7565b60f81b8382815181106123c9576123c9613ad8565b60200101906001600160f81b03191690815f1a905350806123e981613ee0565b915050612380565b50509392505050565b6060610307610218836103c3565b60605f805f612416856108ff565b919450925090505f81600181111561243057612430613ea4565b1461247d5760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420524c502062797465732076616c75652e00000000000000006044820152606401610461565b61248c856020015184846129d8565b95945050505050565b60605f82516002026001600160401b038111156124b4576124b46138a0565b6040519080825280601f01601f1916602001820160405280156124de576020820181803683370190505b5090505f5b83518110156115cb57600484828151811061250057612500613ad8565b602001015160f81c60f81b6001600160f81b031916901c82826002028151811061252c5761252c613ad8565b60200101906001600160f81b03191690815f1a905350601084828151811061255657612556613ad8565b016020015160f81c8161256b5761256b613b13565b0660f81b82826002026001018151811061258757612587613ad8565b60200101906001600160f81b03191690815f1a9053506001016124e3565b5f6020825110156125b857506020015190565b81806020019051810190610307919061410a565b5f60606020835f015110156125eb576125e483612940565b90506125f7565b6125f483612408565b90505b610e18816125a5565b606061030761261e83602001515f815181106110c9576110c9613ad8565b612495565b6060818351035f03612643575060408051602081019091525f8152610307565b610e18838384865103612800565b5f805b8084511180156126645750808351115b80156126b5575082818151811061267d5761267d613ad8565b602001015160f81c60f81b6001600160f81b0319168482815181106126a4576126a4613ad8565b01602001516001600160f81b031916145b15610e1857806126c481613ee0565b915050612654565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561270557505f9150600390508261278a565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015612756573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b03811661278157505f92506001915082905061278a565b92505f91508190505b9450945094915050565b80515f906001036127a657505f919050565b81516015146127f75760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020616464726573732076616c75652e0000000000006044820152606401610461565b610307826106d7565b60608182601f0110156128465760405162461bcd60e51b815260206004820152600e60248201526d736c6963655f6f766572666c6f7760901b6044820152606401610461565b8282840110156128895760405162461bcd60e51b815260206004820152600e60248201526d736c6963655f6f766572666c6f7760901b6044820152606401610461565b818301845110156128d05760405162461bcd60e51b8152602060048201526011602482015270736c6963655f6f75744f66426f756e647360781b6044820152606401610461565b6060821580156128ee5760405191505f825260208201604052610fc1565b6040519150601f8416801560200281840101858101878315602002848b0101015b8183101561292757805183526020928301920161290f565b5050858452601f01601f19166040525050949350505050565b606061030782612a85565b60605f8251600381111561296157612961613ea4565b0361296f5761030782612a99565b60018251600381111561298457612984613ea4565b036129925761030782612d85565b6002825160038111156129a7576129a7613ea4565b036129b55761030782612ff6565b6003825160038111156129ca576129ca613ea4565b03610e795761030782613264565b60605f826001600160401b038111156129f3576129f36138a0565b6040519080825280601f01601f191660200182016040528015612a1d576020820181803683370190505b50905080515f03612a2f579050610e18565b848401602082015f5b60208604811015612a59578251825260209283019290910190600101612a38565b505f6001602087066020036101000a039050808251168119845116178252839450505050509392505050565b606061030782602001515f845f01516129d8565b60605f82602001516001600160401b03165f14612abb57506020820151612afe565b6101c08301515115612afe576023836101e001516001600160401b031610612afe5760026023846101e00151612af19190613f46565b612afb9190613f66565b90505b5f8360400151612b0e575f612b11565b60035b612b1c906006613ff3565b60ff1690505f816001600160401b03811115612b3a57612b3a6138a0565b604051908082528060200260200182016040528015612b6d57816020015b6060815260200190600190039081612b585790505b509050612b7d8560600151610c81565b815f81518110612b8f57612b8f613ad8565b6020026020010181905250612ba78560800151610c81565b81600181518110612bba57612bba613ad8565b6020026020010181905250612bd28560e00151610c81565b81600281518110612be557612be5613ad8565b6020026020010181905250612bfe856101000151613607565b81600381518110612c1157612c11613ad8565b6020026020010181905250612c2a856101200151610c81565b81600481518110612c3d57612c3d613ad8565b6020026020010181905250612c56856101400151610fca565b81600581518110612c6957612c69613ad8565b6020026020010181905250846040015115612d7c5784602001516001600160401b03165f03612cd0576040515f602082015260210160405160208183030381529060405281600681518110612cc057612cc0613ad8565b6020026020010181905250612d01565b612ce2836001600160401b0316610c81565b81600681518110612cf557612cf5613ad8565b60200260200101819052505b612d365f5b6040519080825280601f01601f191660200182016040528015612d30576020820181803683370190505b50610fca565b81600781518110612d4957612d49613ad8565b6020908102919091010152612d5d5f612d06565b81600881518110612d7057612d70613ad8565b60200260200101819052505b61248c8161363f565b60408051600880825261012082019092526060915f9190816020015b6060815260200190600190039081612da1579050509050612dce83602001516001600160401b0316610c81565b815f81518110612de057612de0613ad8565b6020026020010181905250612df88360600151610c81565b81600181518110612e0b57612e0b613ad8565b6020026020010181905250612e238360800151610c81565b81600281518110612e3657612e36613ad8565b6020026020010181905250612e4e8360e00151610c81565b81600381518110612e6157612e61613ad8565b6020026020010181905250612e7a836101000151613607565b81600481518110612e8d57612e8d613ad8565b6020026020010181905250612ea6836101200151610c81565b81600581518110612eb957612eb9613ad8565b6020026020010181905250612ed2836101400151610fca565b81600681518110612ee557612ee5613ad8565b60200260200101819052505f836101600151516001600160401b03811115612f0f57612f0f6138a0565b604051908082528060200260200182016040528015612f4257816020015b6060815260200190600190039081612f2d5790505b5090505f5b84610160015151811015612f99578461016001518181518110612f6c57612f6c613ad8565b6020026020010151828281518110612f8657612f86613ad8565b6020908102919091010152600101612f47565b50612fa38161363f565b82600781518110612fb657612fb6613ad8565b602090810291909101015260015b612fcd8361363f565b604051602001612fde929190614121565b60405160208183030381529060405292505050919050565b60408051600980825261014082019092526060915f9190816020015b606081526020019060019003908161301257905050905061303f83602001516001600160401b0316610c81565b815f8151811061305157613051613ad8565b60200260200101819052506130698360600151610c81565b8160018151811061307c5761307c613ad8565b60200260200101819052506130948360a00151610c81565b816002815181106130a7576130a7613ad8565b60200260200101819052506130bf8360c00151610c81565b816003815181106130d2576130d2613ad8565b60200260200101819052506130ea8360e00151610c81565b816004815181106130fd576130fd613ad8565b6020026020010181905250613116836101000151613607565b8160058151811061312957613129613ad8565b6020026020010181905250613142836101200151610c81565b8160068151811061315557613155613ad8565b602002602001018190525061316e836101400151610fca565b8160078151811061318157613181613ad8565b60200260200101819052505f836101600151516001600160401b038111156131ab576131ab6138a0565b6040519080825280602002602001820160405280156131de57816020015b60608152602001906001900390816131c95790505b5090505f5b8461016001515181101561323557846101600151818151811061320857613208613ad8565b602002602001015182828151811061322257613222613ad8565b60209081029190910101526001016131e3565b5061323f8161363f565b8260088151811061325257613252613ad8565b60209081029190910101526002612fc4565b60408051600b80825261018082019092526060915f9190816020015b60608152602001906001900390816132805790505090506132ad83602001516001600160401b0316610c81565b815f815181106132bf576132bf613ad8565b60200260200101819052506132d78360600151610c81565b816001815181106132ea576132ea613ad8565b60200260200101819052506133028360a00151610c81565b8160028151811061331557613315613ad8565b602002602001018190525061332d8360c00151610c81565b8160038151811061334057613340613ad8565b60200260200101819052506133588360e00151610c81565b8160048151811061336b5761336b613ad8565b6020026020010181905250613384836101000151613607565b8160058151811061339757613397613ad8565b60200260200101819052506133b0836101200151610c81565b816006815181106133c3576133c3613ad8565b60200260200101819052506133dc836101400151610fca565b816007815181106133ef576133ef613ad8565b60200260200101819052505f836101600151516001600160401b03811115613419576134196138a0565b60405190808252806020026020018201604052801561344c57816020015b60608152602001906001900390816134375790505b5090505f5b846101600151518110156134a357846101600151818151811061347657613476613ad8565b602002602001015182828151811061349057613490613ad8565b6020908102919091010152600101613451565b506134ad8161363f565b826008815181106134c0576134c0613ad8565b60200260200101819052506134d9846101800151610c81565b826009815181106134ec576134ec613ad8565b60200260200101819052505f846101a00151516001600160401b03811115613516576135166138a0565b60405190808252806020026020018201604052801561354957816020015b60608152602001906001900390816135345790505b5090505f5b856101a00151518110156135aa57613585866101a00151828151811061357657613576613ad8565b60200260200101515f1c610c81565b82828151811061359757613597613ad8565b602090810291909101015260010161354e565b506135b48161363f565b83600a815181106135c7576135c7613ad8565b602090810291909101015260036135dd8461363f565b6040516020016135ee929190614121565b6040516020818303038152906040529350505050919050565b604051606082811b6bffffffffffffffffffffffff191660208301529061030790603401604051602081830303815290604052610fca565b60605f61364b83613682565b9050613659815160c0612253565b8160405160200161366b929190613ef8565b604051602081830303815290604052915050919050565b606081515f036136a157604080515f80825260208201909252906115cb565b5f805b83518110156136dd578381815181106136bf576136bf613ad8565b602002602001015151826136d39190613e91565b91506001016136a4565b5f826001600160401b038111156136f6576136f66138a0565b6040519080825280601f01601f191660200182016040528015613720576020820181803683370190505b505f92509050602081015b8551831015610fc1575f86848151811061374757613747613ad8565b602002602001015190505f6020820190506137648382845161379a565b87858151811061377657613776613ad8565b6020026020010151518361378a9190613e91565b600190950194925061372b915050565b8282825b602081106137d657815183526137b5602084613e91565b92506137c2602083613e91565b91506137cf602082613b00565b905061379e565b5f60016137e4836020613b00565b6137f0906101006140ec565b6137fa9190613b00565b925184518416931916929092179092525050505050565b604080516102008101909152805f81526020015f6001600160401b031681526020015f151581526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f815260200160608152602001606081526020015f815260200160608152602001606081526020015f6001600160401b031681525090565b634e487b7160e01b5f52604160045260245ffd5b60405160c081016001600160401b03811182821017156138d6576138d66138a0565b60405290565b604051601f8201601f191681016001600160401b0381118282101715613904576139046138a0565b604052919050565b5f82601f83011261391b575f80fd5b81356001600160401b03811115613934576139346138a0565b613947601f8201601f19166020016138dc565b81815284602083860101111561395b575f80fd5b816020850160208301375f918101602001919091529392505050565b5f60208284031215613987575f80fd5b81356001600160401b0381111561399c575f80fd5b6139a88482850161390c565b949350505050565b5f602082840312156139c0575f80fd5b5035919050565b80356001600160a01b038116811461224e575f80fd5b5f8083601f8401126139ed575f80fd5b5081356001600160401b03811115613a03575f80fd5b602083019150836020828501011115613a1a575f80fd5b9250929050565b5f805f805f8060a08789031215613a36575f80fd5b86356001600160401b0380821115613a4c575f80fd5b90880190610160828b031215613a60575f80fd5b90965060208801359080821115613a75575f80fd5b908801906080828b031215613a88575f80fd5b819650613a9760408a016139c7565b95506060890135915080821115613aac575f80fd5b50613ab989828a016139dd565b9094509250613acc9050608088016139c7565b90509295509295509295565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561030757610307613aec565b634e487b7160e01b5f52601260045260245ffd5b5f82613b3557613b35613b13565b500490565b5f808335601e19843603018112613b4f575f80fd5b8301803591506001600160401b03821115613b68575f80fd5b602001915036819003821315613a1a575f80fd5b80356001600160401b038116811461224e575f80fd5b5f60208284031215613ba2575f80fd5b81356001600160401b0380821115613bb8575f80fd5b9083019060608286031215613bcb575f80fd5b604051606081018181108382111715613be657613be66138a0565b604052613bf283613b7c565b8152602083013582811115613c05575f80fd5b613c118782860161390c565b602083015250604083013582811115613c28575f80fd5b613c348782860161390c565b60408301525095945050505050565b5f6001600160401b03821115613c5b57613c5b6138a0565b5060051b60200190565b5f82601f830112613c74575f80fd5b81356020613c89613c8483613c43565b6138dc565b82815260059290921b84018101918181019086841115613ca7575f80fd5b8286015b84811015613ce45780356001600160401b03811115613cc8575f80fd5b613cd68986838b010161390c565b845250918301918301613cab565b509695505050505050565b5f82601f830112613cfe575f80fd5b81356020613d0e613c8483613c43565b8083825260208201915060208460051b870101935086841115613d2f575f80fd5b602086015b84811015613ce45780358352918301918301613d34565b5f60208284031215613d5b575f80fd5b81356001600160401b0380821115613d71575f80fd5b9083019060c08286031215613d84575f80fd5b613d8c6138b4565b82358152602083013582811115613da1575f80fd5b613dad8782860161390c565b602083015250604083013582811115613dc4575f80fd5b613dd08782860161390c565b604083015250606083013582811115613de7575f80fd5b613df38782860161390c565b606083015250608083013582811115613e0a575f80fd5b613e1687828601613c65565b60808301525060a083013582811115613e2d575f80fd5b613e3987828601613cef565b60a08301525095945050505050565b5f60208284031215613e58575f80fd5b610e1882613b7c565b5f60208284031215613e71575f80fd5b610e18826139c7565b808202811582820484141761030757610307613aec565b8082018082111561030757610307613aec565b634e487b7160e01b5f52602160045260245ffd5b5f81518060208401855e5f93019283525090919050565b8281525f6139a86020830184613eb8565b5f60018201613ef157613ef1613aec565b5060010190565b5f6139a8613f068386613eb8565b84613eb8565b5f60ff831680613f1e57613f1e613b13565b8060ff84160691505092915050565b60ff828116828216039081111561030757610307613aec565b6001600160401b038281168282160390808211156115cb576115cb613aec565b5f6001600160401b0380841680613f7f57613f7f613b13565b92169190910492915050565b5f6001600160401b0380841680613fa457613fa4613b13565b92169190910692915050565b6001600160401b038181168382160190808211156115cb576115cb613aec565b928352602083019190915260f81b6001600160f81b031916604082015260410190565b60ff818116838216019081111561030757610307613aec565b600181815b8085111561404657815f190482111561402c5761402c613aec565b8085161561403957918102915b93841c9390800290614011565b509250929050565b5f8261405c57506001610307565b8161406857505f610307565b816001811461407e5760028114614088576140a4565b6001915050610307565b60ff84111561409957614099613aec565b50506001821b610307565b5060208310610133831016604e8410600b84101617156140c7575081810a610307565b6140d1838361400c565b805f19048211156140e4576140e4613aec565b029392505050565b5f610e18838361404e565b5f8261410557614105613b13565b500690565b5f6020828403121561411a575f80fd5b5051919050565b60f883901b6001600160f81b03191681525f6139a86001830184613eb856fea2646970667358221220d8cc604fa752454e198f785278ba72bd5dba4fa50e7e42a09d47a4db4b54c20564736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80c\x9F2\x9D\x0B\x11a\0nW\x80c\x9F2\x9D\x0B\x14a\x01xW\x80c\xAB\xEE\xB3f\x14a\x01\x81W\x80c\xC29\xEFB\x14a\x01\x94W\x80c\xCA\xC6-\xD9\x14a\x01\xA7W\x80c\xF4^\x81\x18\x14a\x01\xB0W\x80c\xF5\xBE\xEA\x8C\x14a\x01\xB9W_\x80\xFD[\x80c\x18\x9C\xC9\xD0\x14a\0\xB5W\x80c*\x04\xFF\x85\x14a\0\xD0W\x80cIh\xF6\xC5\x14a\0\xD8W\x80cV\xB4\xA9*\x14a\0\xE0W\x80ct7p)\x14a\x01>W\x80c\x86\xFBa\xED\x14a\x01FW[_\x80\xFD[a\0\xBDa\x01\xCCV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBD`\x0C\x81V[a\0\xBD_T\x81V[a\0\xF3a\0\xEE6`\x04a9wV[a\x01\xDBV[`@Qa\0\xC7\x91\x90_`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[a\0\xBD` \x81V[a\x01`r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC7V[a\0\xBD`\x01T\x81V[a\0\xBDa\x01\x8F6`\x04a9\xB0V[a\x02\xECV[a\0\xBDa\x01\xA26`\x04a:!V[a\x03\rV[a\0\xBDa\x1F\xFF\x81V[a\0\xBDa\x01\0\x81V[a\0\xBDa\x01\xC76`\x04a9\xB0V[a\x03\xAAV[_a\x01\xD6Ba\x02\xECV[\x90P\x90V[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\x02\x1Da\x02\x18\x84a\x03\xC3V[a\x03\xEFV[\x90Pa\x02A\x81_\x81Q\x81\x10a\x024Wa\x024a:\xD8V[` \x02` \x01\x01Qa\x05\xDAV[\x82R\x80Qa\x02\\\x90\x82\x90`\x03\x90\x81\x10a\x024Wa\x024a:\xD8V[` \x83\x01R\x80Qa\x02z\x90\x82\x90`\x04\x90\x81\x10a\x024Wa\x024a:\xD8V[`@\x83\x01R\x80Qa\x02\xA5\x90\x82\x90`\x08\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[` \x02` \x01\x01Qa\x06\xD7V[``\x83\x01R\x80Qa\x02\xC3\x90\x82\x90`\x0B\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x80\x83\x01R\x80Qa\x02\xE1\x90\x82\x90`\x0F\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xA0\x83\x01RP\x91\x90PV[_`\x0C`\x01T\x83a\x02\xFD\x91\x90a;\0V[a\x03\x07\x91\x90a;'V[\x92\x91PPV[_\x80a\x03\x1C` \x88\x01\x88a;:V[\x81\x01\x90a\x03)\x91\x90a;\x92V[\x90P_a\x038\x85\x87\x01\x87a=KV[\x82Q\x90\x91P`\x01`\x01`@\x1B\x03\x16a\x03Xa\x01@\x8B\x01a\x01 \x8C\x01a>HV[`\x01`\x01`@\x1B\x03\x16\x14a\x03\x7FW`@Qc0\xD3\xBA\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x9B\x82\x82a\x03\x96a\x01 \x8D\x01a\x01\0\x8E\x01a>aV[a\x06\xE1V[PP_T\x97\x96PPPPPPPV[_a\x03\xB6`\x0C\x83a>zV[`\x01Ta\x03\x07\x91\x90a>\x91V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R\x82Q\x82R\x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[``_\x80a\x03\xFC\x84a\x08\xFFV[\x91\x93P\x90\x91P`\x01\x90P\x81`\x01\x81\x11\x15a\x04\x18Wa\x04\x18a>\xA4V[\x14a\x04jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid RLP list value.\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\x82W\x90PP\x90P_\x83[\x86Q\x81\x10\x15a\x05\xCFW` \x82\x10a\x05\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FProvided RLP list exceeds max li`D\x82\x01Ri9\xBA\x1062\xB73\xBA4\x17`\xB1\x1B`d\x82\x01R`\x84\x01a\x04aV[_\x80a\x05T`@Q\x80`@\x01`@R\x80\x85\x8C_\x01Qa\x058\x91\x90a;\0V[\x81R` \x01\x85\x8C` \x01Qa\x05M\x91\x90a>\x91V[\x90Ra\x08\xFFV[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83a\x05p\x91\x90a>\x91V[\x81R` \x01\x84\x8B` \x01Qa\x05\x85\x91\x90a>\x91V[\x81RP\x85\x85\x81Q\x81\x10a\x05\x9AWa\x05\x9Aa:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x05\xB0`\x01\x85a>\x91V[\x93Pa\x05\xBC\x81\x83a>\x91V[a\x05\xC6\x90\x84a>\x91V[\x92PPPa\x04\xADV[P\x81R\x94\x93PPPPV[_`!\x82_\x01Q\x11\x15a\x06/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP bytes32 value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[_\x80_a\x06;\x85a\x08\xFFV[\x91\x94P\x92P\x90P_\x81`\x01\x81\x11\x15a\x06UWa\x06Ua>\xA4V[\x14a\x06\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP bytes32 value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[_\x83\x86` \x01Qa\x06\xB3\x91\x90a>\x91V[\x80Q\x90\x91P` \x84\x10\x15a\x06\xCDW` \x84\x90\x03a\x01\0\n\x90\x04[\x96\x95PPPPPPV[_a\x03\x07\x82a\x05\xDAV[\x82Q`\x01`\x01`@\x1B\x03\x16` a\x06\xF6a\x01\xCCV[a\x07\0\x91\x90a;\0V[\x81\x11\x15a\x07 W`@Qc\xB6\x14K\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\0a\x07+a\x01\xCCV[a\x075\x91\x90a;\0V[\x81\x10\x15a\x07UW`@Qc\x0C\xDC\xEBy`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q_\x90a\x07e\x90`\x01\x90a;\0V[\x90PC\x81\x11\x80a\x07\x7FWPa\x07|a\x01\0Ca;\0V[\x81\x10[\x15a\x07\x9DW`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q_\x90a\x07\xAD\x90`\x01\x90a;\0V[` \x80\x87\x01Q\x80Q\x91\x01 \x90@\x91P\x80\x82\x14a\x07\xDCW`@Qc\x1F\x03F[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x07\xE7\x89a\x0B\xF9V[\x92P\x92PP\x81`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\x1EW`@Qc\xAA\xAA\x91A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08,\x89`@\x01Qa\x01\xDBV[\x80Q\x90\x91P\x84\x14a\x08PW`@Qc \xFAl\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08w\x8A`\xA0\x01Q_\x81Q\x81\x10a\x08jWa\x08ja:\xD8V[` \x02` \x01\x01Qa\x0C\x81V[\x90P_\x80a\x08\xA7\x83\x8D`\x80\x01Q_\x81Q\x81\x10a\x08\x95Wa\x08\x95a:\xD8V[` \x02` \x01\x01Q\x86`@\x01Qa\x0C\x94V[\x91P\x91P\x81a\x08\xC9W`@Qc\tL\xEC_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q` \x82\x01 \x85Q\x14a\x08\xF0W`@QcDb\xB3\x8F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPPPV[_\x80_\x80\x84_\x01Q\x11a\tTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FRLP item cannot be null.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[` \x84\x01Q\x80Q_\x1A`\x7F\x81\x11a\tvW_`\x01_\x94P\x94P\x94PPPa\x0B\xF2V[`\xB7\x81\x11a\t\xE5W\x85Q`\x7F\x19\x82\x01\x90\x81\x10a\t\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid RLP short string.\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[`\x01\x95P\x93P_\x92Pa\x0B\xF2\x91PPV[`\xBF\x81\x11a\n\xBCW\x85Q`\xB6\x19\x82\x01\x90\x81\x10a\nCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FInvalid RLP long string length.\0`D\x82\x01R`d\x01a\x04aV[_\x81` \x03a\x01\0\n`\x01\x85\x01Q\x04\x90P\x80\x82\x01\x88_\x01Q\x11a\n\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid RLP long string.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[`\x01\x90\x91\x01\x95P\x93P_\x92Pa\x0B\xF2\x91PPV[`\xF7\x81\x11a\x0B+W\x85Q`\xBF\x19\x82\x01\x90\x81\x10a\x0B\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid RLP short list.\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[`\x01\x95P\x93P\x84\x92Pa\x0B\xF2\x91PPV[\x85Q`\xF6\x19\x82\x01\x90\x81\x10a\x0B\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid RLP long list length.\0\0\0`D\x82\x01R`d\x01a\x04aV[_\x81` \x03a\x01\0\n`\x01\x85\x01Q\x04\x90P\x80\x82\x01\x88_\x01Q\x11a\x0B\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru$\xB7;0\xB64\xB2\x10)&(\x1067\xB73\x9064\xB9\xBA\x17`Q\x1B`D\x82\x01R`d\x01a\x04aV[`\x01\x91\x82\x01\x96P\x94P\x92Pa\x0B\xF2\x91PPV[\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x81\x90a\x0C,a\x0C\"\x85a\riV[\x85` \x01Qa\r\xB0V[\x91P_a\x0C<\x85`@\x01Qa\r\xD8V[\x90Pa\x0CG\x81a\x0E\x92V[\x93P`@Q\x80``\x01`@R\x80\x86`@\x01Q\x80Q\x90` \x01 \x81R` \x01\x82``\x01Q\x81R` \x01\x82`\xE0\x01Q\x81RP\x91PP\x91\x93\x90\x92PV[``a\x03\x07a\x0C\x8F\x83a\x0E\xADV[a\x0F\xCAV[_``_a\x0C\xA1\x85a\x108V[\x90P_\x80_a\x0C\xB1\x84\x8A\x89a\x11#V[\x81Q\x92\x95P\x90\x93P\x91P\x15\x80\x80a\x0C\xC5WP\x81[a\r\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FProvided proof is invalid.\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[_\x81a\r+W`@Q\x80` \x01`@R\x80_\x81RPa\rWV[a\rW\x86a\r:`\x01\x88a;\0V[\x81Q\x81\x10a\rJWa\rJa:\xD8V[` \x02` \x01\x01Qa\x15(V[\x91\x9B\x91\x9AP\x90\x98PPPPPPPPPV[_\x81`@\x01Q\x80Q\x90` \x01 a\r\x82\x83_\x01Qa\x15RV[`@Q` \x01a\r\x93\x92\x91\x90a>\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[_\x80_\x80a\r\xBE\x86\x86a\x15\xD2V[\x92P\x92P\x92Pa\r\xCE\x82\x82a\x16\x1BV[P\x90\x94\x93PPPPV[a\r\xE0a8\x11V[_\x82_\x81Q\x81\x10a\r\xF3Wa\r\xF3a:\xD8V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P`\x7F`\xF8\x1B\x81\x10a\x0E\x1FWa\x0E\x18\x83a\x16\xD7V[\x93\x92PPPV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x01`\xF8\x1B\x03a\x0E=Wa\x0E\x18\x83a\x19\rV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x01`\xF9\x1B\x03a\x0E[Wa\x0E\x18\x83a\x1B\xCDV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x03`\xF8\x1B\x03a\x0EyWa\x0E\x18\x83a\x1ERV[`@Qco\xC3\xDA\xA3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x03\x07a\x0E\x9F\x83a!\xDFV[a\x0E\xA8\x84a!\xF7V[a\r\xB0V[``_\x82`@Q` \x01a\x0E\xC3\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_[` \x81\x10\x15a\x0F\x0EW\x81\x81\x81Q\x81\x10a\x0E\xF1Wa\x0E\xF1a:\xD8V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16_\x03a\x0F\x0EW`\x01\x01a\x0E\xD6V[_a\x0F\x1A\x82` a;\0V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F1Wa\x0F1a8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0F[W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a\x0F\xC1W\x83\x83a\x0Ft\x81a>\xE0V[\x94P\x81Q\x81\x10a\x0F\x86Wa\x0F\x86a:\xD8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0F\xA3Wa\x0F\xA3a:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x0F`V[P\x94\x93PPPPV[``\x80\x82Q`\x01\x14\x80\x15a\x0F\xF7WP`\x80\x83_\x81Q\x81\x10a\x0F\xEDWa\x0F\xEDa:\xD8V[\x01` \x01Q`\xF8\x1C\x10[\x15a\x10\x03WP\x81a\x03\x07V[a\x10\x0F\x83Q`\x80a\"SV[\x83`@Q` \x01a\x10!\x92\x91\x90a>\xF8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[``_a\x10D\x83a#\xFAV[\x90P_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10`Wa\x10`a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xA5W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10~W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x11\x1BW_a\x10\xD6\x84\x83\x81Q\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[` \x02` \x01\x01Qa$\x08V[\x90P`@Q\x80`@\x01`@R\x80\x82\x81R` \x01a\x10\xF2\x83a#\xFAV[\x81RP\x83\x83\x81Q\x81\x10a\x11\x07Wa\x11\x07a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x10\xAAV[P\x93\x92PPPV[_``\x81\x80\x80a\x112\x87a$\x95V[\x90P_\x86\x90P_\x80a\x11W`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[_[\x8CQ\x81\x10\x15a\x15\0W\x8C\x81\x81Q\x81\x10a\x11tWa\x11ta:\xD8V[` \x02` \x01\x01Q\x91P\x82\x84a\x11\x8A\x91\x90a>\x91V[\x93Pa\x11\x97`\x01\x88a>\x91V[\x96P\x83_\x03a\x11\xF1W\x81Q\x80Q` \x90\x91\x01 \x85\x14a\x11\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\t-\xCE\xCC-\x8D,\x84\x0EM\xED\xEE\x84\r\x0C.m`{\x1B`D\x82\x01R`d\x01a\x04aV[a\x12\xADV[\x81QQ` \x11a\x12SW\x81Q\x80Q` \x90\x91\x01 \x85\x14a\x11\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FInvalid large internal hash\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[\x84a\x12`\x83_\x01Qa%\xA5V[\x14a\x12\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid internal node hash\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[a\x12\xB9`\x10`\x01a>\x91V[\x82` \x01QQ\x03a\x13)W\x85Q\x84\x14a\x15\0W_\x86\x85\x81Q\x81\x10a\x12\xDFWa\x12\xDFa:\xD8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_\x83` \x01Q\x82`\xFF\x16\x81Q\x81\x10a\x13\tWa\x13\ta:\xD8V[` \x02` \x01\x01Q\x90Pa\x13\x1C\x81a%\xCCV[\x96P`\x01\x94PPPa\x14\xF8V[`\x02\x82` \x01QQ\x03a\x14\xB0W_a\x13@\x83a&\0V[\x90P_\x81_\x81Q\x81\x10a\x13UWa\x13Ua:\xD8V[\x01` \x01Q`\xF8\x1C\x90P_a\x13k`\x02\x83a?\x0CV[a\x13v\x90`\x02a?-V[\x90P_a\x13\x86\x84\x83`\xFF\x16a&#V[\x90P_a\x13\x93\x8B\x8Aa&#V[\x90P_a\x13\xA0\x83\x83a&QV[\x90P`\xFF\x85\x16`\x02\x14\x80a\x13\xB7WP`\xFF\x85\x16`\x03\x14[\x15a\x13\xF1W\x80\x83Q\x14\x80\x15a\x13\xCCWP\x80\x82Q\x14[\x15a\x13\xDEWa\x13\xDB\x81\x8Ba>\x91V[\x99P[P`\x01`\xFF\x1B\x99Pa\x15\0\x94PPPPPV[`\xFF\x85\x16\x15\x80a\x14\x04WP`\xFF\x85\x16`\x01\x14[\x15a\x14YW\x80_\x03a\x14#WP`\x01`\xFF\x1B\x99Pa\x15\0\x94PPPPPV[a\x14J\x88` \x01Q`\x01\x81Q\x81\x10a\x14=Wa\x14=a:\xD8V[` \x02` \x01\x01Qa%\xCCV[\x9AP\x97Pa\x14\xF8\x94PPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FReceived a node with an unknown `D\x82\x01Re\x0E\x0EL\xAC\xCD/`\xD3\x1B`d\x82\x01R`\x84\x01a\x04aV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FReceived an unparsable node.\0\0\0\0`D\x82\x01R`d\x01a\x04aV[`\x01\x01a\x11YV[P`\x01`\xFF\x1B\x84\x14\x86a\x15\x13\x87\x86a&#V[\x90\x9E\x90\x9DP\x90\x9BP\x99PPPPPPPPPPV[` \x81\x01Q\x80Q``\x91a\x03\x07\x91a\x15B\x90`\x01\x90a;\0V[\x81Q\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[`@\x80Q`\x08\x80\x82R\x81\x83\x01\x90\x92R``\x91_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`\x08\x81\x10\x15a\x15\xCBWa\x15\x8C\x81`\x08a>zV[\x84`\x01`\x01`@\x1B\x03\x16\x90\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x15\xADWa\x15\xADa:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x15xV[P\x92\x91PPV[_\x80_\x83Q`A\x03a\x16\tW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa\x15\xFB\x88\x82\x85\x85a&\xCCV[\x95P\x95P\x95PPPPa\x16\x14V[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a\x16.Wa\x16.a>\xA4V[\x03a\x167WPPV[`\x01\x82`\x03\x81\x11\x15a\x16KWa\x16Ka>\xA4V[\x03a\x16iW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x16}Wa\x16}a>\xA4V[\x03a\x16\x9EW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04aV[`\x03\x82`\x03\x81\x11\x15a\x16\xB2Wa\x16\xB2a>\xA4V[\x03a\x16\xD3W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04aV[PPV[a\x16\xDFa8\x11V[_\x80\x82Ra\x16\xEFa\x02\x18\x84a\x03\xC3V[\x90P\x80Q`\t\x14\x15\x80\x15a\x17\x05WP\x80Q`\x06\x14\x15[\x15a\x17#W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x178\x81_\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[``\x83\x01R\x80Qa\x17V\x90\x82\x90`\x01\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x80\x83\x01R\x80Qa\x17t\x90\x82\x90`\x02\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xE0\x83\x01R\x80Qa\x17\x9F\x90\x82\x90`\x03\x90\x81\x10a\x17\x92Wa\x17\x92a:\xD8V[` \x02` \x01\x01Qa'\x94V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x83\x01R\x80Qa\x17\xC7\x90\x82\x90`\x04\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x01 \x83\x01R\x80Qa\x17\xE6\x90\x82\x90`\x05\x90\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[a\x01@\x83\x01R\x80Q`\x06\x03a\x17\xFBWP\x91\x90PV[_a\x18\x12\x82`\x06\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[\x90P_a\x18+\x83`\x07\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[\x90P_a\x18D\x84`\x08\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[\x90P\x81\x15\x80\x15a\x18RWP\x80\x15[\x15a\x18rW`\x01`\x01`@\x1B\x03\x83\x16` \x86\x01R`\x01`@\x86\x01Ra\x19\x04V[`#\x83`\x01`\x01`@\x1B\x03\x16\x10a\x18\xBAW`\x02a\x18\x90`#\x85a?FV[a\x18\x9A\x91\x90a?fV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x87\x01R\x83\x16a\x01\xE0\x86\x01R`\x01`@\x86\x01R[_a\x18\xC9`\x02`\x01\x86\x18a?\x8BV[a\x18\xD4\x90`\x1Ba?\xB0V[`@Q\x90\x91Pa\x18\xEC\x90\x84\x90\x84\x90\x84\x90` \x01a?\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x87\x01RP[PPPP\x91\x90PV[a\x19\x15a8\x11V[`\x01\x80\x82R\x82Q_\x91a\x195\x91a\x19-\x90\x82\x90a;\0V[\x85\x91\x90a(\0V[\x90P_a\x19Da\x02\x18\x83a\x03\xC3V[\x90P\x80Q`\x08\x14\x15\x80\x15a\x19ZWP\x80Q`\x0B\x14\x15[\x15a\x19xW`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19\x8D\x81_\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa\x19\xB4\x90\x82\x90`\x01\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[``\x84\x01R\x80Qa\x19\xD2\x90\x82\x90`\x02\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x80\x84\x01R\x80Qa\x19\xF0\x90\x82\x90`\x03\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xE0\x84\x01R\x80Qa\x1A\x0E\x90\x82\x90`\x04\x90\x81\x10a\x17\x92Wa\x17\x92a:\xD8V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\x1A6\x90\x82\x90`\x05\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x01 \x84\x01R\x80Qa\x1AU\x90\x82\x90`\x06\x90\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[\x83a\x01@\x01\x81\x90RP_a\x1A\x82\x82`\x07\x81Q\x81\x10a\x1AuWa\x1Aua:\xD8V[` \x02` \x01\x01Qa\x03\xEFV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x9DWa\x1A\x9Da8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xD0W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A\xBBW\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a\x1B.Wa\x1B\x04\x82\x82\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a:\xD8V[` \x02` \x01\x01Qa)@V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a\x1B\x1BWa\x1B\x1Ba:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xD9V[P\x81Q`\x08\x03a\x1B@WPPP\x91\x90PV[_a\x1BW\x83`\x08\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x1Bb\x90`\x1Ba?\xF3V[\x90P_a\x1B{\x84`\t\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P_a\x1B\x94\x85`\n\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P\x81\x81\x84`@Q` \x01a\x1B\xAB\x93\x92\x91\x90a?\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x88\x01RP\x94\x96\x95PPPPPPV[a\x1B\xD5a8\x11V[`\x02\x81R\x81Q_\x90a\x1B\xEF\x90`\x01\x90a\x19-\x90\x82\x90a;\0V[\x90P_a\x1B\xFEa\x02\x18\x83a\x03\xC3V[\x90P\x80Q`\t\x14\x15\x80\x15a\x1C\x14WP\x80Q`\x0C\x14\x15[\x15a\x1C2W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1CG\x81_\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa\x1Cn\x90\x82\x90`\x01\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[``\x84\x01R\x80Qa\x1C\x8C\x90\x82\x90`\x02\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xA0\x84\x01R\x80Qa\x1C\xAA\x90\x82\x90`\x03\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xC0\x84\x01R\x80Qa\x1C\xC8\x90\x82\x90`\x04\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xE0\x84\x01R\x80Qa\x1C\xE6\x90\x82\x90`\x05\x90\x81\x10a\x17\x92Wa\x17\x92a:\xD8V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\x1D\x0E\x90\x82\x90`\x06\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x01 \x84\x01R\x80Qa\x1D-\x90\x82\x90`\x07\x90\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[\x83a\x01@\x01\x81\x90RP_a\x1DM\x82`\x08\x81Q\x81\x10a\x1AuWa\x1Aua:\xD8V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DhWa\x1Dha8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x9BW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1D\x86W\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a\x1D\xECWa\x1D\xC2\x82\x82\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a:\xD8V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a\x1D\xD9Wa\x1D\xD9a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1D\xA4V[P\x81Q`\t\x03a\x1D\xFEWPPP\x91\x90PV[_a\x1E\x15\x83`\t\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x1E \x90`\x1Ba?\xF3V[\x90P_a\x1E9\x84`\n\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P_a\x1B\x94\x85`\x0B\x81Q\x81\x10a\x024Wa\x024a:\xD8V[a\x1EZa8\x11V[`\x03\x81R\x81Q_\x90a\x1Et\x90`\x01\x90a\x19-\x90\x82\x90a;\0V[\x90P_a\x1E\x83a\x02\x18\x83a\x03\xC3V[\x90P\x80Q`\x0B\x14\x15\x80\x15a\x1E\x99WP\x80Q`\x0E\x14\x15[\x15a\x1E\xB7W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xCC\x81_\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa\x1E\xF3\x90\x82\x90`\x01\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[``\x84\x01R\x80Qa\x1F\x11\x90\x82\x90`\x02\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xA0\x84\x01R\x80Qa\x1F/\x90\x82\x90`\x03\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xC0\x84\x01R\x80Qa\x1FM\x90\x82\x90`\x04\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[`\xE0\x84\x01R\x80Qa\x1Fk\x90\x82\x90`\x05\x90\x81\x10a\x17\x92Wa\x17\x92a:\xD8V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\x1F\x93\x90\x82\x90`\x06\x90\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a\x01 \x84\x01R\x80Qa\x1F\xB2\x90\x82\x90`\x07\x90\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[\x83a\x01@\x01\x81\x90RP_a\x1F\xD2\x82`\x08\x81Q\x81\x10a\x1AuWa\x1Aua:\xD8V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xEDWa\x1F\xEDa8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a  W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a \x0BW\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a qWa G\x82\x82\x81Q\x81\x10a\x1A\xF7Wa\x1A\xF7a:\xD8V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a ^Wa ^a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a )V[Pa \x88\x82`\t\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[\x84a\x01\x80\x01\x81\x81RPP_a \xA9\x83`\n\x81Q\x81\x10a\x1AuWa\x1Aua:\xD8V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a \xC4Wa \xC4a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xEDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[Pa\x01\xA0\x86\x01R_[\x81Q\x81\x10\x15a!>Wa!\x14\x82\x82\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x86a\x01\xA0\x01Q\x82\x81Q\x81\x10a!+Wa!+a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a \xF6V[P\x82Q`\x0B\x03a!QWPPPP\x91\x90PV[_a!h\x84`\x0B\x81Q\x81\x10a\x02\x98Wa\x02\x98a:\xD8V[a!s\x90`\x1Ba?\xF3V[\x90P_a!\x8C\x85`\x0C\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P_a!\xA5\x86`\r\x81Q\x81\x10a\x024Wa\x024a:\xD8V[\x90P\x81\x81\x84`@Q` \x01a!\xBC\x93\x92\x91\x90a?\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x89\x01RP\x95\x97\x96PPPPPPPV[_a!\xE9\x82a)KV[\x80Q\x90` \x01 \x90P\x91\x90PV[``\x81a\x01\xC0\x01QQ_\x03a\"\x1FW`@Qc\xD4f\xBD\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x01\xC0\x01QQ`A\x14a\"FW`@QcK\xE62\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x01\xC0\x81\x01Q[\x91\x90PV[``\x80`8\x84\x10\x15a\"\xB8W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90Pa\"\x88\x83\x85a?\xF3V[`\xF8\x1B\x81_\x81Q\x81\x10a\"\x9DWa\"\x9Da:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa\x0E\x18V[_`\x01[a\"\xC6\x81\x87a;'V[\x15a\"\xECW\x81a\"\xD5\x81a>\xE0V[\x92Pa\"\xE5\x90Pa\x01\0\x82a>zV[\x90Pa\"\xBCV[a\"\xF7\x82`\x01a>\x91V[`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x0EWa#\x0Ea8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#8W` \x82\x01\x81\x806\x837\x01\x90P[P\x92Pa#E\x85\x83a?\xF3V[a#P\x90`7a?\xF3V[`\xF8\x1B\x83_\x81Q\x81\x10a#eWa#ea:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x90P[\x81\x81\x11a#\xF1Wa\x01\0a#\x94\x82\x84a;\0V[a#\xA0\x90a\x01\0a@\xECV[a#\xAA\x90\x88a;'V[a#\xB4\x91\x90a@\xF7V[`\xF8\x1B\x83\x82\x81Q\x81\x10a#\xC9Wa#\xC9a:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x80a#\xE9\x81a>\xE0V[\x91PPa#\x80V[PP\x93\x92PPPV[``a\x03\x07a\x02\x18\x83a\x03\xC3V[``_\x80_a$\x16\x85a\x08\xFFV[\x91\x94P\x92P\x90P_\x81`\x01\x81\x11\x15a$0Wa$0a>\xA4V[\x14a$}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid RLP bytes value.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[a$\x8C\x85` \x01Q\x84\x84a)\xD8V[\x95\x94PPPPPV[``_\x82Q`\x02\x02`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xB4Wa$\xB4a8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a$\xDEW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x15\xCBW`\x04\x84\x82\x81Q\x81\x10a%\0Wa%\0a:\xD8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x82\x82`\x02\x02\x81Q\x81\x10a%,Wa%,a:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x10\x84\x82\x81Q\x81\x10a%VWa%Va:\xD8V[\x01` \x01Q`\xF8\x1C\x81a%kWa%ka;\x13V[\x06`\xF8\x1B\x82\x82`\x02\x02`\x01\x01\x81Q\x81\x10a%\x87Wa%\x87a:\xD8V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a$\xE3V[_` \x82Q\x10\x15a%\xB8WP` \x01Q\x90V[\x81\x80` \x01\x90Q\x81\x01\x90a\x03\x07\x91\x90aA\nV[_``` \x83_\x01Q\x10\x15a%\xEBWa%\xE4\x83a)@V[\x90Pa%\xF7V[a%\xF4\x83a$\x08V[\x90P[a\x0E\x18\x81a%\xA5V[``a\x03\x07a&\x1E\x83` \x01Q_\x81Q\x81\x10a\x10\xC9Wa\x10\xC9a:\xD8V[a$\x95V[``\x81\x83Q\x03_\x03a&CWP`@\x80Q` \x81\x01\x90\x91R_\x81Ra\x03\x07V[a\x0E\x18\x83\x83\x84\x86Q\x03a(\0V[_\x80[\x80\x84Q\x11\x80\x15a&dWP\x80\x83Q\x11[\x80\x15a&\xB5WP\x82\x81\x81Q\x81\x10a&}Wa&}a:\xD8V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a&\xA4Wa&\xA4a:\xD8V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14[\x15a\x0E\x18W\x80a&\xC4\x81a>\xE0V[\x91PPa&TV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a'\x05WP_\x91P`\x03\x90P\x82a'\x8AV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a'VW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a'\x81WP_\x92P`\x01\x91P\x82\x90Pa'\x8AV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[\x80Q_\x90`\x01\x03a'\xA6WP_\x91\x90PV[\x81Q`\x15\x14a'\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP address value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04aV[a\x03\x07\x82a\x06\xD7V[``\x81\x82`\x1F\x01\x10\x15a(FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x04aV[\x82\x82\x84\x01\x10\x15a(\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x04aV[\x81\x83\x01\x84Q\x10\x15a(\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x04aV[``\x82\x15\x80\x15a(\xEEW`@Q\x91P_\x82R` \x82\x01`@Ra\x0F\xC1V[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a)'W\x80Q\x83R` \x92\x83\x01\x92\x01a)\x0FV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RPP\x94\x93PPPPV[``a\x03\x07\x82a*\x85V[``_\x82Q`\x03\x81\x11\x15a)aWa)aa>\xA4V[\x03a)oWa\x03\x07\x82a*\x99V[`\x01\x82Q`\x03\x81\x11\x15a)\x84Wa)\x84a>\xA4V[\x03a)\x92Wa\x03\x07\x82a-\x85V[`\x02\x82Q`\x03\x81\x11\x15a)\xA7Wa)\xA7a>\xA4V[\x03a)\xB5Wa\x03\x07\x82a/\xF6V[`\x03\x82Q`\x03\x81\x11\x15a)\xCAWa)\xCAa>\xA4V[\x03a\x0EyWa\x03\x07\x82a2dV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xF3Wa)\xF3a8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a*\x1DW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x80Q_\x03a*/W\x90Pa\x0E\x18V[\x84\x84\x01` \x82\x01_[` \x86\x04\x81\x10\x15a*YW\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a*8V[P_`\x01` \x87\x06` \x03a\x01\0\n\x03\x90P\x80\x82Q\x16\x81\x19\x84Q\x16\x17\x82R\x83\x94PPPPP\x93\x92PPPV[``a\x03\x07\x82` \x01Q_\x84_\x01Qa)\xD8V[``_\x82` \x01Q`\x01`\x01`@\x1B\x03\x16_\x14a*\xBBWP` \x82\x01Qa*\xFEV[a\x01\xC0\x83\x01QQ\x15a*\xFEW`#\x83a\x01\xE0\x01Q`\x01`\x01`@\x1B\x03\x16\x10a*\xFEW`\x02`#\x84a\x01\xE0\x01Qa*\xF1\x91\x90a?FV[a*\xFB\x91\x90a?fV[\x90P[_\x83`@\x01Qa+\x0EW_a+\x11V[`\x03[a+\x1C\x90`\x06a?\xF3V[`\xFF\x16\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a+:Wa+:a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+mW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a+XW\x90P[P\x90Pa+}\x85``\x01Qa\x0C\x81V[\x81_\x81Q\x81\x10a+\x8FWa+\x8Fa:\xD8V[` \x02` \x01\x01\x81\x90RPa+\xA7\x85`\x80\x01Qa\x0C\x81V[\x81`\x01\x81Q\x81\x10a+\xBAWa+\xBAa:\xD8V[` \x02` \x01\x01\x81\x90RPa+\xD2\x85`\xE0\x01Qa\x0C\x81V[\x81`\x02\x81Q\x81\x10a+\xE5Wa+\xE5a:\xD8V[` \x02` \x01\x01\x81\x90RPa+\xFE\x85a\x01\0\x01Qa6\x07V[\x81`\x03\x81Q\x81\x10a,\x11Wa,\x11a:\xD8V[` \x02` \x01\x01\x81\x90RPa,*\x85a\x01 \x01Qa\x0C\x81V[\x81`\x04\x81Q\x81\x10a,=Wa,=a:\xD8V[` \x02` \x01\x01\x81\x90RPa,V\x85a\x01@\x01Qa\x0F\xCAV[\x81`\x05\x81Q\x81\x10a,iWa,ia:\xD8V[` \x02` \x01\x01\x81\x90RP\x84`@\x01Q\x15a-|W\x84` \x01Q`\x01`\x01`@\x1B\x03\x16_\x03a,\xD0W`@Q_` \x82\x01R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81`\x06\x81Q\x81\x10a,\xC0Wa,\xC0a:\xD8V[` \x02` \x01\x01\x81\x90RPa-\x01V[a,\xE2\x83`\x01`\x01`@\x1B\x03\x16a\x0C\x81V[\x81`\x06\x81Q\x81\x10a,\xF5Wa,\xF5a:\xD8V[` \x02` \x01\x01\x81\x90RP[a-6_[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a-0W` \x82\x01\x81\x806\x837\x01\x90P[Pa\x0F\xCAV[\x81`\x07\x81Q\x81\x10a-IWa-Ia:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra-]_a-\x06V[\x81`\x08\x81Q\x81\x10a-pWa-pa:\xD8V[` \x02` \x01\x01\x81\x90RP[a$\x8C\x81a6?V[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a-\xA1W\x90PP\x90Pa-\xCE\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x0C\x81V[\x81_\x81Q\x81\x10a-\xE0Wa-\xE0a:\xD8V[` \x02` \x01\x01\x81\x90RPa-\xF8\x83``\x01Qa\x0C\x81V[\x81`\x01\x81Q\x81\x10a.\x0BWa.\x0Ba:\xD8V[` \x02` \x01\x01\x81\x90RPa.#\x83`\x80\x01Qa\x0C\x81V[\x81`\x02\x81Q\x81\x10a.6Wa.6a:\xD8V[` \x02` \x01\x01\x81\x90RPa.N\x83`\xE0\x01Qa\x0C\x81V[\x81`\x03\x81Q\x81\x10a.aWa.aa:\xD8V[` \x02` \x01\x01\x81\x90RPa.z\x83a\x01\0\x01Qa6\x07V[\x81`\x04\x81Q\x81\x10a.\x8DWa.\x8Da:\xD8V[` \x02` \x01\x01\x81\x90RPa.\xA6\x83a\x01 \x01Qa\x0C\x81V[\x81`\x05\x81Q\x81\x10a.\xB9Wa.\xB9a:\xD8V[` \x02` \x01\x01\x81\x90RPa.\xD2\x83a\x01@\x01Qa\x0F\xCAV[\x81`\x06\x81Q\x81\x10a.\xE5Wa.\xE5a:\xD8V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x0FWa/\x0Fa8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a/BW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a/-W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a/\x99W\x84a\x01`\x01Q\x81\x81Q\x81\x10a/lWa/la:\xD8V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a/\x86Wa/\x86a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a/GV[Pa/\xA3\x81a6?V[\x82`\x07\x81Q\x81\x10a/\xB6Wa/\xB6a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01[a/\xCD\x83a6?V[`@Q` \x01a/\xDE\x92\x91\x90aA!V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`@\x80Q`\t\x80\x82Ra\x01@\x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a0\x12W\x90PP\x90Pa0?\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x0C\x81V[\x81_\x81Q\x81\x10a0QWa0Qa:\xD8V[` \x02` \x01\x01\x81\x90RPa0i\x83``\x01Qa\x0C\x81V[\x81`\x01\x81Q\x81\x10a0|Wa0|a:\xD8V[` \x02` \x01\x01\x81\x90RPa0\x94\x83`\xA0\x01Qa\x0C\x81V[\x81`\x02\x81Q\x81\x10a0\xA7Wa0\xA7a:\xD8V[` \x02` \x01\x01\x81\x90RPa0\xBF\x83`\xC0\x01Qa\x0C\x81V[\x81`\x03\x81Q\x81\x10a0\xD2Wa0\xD2a:\xD8V[` \x02` \x01\x01\x81\x90RPa0\xEA\x83`\xE0\x01Qa\x0C\x81V[\x81`\x04\x81Q\x81\x10a0\xFDWa0\xFDa:\xD8V[` \x02` \x01\x01\x81\x90RPa1\x16\x83a\x01\0\x01Qa6\x07V[\x81`\x05\x81Q\x81\x10a1)Wa1)a:\xD8V[` \x02` \x01\x01\x81\x90RPa1B\x83a\x01 \x01Qa\x0C\x81V[\x81`\x06\x81Q\x81\x10a1UWa1Ua:\xD8V[` \x02` \x01\x01\x81\x90RPa1n\x83a\x01@\x01Qa\x0F\xCAV[\x81`\x07\x81Q\x81\x10a1\x81Wa1\x81a:\xD8V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xABWa1\xABa8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a1\xDEW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a1\xC9W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a25W\x84a\x01`\x01Q\x81\x81Q\x81\x10a2\x08Wa2\x08a:\xD8V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a2\"Wa2\"a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a1\xE3V[Pa2?\x81a6?V[\x82`\x08\x81Q\x81\x10a2RWa2Ra:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x02a/\xC4V[`@\x80Q`\x0B\x80\x82Ra\x01\x80\x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a2\x80W\x90PP\x90Pa2\xAD\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x0C\x81V[\x81_\x81Q\x81\x10a2\xBFWa2\xBFa:\xD8V[` \x02` \x01\x01\x81\x90RPa2\xD7\x83``\x01Qa\x0C\x81V[\x81`\x01\x81Q\x81\x10a2\xEAWa2\xEAa:\xD8V[` \x02` \x01\x01\x81\x90RPa3\x02\x83`\xA0\x01Qa\x0C\x81V[\x81`\x02\x81Q\x81\x10a3\x15Wa3\x15a:\xD8V[` \x02` \x01\x01\x81\x90RPa3-\x83`\xC0\x01Qa\x0C\x81V[\x81`\x03\x81Q\x81\x10a3@Wa3@a:\xD8V[` \x02` \x01\x01\x81\x90RPa3X\x83`\xE0\x01Qa\x0C\x81V[\x81`\x04\x81Q\x81\x10a3kWa3ka:\xD8V[` \x02` \x01\x01\x81\x90RPa3\x84\x83a\x01\0\x01Qa6\x07V[\x81`\x05\x81Q\x81\x10a3\x97Wa3\x97a:\xD8V[` \x02` \x01\x01\x81\x90RPa3\xB0\x83a\x01 \x01Qa\x0C\x81V[\x81`\x06\x81Q\x81\x10a3\xC3Wa3\xC3a:\xD8V[` \x02` \x01\x01\x81\x90RPa3\xDC\x83a\x01@\x01Qa\x0F\xCAV[\x81`\x07\x81Q\x81\x10a3\xEFWa3\xEFa:\xD8V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a4\x19Wa4\x19a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a4LW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a47W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a4\xA3W\x84a\x01`\x01Q\x81\x81Q\x81\x10a4vWa4va:\xD8V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a4\x90Wa4\x90a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a4QV[Pa4\xAD\x81a6?V[\x82`\x08\x81Q\x81\x10a4\xC0Wa4\xC0a:\xD8V[` \x02` \x01\x01\x81\x90RPa4\xD9\x84a\x01\x80\x01Qa\x0C\x81V[\x82`\t\x81Q\x81\x10a4\xECWa4\xECa:\xD8V[` \x02` \x01\x01\x81\x90RP_\x84a\x01\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x16Wa5\x16a8\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5IW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a54W\x90P[P\x90P_[\x85a\x01\xA0\x01QQ\x81\x10\x15a5\xAAWa5\x85\x86a\x01\xA0\x01Q\x82\x81Q\x81\x10a5vWa5va:\xD8V[` \x02` \x01\x01Q_\x1Ca\x0C\x81V[\x82\x82\x81Q\x81\x10a5\x97Wa5\x97a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a5NV[Pa5\xB4\x81a6?V[\x83`\n\x81Q\x81\x10a5\xC7Wa5\xC7a:\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x03a5\xDD\x84a6?V[`@Q` \x01a5\xEE\x92\x91\x90aA!V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x91\x90PV[`@Q``\x82\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x83\x01R\x90a\x03\x07\x90`4\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0F\xCAV[``_a6K\x83a6\x82V[\x90Pa6Y\x81Q`\xC0a\"SV[\x81`@Q` \x01a6k\x92\x91\x90a>\xF8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[``\x81Q_\x03a6\xA1W`@\x80Q_\x80\x82R` \x82\x01\x90\x92R\x90a\x15\xCBV[_\x80[\x83Q\x81\x10\x15a6\xDDW\x83\x81\x81Q\x81\x10a6\xBFWa6\xBFa:\xD8V[` \x02` \x01\x01QQ\x82a6\xD3\x91\x90a>\x91V[\x91P`\x01\x01a6\xA4V[_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xF6Wa6\xF6a8\xA0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a7 W` \x82\x01\x81\x806\x837\x01\x90P[P_\x92P\x90P` \x81\x01[\x85Q\x83\x10\x15a\x0F\xC1W_\x86\x84\x81Q\x81\x10a7GWa7Ga:\xD8V[` \x02` \x01\x01Q\x90P_` \x82\x01\x90Pa7d\x83\x82\x84Qa7\x9AV[\x87\x85\x81Q\x81\x10a7vWa7va:\xD8V[` \x02` \x01\x01QQ\x83a7\x8A\x91\x90a>\x91V[`\x01\x90\x95\x01\x94\x92Pa7+\x91PPV[\x82\x82\x82[` \x81\x10a7\xD6W\x81Q\x83Ra7\xB5` \x84a>\x91V[\x92Pa7\xC2` \x83a>\x91V[\x91Pa7\xCF` \x82a;\0V[\x90Pa7\x9EV[_`\x01a7\xE4\x83` a;\0V[a7\xF0\x90a\x01\0a@\xECV[a7\xFA\x91\x90a;\0V[\x92Q\x84Q\x84\x16\x93\x19\x16\x92\x90\x92\x17\x90\x92RPPPPPV[`@\x80Qa\x02\0\x81\x01\x90\x91R\x80_\x81R` \x01_`\x01`\x01`@\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_`\x01`\x01`@\x1B\x03\x16\x81RP\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8\xD6Wa8\xD6a8\xA0V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a9\x04Wa9\x04a8\xA0V[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a9\x1BW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a94Wa94a8\xA0V[a9G`\x1F\x82\x01`\x1F\x19\x16` \x01a8\xDCV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a9[W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a9\x87W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x9CW_\x80\xFD[a9\xA8\x84\x82\x85\x01a9\x0CV[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a9\xC0W_\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"NW_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a9\xEDW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x03W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a:\x1AW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a:6W_\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:LW_\x80\xFD[\x90\x88\x01\x90a\x01`\x82\x8B\x03\x12\x15a:`W_\x80\xFD[\x90\x96P` \x88\x015\x90\x80\x82\x11\x15a:uW_\x80\xFD[\x90\x88\x01\x90`\x80\x82\x8B\x03\x12\x15a:\x88W_\x80\xFD[\x81\x96Pa:\x97`@\x8A\x01a9\xC7V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a:\xACW_\x80\xFD[Pa:\xB9\x89\x82\x8A\x01a9\xDDV[\x90\x94P\x92Pa:\xCC\x90P`\x80\x88\x01a9\xC7V[\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x07Wa\x03\x07a:\xECV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a;5Wa;5a;\x13V[P\x04\x90V[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a;OW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a;hW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a:\x1AW_\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\"NW_\x80\xFD[_` \x82\x84\x03\x12\x15a;\xA2W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\xB8W_\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a;\xCBW_\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a;\xE6Wa;\xE6a8\xA0V[`@Ra;\xF2\x83a;|V[\x81R` \x83\x015\x82\x81\x11\x15a<\x05W_\x80\xFD[a<\x11\x87\x82\x86\x01a9\x0CV[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a<(W_\x80\xFD[a<4\x87\x82\x86\x01a9\x0CV[`@\x83\x01RP\x95\x94PPPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a<[Wa<[a8\xA0V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a<tW_\x80\xFD[\x815` a<\x89a<\x84\x83a<CV[a8\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<\xA7W_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<\xE4W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xC8W_\x80\xFD[a<\xD6\x89\x86\x83\x8B\x01\x01a9\x0CV[\x84RP\x91\x83\x01\x91\x83\x01a<\xABV[P\x96\x95PPPPPPV[_\x82`\x1F\x83\x01\x12a<\xFEW_\x80\xFD[\x815` a=\x0Ea<\x84\x83a<CV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a=/W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15a<\xE4W\x805\x83R\x91\x83\x01\x91\x83\x01a=4V[_` \x82\x84\x03\x12\x15a=[W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=qW_\x80\xFD[\x90\x83\x01\x90`\xC0\x82\x86\x03\x12\x15a=\x84W_\x80\xFD[a=\x8Ca8\xB4V[\x825\x81R` \x83\x015\x82\x81\x11\x15a=\xA1W_\x80\xFD[a=\xAD\x87\x82\x86\x01a9\x0CV[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15a=\xC4W_\x80\xFD[a=\xD0\x87\x82\x86\x01a9\x0CV[`@\x83\x01RP``\x83\x015\x82\x81\x11\x15a=\xE7W_\x80\xFD[a=\xF3\x87\x82\x86\x01a9\x0CV[``\x83\x01RP`\x80\x83\x015\x82\x81\x11\x15a>\nW_\x80\xFD[a>\x16\x87\x82\x86\x01a<eV[`\x80\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a>-W_\x80\xFD[a>9\x87\x82\x86\x01a<\xEFV[`\xA0\x83\x01RP\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a>XW_\x80\xFD[a\x0E\x18\x82a;|V[_` \x82\x84\x03\x12\x15a>qW_\x80\xFD[a\x0E\x18\x82a9\xC7V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x07Wa\x03\x07a:\xECV[\x80\x82\x01\x80\x82\x11\x15a\x03\x07Wa\x03\x07a:\xECV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x82\x81R_a9\xA8` \x83\x01\x84a>\xB8V[_`\x01\x82\x01a>\xF1Wa>\xF1a:\xECV[P`\x01\x01\x90V[_a9\xA8a?\x06\x83\x86a>\xB8V[\x84a>\xB8V[_`\xFF\x83\x16\x80a?\x1EWa?\x1Ea;\x13V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x03\x07Wa\x03\x07a:\xECV[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x15\xCBWa\x15\xCBa:\xECV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a?\x7FWa?\x7Fa;\x13V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a?\xA4Wa?\xA4a;\x13V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x15\xCBWa\x15\xCBa:\xECV[\x92\x83R` \x83\x01\x91\x90\x91R`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`@\x82\x01R`A\x01\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\x07Wa\x03\x07a:\xECV[`\x01\x81\x81[\x80\x85\x11\x15a@FW\x81_\x19\x04\x82\x11\x15a@,Wa@,a:\xECV[\x80\x85\x16\x15a@9W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a@\x11V[P\x92P\x92\x90PV[_\x82a@\\WP`\x01a\x03\x07V[\x81a@hWP_a\x03\x07V[\x81`\x01\x81\x14a@~W`\x02\x81\x14a@\x88Wa@\xA4V[`\x01\x91PPa\x03\x07V[`\xFF\x84\x11\x15a@\x99Wa@\x99a:\xECV[PP`\x01\x82\x1Ba\x03\x07V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a@\xC7WP\x81\x81\na\x03\x07V[a@\xD1\x83\x83a@\x0CV[\x80_\x19\x04\x82\x11\x15a@\xE4Wa@\xE4a:\xECV[\x02\x93\x92PPPV[_a\x0E\x18\x83\x83a@NV[_\x82aA\x05WaA\x05a;\x13V[P\x06\x90V[_` \x82\x84\x03\x12\x15aA\x1AW_\x80\xFD[PQ\x91\x90PV[`\xF8\x83\x90\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x81R_a9\xA8`\x01\x83\x01\x84a>\xB8V\xFE\xA2dipfsX\"\x12 \xD8\xCC`O\xA7REN\x19\x8FxRx\xBAr\xBD]\xBAO\xA5\x0E~B\xA0\x9DG\xA4\xDBKT\xC2\x05dsolcC\0\x08\x19\x003",
    );
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `BeaconRootNotFound()` and selector `0xe9697b68`.
	```solidity
	error BeaconRootNotFound();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct BeaconRootNotFound;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<BeaconRootNotFound> for UnderlyingRustTuple<'_> {
			fn from(value: BeaconRootNotFound) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for BeaconRootNotFound {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for BeaconRootNotFound {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "BeaconRootNotFound()";
			const SELECTOR: [u8; 4] = [233u8, 105u8, 123u8, 104u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `BlockIsNotFinalized()` and selector `0xb6144bff`.
	```solidity
	error BlockIsNotFinalized();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct BlockIsNotFinalized;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<BlockIsNotFinalized> for UnderlyingRustTuple<'_> {
			fn from(value: BlockIsNotFinalized) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for BlockIsNotFinalized {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for BlockIsNotFinalized {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "BlockIsNotFinalized()";
			const SELECTOR: [u8; 4] = [182u8, 20u8, 75u8, 255u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `BlockIsTooOld()` and selector `0x3373ade4`.
	```solidity
	error BlockIsTooOld();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct BlockIsTooOld;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<BlockIsTooOld> for UnderlyingRustTuple<'_> {
			fn from(value: BlockIsTooOld) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for BlockIsTooOld {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for BlockIsTooOld {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "BlockIsTooOld()";
			const SELECTOR: [u8; 4] = [51u8, 115u8, 173u8, 228u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `ChallengeAlreadyExists()` and selector `0x44504357`.
	```solidity
	error ChallengeAlreadyExists();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct ChallengeAlreadyExists;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<ChallengeAlreadyExists> for UnderlyingRustTuple<'_> {
			fn from(value: ChallengeAlreadyExists) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChallengeAlreadyExists {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for ChallengeAlreadyExists {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "ChallengeAlreadyExists()";
			const SELECTOR: [u8; 4] = [68u8, 80u8, 67u8, 87u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `ChallengeDoesNotExist()` and selector `0x03126e30`.
	```solidity
	error ChallengeDoesNotExist();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct ChallengeDoesNotExist;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<ChallengeDoesNotExist> for UnderlyingRustTuple<'_> {
			fn from(value: ChallengeDoesNotExist) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChallengeDoesNotExist {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for ChallengeDoesNotExist {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "ChallengeDoesNotExist()";
			const SELECTOR: [u8; 4] = [3u8, 18u8, 110u8, 48u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `DelegationExpired()` and selector `0x30d3ba07`.
	```solidity
	error DelegationExpired();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct DelegationExpired;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<DelegationExpired> for UnderlyingRustTuple<'_> {
			fn from(value: DelegationExpired) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelegationExpired {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for DelegationExpired {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "DelegationExpired()";
			const SELECTOR: [u8; 4] = [48u8, 211u8, 186u8, 7u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
	```solidity
	error ECDSAInvalidSignature();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct ECDSAInvalidSignature;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
			fn from(value: ECDSAInvalidSignature) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for ECDSAInvalidSignature {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
			const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
	```solidity
	error ECDSAInvalidSignatureLength(uint256 length);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct ECDSAInvalidSignatureLength {
		#[allow(missing_docs)]
		pub length: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<ECDSAInvalidSignatureLength> for UnderlyingRustTuple<'_> {
			fn from(value: ECDSAInvalidSignatureLength) -> Self {
				(value.length,)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureLength {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { length: tuple.0 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
			const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.length),)
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
	```solidity
	error ECDSAInvalidSignatureS(bytes32 s);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct ECDSAInvalidSignatureS {
		#[allow(missing_docs)]
		pub s: alloy::sol_types::private::FixedBytes<32>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
			fn from(value: ECDSAInvalidSignatureS) -> Self {
				(value.s,)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { s: tuple.0 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
			const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.s),)
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `EthTransferFailed()` and selector `0x6d963f88`.
	```solidity
	error EthTransferFailed();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct EthTransferFailed;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<EthTransferFailed> for UnderlyingRustTuple<'_> {
			fn from(value: EthTransferFailed) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for EthTransferFailed {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for EthTransferFailed {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "EthTransferFailed()";
			const SELECTOR: [u8; 4] = [109u8, 150u8, 63u8, 136u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `FraudProofWindowActive()` and selector `0x1f304cc1`.
	```solidity
	error FraudProofWindowActive();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct FraudProofWindowActive;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<FraudProofWindowActive> for UnderlyingRustTuple<'_> {
			fn from(value: FraudProofWindowActive) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for FraudProofWindowActive {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for FraudProofWindowActive {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "FraudProofWindowActive()";
			const SELECTOR: [u8; 4] = [31u8, 48u8, 76u8, 193u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `IncorrectChallengeBond()` and selector `0x9b8cc146`.
	```solidity
	error IncorrectChallengeBond();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct IncorrectChallengeBond;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<IncorrectChallengeBond> for UnderlyingRustTuple<'_> {
			fn from(value: IncorrectChallengeBond) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for IncorrectChallengeBond {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for IncorrectChallengeBond {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "IncorrectChallengeBond()";
			const SELECTOR: [u8; 4] = [155u8, 140u8, 193u8, 70u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InvalidBlockHash()` and selector `0x3e068cb6`.
	```solidity
	error InvalidBlockHash();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InvalidBlockHash;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InvalidBlockHash> for UnderlyingRustTuple<'_> {
			fn from(value: InvalidBlockHash) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBlockHash {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InvalidBlockHash {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InvalidBlockHash()";
			const SELECTOR: [u8; 4] = [62u8, 6u8, 140u8, 182u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InvalidBlockNumber()` and selector `0x4e47846c`.
	```solidity
	error InvalidBlockNumber();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InvalidBlockNumber;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InvalidBlockNumber> for UnderlyingRustTuple<'_> {
			fn from(value: InvalidBlockNumber) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBlockNumber {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InvalidBlockNumber {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InvalidBlockNumber()";
			const SELECTOR: [u8; 4] = [78u8, 71u8, 132u8, 108u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InvalidFieldCount()` and selector `0xc2871a37`.
	```solidity
	error InvalidFieldCount();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InvalidFieldCount;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InvalidFieldCount> for UnderlyingRustTuple<'_> {
			fn from(value: InvalidFieldCount) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidFieldCount {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InvalidFieldCount {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InvalidFieldCount()";
			const SELECTOR: [u8; 4] = [194u8, 135u8, 26u8, 55u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InvalidParentBlockHash()` and selector `0x41f4d916`.
	```solidity
	error InvalidParentBlockHash();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InvalidParentBlockHash;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InvalidParentBlockHash> for UnderlyingRustTuple<'_> {
			fn from(value: InvalidParentBlockHash) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidParentBlockHash {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InvalidParentBlockHash {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InvalidParentBlockHash()";
			const SELECTOR: [u8; 4] = [65u8, 244u8, 217u8, 22u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InvalidSignatureLength()` and selector `0x4be6321b`.
	```solidity
	error InvalidSignatureLength();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InvalidSignatureLength;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InvalidSignatureLength> for UnderlyingRustTuple<'_> {
			fn from(value: InvalidSignatureLength) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignatureLength {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InvalidSignatureLength {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InvalidSignatureLength()";
			const SELECTOR: [u8; 4] = [75u8, 230u8, 50u8, 27u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `NoSignature()` and selector `0xd466bd15`.
	```solidity
	error NoSignature();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct NoSignature;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<NoSignature> for UnderlyingRustTuple<'_> {
			fn from(value: NoSignature) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoSignature {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for NoSignature {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "NoSignature()";
			const SELECTOR: [u8; 4] = [212u8, 102u8, 189u8, 21u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `NotURC()` and selector `0xc7cb1eae`.
	```solidity
	error NotURC();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct NotURC;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<NotURC> for UnderlyingRustTuple<'_> {
			fn from(value: NotURC) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotURC {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for NotURC {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "NotURC()";
			const SELECTOR: [u8; 4] = [199u8, 203u8, 30u8, 174u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `TransactionExcluded()` and selector `0x094cec5f`.
	```solidity
	error TransactionExcluded();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct TransactionExcluded;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<TransactionExcluded> for UnderlyingRustTuple<'_> {
			fn from(value: TransactionExcluded) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for TransactionExcluded {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for TransactionExcluded {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "TransactionExcluded()";
			const SELECTOR: [u8; 4] = [9u8, 76u8, 236u8, 95u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `UnexpectedSigner()` and selector `0xaaaa9141`.
	```solidity
	error UnexpectedSigner();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct UnexpectedSigner;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnexpectedSigner> for UnderlyingRustTuple<'_> {
			fn from(value: UnexpectedSigner) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnexpectedSigner {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for UnexpectedSigner {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "UnexpectedSigner()";
			const SELECTOR: [u8; 4] = [170u8, 170u8, 145u8, 65u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `UnsupportedTxType()` and selector `0xdf87b546`.
	```solidity
	error UnsupportedTxType();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct UnsupportedTxType;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnsupportedTxType> for UnderlyingRustTuple<'_> {
			fn from(value: UnsupportedTxType) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnsupportedTxType {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for UnsupportedTxType {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "UnsupportedTxType()";
			const SELECTOR: [u8; 4] = [223u8, 135u8, 181u8, 70u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `WrongChallengerAddress()` and selector `0x7546e8c1`.
	```solidity
	error WrongChallengerAddress();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct WrongChallengerAddress;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<WrongChallengerAddress> for UnderlyingRustTuple<'_> {
			fn from(value: WrongChallengerAddress) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongChallengerAddress {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for WrongChallengerAddress {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "WrongChallengerAddress()";
			const SELECTOR: [u8; 4] = [117u8, 70u8, 232u8, 193u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `WrongTransactionHashProof()` and selector `0x88c5671e`.
	```solidity
	error WrongTransactionHashProof();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct WrongTransactionHashProof;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<WrongTransactionHashProof> for UnderlyingRustTuple<'_> {
			fn from(value: WrongTransactionHashProof) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongTransactionHashProof {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for WrongTransactionHashProof {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "WrongTransactionHashProof()";
			const SELECTOR: [u8; 4] = [136u8, 197u8, 103u8, 30u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	/**Constructor`.
	```solidity
	constructor(uint256 _slashAmountWei);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct constructorCall {
		#[allow(missing_docs)]
		pub _slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
	}
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
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
			impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
				fn from(value: constructorCall) -> Self {
					(value._slashAmountWei,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _slashAmountWei: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolConstructor for constructorCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self._slashAmountWei),)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `BEACON_ROOTS_CONTRACT()` and selector `0x86fb61ed`.
	```solidity
	function BEACON_ROOTS_CONTRACT() external view returns (address);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct BEACON_ROOTS_CONTRACTCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`BEACON_ROOTS_CONTRACT()`](BEACON_ROOTS_CONTRACTCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct BEACON_ROOTS_CONTRACTReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::Address,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
			impl ::core::convert::From<BEACON_ROOTS_CONTRACTCall> for UnderlyingRustTuple<'_> {
				fn from(value: BEACON_ROOTS_CONTRACTCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for BEACON_ROOTS_CONTRACTCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
			impl ::core::convert::From<BEACON_ROOTS_CONTRACTReturn> for UnderlyingRustTuple<'_> {
				fn from(value: BEACON_ROOTS_CONTRACTReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for BEACON_ROOTS_CONTRACTReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for BEACON_ROOTS_CONTRACTCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::Address;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "BEACON_ROOTS_CONTRACT()";
			const SELECTOR: [u8; 4] = [134u8, 251u8, 97u8, 237u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: BEACON_ROOTS_CONTRACTReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: BEACON_ROOTS_CONTRACTReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `BLOCKHASH_EVM_LOOKBACK()` and selector `0xf45e8118`.
	```solidity
	function BLOCKHASH_EVM_LOOKBACK() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct BLOCKHASH_EVM_LOOKBACKCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`BLOCKHASH_EVM_LOOKBACK()`](BLOCKHASH_EVM_LOOKBACKCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct BLOCKHASH_EVM_LOOKBACKReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
			impl ::core::convert::From<BLOCKHASH_EVM_LOOKBACKCall> for UnderlyingRustTuple<'_> {
				fn from(value: BLOCKHASH_EVM_LOOKBACKCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for BLOCKHASH_EVM_LOOKBACKCall {
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
			impl ::core::convert::From<BLOCKHASH_EVM_LOOKBACKReturn> for UnderlyingRustTuple<'_> {
				fn from(value: BLOCKHASH_EVM_LOOKBACKReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for BLOCKHASH_EVM_LOOKBACKReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for BLOCKHASH_EVM_LOOKBACKCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "BLOCKHASH_EVM_LOOKBACK()";
			const SELECTOR: [u8; 4] = [244u8, 94u8, 129u8, 24u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: BLOCKHASH_EVM_LOOKBACKReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: BLOCKHASH_EVM_LOOKBACKReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `EIP4788_WINDOW()` and selector `0xcac62dd9`.
	```solidity
	function EIP4788_WINDOW() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct EIP4788_WINDOWCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`EIP4788_WINDOW()`](EIP4788_WINDOWCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct EIP4788_WINDOWReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
			impl ::core::convert::From<EIP4788_WINDOWCall> for UnderlyingRustTuple<'_> {
				fn from(value: EIP4788_WINDOWCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for EIP4788_WINDOWCall {
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
			impl ::core::convert::From<EIP4788_WINDOWReturn> for UnderlyingRustTuple<'_> {
				fn from(value: EIP4788_WINDOWReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for EIP4788_WINDOWReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for EIP4788_WINDOWCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "EIP4788_WINDOW()";
			const SELECTOR: [u8; 4] = [202u8, 198u8, 45u8, 217u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: EIP4788_WINDOWReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: EIP4788_WINDOWReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `ETH2_GENESIS_TIMESTAMP()` and selector `0x9f329d0b`.
	```solidity
	function ETH2_GENESIS_TIMESTAMP() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct ETH2_GENESIS_TIMESTAMPCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`ETH2_GENESIS_TIMESTAMP()`](ETH2_GENESIS_TIMESTAMPCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct ETH2_GENESIS_TIMESTAMPReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
			impl ::core::convert::From<ETH2_GENESIS_TIMESTAMPCall> for UnderlyingRustTuple<'_> {
				fn from(value: ETH2_GENESIS_TIMESTAMPCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for ETH2_GENESIS_TIMESTAMPCall {
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
			impl ::core::convert::From<ETH2_GENESIS_TIMESTAMPReturn> for UnderlyingRustTuple<'_> {
				fn from(value: ETH2_GENESIS_TIMESTAMPReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for ETH2_GENESIS_TIMESTAMPReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for ETH2_GENESIS_TIMESTAMPCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "ETH2_GENESIS_TIMESTAMP()";
			const SELECTOR: [u8; 4] = [159u8, 50u8, 157u8, 11u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: ETH2_GENESIS_TIMESTAMPReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: ETH2_GENESIS_TIMESTAMPReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `JUSTIFICATION_DELAY()` and selector `0x74377029`.
	```solidity
	function JUSTIFICATION_DELAY() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct JUSTIFICATION_DELAYCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`JUSTIFICATION_DELAY()`](JUSTIFICATION_DELAYCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct JUSTIFICATION_DELAYReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
			impl ::core::convert::From<JUSTIFICATION_DELAYCall> for UnderlyingRustTuple<'_> {
				fn from(value: JUSTIFICATION_DELAYCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for JUSTIFICATION_DELAYCall {
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
			impl ::core::convert::From<JUSTIFICATION_DELAYReturn> for UnderlyingRustTuple<'_> {
				fn from(value: JUSTIFICATION_DELAYReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for JUSTIFICATION_DELAYReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for JUSTIFICATION_DELAYCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "JUSTIFICATION_DELAY()";
			const SELECTOR: [u8; 4] = [116u8, 55u8, 112u8, 41u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: JUSTIFICATION_DELAYReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: JUSTIFICATION_DELAYReturn = r.into();
					r._0
				})
			}
		}
	};
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
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: SLASH_AMOUNT_WEIReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: SLASH_AMOUNT_WEIReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `SLOT_TIME()` and selector `0x2a04ff85`.
	```solidity
	function SLOT_TIME() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SLOT_TIMECall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`SLOT_TIME()`](SLOT_TIMECall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SLOT_TIMEReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
			impl ::core::convert::From<SLOT_TIMECall> for UnderlyingRustTuple<'_> {
				fn from(value: SLOT_TIMECall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLOT_TIMECall {
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
			impl ::core::convert::From<SLOT_TIMEReturn> for UnderlyingRustTuple<'_> {
				fn from(value: SLOT_TIMEReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLOT_TIMEReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for SLOT_TIMECall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "SLOT_TIME()";
			const SELECTOR: [u8; 4] = [42u8, 4u8, 255u8, 133u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: SLOT_TIMEReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: SLOT_TIMEReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `_decodeBlockHeaderRLP(bytes)` and selector `0x56b4a92a`.
	```solidity
	function _decodeBlockHeaderRLP(bytes memory headerRLP) external pure returns (PreconfStructs.BlockHeaderData memory blockHeader);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _decodeBlockHeaderRLPCall {
		#[allow(missing_docs)]
		pub headerRLP: alloy::sol_types::private::Bytes,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`_decodeBlockHeaderRLP(bytes)`](_decodeBlockHeaderRLPCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _decodeBlockHeaderRLPReturn {
		#[allow(missing_docs)]
		pub blockHeader: <PreconfStructs::BlockHeaderData as alloy::sol_types::SolType>::RustType,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
			impl ::core::convert::From<_decodeBlockHeaderRLPCall> for UnderlyingRustTuple<'_> {
				fn from(value: _decodeBlockHeaderRLPCall) -> Self {
					(value.headerRLP,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _decodeBlockHeaderRLPCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { headerRLP: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (PreconfStructs::BlockHeaderData,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<PreconfStructs::BlockHeaderData as alloy::sol_types::SolType>::RustType,);
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
			impl ::core::convert::From<_decodeBlockHeaderRLPReturn> for UnderlyingRustTuple<'_> {
				fn from(value: _decodeBlockHeaderRLPReturn) -> Self {
					(value.blockHeader,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _decodeBlockHeaderRLPReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { blockHeader: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for _decodeBlockHeaderRLPCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = <PreconfStructs::BlockHeaderData as alloy::sol_types::SolType>::RustType;
			type ReturnTuple<'a> = (PreconfStructs::BlockHeaderData,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "_decodeBlockHeaderRLP(bytes)";
			const SELECTOR: [u8; 4] = [86u8, 180u8, 169u8, 42u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.headerRLP),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<PreconfStructs::BlockHeaderData as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: _decodeBlockHeaderRLPReturn = r.into();
					r.blockHeader
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: _decodeBlockHeaderRLPReturn = r.into();
					r.blockHeader
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `_getCurrentSlot()` and selector `0x189cc9d0`.
	```solidity
	function _getCurrentSlot() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _getCurrentSlotCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`_getCurrentSlot()`](_getCurrentSlotCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _getCurrentSlotReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
			impl ::core::convert::From<_getCurrentSlotCall> for UnderlyingRustTuple<'_> {
				fn from(value: _getCurrentSlotCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _getCurrentSlotCall {
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
			impl ::core::convert::From<_getCurrentSlotReturn> for UnderlyingRustTuple<'_> {
				fn from(value: _getCurrentSlotReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _getCurrentSlotReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for _getCurrentSlotCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "_getCurrentSlot()";
			const SELECTOR: [u8; 4] = [24u8, 156u8, 201u8, 208u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: _getCurrentSlotReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: _getCurrentSlotReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `_getSlotFromTimestamp(uint256)` and selector `0xabeeb366`.
	```solidity
	function _getSlotFromTimestamp(uint256 _timestamp) external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _getSlotFromTimestampCall {
		#[allow(missing_docs)]
		pub _timestamp: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`_getSlotFromTimestamp(uint256)`](_getSlotFromTimestampCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _getSlotFromTimestampReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
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
			impl ::core::convert::From<_getSlotFromTimestampCall> for UnderlyingRustTuple<'_> {
				fn from(value: _getSlotFromTimestampCall) -> Self {
					(value._timestamp,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _getSlotFromTimestampCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _timestamp: tuple.0 }
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
			impl ::core::convert::From<_getSlotFromTimestampReturn> for UnderlyingRustTuple<'_> {
				fn from(value: _getSlotFromTimestampReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _getSlotFromTimestampReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for _getSlotFromTimestampCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "_getSlotFromTimestamp(uint256)";
			const SELECTOR: [u8; 4] = [171u8, 238u8, 179u8, 102u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self._timestamp),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: _getSlotFromTimestampReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: _getSlotFromTimestampReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `_getTimestampFromSlot(uint256)` and selector `0xf5beea8c`.
	```solidity
	function _getTimestampFromSlot(uint256 _slot) external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _getTimestampFromSlotCall {
		#[allow(missing_docs)]
		pub _slot: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`_getTimestampFromSlot(uint256)`](_getTimestampFromSlotCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _getTimestampFromSlotReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
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
			impl ::core::convert::From<_getTimestampFromSlotCall> for UnderlyingRustTuple<'_> {
				fn from(value: _getTimestampFromSlotCall) -> Self {
					(value._slot,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _getTimestampFromSlotCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _slot: tuple.0 }
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
			impl ::core::convert::From<_getTimestampFromSlotReturn> for UnderlyingRustTuple<'_> {
				fn from(value: _getTimestampFromSlotReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _getTimestampFromSlotReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for _getTimestampFromSlotCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "_getTimestampFromSlot(uint256)";
			const SELECTOR: [u8; 4] = [245u8, 190u8, 234u8, 140u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self._slot),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: _getTimestampFromSlotReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: _getTimestampFromSlotReturn = r.into();
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
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
					(value.delegation, value.commitment, value.committer, value.evidence, value.challenger)
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
					Self { slashAmountWei: tuple.0 }
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
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<ISlasher::Delegation as alloy_sol_types::SolType>::tokenize(&self.delegation),
					<ISlasher::Commitment as alloy_sol_types::SolType>::tokenize(&self.commitment),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.committer),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.evidence),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.challenger),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: slashReturn = r.into();
					r.slashAmountWei
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: slashReturn = r.into();
					r.slashAmountWei
				})
			}
		}
	};
	///Container for all the [`StateLockSlasher`](self) function calls.
	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum StateLockSlasherCalls {
		#[allow(missing_docs)]
		BEACON_ROOTS_CONTRACT(BEACON_ROOTS_CONTRACTCall),
		#[allow(missing_docs)]
		BLOCKHASH_EVM_LOOKBACK(BLOCKHASH_EVM_LOOKBACKCall),
		#[allow(missing_docs)]
		EIP4788_WINDOW(EIP4788_WINDOWCall),
		#[allow(missing_docs)]
		ETH2_GENESIS_TIMESTAMP(ETH2_GENESIS_TIMESTAMPCall),
		#[allow(missing_docs)]
		JUSTIFICATION_DELAY(JUSTIFICATION_DELAYCall),
		#[allow(missing_docs)]
		SLASH_AMOUNT_WEI(SLASH_AMOUNT_WEICall),
		#[allow(missing_docs)]
		SLOT_TIME(SLOT_TIMECall),
		#[allow(missing_docs)]
		_decodeBlockHeaderRLP(_decodeBlockHeaderRLPCall),
		#[allow(missing_docs)]
		_getCurrentSlot(_getCurrentSlotCall),
		#[allow(missing_docs)]
		_getSlotFromTimestamp(_getSlotFromTimestampCall),
		#[allow(missing_docs)]
		_getTimestampFromSlot(_getTimestampFromSlotCall),
		#[allow(missing_docs)]
		slash(slashCall),
	}
	impl StateLockSlasherCalls {
		/// All the selectors of this enum.
		///
		/// Note that the selectors might not be in the same order as the variants.
		/// No guarantees are made about the order of the selectors.
		///
		/// Prefer using `SolInterface` methods instead.
		pub const SELECTORS: &'static [[u8; 4usize]] = &[
			[24u8, 156u8, 201u8, 208u8],
			[42u8, 4u8, 255u8, 133u8],
			[73u8, 104u8, 246u8, 197u8],
			[86u8, 180u8, 169u8, 42u8],
			[116u8, 55u8, 112u8, 41u8],
			[134u8, 251u8, 97u8, 237u8],
			[159u8, 50u8, 157u8, 11u8],
			[171u8, 238u8, 179u8, 102u8],
			[194u8, 57u8, 239u8, 66u8],
			[202u8, 198u8, 45u8, 217u8],
			[244u8, 94u8, 129u8, 24u8],
			[245u8, 190u8, 234u8, 140u8],
		];
		/// The names of the variants in the same order as `SELECTORS`.
		pub const VARIANT_NAMES: &'static [&'static str] = &[
			::core::stringify!(_getCurrentSlot),
			::core::stringify!(SLOT_TIME),
			::core::stringify!(SLASH_AMOUNT_WEI),
			::core::stringify!(_decodeBlockHeaderRLP),
			::core::stringify!(JUSTIFICATION_DELAY),
			::core::stringify!(BEACON_ROOTS_CONTRACT),
			::core::stringify!(ETH2_GENESIS_TIMESTAMP),
			::core::stringify!(_getSlotFromTimestamp),
			::core::stringify!(slash),
			::core::stringify!(EIP4788_WINDOW),
			::core::stringify!(BLOCKHASH_EVM_LOOKBACK),
			::core::stringify!(_getTimestampFromSlot),
		];
		/// The signatures in the same order as `SELECTORS`.
		pub const SIGNATURES: &'static [&'static str] = &[
			<_getCurrentSlotCall as alloy_sol_types::SolCall>::SIGNATURE,
			<SLOT_TIMECall as alloy_sol_types::SolCall>::SIGNATURE,
			<SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::SIGNATURE,
			<_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::SIGNATURE,
			<JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::SIGNATURE,
			<BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::SIGNATURE,
			<ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::SIGNATURE,
			<_getSlotFromTimestampCall as alloy_sol_types::SolCall>::SIGNATURE,
			<slashCall as alloy_sol_types::SolCall>::SIGNATURE,
			<EIP4788_WINDOWCall as alloy_sol_types::SolCall>::SIGNATURE,
			<BLOCKHASH_EVM_LOOKBACKCall as alloy_sol_types::SolCall>::SIGNATURE,
			<_getTimestampFromSlotCall as alloy_sol_types::SolCall>::SIGNATURE,
		];
		/// Returns the signature for the given selector, if known.
		#[inline]
		pub fn signature_by_selector(selector: [u8; 4usize]) -> ::core::option::Option<&'static str> {
			match Self::SELECTORS.binary_search(&selector) {
				::core::result::Result::Ok(idx) => ::core::option::Option::Some(Self::SIGNATURES[idx]),
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
	impl alloy_sol_types::SolInterface for StateLockSlasherCalls {
		const NAME: &'static str = "StateLockSlasherCalls";
		const MIN_DATA_LENGTH: usize = 0usize;
		const COUNT: usize = 12usize;
		#[inline]
		fn selector(&self) -> [u8; 4] {
			match self {
				Self::BEACON_ROOTS_CONTRACT(_) => <BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::BLOCKHASH_EVM_LOOKBACK(_) => <BLOCKHASH_EVM_LOOKBACKCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::EIP4788_WINDOW(_) => <EIP4788_WINDOWCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::ETH2_GENESIS_TIMESTAMP(_) => <ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::JUSTIFICATION_DELAY(_) => <JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::SLASH_AMOUNT_WEI(_) => <SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::SELECTOR,
				Self::SLOT_TIME(_) => <SLOT_TIMECall as alloy_sol_types::SolCall>::SELECTOR,
				Self::_decodeBlockHeaderRLP(_) => <_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::_getCurrentSlot(_) => <_getCurrentSlotCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::_getSlotFromTimestamp(_) => <_getSlotFromTimestampCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::_getTimestampFromSlot(_) => <_getTimestampFromSlotCall as alloy_sol_types::SolCall>::SELECTOR,
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
			static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls>] = &[
				{
					fn _getCurrentSlot(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<_getCurrentSlotCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::_getCurrentSlot)
					}
					_getCurrentSlot
				},
				{
					fn SLOT_TIME(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<SLOT_TIMECall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::SLOT_TIME)
					}
					SLOT_TIME
				},
				{
					fn SLASH_AMOUNT_WEI(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::SLASH_AMOUNT_WEI)
					}
					SLASH_AMOUNT_WEI
				},
				{
					fn _decodeBlockHeaderRLP(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::_decodeBlockHeaderRLP)
					}
					_decodeBlockHeaderRLP
				},
				{
					fn JUSTIFICATION_DELAY(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::JUSTIFICATION_DELAY)
					}
					JUSTIFICATION_DELAY
				},
				{
					fn BEACON_ROOTS_CONTRACT(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::BEACON_ROOTS_CONTRACT)
					}
					BEACON_ROOTS_CONTRACT
				},
				{
					fn ETH2_GENESIS_TIMESTAMP(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::ETH2_GENESIS_TIMESTAMP)
					}
					ETH2_GENESIS_TIMESTAMP
				},
				{
					fn _getSlotFromTimestamp(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<_getSlotFromTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::_getSlotFromTimestamp)
					}
					_getSlotFromTimestamp
				},
				{
					fn slash(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<slashCall as alloy_sol_types::SolCall>::abi_decode_raw(data).map(StateLockSlasherCalls::slash)
					}
					slash
				},
				{
					fn EIP4788_WINDOW(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<EIP4788_WINDOWCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::EIP4788_WINDOW)
					}
					EIP4788_WINDOW
				},
				{
					fn BLOCKHASH_EVM_LOOKBACK(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<BLOCKHASH_EVM_LOOKBACKCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::BLOCKHASH_EVM_LOOKBACK)
					}
					BLOCKHASH_EVM_LOOKBACK
				},
				{
					fn _getTimestampFromSlot(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<_getTimestampFromSlotCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(StateLockSlasherCalls::_getTimestampFromSlot)
					}
					_getTimestampFromSlot
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
		fn abi_decode_raw_validate(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
			static DECODE_VALIDATE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls>] = &[
				{
					fn _getCurrentSlot(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<_getCurrentSlotCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::_getCurrentSlot)
					}
					_getCurrentSlot
				},
				{
					fn SLOT_TIME(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<SLOT_TIMECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::SLOT_TIME)
					}
					SLOT_TIME
				},
				{
					fn SLASH_AMOUNT_WEI(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::SLASH_AMOUNT_WEI)
					}
					SLASH_AMOUNT_WEI
				},
				{
					fn _decodeBlockHeaderRLP(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::_decodeBlockHeaderRLP)
					}
					_decodeBlockHeaderRLP
				},
				{
					fn JUSTIFICATION_DELAY(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::JUSTIFICATION_DELAY)
					}
					JUSTIFICATION_DELAY
				},
				{
					fn BEACON_ROOTS_CONTRACT(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::BEACON_ROOTS_CONTRACT)
					}
					BEACON_ROOTS_CONTRACT
				},
				{
					fn ETH2_GENESIS_TIMESTAMP(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::ETH2_GENESIS_TIMESTAMP)
					}
					ETH2_GENESIS_TIMESTAMP
				},
				{
					fn _getSlotFromTimestamp(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<_getSlotFromTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::_getSlotFromTimestamp)
					}
					_getSlotFromTimestamp
				},
				{
					fn slash(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<slashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::slash)
					}
					slash
				},
				{
					fn EIP4788_WINDOW(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<EIP4788_WINDOWCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::EIP4788_WINDOW)
					}
					EIP4788_WINDOW
				},
				{
					fn BLOCKHASH_EVM_LOOKBACK(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<BLOCKHASH_EVM_LOOKBACKCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::BLOCKHASH_EVM_LOOKBACK)
					}
					BLOCKHASH_EVM_LOOKBACK
				},
				{
					fn _getTimestampFromSlot(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherCalls> {
						<_getTimestampFromSlotCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(StateLockSlasherCalls::_getTimestampFromSlot)
					}
					_getTimestampFromSlot
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
				Self::BEACON_ROOTS_CONTRACT(inner) => {
					<BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::BLOCKHASH_EVM_LOOKBACK(inner) => {
					<BLOCKHASH_EVM_LOOKBACKCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::EIP4788_WINDOW(inner) => {
					<EIP4788_WINDOWCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::ETH2_GENESIS_TIMESTAMP(inner) => {
					<ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::JUSTIFICATION_DELAY(inner) => {
					<JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::SLASH_AMOUNT_WEI(inner) => {
					<SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::SLOT_TIME(inner) => <SLOT_TIMECall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::_decodeBlockHeaderRLP(inner) => {
					<_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::_getCurrentSlot(inner) => {
					<_getCurrentSlotCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::_getSlotFromTimestamp(inner) => {
					<_getSlotFromTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::_getTimestampFromSlot(inner) => {
					<_getTimestampFromSlotCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::slash(inner) => <slashCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
			}
		}
		#[inline]
		fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
			match self {
				Self::BEACON_ROOTS_CONTRACT(inner) => {
					<BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::BLOCKHASH_EVM_LOOKBACK(inner) => {
					<BLOCKHASH_EVM_LOOKBACKCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::EIP4788_WINDOW(inner) => {
					<EIP4788_WINDOWCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::ETH2_GENESIS_TIMESTAMP(inner) => {
					<ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::JUSTIFICATION_DELAY(inner) => {
					<JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::SLASH_AMOUNT_WEI(inner) => {
					<SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::SLOT_TIME(inner) => <SLOT_TIMECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::_decodeBlockHeaderRLP(inner) => {
					<_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::_getCurrentSlot(inner) => {
					<_getCurrentSlotCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::_getSlotFromTimestamp(inner) => {
					<_getSlotFromTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::_getTimestampFromSlot(inner) => {
					<_getTimestampFromSlotCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::slash(inner) => <slashCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
			}
		}
	}
	///Container for all the [`StateLockSlasher`](self) custom errors.
	#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
	pub enum StateLockSlasherErrors {
		#[allow(missing_docs)]
		BeaconRootNotFound(BeaconRootNotFound),
		#[allow(missing_docs)]
		BlockIsNotFinalized(BlockIsNotFinalized),
		#[allow(missing_docs)]
		BlockIsTooOld(BlockIsTooOld),
		#[allow(missing_docs)]
		ChallengeAlreadyExists(ChallengeAlreadyExists),
		#[allow(missing_docs)]
		ChallengeDoesNotExist(ChallengeDoesNotExist),
		#[allow(missing_docs)]
		DelegationExpired(DelegationExpired),
		#[allow(missing_docs)]
		ECDSAInvalidSignature(ECDSAInvalidSignature),
		#[allow(missing_docs)]
		ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
		#[allow(missing_docs)]
		ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
		#[allow(missing_docs)]
		EthTransferFailed(EthTransferFailed),
		#[allow(missing_docs)]
		FraudProofWindowActive(FraudProofWindowActive),
		#[allow(missing_docs)]
		IncorrectChallengeBond(IncorrectChallengeBond),
		#[allow(missing_docs)]
		InvalidBlockHash(InvalidBlockHash),
		#[allow(missing_docs)]
		InvalidBlockNumber(InvalidBlockNumber),
		#[allow(missing_docs)]
		InvalidFieldCount(InvalidFieldCount),
		#[allow(missing_docs)]
		InvalidParentBlockHash(InvalidParentBlockHash),
		#[allow(missing_docs)]
		InvalidSignatureLength(InvalidSignatureLength),
		#[allow(missing_docs)]
		NoSignature(NoSignature),
		#[allow(missing_docs)]
		NotURC(NotURC),
		#[allow(missing_docs)]
		TransactionExcluded(TransactionExcluded),
		#[allow(missing_docs)]
		UnexpectedSigner(UnexpectedSigner),
		#[allow(missing_docs)]
		UnsupportedTxType(UnsupportedTxType),
		#[allow(missing_docs)]
		WrongChallengerAddress(WrongChallengerAddress),
		#[allow(missing_docs)]
		WrongTransactionHashProof(WrongTransactionHashProof),
	}
	impl StateLockSlasherErrors {
		/// All the selectors of this enum.
		///
		/// Note that the selectors might not be in the same order as the variants.
		/// No guarantees are made about the order of the selectors.
		///
		/// Prefer using `SolInterface` methods instead.
		pub const SELECTORS: &'static [[u8; 4usize]] = &[
			[3u8, 18u8, 110u8, 48u8],
			[9u8, 76u8, 236u8, 95u8],
			[31u8, 48u8, 76u8, 193u8],
			[48u8, 211u8, 186u8, 7u8],
			[51u8, 115u8, 173u8, 228u8],
			[62u8, 6u8, 140u8, 182u8],
			[65u8, 244u8, 217u8, 22u8],
			[68u8, 80u8, 67u8, 87u8],
			[75u8, 230u8, 50u8, 27u8],
			[78u8, 71u8, 132u8, 108u8],
			[109u8, 150u8, 63u8, 136u8],
			[117u8, 70u8, 232u8, 193u8],
			[136u8, 197u8, 103u8, 30u8],
			[155u8, 140u8, 193u8, 70u8],
			[170u8, 170u8, 145u8, 65u8],
			[182u8, 20u8, 75u8, 255u8],
			[194u8, 135u8, 26u8, 55u8],
			[199u8, 203u8, 30u8, 174u8],
			[212u8, 102u8, 189u8, 21u8],
			[215u8, 139u8, 206u8, 12u8],
			[223u8, 135u8, 181u8, 70u8],
			[233u8, 105u8, 123u8, 104u8],
			[246u8, 69u8, 238u8, 223u8],
			[252u8, 230u8, 152u8, 247u8],
		];
		/// The names of the variants in the same order as `SELECTORS`.
		pub const VARIANT_NAMES: &'static [&'static str] = &[
			::core::stringify!(ChallengeDoesNotExist),
			::core::stringify!(TransactionExcluded),
			::core::stringify!(FraudProofWindowActive),
			::core::stringify!(DelegationExpired),
			::core::stringify!(BlockIsTooOld),
			::core::stringify!(InvalidBlockHash),
			::core::stringify!(InvalidParentBlockHash),
			::core::stringify!(ChallengeAlreadyExists),
			::core::stringify!(InvalidSignatureLength),
			::core::stringify!(InvalidBlockNumber),
			::core::stringify!(EthTransferFailed),
			::core::stringify!(WrongChallengerAddress),
			::core::stringify!(WrongTransactionHashProof),
			::core::stringify!(IncorrectChallengeBond),
			::core::stringify!(UnexpectedSigner),
			::core::stringify!(BlockIsNotFinalized),
			::core::stringify!(InvalidFieldCount),
			::core::stringify!(NotURC),
			::core::stringify!(NoSignature),
			::core::stringify!(ECDSAInvalidSignatureS),
			::core::stringify!(UnsupportedTxType),
			::core::stringify!(BeaconRootNotFound),
			::core::stringify!(ECDSAInvalidSignature),
			::core::stringify!(ECDSAInvalidSignatureLength),
		];
		/// The signatures in the same order as `SELECTORS`.
		pub const SIGNATURES: &'static [&'static str] = &[
			<ChallengeDoesNotExist as alloy_sol_types::SolError>::SIGNATURE,
			<TransactionExcluded as alloy_sol_types::SolError>::SIGNATURE,
			<FraudProofWindowActive as alloy_sol_types::SolError>::SIGNATURE,
			<DelegationExpired as alloy_sol_types::SolError>::SIGNATURE,
			<BlockIsTooOld as alloy_sol_types::SolError>::SIGNATURE,
			<InvalidBlockHash as alloy_sol_types::SolError>::SIGNATURE,
			<InvalidParentBlockHash as alloy_sol_types::SolError>::SIGNATURE,
			<ChallengeAlreadyExists as alloy_sol_types::SolError>::SIGNATURE,
			<InvalidSignatureLength as alloy_sol_types::SolError>::SIGNATURE,
			<InvalidBlockNumber as alloy_sol_types::SolError>::SIGNATURE,
			<EthTransferFailed as alloy_sol_types::SolError>::SIGNATURE,
			<WrongChallengerAddress as alloy_sol_types::SolError>::SIGNATURE,
			<WrongTransactionHashProof as alloy_sol_types::SolError>::SIGNATURE,
			<IncorrectChallengeBond as alloy_sol_types::SolError>::SIGNATURE,
			<UnexpectedSigner as alloy_sol_types::SolError>::SIGNATURE,
			<BlockIsNotFinalized as alloy_sol_types::SolError>::SIGNATURE,
			<InvalidFieldCount as alloy_sol_types::SolError>::SIGNATURE,
			<NotURC as alloy_sol_types::SolError>::SIGNATURE,
			<NoSignature as alloy_sol_types::SolError>::SIGNATURE,
			<ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
			<UnsupportedTxType as alloy_sol_types::SolError>::SIGNATURE,
			<BeaconRootNotFound as alloy_sol_types::SolError>::SIGNATURE,
			<ECDSAInvalidSignature as alloy_sol_types::SolError>::SIGNATURE,
			<ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SIGNATURE,
		];
		/// Returns the signature for the given selector, if known.
		#[inline]
		pub fn signature_by_selector(selector: [u8; 4usize]) -> ::core::option::Option<&'static str> {
			match Self::SELECTORS.binary_search(&selector) {
				::core::result::Result::Ok(idx) => ::core::option::Option::Some(Self::SIGNATURES[idx]),
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
	impl alloy_sol_types::SolInterface for StateLockSlasherErrors {
		const NAME: &'static str = "StateLockSlasherErrors";
		const MIN_DATA_LENGTH: usize = 0usize;
		const COUNT: usize = 24usize;
		#[inline]
		fn selector(&self) -> [u8; 4] {
			match self {
				Self::BeaconRootNotFound(_) => <BeaconRootNotFound as alloy_sol_types::SolError>::SELECTOR,
				Self::BlockIsNotFinalized(_) => <BlockIsNotFinalized as alloy_sol_types::SolError>::SELECTOR,
				Self::BlockIsTooOld(_) => <BlockIsTooOld as alloy_sol_types::SolError>::SELECTOR,
				Self::ChallengeAlreadyExists(_) => <ChallengeAlreadyExists as alloy_sol_types::SolError>::SELECTOR,
				Self::ChallengeDoesNotExist(_) => <ChallengeDoesNotExist as alloy_sol_types::SolError>::SELECTOR,
				Self::DelegationExpired(_) => <DelegationExpired as alloy_sol_types::SolError>::SELECTOR,
				Self::ECDSAInvalidSignature(_) => <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR,
				Self::ECDSAInvalidSignatureLength(_) => {
					<ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
				}
				Self::ECDSAInvalidSignatureS(_) => <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR,
				Self::EthTransferFailed(_) => <EthTransferFailed as alloy_sol_types::SolError>::SELECTOR,
				Self::FraudProofWindowActive(_) => <FraudProofWindowActive as alloy_sol_types::SolError>::SELECTOR,
				Self::IncorrectChallengeBond(_) => <IncorrectChallengeBond as alloy_sol_types::SolError>::SELECTOR,
				Self::InvalidBlockHash(_) => <InvalidBlockHash as alloy_sol_types::SolError>::SELECTOR,
				Self::InvalidBlockNumber(_) => <InvalidBlockNumber as alloy_sol_types::SolError>::SELECTOR,
				Self::InvalidFieldCount(_) => <InvalidFieldCount as alloy_sol_types::SolError>::SELECTOR,
				Self::InvalidParentBlockHash(_) => <InvalidParentBlockHash as alloy_sol_types::SolError>::SELECTOR,
				Self::InvalidSignatureLength(_) => <InvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR,
				Self::NoSignature(_) => <NoSignature as alloy_sol_types::SolError>::SELECTOR,
				Self::NotURC(_) => <NotURC as alloy_sol_types::SolError>::SELECTOR,
				Self::TransactionExcluded(_) => <TransactionExcluded as alloy_sol_types::SolError>::SELECTOR,
				Self::UnexpectedSigner(_) => <UnexpectedSigner as alloy_sol_types::SolError>::SELECTOR,
				Self::UnsupportedTxType(_) => <UnsupportedTxType as alloy_sol_types::SolError>::SELECTOR,
				Self::WrongChallengerAddress(_) => <WrongChallengerAddress as alloy_sol_types::SolError>::SELECTOR,
				Self::WrongTransactionHashProof(_) => {
					<WrongTransactionHashProof as alloy_sol_types::SolError>::SELECTOR
				}
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
			static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors>] = &[
				{
					fn ChallengeDoesNotExist(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::ChallengeDoesNotExist)
					}
					ChallengeDoesNotExist
				},
				{
					fn TransactionExcluded(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<TransactionExcluded as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::TransactionExcluded)
					}
					TransactionExcluded
				},
				{
					fn FraudProofWindowActive(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<FraudProofWindowActive as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::FraudProofWindowActive)
					}
					FraudProofWindowActive
				},
				{
					fn DelegationExpired(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<DelegationExpired as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::DelegationExpired)
					}
					DelegationExpired
				},
				{
					fn BlockIsTooOld(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<BlockIsTooOld as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::BlockIsTooOld)
					}
					BlockIsTooOld
				},
				{
					fn InvalidBlockHash(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidBlockHash as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::InvalidBlockHash)
					}
					InvalidBlockHash
				},
				{
					fn InvalidParentBlockHash(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidParentBlockHash as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::InvalidParentBlockHash)
					}
					InvalidParentBlockHash
				},
				{
					fn ChallengeAlreadyExists(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::ChallengeAlreadyExists)
					}
					ChallengeAlreadyExists
				},
				{
					fn InvalidSignatureLength(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::InvalidSignatureLength)
					}
					InvalidSignatureLength
				},
				{
					fn InvalidBlockNumber(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::InvalidBlockNumber)
					}
					InvalidBlockNumber
				},
				{
					fn EthTransferFailed(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<EthTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::EthTransferFailed)
					}
					EthTransferFailed
				},
				{
					fn WrongChallengerAddress(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<WrongChallengerAddress as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::WrongChallengerAddress)
					}
					WrongChallengerAddress
				},
				{
					fn WrongTransactionHashProof(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<WrongTransactionHashProof as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::WrongTransactionHashProof)
					}
					WrongTransactionHashProof
				},
				{
					fn IncorrectChallengeBond(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<IncorrectChallengeBond as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::IncorrectChallengeBond)
					}
					IncorrectChallengeBond
				},
				{
					fn UnexpectedSigner(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<UnexpectedSigner as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::UnexpectedSigner)
					}
					UnexpectedSigner
				},
				{
					fn BlockIsNotFinalized(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<BlockIsNotFinalized as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::BlockIsNotFinalized)
					}
					BlockIsNotFinalized
				},
				{
					fn InvalidFieldCount(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidFieldCount as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::InvalidFieldCount)
					}
					InvalidFieldCount
				},
				{
					fn NotURC(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<NotURC as alloy_sol_types::SolError>::abi_decode_raw(data).map(StateLockSlasherErrors::NotURC)
					}
					NotURC
				},
				{
					fn NoSignature(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<NoSignature as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::NoSignature)
					}
					NoSignature
				},
				{
					fn ECDSAInvalidSignatureS(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::ECDSAInvalidSignatureS)
					}
					ECDSAInvalidSignatureS
				},
				{
					fn UnsupportedTxType(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<UnsupportedTxType as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::UnsupportedTxType)
					}
					UnsupportedTxType
				},
				{
					fn BeaconRootNotFound(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<BeaconRootNotFound as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::BeaconRootNotFound)
					}
					BeaconRootNotFound
				},
				{
					fn ECDSAInvalidSignature(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::ECDSAInvalidSignature)
					}
					ECDSAInvalidSignature
				},
				{
					fn ECDSAInvalidSignatureLength(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(StateLockSlasherErrors::ECDSAInvalidSignatureLength)
					}
					ECDSAInvalidSignatureLength
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
		fn abi_decode_raw_validate(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
			static DECODE_VALIDATE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors>] = &[
				{
					fn ChallengeDoesNotExist(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::ChallengeDoesNotExist)
					}
					ChallengeDoesNotExist
				},
				{
					fn TransactionExcluded(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<TransactionExcluded as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::TransactionExcluded)
					}
					TransactionExcluded
				},
				{
					fn FraudProofWindowActive(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<FraudProofWindowActive as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::FraudProofWindowActive)
					}
					FraudProofWindowActive
				},
				{
					fn DelegationExpired(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<DelegationExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::DelegationExpired)
					}
					DelegationExpired
				},
				{
					fn BlockIsTooOld(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<BlockIsTooOld as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::BlockIsTooOld)
					}
					BlockIsTooOld
				},
				{
					fn InvalidBlockHash(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidBlockHash as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::InvalidBlockHash)
					}
					InvalidBlockHash
				},
				{
					fn InvalidParentBlockHash(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidParentBlockHash as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::InvalidParentBlockHash)
					}
					InvalidParentBlockHash
				},
				{
					fn ChallengeAlreadyExists(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::ChallengeAlreadyExists)
					}
					ChallengeAlreadyExists
				},
				{
					fn InvalidSignatureLength(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::InvalidSignatureLength)
					}
					InvalidSignatureLength
				},
				{
					fn InvalidBlockNumber(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidBlockNumber as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::InvalidBlockNumber)
					}
					InvalidBlockNumber
				},
				{
					fn EthTransferFailed(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<EthTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::EthTransferFailed)
					}
					EthTransferFailed
				},
				{
					fn WrongChallengerAddress(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<WrongChallengerAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::WrongChallengerAddress)
					}
					WrongChallengerAddress
				},
				{
					fn WrongTransactionHashProof(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<WrongTransactionHashProof as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::WrongTransactionHashProof)
					}
					WrongTransactionHashProof
				},
				{
					fn IncorrectChallengeBond(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<IncorrectChallengeBond as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::IncorrectChallengeBond)
					}
					IncorrectChallengeBond
				},
				{
					fn UnexpectedSigner(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<UnexpectedSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::UnexpectedSigner)
					}
					UnexpectedSigner
				},
				{
					fn BlockIsNotFinalized(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<BlockIsNotFinalized as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::BlockIsNotFinalized)
					}
					BlockIsNotFinalized
				},
				{
					fn InvalidFieldCount(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<InvalidFieldCount as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::InvalidFieldCount)
					}
					InvalidFieldCount
				},
				{
					fn NotURC(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<NotURC as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::NotURC)
					}
					NotURC
				},
				{
					fn NoSignature(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<NoSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::NoSignature)
					}
					NoSignature
				},
				{
					fn ECDSAInvalidSignatureS(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::ECDSAInvalidSignatureS)
					}
					ECDSAInvalidSignatureS
				},
				{
					fn UnsupportedTxType(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<UnsupportedTxType as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::UnsupportedTxType)
					}
					UnsupportedTxType
				},
				{
					fn BeaconRootNotFound(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<BeaconRootNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::BeaconRootNotFound)
					}
					BeaconRootNotFound
				},
				{
					fn ECDSAInvalidSignature(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::ECDSAInvalidSignature)
					}
					ECDSAInvalidSignature
				},
				{
					fn ECDSAInvalidSignatureLength(data: &[u8]) -> alloy_sol_types::Result<StateLockSlasherErrors> {
						<ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(StateLockSlasherErrors::ECDSAInvalidSignatureLength)
					}
					ECDSAInvalidSignatureLength
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
				Self::BeaconRootNotFound(inner) => {
					<BeaconRootNotFound as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::BlockIsNotFinalized(inner) => {
					<BlockIsNotFinalized as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::BlockIsTooOld(inner) => <BlockIsTooOld as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::ChallengeAlreadyExists(inner) => {
					<ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::ChallengeDoesNotExist(inner) => {
					<ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::DelegationExpired(inner) => {
					<DelegationExpired as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::ECDSAInvalidSignature(inner) => {
					<ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::ECDSAInvalidSignatureLength(inner) => {
					<ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::ECDSAInvalidSignatureS(inner) => {
					<ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::EthTransferFailed(inner) => {
					<EthTransferFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::FraudProofWindowActive(inner) => {
					<FraudProofWindowActive as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::IncorrectChallengeBond(inner) => {
					<IncorrectChallengeBond as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::InvalidBlockHash(inner) => {
					<InvalidBlockHash as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::InvalidBlockNumber(inner) => {
					<InvalidBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::InvalidFieldCount(inner) => {
					<InvalidFieldCount as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::InvalidParentBlockHash(inner) => {
					<InvalidParentBlockHash as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::InvalidSignatureLength(inner) => {
					<InvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::NoSignature(inner) => <NoSignature as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::NotURC(inner) => <NotURC as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::TransactionExcluded(inner) => {
					<TransactionExcluded as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::UnexpectedSigner(inner) => {
					<UnexpectedSigner as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::UnsupportedTxType(inner) => {
					<UnsupportedTxType as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::WrongChallengerAddress(inner) => {
					<WrongChallengerAddress as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::WrongTransactionHashProof(inner) => {
					<WrongTransactionHashProof as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
			}
		}
		#[inline]
		fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
			match self {
				Self::BeaconRootNotFound(inner) => {
					<BeaconRootNotFound as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::BlockIsNotFinalized(inner) => {
					<BlockIsNotFinalized as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::BlockIsTooOld(inner) => <BlockIsTooOld as alloy_sol_types::SolError>::abi_encode_raw(inner, out),
				Self::ChallengeAlreadyExists(inner) => {
					<ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::ChallengeDoesNotExist(inner) => {
					<ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::DelegationExpired(inner) => {
					<DelegationExpired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::ECDSAInvalidSignature(inner) => {
					<ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::ECDSAInvalidSignatureLength(inner) => {
					<ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::ECDSAInvalidSignatureS(inner) => {
					<ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::EthTransferFailed(inner) => {
					<EthTransferFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::FraudProofWindowActive(inner) => {
					<FraudProofWindowActive as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::IncorrectChallengeBond(inner) => {
					<IncorrectChallengeBond as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::InvalidBlockHash(inner) => {
					<InvalidBlockHash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::InvalidBlockNumber(inner) => {
					<InvalidBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::InvalidFieldCount(inner) => {
					<InvalidFieldCount as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::InvalidParentBlockHash(inner) => {
					<InvalidParentBlockHash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::InvalidSignatureLength(inner) => {
					<InvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::NoSignature(inner) => <NoSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out),
				Self::NotURC(inner) => <NotURC as alloy_sol_types::SolError>::abi_encode_raw(inner, out),
				Self::TransactionExcluded(inner) => {
					<TransactionExcluded as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::UnexpectedSigner(inner) => {
					<UnexpectedSigner as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::UnsupportedTxType(inner) => {
					<UnsupportedTxType as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::WrongChallengerAddress(inner) => {
					<WrongChallengerAddress as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::WrongTransactionHashProof(inner) => {
					<WrongTransactionHashProof as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
			}
		}
	}
	use alloy::contract as alloy_contract;
	/**Creates a new wrapper around an on-chain [`StateLockSlasher`](self) contract instance.

	See the [wrapper's documentation](`StateLockSlasherInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> StateLockSlasherInstance<P, N> {
		StateLockSlasherInstance::<P, N>::new(address, __provider)
	}
	/**Deploys this contract using the given `provider` and constructor arguments, if any.

	Returns a new instance of the contract, if the deployment was successful.

	For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
	#[inline]
	pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		__provider: P,
		_slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
	) -> impl ::core::future::Future<Output = alloy_contract::Result<StateLockSlasherInstance<P, N>>> {
		StateLockSlasherInstance::<P, N>::deploy(__provider, _slashAmountWei)
	}
	/**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
	and constructor arguments, if any.

	This is a simple wrapper around creating a `RawCallBuilder` with the data set to
	the bytecode concatenated with the constructor's ABI-encoded arguments.*/
	#[inline]
	pub fn deploy_builder<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		__provider: P,
		_slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
	) -> alloy_contract::RawCallBuilder<P, N> {
		StateLockSlasherInstance::<P, N>::deploy_builder(__provider, _slashAmountWei)
	}
	/**A [`StateLockSlasher`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`StateLockSlasher`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct StateLockSlasherInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for StateLockSlasherInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("StateLockSlasherInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> StateLockSlasherInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`StateLockSlasher`](self) contract instance.

		See the [wrapper's documentation](`StateLockSlasherInstance`) for more details.*/
		#[inline]
		pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
			Self { address, provider: __provider, _network: ::core::marker::PhantomData }
		}
		/**Deploys this contract using the given `provider` and constructor arguments, if any.

		Returns a new instance of the contract, if the deployment was successful.

		For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
		#[inline]
		pub async fn deploy(
			__provider: P,
			_slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
		) -> alloy_contract::Result<StateLockSlasherInstance<P, N>> {
			let call_builder = Self::deploy_builder(__provider, _slashAmountWei);
			let contract_address = call_builder.deploy().await?;
			Ok(Self::new(contract_address, call_builder.provider))
		}
		/**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
		and constructor arguments, if any.

		This is a simple wrapper around creating a `RawCallBuilder` with the data set to
		the bytecode concatenated with the constructor's ABI-encoded arguments.*/
		#[inline]
		pub fn deploy_builder(
			__provider: P,
			_slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
		) -> alloy_contract::RawCallBuilder<P, N> {
			alloy_contract::RawCallBuilder::new_raw_deploy(
				__provider,
				[&BYTECODE[..], &alloy_sol_types::SolConstructor::abi_encode(&constructorCall { _slashAmountWei })[..]]
					.concat()
					.into(),
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
	impl<P: ::core::clone::Clone, N> StateLockSlasherInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> StateLockSlasherInstance<P, N> {
			StateLockSlasherInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> StateLockSlasherInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
		///Creates a new call builder for the [`BEACON_ROOTS_CONTRACT`] function.
		pub fn BEACON_ROOTS_CONTRACT(&self) -> alloy_contract::SolCallBuilder<&P, BEACON_ROOTS_CONTRACTCall, N> {
			self.call_builder(&BEACON_ROOTS_CONTRACTCall)
		}
		///Creates a new call builder for the [`BLOCKHASH_EVM_LOOKBACK`] function.
		pub fn BLOCKHASH_EVM_LOOKBACK(&self) -> alloy_contract::SolCallBuilder<&P, BLOCKHASH_EVM_LOOKBACKCall, N> {
			self.call_builder(&BLOCKHASH_EVM_LOOKBACKCall)
		}
		///Creates a new call builder for the [`EIP4788_WINDOW`] function.
		pub fn EIP4788_WINDOW(&self) -> alloy_contract::SolCallBuilder<&P, EIP4788_WINDOWCall, N> {
			self.call_builder(&EIP4788_WINDOWCall)
		}
		///Creates a new call builder for the [`ETH2_GENESIS_TIMESTAMP`] function.
		pub fn ETH2_GENESIS_TIMESTAMP(&self) -> alloy_contract::SolCallBuilder<&P, ETH2_GENESIS_TIMESTAMPCall, N> {
			self.call_builder(&ETH2_GENESIS_TIMESTAMPCall)
		}
		///Creates a new call builder for the [`JUSTIFICATION_DELAY`] function.
		pub fn JUSTIFICATION_DELAY(&self) -> alloy_contract::SolCallBuilder<&P, JUSTIFICATION_DELAYCall, N> {
			self.call_builder(&JUSTIFICATION_DELAYCall)
		}
		///Creates a new call builder for the [`SLASH_AMOUNT_WEI`] function.
		pub fn SLASH_AMOUNT_WEI(&self) -> alloy_contract::SolCallBuilder<&P, SLASH_AMOUNT_WEICall, N> {
			self.call_builder(&SLASH_AMOUNT_WEICall)
		}
		///Creates a new call builder for the [`SLOT_TIME`] function.
		pub fn SLOT_TIME(&self) -> alloy_contract::SolCallBuilder<&P, SLOT_TIMECall, N> {
			self.call_builder(&SLOT_TIMECall)
		}
		///Creates a new call builder for the [`_decodeBlockHeaderRLP`] function.
		pub fn _decodeBlockHeaderRLP(
			&self,
			headerRLP: alloy::sol_types::private::Bytes,
		) -> alloy_contract::SolCallBuilder<&P, _decodeBlockHeaderRLPCall, N> {
			self.call_builder(&_decodeBlockHeaderRLPCall { headerRLP })
		}
		///Creates a new call builder for the [`_getCurrentSlot`] function.
		pub fn _getCurrentSlot(&self) -> alloy_contract::SolCallBuilder<&P, _getCurrentSlotCall, N> {
			self.call_builder(&_getCurrentSlotCall)
		}
		///Creates a new call builder for the [`_getSlotFromTimestamp`] function.
		pub fn _getSlotFromTimestamp(
			&self,
			_timestamp: alloy::sol_types::private::primitives::aliases::U256,
		) -> alloy_contract::SolCallBuilder<&P, _getSlotFromTimestampCall, N> {
			self.call_builder(&_getSlotFromTimestampCall { _timestamp })
		}
		///Creates a new call builder for the [`_getTimestampFromSlot`] function.
		pub fn _getTimestampFromSlot(
			&self,
			_slot: alloy::sol_types::private::primitives::aliases::U256,
		) -> alloy_contract::SolCallBuilder<&P, _getTimestampFromSlotCall, N> {
			self.call_builder(&_getTimestampFromSlotCall { _slot })
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
			self.call_builder(&slashCall { delegation, commitment, committer, evidence, challenger })
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> StateLockSlasherInstance<P, N> {
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
			alloy_contract::Event::new_sol(&self.provider, &self.address)
		}
	}
}
