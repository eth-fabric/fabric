///Module containing a contract's types and functions.
/**

```solidity
library BLS {
	struct G1Point { bytes32 x_a; bytes32 x_b; bytes32 y_a; bytes32 y_b; }
	struct G2Point { bytes32 x_c0_a; bytes32 x_c0_b; bytes32 x_c1_a; bytes32 x_c1_b; bytes32 y_c0_a; bytes32 y_c0_b; bytes32 y_c1_a; bytes32 y_c1_b; }
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
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct G2Point { bytes32 x_c0_a; bytes32 x_c0_b; bytes32 x_c1_a; bytes32 x_c1_b; bytes32 y_c0_a; bytes32 y_c0_b; bytes32 y_c1_a; bytes32 y_c1_b; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct G2Point {
		#[allow(missing_docs)]
		pub x_c0_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub x_c0_b: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub x_c1_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub x_c1_b: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_c0_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_c0_b: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_c1_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_c1_b: alloy::sol_types::private::FixedBytes<32>,
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
		impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
			fn from(value: G2Point) -> Self {
				(
					value.x_c0_a,
					value.x_c0_b,
					value.x_c1_a,
					value.x_c1_b,
					value.y_c0_a,
					value.y_c0_b,
					value.y_c1_a,
					value.y_c1_b,
				)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self {
					x_c0_a: tuple.0,
					x_c0_b: tuple.1,
					x_c1_a: tuple.2,
					x_c1_b: tuple.3,
					y_c0_a: tuple.4,
					y_c0_b: tuple.5,
					y_c1_a: tuple.6,
					y_c1_b: tuple.7,
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for G2Point {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_c0_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_c0_b),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_c1_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_c1_b),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_c0_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_c0_b),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_c1_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_c1_b),
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
		impl alloy_sol_types::SolType for G2Point {
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
		impl alloy_sol_types::SolStruct for G2Point {
			const NAME: &'static str = "G2Point";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"G2Point(bytes32 x_c0_a,bytes32 x_c0_b,bytes32 x_c1_a,bytes32 x_c1_b,bytes32 y_c0_a,bytes32 y_c0_b,bytes32 y_c1_a,bytes32 y_c1_b)",
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
						&self.x_c0_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_c0_b,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_c1_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_c1_b,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_c0_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_c0_b,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_c1_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_c1_b,
					)
					.0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for G2Point {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.x_c0_a,
					) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.x_c0_b,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.x_c1_a,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.x_c1_b,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_c0_a,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_c0_b,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_c1_a,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_c1_b,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_c0_a,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_c0_b,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_c1_a,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_c1_b,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_c0_a,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_c0_b,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_c1_a,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_c1_b,
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
	struct G2Point {
		bytes32 x_c0_a;
		bytes32 x_c0_b;
		bytes32 x_c1_a;
		bytes32 x_c1_b;
		bytes32 y_c0_a;
		bytes32 y_c0_b;
		bytes32 y_c1_a;
		bytes32 y_c1_b;
	}
}

interface BLSUtils {
	function _toLittleEndian(uint64 x) external pure returns (bytes32);
	function verify(bytes32 messageHash, BLS.G2Point memory signature, BLS.G1Point memory publicKey, bytes32 signingDomain, bytes32 signingId, uint64 nonce, bytes32 chainId) external view returns (bool);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
	"type": "function",
	"name": "_toLittleEndian",
	"inputs": [
	  {
		"name": "x",
		"type": "uint64",
		"internalType": "uint64"
	  }
	],
	"outputs": [
	  {
		"name": "",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"stateMutability": "pure"
  },
  {
	"type": "function",
	"name": "verify",
	"inputs": [
	  {
		"name": "messageHash",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "signature",
		"type": "tuple",
		"internalType": "struct BLS.G2Point",
		"components": [
		  {
			"name": "x_c0_a",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "x_c0_b",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "x_c1_a",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "x_c1_b",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "y_c0_a",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "y_c0_b",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "y_c1_a",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "y_c1_b",
			"type": "bytes32",
			"internalType": "bytes32"
		  }
		]
	  },
	  {
		"name": "publicKey",
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
		"name": "signingDomain",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "signingId",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "nonce",
		"type": "uint64",
		"internalType": "uint64"
	  },
	  {
		"name": "chainId",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [
	  {
		"name": "",
		"type": "bool",
		"internalType": "bool"
	  }
	],
	"stateMutability": "view"
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
pub mod BLSUtils {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	/// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610a26610034600b8282823980515f1a607314602857634e487b7160e01b5f525f60045260245ffd5b305f52607381538281f3fe730000000000000000000000000000000000000000301460806040526004361061003f575f3560e01c80630637192f146100435780635eea787d1461006b575b5f80fd5b61005661005136600461087a565b61008c565b60405190151581526020015b60405180910390f35b61007e610079366004610947565b610258565b604051908152602001610062565b5f8061009b89878787876102e2565b604080516002808252606082019092529192505f9190816020015b604080516080810182525f8082526020808301829052928201819052606082015282525f199092019101816100b657905050905061019260408051608080820183525f808352602083018190528284018190526060830152825190810190925290806f17f1d3a73197d7942695638c4fa9ac0f81526020017fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb81526020016f114d1d6855d545a8aa7d76c8cf2e21f281526020017f67816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca9052919050565b815f815181106101a4576101a4610960565b602002602001018190525087816001815181106101c3576101c3610960565b6020908102919091010152604080516002808252606082019092525f91816020015b6101ed61077a565b8152602001906001900390816101e557905050905089815f8151811061021557610215610960565b6020026020010181905250828160018151811061023457610234610960565b602002602001018190525061024982826104f4565b9b9a5050505050505050505050565b6040805160088082528183019092525f918291906020820181803683370190505090505f5b60088110156102d157610291816008610974565b8467ffffffffffffffff16901c60f81b8282815181106102b3576102b3610960565b60200101906001600160f81b03191690815f1a90535060010161027d565b506102db8161099d565b9392505050565b6102ea61077a565b5f600280888760405160200161030a929190918252602082015260400190565b60408051601f1981840301815290829052610324916109c3565b602060405180830381855afa15801561033f573d5f803e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061036291906109d9565b600261036d87610258565b604080516020810192909252810187905260600160408051601f198184030181529082905261039b916109c3565b602060405180830381855afa1580156103b6573d5f803e3d5ffd5b5050506040513d601f19601f820116820180604052508101906103d991906109d9565b60408051602081019390935282015260600160408051601f1981840301815290829052610405916109c3565b602060405180830381855afa158015610420573d5f803e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061044391906109d9565b90505f60028288604051602001610464929190918252602082015260400190565b60408051601f198184030181529082905261047e916109c3565b602060405180830381855afa158015610499573d5f803e3d5ffd5b5050506040513d601f19601f820116820180604052508101906104bc91906109d9565b90506104e8816040516020016104d491815260200190565b604051602081830303815290604052610563565b98975050505050505050565b5f82516040518484035f5b83811461052e576020870196508061018002830160808851825e61010088840151608083015e506001016104ff565b505060205f836101800283600f5afa60203d1485518414161661055857634df45e2f5f526004601cfd5b50505f519392505050565b61056b61077a565b61061c565b8181537f424c535f5349475f424c53313233383147325f584d443a5348412d3235365f5360018201526b5357555f524f5f504f505f2b60a01b6021820152602d01919050565b5f60205f848460025afa60203d14166105cd575f36fd5b50505f51919050565b604082606083015e6040826101008360055afa60403d14166105f6575f36fd5b5050565b6101008260808360115afa6101003d14166105f6576389083b915f526004601cfd5b60405161010081016040368237835184602001826040015e8351816040010161010060f01b815261065b826106545f84600201610570565b03836105b6565b90508060205280825261067682610654600185602001610570565b83528260025b8151831884526020820191506106a0846106998387602001610570565b03856105b6565b825260018101906007190161067c57505060408083526020808401528281015250600160a08201526f1a0111ea397fe69a4b1ba7b6434bacd760c08201527f64774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab60e082015261070e82826105d6565b61071b60408301826105d6565b61072860808301826105d6565b61073560c08301826105d6565b5061074082826105fa565b6107518261010001826080016105fa565b506101008161020083600d5afa6101003d14166107755763c55e5e335f526004601cfd5b919050565b60408051610100810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081019190915290565b604051610100810167ffffffffffffffff811182821017156107ed57634e487b7160e01b5f52604160045260245ffd5b60405290565b5f60808284031215610803575f80fd5b6040516080810181811067ffffffffffffffff8211171561083257634e487b7160e01b5f52604160045260245ffd5b8060405250809150823581526020830135602082015260408301356040820152606083013560608201525092915050565b803567ffffffffffffffff81168114610775575f80fd5b5f805f805f805f878903610220811215610892575f80fd5b8835975061010080601f19830112156108a9575f80fd5b6108b16107bd565b915060208a0135825260408a0135602083015260608a0135604083015260808a0135606083015260a08a0135608083015260c08a013560a083015260e08a013560c0830152808a013560e08301525080965050610912896101208a016107f3565b94506101a088013593506101c088013592506109316101e08901610863565b9150610200880135905092959891949750929550565b5f60208284031215610957575f80fd5b6102db82610863565b634e487b7160e01b5f52603260045260245ffd5b808202811582820484141761099757634e487b7160e01b5f52601160045260245ffd5b92915050565b805160208083015191908110156109bd575f198160200360031b1b821691505b50919050565b5f82518060208501845e5f920191825250919050565b5f602082840312156109e9575f80fd5b505191905056fea26469706673582212205d09b55256de6ef3c64fb3bf57881599f7f39f10c472dcd7160bd3cdcb09413464736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\n&a\x004`\x0B\x82\x82\x829\x80Q_\x1A`s\x14`(WcNH{q`\xE0\x1B_R_`\x04R`$_\xFD[0_R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0?W_5`\xE0\x1C\x80c\x067\x19/\x14a\0CW\x80c^\xEAx}\x14a\0kW[_\x80\xFD[a\0Va\0Q6`\x04a\x08zV[a\0\x8CV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0~a\0y6`\x04a\tGV[a\x02XV[`@Q\x90\x81R` \x01a\0bV[_\x80a\0\x9B\x89\x87\x87\x87\x87a\x02\xE2V[`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R\x91\x92P_\x91\x90\x81` \x01[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\0\xB6W\x90PP\x90Pa\x01\x92`@\x80Q`\x80\x80\x82\x01\x83R_\x80\x83R` \x83\x01\x81\x90R\x82\x84\x01\x81\x90R``\x83\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80o\x17\xF1\xD3\xA71\x97\xD7\x94&\x95c\x8CO\xA9\xAC\x0F\x81R` \x01\x7F\xC3h\x8CO\x97t\xB9\x05\xA1N:?\x17\x1B\xACXlU\xE8?\xF9z\x1A\xEF\xFB:\xF0\n\xDB\"\xC6\xBB\x81R` \x01o\x11M\x1DhU\xD5E\xA8\xAA}v\xC8\xCF.!\xF2\x81R` \x01\x7Fg\x81j\xEF\x1D\xB5\x07\xC9fU\xB9\xD5\xCA\xACB6No8\xBA\x0E\xCBu\x1B\xADT\xDC\xD6\xB99\xC2\xCA\x90R\x91\x90PV[\x81_\x81Q\x81\x10a\x01\xA4Wa\x01\xA4a\t`V[` \x02` \x01\x01\x81\x90RP\x87\x81`\x01\x81Q\x81\x10a\x01\xC3Wa\x01\xC3a\t`V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R_\x91\x81` \x01[a\x01\xEDa\x07zV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\xE5W\x90PP\x90P\x89\x81_\x81Q\x81\x10a\x02\x15Wa\x02\x15a\t`V[` \x02` \x01\x01\x81\x90RP\x82\x81`\x01\x81Q\x81\x10a\x024Wa\x024a\t`V[` \x02` \x01\x01\x81\x90RPa\x02I\x82\x82a\x04\xF4V[\x9B\x9APPPPPPPPPPPV[`@\x80Q`\x08\x80\x82R\x81\x83\x01\x90\x92R_\x91\x82\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`\x08\x81\x10\x15a\x02\xD1Wa\x02\x91\x81`\x08a\ttV[\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x02\xB3Wa\x02\xB3a\t`V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x02}V[Pa\x02\xDB\x81a\t\x9DV[\x93\x92PPPV[a\x02\xEAa\x07zV[_`\x02\x80\x88\x87`@Q` \x01a\x03\n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03$\x91a\t\xC3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x03?W=_\x80>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03b\x91\x90a\t\xD9V[`\x02a\x03m\x87a\x02XV[`@\x80Q` \x81\x01\x92\x90\x92R\x81\x01\x87\x90R``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03\x9B\x91a\t\xC3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x03\xB6W=_\x80>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xD9\x91\x90a\t\xD9V[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04\x05\x91a\t\xC3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x04 W=_\x80>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04C\x91\x90a\t\xD9V[\x90P_`\x02\x82\x88`@Q` \x01a\x04d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04~\x91a\t\xC3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x04\x99W=_\x80>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xBC\x91\x90a\t\xD9V[\x90Pa\x04\xE8\x81`@Q` \x01a\x04\xD4\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x05cV[\x98\x97PPPPPPPPV[_\x82Q`@Q\x84\x84\x03_[\x83\x81\x14a\x05.W` \x87\x01\x96P\x80a\x01\x80\x02\x83\x01`\x80\x88Q\x82^a\x01\0\x88\x84\x01Q`\x80\x83\x01^P`\x01\x01a\x04\xFFV[PP` _\x83a\x01\x80\x02\x83`\x0FZ\xFA` =\x14\x85Q\x84\x14\x16\x16a\x05XWcM\xF4^/_R`\x04`\x1C\xFD[PP_Q\x93\x92PPPV[a\x05ka\x07zV[a\x06\x1CV[\x81\x81S\x7FBLS_SIG_BLS12381G2_XMD:SHA-256_S`\x01\x82\x01RkSWU_RO_POP_+`\xA0\x1B`!\x82\x01R`-\x01\x91\x90PV[_` _\x84\x84`\x02Z\xFA` =\x14\x16a\x05\xCDW_6\xFD[PP_Q\x91\x90PV[`@\x82``\x83\x01^`@\x82a\x01\0\x83`\x05Z\xFA`@=\x14\x16a\x05\xF6W_6\xFD[PPV[a\x01\0\x82`\x80\x83`\x11Z\xFAa\x01\0=\x14\x16a\x05\xF6Wc\x89\x08;\x91_R`\x04`\x1C\xFD[`@Qa\x01\0\x81\x01`@6\x827\x83Q\x84` \x01\x82`@\x01^\x83Q\x81`@\x01\x01a\x01\0`\xF0\x1B\x81Ra\x06[\x82a\x06T_\x84`\x02\x01a\x05pV[\x03\x83a\x05\xB6V[\x90P\x80` R\x80\x82Ra\x06v\x82a\x06T`\x01\x85` \x01a\x05pV[\x83R\x82`\x02[\x81Q\x83\x18\x84R` \x82\x01\x91Pa\x06\xA0\x84a\x06\x99\x83\x87` \x01a\x05pV[\x03\x85a\x05\xB6V[\x82R`\x01\x81\x01\x90`\x07\x19\x01a\x06|WPP`@\x80\x83R` \x80\x84\x01R\x82\x81\x01RP`\x01`\xA0\x82\x01Ro\x1A\x01\x11\xEA9\x7F\xE6\x9AK\x1B\xA7\xB6CK\xAC\xD7`\xC0\x82\x01R\x7FdwK\x84\xF3\x85\x12\xBFg0\xD2\xA0\xF6\xB0\xF6$\x1E\xAB\xFF\xFE\xB1S\xFF\xFF\xB9\xFE\xFF\xFF\xFF\xFF\xAA\xAB`\xE0\x82\x01Ra\x07\x0E\x82\x82a\x05\xD6V[a\x07\x1B`@\x83\x01\x82a\x05\xD6V[a\x07(`\x80\x83\x01\x82a\x05\xD6V[a\x075`\xC0\x83\x01\x82a\x05\xD6V[Pa\x07@\x82\x82a\x05\xFAV[a\x07Q\x82a\x01\0\x01\x82`\x80\x01a\x05\xFAV[Pa\x01\0\x81a\x02\0\x83`\rZ\xFAa\x01\0=\x14\x16a\x07uWc\xC5^^3_R`\x04`\x1C\xFD[\x91\x90PV[`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x90V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xEDWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90V[_`\x80\x82\x84\x03\x12\x15a\x08\x03W_\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x082WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07uW_\x80\xFD[_\x80_\x80_\x80_\x87\x89\x03a\x02 \x81\x12\x15a\x08\x92W_\x80\xFD[\x885\x97Pa\x01\0\x80`\x1F\x19\x83\x01\x12\x15a\x08\xA9W_\x80\xFD[a\x08\xB1a\x07\xBDV[\x91P` \x8A\x015\x82R`@\x8A\x015` \x83\x01R``\x8A\x015`@\x83\x01R`\x80\x8A\x015``\x83\x01R`\xA0\x8A\x015`\x80\x83\x01R`\xC0\x8A\x015`\xA0\x83\x01R`\xE0\x8A\x015`\xC0\x83\x01R\x80\x8A\x015`\xE0\x83\x01RP\x80\x96PPa\t\x12\x89a\x01 \x8A\x01a\x07\xF3V[\x94Pa\x01\xA0\x88\x015\x93Pa\x01\xC0\x88\x015\x92Pa\t1a\x01\xE0\x89\x01a\x08cV[\x91Pa\x02\0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[_` \x82\x84\x03\x12\x15a\tWW_\x80\xFD[a\x02\xDB\x82a\x08cV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\t\x97WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\t\xBDW_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\t\xE9W_\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 ]\t\xB5RV\xDEn\xF3\xC6O\xB3\xBFW\x88\x15\x99\xF7\xF3\x9F\x10\xC4r\xDC\xD7\x16\x0B\xD3\xCD\xCB\tA4dsolcC\0\x08\x19\x003",
    );
	/// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x730000000000000000000000000000000000000000301460806040526004361061003f575f3560e01c80630637192f146100435780635eea787d1461006b575b5f80fd5b61005661005136600461087a565b61008c565b60405190151581526020015b60405180910390f35b61007e610079366004610947565b610258565b604051908152602001610062565b5f8061009b89878787876102e2565b604080516002808252606082019092529192505f9190816020015b604080516080810182525f8082526020808301829052928201819052606082015282525f199092019101816100b657905050905061019260408051608080820183525f808352602083018190528284018190526060830152825190810190925290806f17f1d3a73197d7942695638c4fa9ac0f81526020017fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb81526020016f114d1d6855d545a8aa7d76c8cf2e21f281526020017f67816aef1db507c96655b9d5caac42364e6f38ba0ecb751bad54dcd6b939c2ca9052919050565b815f815181106101a4576101a4610960565b602002602001018190525087816001815181106101c3576101c3610960565b6020908102919091010152604080516002808252606082019092525f91816020015b6101ed61077a565b8152602001906001900390816101e557905050905089815f8151811061021557610215610960565b6020026020010181905250828160018151811061023457610234610960565b602002602001018190525061024982826104f4565b9b9a5050505050505050505050565b6040805160088082528183019092525f918291906020820181803683370190505090505f5b60088110156102d157610291816008610974565b8467ffffffffffffffff16901c60f81b8282815181106102b3576102b3610960565b60200101906001600160f81b03191690815f1a90535060010161027d565b506102db8161099d565b9392505050565b6102ea61077a565b5f600280888760405160200161030a929190918252602082015260400190565b60408051601f1981840301815290829052610324916109c3565b602060405180830381855afa15801561033f573d5f803e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061036291906109d9565b600261036d87610258565b604080516020810192909252810187905260600160408051601f198184030181529082905261039b916109c3565b602060405180830381855afa1580156103b6573d5f803e3d5ffd5b5050506040513d601f19601f820116820180604052508101906103d991906109d9565b60408051602081019390935282015260600160408051601f1981840301815290829052610405916109c3565b602060405180830381855afa158015610420573d5f803e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061044391906109d9565b90505f60028288604051602001610464929190918252602082015260400190565b60408051601f198184030181529082905261047e916109c3565b602060405180830381855afa158015610499573d5f803e3d5ffd5b5050506040513d601f19601f820116820180604052508101906104bc91906109d9565b90506104e8816040516020016104d491815260200190565b604051602081830303815290604052610563565b98975050505050505050565b5f82516040518484035f5b83811461052e576020870196508061018002830160808851825e61010088840151608083015e506001016104ff565b505060205f836101800283600f5afa60203d1485518414161661055857634df45e2f5f526004601cfd5b50505f519392505050565b61056b61077a565b61061c565b8181537f424c535f5349475f424c53313233383147325f584d443a5348412d3235365f5360018201526b5357555f524f5f504f505f2b60a01b6021820152602d01919050565b5f60205f848460025afa60203d14166105cd575f36fd5b50505f51919050565b604082606083015e6040826101008360055afa60403d14166105f6575f36fd5b5050565b6101008260808360115afa6101003d14166105f6576389083b915f526004601cfd5b60405161010081016040368237835184602001826040015e8351816040010161010060f01b815261065b826106545f84600201610570565b03836105b6565b90508060205280825261067682610654600185602001610570565b83528260025b8151831884526020820191506106a0846106998387602001610570565b03856105b6565b825260018101906007190161067c57505060408083526020808401528281015250600160a08201526f1a0111ea397fe69a4b1ba7b6434bacd760c08201527f64774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab60e082015261070e82826105d6565b61071b60408301826105d6565b61072860808301826105d6565b61073560c08301826105d6565b5061074082826105fa565b6107518261010001826080016105fa565b506101008161020083600d5afa6101003d14166107755763c55e5e335f526004601cfd5b919050565b60408051610100810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081019190915290565b604051610100810167ffffffffffffffff811182821017156107ed57634e487b7160e01b5f52604160045260245ffd5b60405290565b5f60808284031215610803575f80fd5b6040516080810181811067ffffffffffffffff8211171561083257634e487b7160e01b5f52604160045260245ffd5b8060405250809150823581526020830135602082015260408301356040820152606083013560608201525092915050565b803567ffffffffffffffff81168114610775575f80fd5b5f805f805f805f878903610220811215610892575f80fd5b8835975061010080601f19830112156108a9575f80fd5b6108b16107bd565b915060208a0135825260408a0135602083015260608a0135604083015260808a0135606083015260a08a0135608083015260c08a013560a083015260e08a013560c0830152808a013560e08301525080965050610912896101208a016107f3565b94506101a088013593506101c088013592506109316101e08901610863565b9150610200880135905092959891949750929550565b5f60208284031215610957575f80fd5b6102db82610863565b634e487b7160e01b5f52603260045260245ffd5b808202811582820484141761099757634e487b7160e01b5f52601160045260245ffd5b92915050565b805160208083015191908110156109bd575f198160200360031b1b821691505b50919050565b5f82518060208501845e5f920191825250919050565b5f602082840312156109e9575f80fd5b505191905056fea26469706673582212205d09b55256de6ef3c64fb3bf57881599f7f39f10c472dcd7160bd3cdcb09413464736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0?W_5`\xE0\x1C\x80c\x067\x19/\x14a\0CW\x80c^\xEAx}\x14a\0kW[_\x80\xFD[a\0Va\0Q6`\x04a\x08zV[a\0\x8CV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0~a\0y6`\x04a\tGV[a\x02XV[`@Q\x90\x81R` \x01a\0bV[_\x80a\0\x9B\x89\x87\x87\x87\x87a\x02\xE2V[`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R\x91\x92P_\x91\x90\x81` \x01[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\0\xB6W\x90PP\x90Pa\x01\x92`@\x80Q`\x80\x80\x82\x01\x83R_\x80\x83R` \x83\x01\x81\x90R\x82\x84\x01\x81\x90R``\x83\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80o\x17\xF1\xD3\xA71\x97\xD7\x94&\x95c\x8CO\xA9\xAC\x0F\x81R` \x01\x7F\xC3h\x8CO\x97t\xB9\x05\xA1N:?\x17\x1B\xACXlU\xE8?\xF9z\x1A\xEF\xFB:\xF0\n\xDB\"\xC6\xBB\x81R` \x01o\x11M\x1DhU\xD5E\xA8\xAA}v\xC8\xCF.!\xF2\x81R` \x01\x7Fg\x81j\xEF\x1D\xB5\x07\xC9fU\xB9\xD5\xCA\xACB6No8\xBA\x0E\xCBu\x1B\xADT\xDC\xD6\xB99\xC2\xCA\x90R\x91\x90PV[\x81_\x81Q\x81\x10a\x01\xA4Wa\x01\xA4a\t`V[` \x02` \x01\x01\x81\x90RP\x87\x81`\x01\x81Q\x81\x10a\x01\xC3Wa\x01\xC3a\t`V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x02\x80\x82R``\x82\x01\x90\x92R_\x91\x81` \x01[a\x01\xEDa\x07zV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\xE5W\x90PP\x90P\x89\x81_\x81Q\x81\x10a\x02\x15Wa\x02\x15a\t`V[` \x02` \x01\x01\x81\x90RP\x82\x81`\x01\x81Q\x81\x10a\x024Wa\x024a\t`V[` \x02` \x01\x01\x81\x90RPa\x02I\x82\x82a\x04\xF4V[\x9B\x9APPPPPPPPPPPV[`@\x80Q`\x08\x80\x82R\x81\x83\x01\x90\x92R_\x91\x82\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`\x08\x81\x10\x15a\x02\xD1Wa\x02\x91\x81`\x08a\ttV[\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x02\xB3Wa\x02\xB3a\t`V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x02}V[Pa\x02\xDB\x81a\t\x9DV[\x93\x92PPPV[a\x02\xEAa\x07zV[_`\x02\x80\x88\x87`@Q` \x01a\x03\n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03$\x91a\t\xC3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x03?W=_\x80>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03b\x91\x90a\t\xD9V[`\x02a\x03m\x87a\x02XV[`@\x80Q` \x81\x01\x92\x90\x92R\x81\x01\x87\x90R``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03\x9B\x91a\t\xC3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x03\xB6W=_\x80>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xD9\x91\x90a\t\xD9V[`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04\x05\x91a\t\xC3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x04 W=_\x80>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04C\x91\x90a\t\xD9V[\x90P_`\x02\x82\x88`@Q` \x01a\x04d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04~\x91a\t\xC3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x04\x99W=_\x80>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xBC\x91\x90a\t\xD9V[\x90Pa\x04\xE8\x81`@Q` \x01a\x04\xD4\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x05cV[\x98\x97PPPPPPPPV[_\x82Q`@Q\x84\x84\x03_[\x83\x81\x14a\x05.W` \x87\x01\x96P\x80a\x01\x80\x02\x83\x01`\x80\x88Q\x82^a\x01\0\x88\x84\x01Q`\x80\x83\x01^P`\x01\x01a\x04\xFFV[PP` _\x83a\x01\x80\x02\x83`\x0FZ\xFA` =\x14\x85Q\x84\x14\x16\x16a\x05XWcM\xF4^/_R`\x04`\x1C\xFD[PP_Q\x93\x92PPPV[a\x05ka\x07zV[a\x06\x1CV[\x81\x81S\x7FBLS_SIG_BLS12381G2_XMD:SHA-256_S`\x01\x82\x01RkSWU_RO_POP_+`\xA0\x1B`!\x82\x01R`-\x01\x91\x90PV[_` _\x84\x84`\x02Z\xFA` =\x14\x16a\x05\xCDW_6\xFD[PP_Q\x91\x90PV[`@\x82``\x83\x01^`@\x82a\x01\0\x83`\x05Z\xFA`@=\x14\x16a\x05\xF6W_6\xFD[PPV[a\x01\0\x82`\x80\x83`\x11Z\xFAa\x01\0=\x14\x16a\x05\xF6Wc\x89\x08;\x91_R`\x04`\x1C\xFD[`@Qa\x01\0\x81\x01`@6\x827\x83Q\x84` \x01\x82`@\x01^\x83Q\x81`@\x01\x01a\x01\0`\xF0\x1B\x81Ra\x06[\x82a\x06T_\x84`\x02\x01a\x05pV[\x03\x83a\x05\xB6V[\x90P\x80` R\x80\x82Ra\x06v\x82a\x06T`\x01\x85` \x01a\x05pV[\x83R\x82`\x02[\x81Q\x83\x18\x84R` \x82\x01\x91Pa\x06\xA0\x84a\x06\x99\x83\x87` \x01a\x05pV[\x03\x85a\x05\xB6V[\x82R`\x01\x81\x01\x90`\x07\x19\x01a\x06|WPP`@\x80\x83R` \x80\x84\x01R\x82\x81\x01RP`\x01`\xA0\x82\x01Ro\x1A\x01\x11\xEA9\x7F\xE6\x9AK\x1B\xA7\xB6CK\xAC\xD7`\xC0\x82\x01R\x7FdwK\x84\xF3\x85\x12\xBFg0\xD2\xA0\xF6\xB0\xF6$\x1E\xAB\xFF\xFE\xB1S\xFF\xFF\xB9\xFE\xFF\xFF\xFF\xFF\xAA\xAB`\xE0\x82\x01Ra\x07\x0E\x82\x82a\x05\xD6V[a\x07\x1B`@\x83\x01\x82a\x05\xD6V[a\x07(`\x80\x83\x01\x82a\x05\xD6V[a\x075`\xC0\x83\x01\x82a\x05\xD6V[Pa\x07@\x82\x82a\x05\xFAV[a\x07Q\x82a\x01\0\x01\x82`\x80\x01a\x05\xFAV[Pa\x01\0\x81a\x02\0\x83`\rZ\xFAa\x01\0=\x14\x16a\x07uWc\xC5^^3_R`\x04`\x1C\xFD[\x91\x90PV[`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x90V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xEDWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90V[_`\x80\x82\x84\x03\x12\x15a\x08\x03W_\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x082WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80`@RP\x80\x91P\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01RP\x92\x91PPV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07uW_\x80\xFD[_\x80_\x80_\x80_\x87\x89\x03a\x02 \x81\x12\x15a\x08\x92W_\x80\xFD[\x885\x97Pa\x01\0\x80`\x1F\x19\x83\x01\x12\x15a\x08\xA9W_\x80\xFD[a\x08\xB1a\x07\xBDV[\x91P` \x8A\x015\x82R`@\x8A\x015` \x83\x01R``\x8A\x015`@\x83\x01R`\x80\x8A\x015``\x83\x01R`\xA0\x8A\x015`\x80\x83\x01R`\xC0\x8A\x015`\xA0\x83\x01R`\xE0\x8A\x015`\xC0\x83\x01R\x80\x8A\x015`\xE0\x83\x01RP\x80\x96PPa\t\x12\x89a\x01 \x8A\x01a\x07\xF3V[\x94Pa\x01\xA0\x88\x015\x93Pa\x01\xC0\x88\x015\x92Pa\t1a\x01\xE0\x89\x01a\x08cV[\x91Pa\x02\0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[_` \x82\x84\x03\x12\x15a\tWW_\x80\xFD[a\x02\xDB\x82a\x08cV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\t\x97WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\t\xBDW_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\t\xE9W_\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 ]\t\xB5RV\xDEn\xF3\xC6O\xB3\xBFW\x88\x15\x99\xF7\xF3\x9F\x10\xC4r\xDC\xD7\x16\x0B\xD3\xCD\xCB\tA4dsolcC\0\x08\x19\x003",
    );
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `_toLittleEndian(uint64)` and selector `0x5eea787d`.
	```solidity
	function _toLittleEndian(uint64 x) external pure returns (bytes32);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _toLittleEndianCall {
		#[allow(missing_docs)]
		pub x: u64,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`_toLittleEndian(uint64)`](_toLittleEndianCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct _toLittleEndianReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::FixedBytes<32>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (u64,);
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
			impl ::core::convert::From<_toLittleEndianCall> for UnderlyingRustTuple<'_> {
				fn from(value: _toLittleEndianCall) -> Self {
					(value.x,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _toLittleEndianCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { x: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
			impl ::core::convert::From<_toLittleEndianReturn> for UnderlyingRustTuple<'_> {
				fn from(value: _toLittleEndianReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for _toLittleEndianReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for _toLittleEndianCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::FixedBytes<32>;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "_toLittleEndian(uint64)";
			const SELECTOR: [u8; 4] = [94u8, 234u8, 120u8, 125u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.x),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: _toLittleEndianReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: _toLittleEndianReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `verify(bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),bytes32,bytes32,uint64,bytes32)` and selector `0xbf4742a6`.
	```solidity
	function verify(bytes32 messageHash, BLS.G2Point memory signature, BLS.G1Point memory publicKey, bytes32 signingDomain, bytes32 signingId, uint64 nonce, bytes32 chainId) external view returns (bool);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct verifyCall {
		#[allow(missing_docs)]
		pub messageHash: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub signature: <BLS::G2Point as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub publicKey: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub signingDomain: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub nonce: u64,
		#[allow(missing_docs)]
		pub chainId: alloy::sol_types::private::FixedBytes<32>,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`verify(bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),bytes32,bytes32,uint64,bytes32)`](verifyCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct verifyReturn {
		#[allow(missing_docs)]
		pub _0: bool,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (
				alloy::sol_types::sol_data::FixedBytes<32>,
				BLS::G2Point,
				BLS::G1Point,
				alloy::sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::Uint<64>,
				alloy::sol_types::sol_data::FixedBytes<32>,
			);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				alloy::sol_types::private::FixedBytes<32>,
				<BLS::G2Point as alloy::sol_types::SolType>::RustType,
				<BLS::G1Point as alloy::sol_types::SolType>::RustType,
				alloy::sol_types::private::FixedBytes<32>,
				alloy::sol_types::private::FixedBytes<32>,
				u64,
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
			impl ::core::convert::From<verifyCall> for UnderlyingRustTuple<'_> {
				fn from(value: verifyCall) -> Self {
					(
						value.messageHash,
						value.signature,
						value.publicKey,
						value.signingDomain,
						value.signingId,
						value.nonce,
						value.chainId,
					)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {
						messageHash: tuple.0,
						signature: tuple.1,
						publicKey: tuple.2,
						signingDomain: tuple.3,
						signingId: tuple.4,
						nonce: tuple.5,
						chainId: tuple.6,
					}
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (bool,);
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
			impl ::core::convert::From<verifyReturn> for UnderlyingRustTuple<'_> {
				fn from(value: verifyReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for verifyCall {
			type Parameters<'a> = (
				alloy::sol_types::sol_data::FixedBytes<32>,
				BLS::G2Point,
				BLS::G1Point,
				alloy::sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::Uint<64>,
				alloy::sol_types::sol_data::FixedBytes<32>,
			);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = bool;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "verify(bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),bytes32,bytes32,uint64,bytes32)";
			const SELECTOR: [u8; 4] = [191u8, 71u8, 66u8, 166u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.messageHash,
					),
					<BLS::G2Point as alloy_sol_types::SolType>::tokenize(&self.signature),
					<BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.publicKey),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.signingDomain,
					),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.signingId),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.nonce),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.chainId),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: verifyReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: verifyReturn = r.into();
					r._0
				})
			}
		}
	};
	///Container for all the [`BLSUtils`](self) function calls.
	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum BLSUtilsCalls {
		#[allow(missing_docs)]
		_toLittleEndian(_toLittleEndianCall),
		#[allow(missing_docs)]
		verify(verifyCall),
	}
	impl BLSUtilsCalls {
		/// All the selectors of this enum.
		///
		/// Note that the selectors might not be in the same order as the variants.
		/// No guarantees are made about the order of the selectors.
		///
		/// Prefer using `SolInterface` methods instead.
		pub const SELECTORS: &'static [[u8; 4usize]] = &[[94u8, 234u8, 120u8, 125u8], [191u8, 71u8, 66u8, 166u8]];
		/// The names of the variants in the same order as `SELECTORS`.
		pub const VARIANT_NAMES: &'static [&'static str] =
			&[::core::stringify!(_toLittleEndian), ::core::stringify!(verify)];
		/// The signatures in the same order as `SELECTORS`.
		pub const SIGNATURES: &'static [&'static str] = &[
			<_toLittleEndianCall as alloy_sol_types::SolCall>::SIGNATURE,
			<verifyCall as alloy_sol_types::SolCall>::SIGNATURE,
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
	impl alloy_sol_types::SolInterface for BLSUtilsCalls {
		const NAME: &'static str = "BLSUtilsCalls";
		const MIN_DATA_LENGTH: usize = 32usize;
		const COUNT: usize = 2usize;
		#[inline]
		fn selector(&self) -> [u8; 4] {
			match self {
				Self::_toLittleEndian(_) => <_toLittleEndianCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::verify(_) => <verifyCall as alloy_sol_types::SolCall>::SELECTOR,
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
			static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<BLSUtilsCalls>] = &[
				{
					fn _toLittleEndian(data: &[u8]) -> alloy_sol_types::Result<BLSUtilsCalls> {
						<_toLittleEndianCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(BLSUtilsCalls::_toLittleEndian)
					}
					_toLittleEndian
				},
				{
					fn verify(data: &[u8]) -> alloy_sol_types::Result<BLSUtilsCalls> {
						<verifyCall as alloy_sol_types::SolCall>::abi_decode_raw(data).map(BLSUtilsCalls::verify)
					}
					verify
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
			static DECODE_VALIDATE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<BLSUtilsCalls>] = &[
				{
					fn _toLittleEndian(data: &[u8]) -> alloy_sol_types::Result<BLSUtilsCalls> {
						<_toLittleEndianCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(BLSUtilsCalls::_toLittleEndian)
					}
					_toLittleEndian
				},
				{
					fn verify(data: &[u8]) -> alloy_sol_types::Result<BLSUtilsCalls> {
						<verifyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(BLSUtilsCalls::verify)
					}
					verify
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
				Self::_toLittleEndian(inner) => {
					<_toLittleEndianCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::verify(inner) => <verifyCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
			}
		}
		#[inline]
		fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
			match self {
				Self::_toLittleEndian(inner) => {
					<_toLittleEndianCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::verify(inner) => <verifyCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
			}
		}
	}
	use alloy::contract as alloy_contract;
	/**Creates a new wrapper around an on-chain [`BLSUtils`](self) contract instance.

	See the [wrapper's documentation](`BLSUtilsInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> BLSUtilsInstance<P, N> {
		BLSUtilsInstance::<P, N>::new(address, __provider)
	}
	/**Deploys this contract using the given `provider` and constructor arguments, if any.

	Returns a new instance of the contract, if the deployment was successful.

	For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
	#[inline]
	pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		__provider: P,
	) -> impl ::core::future::Future<Output = alloy_contract::Result<BLSUtilsInstance<P, N>>> {
		BLSUtilsInstance::<P, N>::deploy(__provider)
	}
	/**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
	and constructor arguments, if any.

	This is a simple wrapper around creating a `RawCallBuilder` with the data set to
	the bytecode concatenated with the constructor's ABI-encoded arguments.*/
	#[inline]
	pub fn deploy_builder<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		__provider: P,
	) -> alloy_contract::RawCallBuilder<P, N> {
		BLSUtilsInstance::<P, N>::deploy_builder(__provider)
	}
	/**A [`BLSUtils`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`BLSUtils`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct BLSUtilsInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for BLSUtilsInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("BLSUtilsInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> BLSUtilsInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`BLSUtils`](self) contract instance.

		See the [wrapper's documentation](`BLSUtilsInstance`) for more details.*/
		#[inline]
		pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
			Self { address, provider: __provider, _network: ::core::marker::PhantomData }
		}
		/**Deploys this contract using the given `provider` and constructor arguments, if any.

		Returns a new instance of the contract, if the deployment was successful.

		For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
		#[inline]
		pub async fn deploy(__provider: P) -> alloy_contract::Result<BLSUtilsInstance<P, N>> {
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
			alloy_contract::RawCallBuilder::new_raw_deploy(__provider, ::core::clone::Clone::clone(&BYTECODE))
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
	impl<P: ::core::clone::Clone, N> BLSUtilsInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> BLSUtilsInstance<P, N> {
			BLSUtilsInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> BLSUtilsInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
		///Creates a new call builder for the [`_toLittleEndian`] function.
		pub fn _toLittleEndian(&self, x: u64) -> alloy_contract::SolCallBuilder<&P, _toLittleEndianCall, N> {
			self.call_builder(&_toLittleEndianCall { x })
		}
		///Creates a new call builder for the [`verify`] function.
		pub fn verify(
			&self,
			messageHash: alloy::sol_types::private::FixedBytes<32>,
			signature: <BLS::G2Point as alloy::sol_types::SolType>::RustType,
			publicKey: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
			signingDomain: alloy::sol_types::private::FixedBytes<32>,
			signingId: alloy::sol_types::private::FixedBytes<32>,
			nonce: u64,
			chainId: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, verifyCall, N> {
			self.call_builder(&verifyCall {
				messageHash,
				signature,
				publicKey,
				signingDomain,
				signingId,
				nonce,
				chainId,
			})
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> BLSUtilsInstance<P, N> {
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
			alloy_contract::Event::new_sol(&self.provider, &self.address)
		}
	}
}
