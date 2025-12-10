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
///Module containing a contract's types and functions.
/**

```solidity
library ISlasher {
	struct Commitment { uint64 commitmentType; bytes payload; bytes32 requestHash; address slasher; }
	struct Delegation { BLS.G1Point proposer; BLS.G1Point delegate; address committer; uint64 slot; bytes metadata; }
	struct SignedCommitment { Commitment commitment; uint64 nonce; bytes32 signingId; bytes signature; }
	struct SignedDelegation { Delegation delegation; uint64 nonce; bytes32 signingId; BLS.G2Point signature; }
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
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct SignedCommitment { Commitment commitment; uint64 nonce; bytes32 signingId; bytes signature; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SignedCommitment {
		#[allow(missing_docs)]
		pub commitment: <Commitment as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub nonce: u64,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub signature: alloy::sol_types::private::Bytes,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			Commitment,
			alloy::sol_types::sol_data::Uint<64>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::Bytes,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			<Commitment as alloy::sol_types::SolType>::RustType,
			u64,
			alloy::sol_types::private::FixedBytes<32>,
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
		impl ::core::convert::From<SignedCommitment> for UnderlyingRustTuple<'_> {
			fn from(value: SignedCommitment) -> Self {
				(value.commitment, value.nonce, value.signingId, value.signature)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedCommitment {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { commitment: tuple.0, nonce: tuple.1, signingId: tuple.2, signature: tuple.3 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for SignedCommitment {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for SignedCommitment {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<Commitment as alloy_sol_types::SolType>::tokenize(&self.commitment),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.nonce),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.signingId),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.signature),
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
		impl alloy_sol_types::SolType for SignedCommitment {
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
		impl alloy_sol_types::SolStruct for SignedCommitment {
			const NAME: &'static str = "SignedCommitment";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"SignedCommitment(Commitment commitment,uint64 nonce,bytes32 signingId,bytes signature)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(1);
				components.push(<Commitment as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<Commitment as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<Commitment as alloy_sol_types::SolType>::eip712_data_word(&self.commitment).0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.nonce).0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.signingId,
					)
					.0,
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(&self.signature)
						.0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for SignedCommitment {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <Commitment as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.commitment)
					+ <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.nonce,
					) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.signingId,
				) + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.signature,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<Commitment as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.commitment, out);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.nonce,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signingId,
					out,
				);
				<alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signature,
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
	struct SignedDelegation { Delegation delegation; uint64 nonce; bytes32 signingId; BLS.G2Point signature; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SignedDelegation {
		#[allow(missing_docs)]
		pub delegation: <Delegation as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub nonce: u64,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub signature: <BLS::G2Point as alloy::sol_types::SolType>::RustType,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			Delegation,
			alloy::sol_types::sol_data::Uint<64>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			BLS::G2Point,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			<Delegation as alloy::sol_types::SolType>::RustType,
			u64,
			alloy::sol_types::private::FixedBytes<32>,
			<BLS::G2Point as alloy::sol_types::SolType>::RustType,
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
		impl ::core::convert::From<SignedDelegation> for UnderlyingRustTuple<'_> {
			fn from(value: SignedDelegation) -> Self {
				(value.delegation, value.nonce, value.signingId, value.signature)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedDelegation {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { delegation: tuple.0, nonce: tuple.1, signingId: tuple.2, signature: tuple.3 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for SignedDelegation {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for SignedDelegation {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<Delegation as alloy_sol_types::SolType>::tokenize(&self.delegation),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.nonce),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.signingId),
					<BLS::G2Point as alloy_sol_types::SolType>::tokenize(&self.signature),
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
		impl alloy_sol_types::SolType for SignedDelegation {
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
		impl alloy_sol_types::SolStruct for SignedDelegation {
			const NAME: &'static str = "SignedDelegation";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"SignedDelegation(Delegation delegation,uint64 nonce,bytes32 signingId,G2Point signature)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(2);
				components.push(<Delegation as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<Delegation as alloy_sol_types::SolStruct>::eip712_components());
				components.push(<BLS::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G2Point as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<Delegation as alloy_sol_types::SolType>::eip712_data_word(&self.delegation).0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.nonce).0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.signingId,
					)
					.0,
					<BLS::G2Point as alloy_sol_types::SolType>::eip712_data_word(&self.signature).0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for SignedDelegation {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <Delegation as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.delegation)
					+ <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.nonce,
					) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.signingId,
				) + <BLS::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.signature)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<Delegation as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.delegation, out);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.nonce,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signingId,
					out,
				);
				<BLS::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.signature, out);
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
	struct InclusionProof { uint256 inclusionBlockNumber; bytes previousBlockHeaderRLP; bytes inclusionBlockHeaderRLP; bytes accountMerkleProof; bytes[] txMerkleProofs; uint256[] txIndexesInBlock; }
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
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct InclusionProof { uint256 inclusionBlockNumber; bytes previousBlockHeaderRLP; bytes inclusionBlockHeaderRLP; bytes accountMerkleProof; bytes[] txMerkleProofs; uint256[] txIndexesInBlock; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InclusionProof {
		#[allow(missing_docs)]
		pub inclusionBlockNumber: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub previousBlockHeaderRLP: alloy::sol_types::private::Bytes,
		#[allow(missing_docs)]
		pub inclusionBlockHeaderRLP: alloy::sol_types::private::Bytes,
		#[allow(missing_docs)]
		pub accountMerkleProof: alloy::sol_types::private::Bytes,
		#[allow(missing_docs)]
		pub txMerkleProofs: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
		#[allow(missing_docs)]
		pub txIndexesInBlock: alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::Uint<256>,
			alloy::sol_types::sol_data::Bytes,
			alloy::sol_types::sol_data::Bytes,
			alloy::sol_types::sol_data::Bytes,
			alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
			alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::primitives::aliases::U256,
			alloy::sol_types::private::Bytes,
			alloy::sol_types::private::Bytes,
			alloy::sol_types::private::Bytes,
			alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
			alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
		impl ::core::convert::From<InclusionProof> for UnderlyingRustTuple<'_> {
			fn from(value: InclusionProof) -> Self {
				(
					value.inclusionBlockNumber,
					value.previousBlockHeaderRLP,
					value.inclusionBlockHeaderRLP,
					value.accountMerkleProof,
					value.txMerkleProofs,
					value.txIndexesInBlock,
				)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InclusionProof {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self {
					inclusionBlockNumber: tuple.0,
					previousBlockHeaderRLP: tuple.1,
					inclusionBlockHeaderRLP: tuple.2,
					accountMerkleProof: tuple.3,
					txMerkleProofs: tuple.4,
					txIndexesInBlock: tuple.5,
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for InclusionProof {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for InclusionProof {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.inclusionBlockNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.previousBlockHeaderRLP,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.inclusionBlockHeaderRLP,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.accountMerkleProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.txMerkleProofs),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.txIndexesInBlock),
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
		impl alloy_sol_types::SolType for InclusionProof {
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
		impl alloy_sol_types::SolStruct for InclusionProof {
			const NAME: &'static str = "InclusionProof";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"InclusionProof(uint256 inclusionBlockNumber,bytes previousBlockHeaderRLP,bytes inclusionBlockHeaderRLP,bytes accountMerkleProof,bytes[] txMerkleProofs,uint256[] txIndexesInBlock)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.inclusionBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.previousBlockHeaderRLP,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.inclusionBlockHeaderRLP,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.accountMerkleProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.txMerkleProofs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.txIndexesInBlock,
                        )
                        .0,
                ]
                    .concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for InclusionProof {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inclusionBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.previousBlockHeaderRLP,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.inclusionBlockHeaderRLP,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.accountMerkleProof,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txMerkleProofs,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txIndexesInBlock,
                    )
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.inclusionBlockNumber,
					out,
				);
				<alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.previousBlockHeaderRLP,
					out,
				);
				<alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.inclusionBlockHeaderRLP,
					out,
				);
				<alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.accountMerkleProof,
					out,
				);
				<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Bytes,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.txMerkleProofs,
                    out,
                );
				<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.txIndexesInBlock,
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
	struct SignedCommitment {
		Commitment commitment;
		uint64 nonce;
		bytes32 signingId;
		bytes signature;
	}
	struct SignedDelegation {
		Delegation delegation;
		uint64 nonce;
		bytes32 signingId;
		BLS.G2Point signature;
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
	struct InclusionProof {
		uint256 inclusionBlockNumber;
		bytes previousBlockHeaderRLP;
		bytes inclusionBlockHeaderRLP;
		bytes accountMerkleProof;
		bytes[] txMerkleProofs;
		uint256[] txIndexesInBlock;
	}
}

interface InclusionPreconfSlasher {
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

	constructor(uint256 _slashAmountWei, address _urc);

	function BEACON_ROOTS_CONTRACT() external view returns (address);
	function BLOCKHASH_EVM_LOOKBACK() external view returns (uint256);
	function CHALLENGE_BOND() external view returns (uint256);
	function CHALLENGE_WINDOW() external view returns (uint256);
	function EIP4788_WINDOW() external view returns (uint256);
	function ETH2_GENESIS_TIMESTAMP() external view returns (uint256);
	function JUSTIFICATION_DELAY() external view returns (uint256);
	function SLASH_AMOUNT_WEI() external view returns (uint256);
	function SLOT_TIME() external view returns (uint256);
	function _decodeBlockHeaderRLP(bytes memory headerRLP) external pure returns (PreconfStructs.BlockHeaderData memory blockHeader);
	function _getCurrentSlot() external view returns (uint256);
	function _getSlotFromTimestamp(uint256 _timestamp) external view returns (uint256);
	function _getTimestampFromSlot(uint256 _slot) external view returns (uint256);
	function challenges(bytes32 challengeID) external view returns (address challenger, uint256 challengeTimestamp);
	function createChallenge(ISlasher.SignedCommitment memory commitment, ISlasher.SignedDelegation memory signedDelegation) external payable returns (bytes32 challengeID);
	function proveChallengeFraudulent(ISlasher.Delegation memory delegation, ISlasher.SignedCommitment memory commitment, PreconfStructs.InclusionProof memory proof) external;
	function slash(ISlasher.Delegation memory delegation, ISlasher.Commitment memory commitment, address committer, bytes memory evidence, address challenger) external returns (uint256 slashAmountWei);
	function urc() external view returns (address);
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
	  },
	  {
		"name": "_urc",
		"type": "address",
		"internalType": "address"
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
	"name": "CHALLENGE_BOND",
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
	"name": "CHALLENGE_WINDOW",
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
	"name": "challenges",
	"inputs": [
	  {
		"name": "challengeID",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [
	  {
		"name": "challenger",
		"type": "address",
		"internalType": "address"
	  },
	  {
		"name": "challengeTimestamp",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "createChallenge",
	"inputs": [
	  {
		"name": "commitment",
		"type": "tuple",
		"internalType": "struct ISlasher.SignedCommitment",
		"components": [
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
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "signature",
			"type": "bytes",
			"internalType": "bytes"
		  }
		]
	  },
	  {
		"name": "signedDelegation",
		"type": "tuple",
		"internalType": "struct ISlasher.SignedDelegation",
		"components": [
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
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  },
		  {
			"name": "signingId",
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
		  }
		]
	  }
	],
	"outputs": [
	  {
		"name": "challengeID",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"stateMutability": "payable"
  },
  {
	"type": "function",
	"name": "proveChallengeFraudulent",
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
		"internalType": "struct ISlasher.SignedCommitment",
		"components": [
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
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "signature",
			"type": "bytes",
			"internalType": "bytes"
		  }
		]
	  },
	  {
		"name": "proof",
		"type": "tuple",
		"internalType": "struct PreconfStructs.InclusionProof",
		"components": [
		  {
			"name": "inclusionBlockNumber",
			"type": "uint256",
			"internalType": "uint256"
		  },
		  {
			"name": "previousBlockHeaderRLP",
			"type": "bytes",
			"internalType": "bytes"
		  },
		  {
			"name": "inclusionBlockHeaderRLP",
			"type": "bytes",
			"internalType": "bytes"
		  },
		  {
			"name": "accountMerkleProof",
			"type": "bytes",
			"internalType": "bytes"
		  },
		  {
			"name": "txMerkleProofs",
			"type": "bytes[]",
			"internalType": "bytes[]"
		  },
		  {
			"name": "txIndexesInBlock",
			"type": "uint256[]",
			"internalType": "uint256[]"
		  }
		]
	  }
	],
	"outputs": [],
	"stateMutability": "nonpayable"
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
	"type": "function",
	"name": "urc",
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
pub mod InclusionPreconfSlasher {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	/// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f80fd5b50604051614a33380380614a33833981016040819052602b91608b565b5f829055600280546001600160a01b0319166001600160a01b0383161790554661426803605e576365156ac06001556085565b46600103607157635fc630576001556085565b466401a2140cff036085576366755d6c6001555b505060c3565b5f8060408385031215609b575f80fd5b825160208401519092506001600160a01b038116811460b8575f80fd5b809150509250929050565b614963806100d05f395ff3fe608060405260043610610105575f3560e01c80639f329d0b11610092578063cac62dd911610062578063cac62dd91461031f578063ccfa336f14610334578063d62aad2914610347578063f45e81181461035c578063f5beea8c14610371575f80fd5b80639f329d0b1461026f578063abeeb36614610284578063c1e69b66146102a3578063c239ef4214610300575f80fd5b80635ddc9e8d116100d85780635ddc9e8d146101c25780636ef8d437146101f9578063743770291461021a57806386fb61ed1461022e5780639790ce7114610254575f80fd5b8063189cc9d0146101095780632a04ff85146101305780634968f6c51461014457806356b4a92a14610158575b5f80fd5b348015610114575f80fd5b5061011d610390565b6040519081526020015b60405180910390f35b34801561013b575f80fd5b5061011d600c81565b34801561014f575f80fd5b5061011d5f5481565b348015610163575f80fd5b50610177610172366004613ed4565b61039f565b60405161012791905f60c082019050825182526020830151602083015260408301516040830152606083015160608301526080830151608083015260a083015160a083015292915050565b3480156101cd575f80fd5b506002546101e1906001600160a01b031681565b6040516001600160a01b039091168152602001610127565b348015610204575f80fd5b50610218610213366004613f34565b6104b0565b005b348015610225575f80fd5b5061011d602081565b348015610239575f80fd5b506101e1720f3df6d732807ef1319fb7b8bb8522d0beac0281565b34801561025f575f80fd5b5061011d670de0b6b3a764000081565b34801561027a575f80fd5b5061011d60015481565b34801561028f575f80fd5b5061011d61029e366004613fbb565b610610565b3480156102ae575f80fd5b506102e16102bd366004613fbb565b60036020525f9081526040902080546001909101546001600160a01b039091169082565b604080516001600160a01b039093168352602083019190915201610127565b34801561030b575f80fd5b5061011d61031a366004613fe8565b610631565b34801561032a575f80fd5b5061011d611fff81565b61011d6103423660046140b9565b6107a7565b348015610352575f80fd5b5061011d611c2081565b348015610367575f80fd5b5061011d61010081565b34801561037c575f80fd5b5061011d61038b366004613fbb565b610907565b5f61039a42610610565b905090565b6040805160c0810182525f80825260208201819052918101829052606081018290526080810182905260a08101829052906103e16103dc84610920565b61094c565b9050610405815f815181106103f8576103f8614118565b6020026020010151610b37565b8252805161042090829060039081106103f8576103f8614118565b6020830152805161043e90829060049081106103f8576103f8614118565b60408301528051610469908290600890811061045c5761045c614118565b6020026020010151610c34565b60608301528051610487908290600b90811061045c5761045c614118565b608083015280516104a5908290600f90811061045c5761045c614118565b60a083015250919050565b5f6104bb838061412c565b846040516020016104cd9291906141d0565b60408051601f1981840301815282825280516020918201205f8181526003835283902084840190935282546001600160a01b031680855260019093015491840191909152925061052f57604051623126e360e41b815260040160405180910390fd5b5f61053a858061412c565b6105489060208101906142f9565b810190610555919061433b565b905061057b81610564866144f4565b6105766101208a016101008b016145d2565b610c3e565b5f8381526003602052604080822080546001600160a01b0319168155600101829055513390670de0b6b3a7640000908381818185875af1925050503d805f81146105e0576040519150601f19603f3d011682016040523d82523d5f602084013e6105e5565b606091505b505090508061060757604051630db2c7f160e31b815260040160405180910390fd5b50505050505050565b5f600c6001548361062191906145ff565b61062b9190614626565b92915050565b6002545f906001600160a01b0316331461065e576040516363e58f5760e11b815260040160405180910390fd5b5f86886040516020016106729291906141d0565b60408051808303601f1901815291815281516020928301205f81815260039093529120549091506001600160a01b038481169116146106c457604051637546e8c160e01b815260040160405180910390fd5b5f8181526003602052604090206001015442906106e490611c2090614639565b111561070357604051631f304cc160e01b815260040160405180910390fd5b5f8181526003602052604080822080546001600160a01b0319168155600101829055516001600160a01b03851690670de0b6b3a7640000908381818185875af1925050503d805f8114610771576040519150601f19603f3d011682016040523d82523d5f602084013e610776565b606091505b505090508061079857604051630db2c7f160e31b815260040160405180910390fd5b50505f54979650505050505050565b5f670de0b6b3a764000034146107d057604051634dc660a360e11b815260040160405180910390fd5b5f6107db848061412c565b6107e99060208101906142f9565b8101906107f6919061433b565b9050610802848061412c565b61080c848061464c565b60405160200161081d9291906141d0565b60408051601f1981840301815291815281516020928301205f81815260039093529120549092506001600160a01b03161561086b57604051634450435760e01b815260040160405180910390fd5b80516001600160401b0316610880848061464c565b6108939061014081019061012001614661565b6001600160401b0316146108ba576040516330d3ba0760e01b815260040160405180910390fd5b506040805180820182523381524260208083019182525f858152600390915292909220905181546001600160a01b0319166001600160a01b03909116178155905160019091015592915050565b5f610913600c8361467a565b60015461062b9190614639565b6040805180820182525f8082526020918201528151808301909252825182529182019181019190915290565b60605f8061095984610e5c565b9193509091506001905081600181111561097557610975614691565b146109c75760405162461bcd60e51b815260206004820152601760248201527f496e76616c696420524c50206c6973742076616c75652e00000000000000000060448201526064015b60405180910390fd5b60408051602080825261042082019092525f91816020015b604080518082019091525f80825260208201528152602001906001900390816109df5790505090505f835b8651811015610b2c5760208210610a765760405162461bcd60e51b815260206004820152602a60248201527f50726f766964656420524c50206c6973742065786365656473206d6178206c6960448201526939ba103632b733ba341760b11b60648201526084016109be565b5f80610ab16040518060400160405280858c5f0151610a9591906145ff565b8152602001858c60200151610aaa9190614639565b9052610e5c565b509150915060405180604001604052808383610acd9190614639565b8152602001848b60200151610ae29190614639565b815250858581518110610af757610af7614118565b6020908102919091010152610b0d600185614639565b9350610b198183614639565b610b239084614639565b92505050610a0a565b508152949350505050565b5f6021825f01511115610b8c5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020627974657333322076616c75652e00000000000060448201526064016109be565b5f805f610b9885610e5c565b919450925090505f816001811115610bb257610bb2614691565b14610bff5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020627974657333322076616c75652e00000000000060448201526064016109be565b5f838660200151610c109190614639565b80519091506020841015610c2a5760208490036101000a90045b9695505050505050565b5f61062b82610b37565b82516001600160401b03166020610c53610390565b610c5d91906145ff565b811115610c7d5760405163b6144bff60e01b815260040160405180910390fd5b610100610c88610390565b610c9291906145ff565b811015610cb257604051630cdceb7960e21b815260040160405180910390fd5b82515f90610cc2906001906145ff565b905043811180610cdc5750610cd9610100436145ff565b81105b15610cfa57604051631391e11b60e21b815260040160405180910390fd5b83515f90610d0a906001906145ff565b602080870151805191012090409150808214610d3957604051631f03465b60e11b815260040160405180910390fd5b5f80610d4489611156565b9250925050816001600160a01b0316876001600160a01b031614610d7b5760405163aaaa914160e01b815260040160405180910390fd5b5f610d89896040015161039f565b80519091508414610dad576040516320fa6c8b60e11b815260040160405180910390fd5b5f610dd48a60a001515f81518110610dc757610dc7614118565b60200260200101516111de565b90505f80610e04838d608001515f81518110610df257610df2614118565b602002602001015186604001516111f1565b9150915081610e265760405163094cec5f60e01b815260040160405180910390fd5b80516020820120855114610e4d57604051634462b38f60e11b815260040160405180910390fd5b50505050505050505050505050565b5f805f80845f015111610eb15760405162461bcd60e51b815260206004820152601860248201527f524c50206974656d2063616e6e6f74206265206e756c6c2e000000000000000060448201526064016109be565b602084015180515f1a607f8111610ed3575f60015f945094509450505061114f565b60b78111610f42578551607f198201908110610f315760405162461bcd60e51b815260206004820152601960248201527f496e76616c696420524c502073686f727420737472696e672e0000000000000060448201526064016109be565b6001955093505f925061114f915050565b60bf811161101957855160b6198201908110610fa05760405162461bcd60e51b815260206004820152601f60248201527f496e76616c696420524c50206c6f6e6720737472696e67206c656e6774682e0060448201526064016109be565b5f816020036101000a6001850151049050808201885f0151116110055760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420524c50206c6f6e6720737472696e672e000000000000000060448201526064016109be565b6001909101955093505f925061114f915050565b60f7811161108857855160bf1982019081106110775760405162461bcd60e51b815260206004820152601760248201527f496e76616c696420524c502073686f7274206c6973742e00000000000000000060448201526064016109be565b60019550935084925061114f915050565b855160f61982019081106110de5760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420524c50206c6f6e67206c697374206c656e6774682e00000060448201526064016109be565b5f816020036101000a6001850151049050808201885f01511161113c5760405162461bcd60e51b815260206004820152601660248201527524b73b30b634b210292628103637b733903634b9ba1760511b60448201526064016109be565b600191820196509450925061114f915050565b9193909250565b604080516060810182525f80825260208201819052918101829052819061118961117f856112c6565b856020015161130d565b91505f6111998560400151611335565b90506111a4816113ef565b935060405180606001604052808660400151805190602001208152602001826060015181526020018260e001518152509150509193909250565b606061062b6111ec8361140a565b611527565b5f60605f6111fe85611595565b90505f805f61120e848a89611680565b815192955090935091501580806112225750815b61126e5760405162461bcd60e51b815260206004820152601a60248201527f50726f76696465642070726f6f6620697320696e76616c69642e00000000000060448201526064016109be565b5f816112885760405180602001604052805f8152506112b4565b6112b4866112976001886145ff565b815181106112a7576112a7614118565b6020026020010151611a85565b919b919a509098505050505050505050565b5f8160400151805190602001206112df835f0151611aaf565b6040516020016112f09291906146bc565b604051602081830303815290604052805190602001209050919050565b5f805f8061131b8686611b2f565b92509250925061132b8282611b78565b5090949350505050565b61133d613d6e565b5f825f8151811061135057611350614118565b01602001516001600160f81b0319169050607f60f81b811061137c5761137583611c34565b9392505050565b6001600160f81b03198116600160f81b0361139a5761137583611e6a565b6001600160f81b03198116600160f91b036113b8576113758361212a565b6001600160f81b03198116600360f81b036113d657611375836123af565b604051636fc3daa360e11b815260040160405180910390fd5b5f61062b6113fc8361273c565b61140584612754565b61130d565b60605f8260405160200161142091815260200190565b60405160208183030381529060405290505f5b602081101561146b5781818151811061144e5761144e614118565b01602001516001600160f81b0319165f0361146b57600101611433565b5f6114778260206145ff565b6001600160401b0381111561148e5761148e613dfd565b6040519080825280601f01601f1916602001820160405280156114b8576020820181803683370190505b5090505f5b815181101561151e5783836114d1816146cd565b9450815181106114e3576114e3614118565b602001015160f81c60f81b82828151811061150057611500614118565b60200101906001600160f81b03191690815f1a9053506001016114bd565b50949350505050565b6060808251600114801561155457506080835f8151811061154a5761154a614118565b016020015160f81c105b1561156057508161062b565b61156c835160806127b0565b8360405160200161157e9291906146e5565b604051602081830303815290604052905092915050565b60605f6115a183612957565b90505f81516001600160401b038111156115bd576115bd613dfd565b60405190808252806020026020018201604052801561160257816020015b60408051808201909152606080825260208201528152602001906001900390816115db5790505b5090505f5b8251811015611678575f61163384838151811061162657611626614118565b6020026020010151612965565b9050604051806040016040528082815260200161164f83612957565b81525083838151811061166457611664614118565b602090810291909101015250600101611607565b509392505050565b5f606081808061168f876129f2565b90505f8690505f806116b4604051806040016040528060608152602001606081525090565b5f5b8c51811015611a5d578c81815181106116d1576116d1614118565b6020026020010151915082846116e79190614639565b93506116f4600188614639565b9650835f0361174e578151805160209091012085146117495760405162461bcd60e51b8152602060048201526011602482015270092dcecc2d8d2c840e4dedee840d0c2e6d607b1b60448201526064016109be565b61180a565b8151516020116117b0578151805160209091012085146117495760405162461bcd60e51b815260206004820152601b60248201527f496e76616c6964206c6172676520696e7465726e616c2068617368000000000060448201526064016109be565b846117bd835f0151612b02565b1461180a5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420696e7465726e616c206e6f6465206861736800000000000060448201526064016109be565b61181660106001614639565b826020015151036118865785518414611a5d575f86858151811061183c5761183c614118565b602001015160f81c60f81b60f81c90505f83602001518260ff168151811061186657611866614118565b6020026020010151905061187981612b29565b9650600194505050611a55565b600282602001515103611a0d575f61189d83612b5d565b90505f815f815181106118b2576118b2614118565b016020015160f81c90505f6118c86002836146f9565b6118d390600261471a565b90505f6118e3848360ff16612b80565b90505f6118f08b8a612b80565b90505f6118fd8383612bae565b905060ff851660021480611914575060ff85166003145b1561194e578083511480156119295750808251145b1561193b57611938818b614639565b99505b50600160ff1b9950611a5d945050505050565b60ff85161580611961575060ff85166001145b156119b657805f036119805750600160ff1b9950611a5d945050505050565b6119a7886020015160018151811061199a5761199a614118565b6020026020010151612b29565b9a509750611a55945050505050565b60405162461bcd60e51b815260206004820152602660248201527f52656365697665642061206e6f6465207769746820616e20756e6b6e6f776e206044820152650e0e4caccd2f60d31b60648201526084016109be565b60405162461bcd60e51b815260206004820152601c60248201527f526563656976656420616e20756e7061727361626c65206e6f64652e0000000060448201526064016109be565b6001016116b6565b50600160ff1b841486611a708786612b80565b909e909d50909b509950505050505050505050565b6020810151805160609161062b91611a9f906001906145ff565b8151811061162657611626614118565b6040805160088082528183019092526060915f91906020820181803683370190505090505f5b6008811015611b2857611ae981600861467a565b846001600160401b0316901c60f81b828281518110611b0a57611b0a614118565b60200101906001600160f81b03191690815f1a905350600101611ad5565b5092915050565b5f805f8351604103611b66576020840151604085015160608601515f1a611b5888828585612c29565b955095509550505050611b71565b505081515f91506002905b9250925092565b5f826003811115611b8b57611b8b614691565b03611b94575050565b6001826003811115611ba857611ba8614691565b03611bc65760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115611bda57611bda614691565b03611bfb5760405163fce698f760e01b8152600481018290526024016109be565b6003826003811115611c0f57611c0f614691565b03611c30576040516335e2f38360e21b8152600481018290526024016109be565b5050565b611c3c613d6e565b5f808252611c4c6103dc84610920565b90508051600914158015611c6257508051600614155b15611c805760405163c2871a3760e01b815260040160405180910390fd5b611c95815f8151811061045c5761045c614118565b60608301528051611cb3908290600190811061045c5761045c614118565b60808301528051611cd1908290600290811061045c5761045c614118565b60e08301528051611cfc9082906003908110611cef57611cef614118565b6020026020010151612cf1565b6001600160a01b03166101008301528051611d24908290600490811061045c5761045c614118565b6101208301528051611d43908290600590811061162657611626614118565b6101408301528051600603611d585750919050565b5f611d6f8260068151811061045c5761045c614118565b90505f611d888360078151811061045c5761045c614118565b90505f611da18460088151811061045c5761045c614118565b905081158015611daf575080155b15611dcf576001600160401b038316602086015260016040860152611e61565b6023836001600160401b031610611e17576002611ded602385614733565b611df79190614753565b6001600160401b03908116602087015283166101e0860152600160408601525b5f611e26600260018618614778565b611e3190601b61479d565b604051909150611e49908490849084906020016147bd565b60408051601f198184030181529190526101c0870152505b50505050919050565b611e72613d6e565b600180825282515f91611e9291611e8a9082906145ff565b859190612d5d565b90505f611ea16103dc83610920565b90508051600814158015611eb757508051600b14155b15611ed55760405163c2871a3760e01b815260040160405180910390fd5b611eea815f8151811061045c5761045c614118565b6001600160401b031660208401528051611f11908290600190811061045c5761045c614118565b60608401528051611f2f908290600290811061045c5761045c614118565b60808401528051611f4d908290600390811061045c5761045c614118565b60e08401528051611f6b9082906004908110611cef57611cef614118565b6001600160a01b03166101008401528051611f93908290600590811061045c5761045c614118565b6101208401528051611fb2908290600690811061162657611626614118565b8361014001819052505f611fdf82600781518110611fd257611fd2614118565b602002602001015161094c565b905080516001600160401b03811115611ffa57611ffa613dfd565b60405190808252806020026020018201604052801561202d57816020015b60608152602001906001900390816120185790505b506101608501525f5b815181101561208b5761206182828151811061205457612054614118565b6020026020010151612e9d565b856101600151828151811061207857612078614118565b6020908102919091010152600101612036565b50815160080361209d57505050919050565b5f6120b48360088151811061045c5761045c614118565b6120bf90601b6147e0565b90505f6120d8846009815181106103f8576103f8614118565b90505f6120f185600a815181106103f8576103f8614118565b9050818184604051602001612108939291906147bd565b60408051601f198184030181529190526101c088015250949695505050505050565b612132613d6e565b6002815281515f9061214c90600190611e8a9082906145ff565b90505f61215b6103dc83610920565b9050805160091415801561217157508051600c14155b1561218f5760405163c2871a3760e01b815260040160405180910390fd5b6121a4815f8151811061045c5761045c614118565b6001600160401b0316602084015280516121cb908290600190811061045c5761045c614118565b606084015280516121e9908290600290811061045c5761045c614118565b60a08401528051612207908290600390811061045c5761045c614118565b60c08401528051612225908290600490811061045c5761045c614118565b60e084015280516122439082906005908110611cef57611cef614118565b6001600160a01b0316610100840152805161226b908290600690811061045c5761045c614118565b610120840152805161228a908290600790811061162657611626614118565b8361014001819052505f6122aa82600881518110611fd257611fd2614118565b905080516001600160401b038111156122c5576122c5613dfd565b6040519080825280602002602001820160405280156122f857816020015b60608152602001906001900390816122e35790505b506101608501525f5b81518110156123495761231f82828151811061205457612054614118565b856101600151828151811061233657612336614118565b6020908102919091010152600101612301565b50815160090361235b57505050919050565b5f6123728360098151811061045c5761045c614118565b61237d90601b6147e0565b90505f61239684600a815181106103f8576103f8614118565b90505f6120f185600b815181106103f8576103f8614118565b6123b7613d6e565b6003815281515f906123d190600190611e8a9082906145ff565b90505f6123e06103dc83610920565b90508051600b141580156123f657508051600e14155b156124145760405163c2871a3760e01b815260040160405180910390fd5b612429815f8151811061045c5761045c614118565b6001600160401b031660208401528051612450908290600190811061045c5761045c614118565b6060840152805161246e908290600290811061045c5761045c614118565b60a0840152805161248c908290600390811061045c5761045c614118565b60c084015280516124aa908290600490811061045c5761045c614118565b60e084015280516124c89082906005908110611cef57611cef614118565b6001600160a01b031661010084015280516124f0908290600690811061045c5761045c614118565b610120840152805161250f908290600790811061162657611626614118565b8361014001819052505f61252f82600881518110611fd257611fd2614118565b905080516001600160401b0381111561254a5761254a613dfd565b60405190808252806020026020018201604052801561257d57816020015b60608152602001906001900390816125685790505b506101608501525f5b81518110156125ce576125a482828151811061205457612054614118565b85610160015182815181106125bb576125bb614118565b6020908102919091010152600101612586565b506125e58260098151811061045c5761045c614118565b846101800181815250505f61260683600a81518110611fd257611fd2614118565b905080516001600160401b0381111561262157612621613dfd565b60405190808252806020026020018201604052801561264a578160200160208202803683370190505b506101a08601525f5b815181101561269b576126718282815181106103f8576103f8614118565b866101a00151828151811061268857612688614118565b6020908102919091010152600101612653565b508251600b036126ae5750505050919050565b5f6126c584600b8151811061045c5761045c614118565b6126d090601b6147e0565b90505f6126e985600c815181106103f8576103f8614118565b90505f61270286600d815181106103f8576103f8614118565b9050818184604051602001612719939291906147bd565b60408051601f198184030181529190526101c08901525095979650505050505050565b5f61274682612ea8565b805190602001209050919050565b6060816101c00151515f0361277c5760405163d466bd1560e01b815260040160405180910390fd5b816101c00151516041146127a357604051634be6321b60e01b815260040160405180910390fd5b506101c08101515b919050565b606080603884101561281557604080516001808252818301909252906020820181803683370190505090506127e583856147e0565b60f81b815f815181106127fa576127fa614118565b60200101906001600160f81b03191690815f1a905350611375565b5f60015b6128238187614626565b156128495781612832816146cd565b925061284290506101008261467a565b9050612819565b612854826001614639565b6001600160401b0381111561286b5761286b613dfd565b6040519080825280601f01601f191660200182016040528015612895576020820181803683370190505b5092506128a285836147e0565b6128ad9060376147e0565b60f81b835f815181106128c2576128c2614118565b60200101906001600160f81b03191690815f1a905350600190505b81811161294e576101006128f182846145ff565b6128fd906101006148d9565b6129079088614626565b61291191906148e4565b60f81b83828151811061292657612926614118565b60200101906001600160f81b03191690815f1a90535080612946816146cd565b9150506128dd565b50509392505050565b606061062b6103dc83610920565b60605f805f61297385610e5c565b919450925090505f81600181111561298d5761298d614691565b146129da5760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420524c502062797465732076616c75652e000000000000000060448201526064016109be565b6129e985602001518484612f35565b95945050505050565b60605f82516002026001600160401b03811115612a1157612a11613dfd565b6040519080825280601f01601f191660200182016040528015612a3b576020820181803683370190505b5090505f5b8351811015611b28576004848281518110612a5d57612a5d614118565b602001015160f81c60f81b6001600160f81b031916901c828260020281518110612a8957612a89614118565b60200101906001600160f81b03191690815f1a9053506010848281518110612ab357612ab3614118565b016020015160f81c81612ac857612ac8614612565b0660f81b828260020260010181518110612ae457612ae4614118565b60200101906001600160f81b03191690815f1a905350600101612a40565b5f602082511015612b1557506020015190565b8180602001905181019061062b91906148f7565b5f60606020835f01511015612b4857612b4183612e9d565b9050612b54565b612b5183612965565b90505b61137581612b02565b606061062b612b7b83602001515f8151811061162657611626614118565b6129f2565b6060818351035f03612ba0575060408051602081019091525f815261062b565b611375838384865103612d5d565b5f805b808451118015612bc15750808351115b8015612c125750828181518110612bda57612bda614118565b602001015160f81c60f81b6001600160f81b031916848281518110612c0157612c01614118565b01602001516001600160f81b031916145b156113755780612c21816146cd565b915050612bb1565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841115612c6257505f91506003905082612ce7565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015612cb3573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116612cde57505f925060019150829050612ce7565b92505f91508190505b9450945094915050565b80515f90600103612d0357505f919050565b8151601514612d545760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020616464726573732076616c75652e00000000000060448201526064016109be565b61062b82610c34565b60608182601f011015612da35760405162461bcd60e51b815260206004820152600e60248201526d736c6963655f6f766572666c6f7760901b60448201526064016109be565b828284011015612de65760405162461bcd60e51b815260206004820152600e60248201526d736c6963655f6f766572666c6f7760901b60448201526064016109be565b81830184511015612e2d5760405162461bcd60e51b8152602060048201526011602482015270736c6963655f6f75744f66426f756e647360781b60448201526064016109be565b606082158015612e4b5760405191505f82526020820160405261151e565b6040519150601f8416801560200281840101858101878315602002848b0101015b81831015612e84578051835260209283019201612e6c565b5050858452601f01601f19166040525050949350505050565b606061062b82612fe2565b60605f82516003811115612ebe57612ebe614691565b03612ecc5761062b82612ff6565b600182516003811115612ee157612ee1614691565b03612eef5761062b826132e2565b600282516003811115612f0457612f04614691565b03612f125761062b82613553565b600382516003811115612f2757612f27614691565b036113d65761062b826137c1565b60605f826001600160401b03811115612f5057612f50613dfd565b6040519080825280601f01601f191660200182016040528015612f7a576020820181803683370190505b50905080515f03612f8c579050611375565b848401602082015f5b60208604811015612fb6578251825260209283019290910190600101612f95565b505f6001602087066020036101000a039050808251168119845116178252839450505050509392505050565b606061062b82602001515f845f0151612f35565b60605f82602001516001600160401b03165f146130185750602082015161305b565b6101c0830151511561305b576023836101e001516001600160401b03161061305b5760026023846101e0015161304e9190614733565b6130589190614753565b90505b5f836040015161306b575f61306e565b60035b6130799060066147e0565b60ff1690505f816001600160401b0381111561309757613097613dfd565b6040519080825280602002602001820160405280156130ca57816020015b60608152602001906001900390816130b55790505b5090506130da85606001516111de565b815f815181106130ec576130ec614118565b602002602001018190525061310485608001516111de565b8160018151811061311757613117614118565b602002602001018190525061312f8560e001516111de565b8160028151811061314257613142614118565b602002602001018190525061315b856101000151613b64565b8160038151811061316e5761316e614118565b60200260200101819052506131878561012001516111de565b8160048151811061319a5761319a614118565b60200260200101819052506131b3856101400151611527565b816005815181106131c6576131c6614118565b60200260200101819052508460400151156132d95784602001516001600160401b03165f0361322d576040515f60208201526021016040516020818303038152906040528160068151811061321d5761321d614118565b602002602001018190525061325e565b61323f836001600160401b03166111de565b8160068151811061325257613252614118565b60200260200101819052505b6132935f5b6040519080825280601f01601f19166020018201604052801561328d576020820181803683370190505b50611527565b816007815181106132a6576132a6614118565b60209081029190910101526132ba5f613263565b816008815181106132cd576132cd614118565b60200260200101819052505b6129e981613b9c565b60408051600880825261012082019092526060915f9190816020015b60608152602001906001900390816132fe57905050905061332b83602001516001600160401b03166111de565b815f8151811061333d5761333d614118565b602002602001018190525061335583606001516111de565b8160018151811061336857613368614118565b602002602001018190525061338083608001516111de565b8160028151811061339357613393614118565b60200260200101819052506133ab8360e001516111de565b816003815181106133be576133be614118565b60200260200101819052506133d7836101000151613b64565b816004815181106133ea576133ea614118565b60200260200101819052506134038361012001516111de565b8160058151811061341657613416614118565b602002602001018190525061342f836101400151611527565b8160068151811061344257613442614118565b60200260200101819052505f836101600151516001600160401b0381111561346c5761346c613dfd565b60405190808252806020026020018201604052801561349f57816020015b606081526020019060019003908161348a5790505b5090505f5b846101600151518110156134f65784610160015181815181106134c9576134c9614118565b60200260200101518282815181106134e3576134e3614118565b60209081029190910101526001016134a4565b5061350081613b9c565b8260078151811061351357613513614118565b602090810291909101015260015b61352a83613b9c565b60405160200161353b92919061490e565b60405160208183030381529060405292505050919050565b60408051600980825261014082019092526060915f9190816020015b606081526020019060019003908161356f57905050905061359c83602001516001600160401b03166111de565b815f815181106135ae576135ae614118565b60200260200101819052506135c683606001516111de565b816001815181106135d9576135d9614118565b60200260200101819052506135f18360a001516111de565b8160028151811061360457613604614118565b602002602001018190525061361c8360c001516111de565b8160038151811061362f5761362f614118565b60200260200101819052506136478360e001516111de565b8160048151811061365a5761365a614118565b6020026020010181905250613673836101000151613b64565b8160058151811061368657613686614118565b602002602001018190525061369f8361012001516111de565b816006815181106136b2576136b2614118565b60200260200101819052506136cb836101400151611527565b816007815181106136de576136de614118565b60200260200101819052505f836101600151516001600160401b0381111561370857613708613dfd565b60405190808252806020026020018201604052801561373b57816020015b60608152602001906001900390816137265790505b5090505f5b8461016001515181101561379257846101600151818151811061376557613765614118565b602002602001015182828151811061377f5761377f614118565b6020908102919091010152600101613740565b5061379c81613b9c565b826008815181106137af576137af614118565b60209081029190910101526002613521565b60408051600b80825261018082019092526060915f9190816020015b60608152602001906001900390816137dd57905050905061380a83602001516001600160401b03166111de565b815f8151811061381c5761381c614118565b602002602001018190525061383483606001516111de565b8160018151811061384757613847614118565b602002602001018190525061385f8360a001516111de565b8160028151811061387257613872614118565b602002602001018190525061388a8360c001516111de565b8160038151811061389d5761389d614118565b60200260200101819052506138b58360e001516111de565b816004815181106138c8576138c8614118565b60200260200101819052506138e1836101000151613b64565b816005815181106138f4576138f4614118565b602002602001018190525061390d8361012001516111de565b8160068151811061392057613920614118565b6020026020010181905250613939836101400151611527565b8160078151811061394c5761394c614118565b60200260200101819052505f836101600151516001600160401b0381111561397657613976613dfd565b6040519080825280602002602001820160405280156139a957816020015b60608152602001906001900390816139945790505b5090505f5b84610160015151811015613a005784610160015181815181106139d3576139d3614118565b60200260200101518282815181106139ed576139ed614118565b60209081029190910101526001016139ae565b50613a0a81613b9c565b82600881518110613a1d57613a1d614118565b6020026020010181905250613a368461018001516111de565b82600981518110613a4957613a49614118565b60200260200101819052505f846101a00151516001600160401b03811115613a7357613a73613dfd565b604051908082528060200260200182016040528015613aa657816020015b6060815260200190600190039081613a915790505b5090505f5b856101a0015151811015613b0757613ae2866101a001518281518110613ad357613ad3614118565b60200260200101515f1c6111de565b828281518110613af457613af4614118565b6020908102919091010152600101613aab565b50613b1181613b9c565b83600a81518110613b2457613b24614118565b60209081029190910101526003613b3a84613b9c565b604051602001613b4b92919061490e565b6040516020818303038152906040529350505050919050565b604051606082811b6bffffffffffffffffffffffff191660208301529061062b90603401604051602081830303815290604052611527565b60605f613ba883613bdf565b9050613bb6815160c06127b0565b81604051602001613bc89291906146e5565b604051602081830303815290604052915050919050565b606081515f03613bfe57604080515f8082526020820190925290611b28565b5f805b8351811015613c3a57838181518110613c1c57613c1c614118565b60200260200101515182613c309190614639565b9150600101613c01565b5f826001600160401b03811115613c5357613c53613dfd565b6040519080825280601f01601f191660200182016040528015613c7d576020820181803683370190505b505f92509050602081015b855183101561151e575f868481518110613ca457613ca4614118565b602002602001015190505f602082019050613cc183828451613cf7565b878581518110613cd357613cd3614118565b60200260200101515183613ce79190614639565b6001909501949250613c88915050565b8282825b60208110613d335781518352613d12602084614639565b9250613d1f602083614639565b9150613d2c6020826145ff565b9050613cfb565b5f6001613d418360206145ff565b613d4d906101006148d9565b613d5791906145ff565b925184518416931916929092179092525050505050565b604080516102008101909152805f81526020015f6001600160401b031681526020015f151581526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f815260200160608152602001606081526020015f815260200160608152602001606081526020015f6001600160401b031681525090565b634e487b7160e01b5f52604160045260245ffd5b60405160c081016001600160401b0381118282101715613e3357613e33613dfd565b60405290565b604051601f8201601f191681016001600160401b0381118282101715613e6157613e61613dfd565b604052919050565b5f82601f830112613e78575f80fd5b81356001600160401b03811115613e9157613e91613dfd565b613ea4601f8201601f1916602001613e39565b818152846020838601011115613eb8575f80fd5b816020850160208301375f918101602001919091529392505050565b5f60208284031215613ee4575f80fd5b81356001600160401b03811115613ef9575f80fd5b613f0584828501613e69565b949350505050565b5f6101608284031215613f1e575f80fd5b50919050565b5f60808284031215613f1e575f80fd5b5f805f60608486031215613f46575f80fd5b83356001600160401b0380821115613f5c575f80fd5b613f6887838801613f0d565b94506020860135915080821115613f7d575f80fd5b613f8987838801613f24565b93506040860135915080821115613f9e575f80fd5b50840160c08187031215613fb0575f80fd5b809150509250925092565b5f60208284031215613fcb575f80fd5b5035919050565b80356001600160a01b03811681146127ab575f80fd5b5f805f805f8060a08789031215613ffd575f80fd5b86356001600160401b0380821115614013575f80fd5b61401f8a838b01613f0d565b97506020890135915080821115614034575f80fd5b6140408a838b01613f24565b965061404e60408a01613fd2565b95506060890135915080821115614063575f80fd5b818901915089601f830112614076575f80fd5b813581811115614084575f80fd5b8a6020828501011115614095575f80fd5b6020830195508094505050506140ad60808801613fd2565b90509295509295509295565b5f80604083850312156140ca575f80fd5b82356001600160401b03808211156140e0575f80fd5b6140ec86838701613f24565b93506020850135915080821115614101575f80fd5b5061410e85828601613f0d565b9150509250929050565b634e487b7160e01b5f52603260045260245ffd5b5f8235607e19833603018112614140575f80fd5b9190910192915050565b80356001600160401b03811681146127ab575f80fd5b5f808335601e19843603018112614175575f80fd5b83016020810192503590506001600160401b03811115614193575f80fd5b8036038213156141a1575f80fd5b9250929050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b604081525f6001600160401b03806141e78661414a565b1660408401526141fa6020860186614160565b6080606086015261420f60c0860182846141a8565b9150506040860135608085015261422860608701613fd2565b6001600160a01b0390811660a086015284820360208601526101609061426f8388803582526020810135602083015260408101356040830152606081013560608301525050565b6142a06080840160808901803582526020810135602083015260408101356040830152606081013560608301525050565b610100816142af828a01613fd2565b169084015250610120836142c488830161414a565b169083015261014092506142da86840187614160565b82858501526142ec83850182846141a8565b9998505050505050505050565b5f808335601e1984360301811261430e575f80fd5b8301803591506001600160401b03821115614327575f80fd5b6020019150368190038213156141a1575f80fd5b5f6020828403121561434b575f80fd5b81356001600160401b0380821115614361575f80fd5b9083019060608286031215614374575f80fd5b60405160608101818110838211171561438f5761438f613dfd565b60405261439b8361414a565b81526020830135828111156143ae575f80fd5b6143ba87828601613e69565b6020830152506040830135828111156143d1575f80fd5b6143dd87828601613e69565b60408301525095945050505050565b5f6001600160401b0382111561440457614404613dfd565b5060051b60200190565b5f82601f83011261441d575f80fd5b8135602061443261442d836143ec565b613e39565b82815260059290921b84018101918181019086841115614450575f80fd5b8286015b8481101561448d5780356001600160401b03811115614471575f80fd5b61447f8986838b0101613e69565b845250918301918301614454565b509695505050505050565b5f82601f8301126144a7575f80fd5b813560206144b761442d836143ec565b8083825260208201915060208460051b8701019350868411156144d8575f80fd5b602086015b8481101561448d57803583529183019183016144dd565b5f60c08236031215614504575f80fd5b61450c613e11565b8235815260208301356001600160401b0380821115614529575f80fd5b61453536838701613e69565b6020840152604085013591508082111561454d575f80fd5b61455936838701613e69565b60408401526060850135915080821115614571575f80fd5b61457d36838701613e69565b60608401526080850135915080821115614595575f80fd5b6145a13683870161440e565b608084015260a08501359150808211156145b9575f80fd5b506145c636828601614498565b60a08301525092915050565b5f602082840312156145e2575f80fd5b61137582613fd2565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561062b5761062b6145eb565b634e487b7160e01b5f52601260045260245ffd5b5f8261463457614634614612565b500490565b8082018082111561062b5761062b6145eb565b5f823561015e19833603018112614140575f80fd5b5f60208284031215614671575f80fd5b6113758261414a565b808202811582820484141761062b5761062b6145eb565b634e487b7160e01b5f52602160045260245ffd5b5f81518060208401855e5f93019283525090919050565b8281525f613f0560208301846146a5565b5f600182016146de576146de6145eb565b5060010190565b5f613f056146f383866146a5565b846146a5565b5f60ff83168061470b5761470b614612565b8060ff84160691505092915050565b60ff828116828216039081111561062b5761062b6145eb565b6001600160401b03828116828216039080821115611b2857611b286145eb565b5f6001600160401b038084168061476c5761476c614612565b92169190910492915050565b5f6001600160401b038084168061479157614791614612565b92169190910692915050565b6001600160401b03818116838216019080821115611b2857611b286145eb565b928352602083019190915260f81b6001600160f81b031916604082015260410190565b60ff818116838216019081111561062b5761062b6145eb565b600181815b8085111561483357815f1904821115614819576148196145eb565b8085161561482657918102915b93841c93908002906147fe565b509250929050565b5f826148495750600161062b565b8161485557505f61062b565b816001811461486b576002811461487557614891565b600191505061062b565b60ff841115614886576148866145eb565b50506001821b61062b565b5060208310610133831016604e8410600b84101617156148b4575081810a61062b565b6148be83836147f9565b805f19048211156148d1576148d16145eb565b029392505050565b5f611375838361483b565b5f826148f2576148f2614612565b500690565b5f60208284031215614907575f80fd5b5051919050565b60f883901b6001600160f81b03191681525f613f0560018301846146a556fea264697066735822122003e45a207dc39dcacb2588346c2027a2e06a6d8abda1c7d4874e308e0181c64d64736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW_\x80\xFD[P`@QaJ38\x03\x80aJ3\x839\x81\x01`@\x81\x90R`+\x91`\x8BV[_\x82\x90U`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UFaBh\x03`^Wce\x15j\xC0`\x01U`\x85V[F`\x01\x03`qWc_\xC60W`\x01U`\x85V[Fd\x01\xA2\x14\x0C\xFF\x03`\x85Wcfu]l`\x01U[PP`\xC3V[_\x80`@\x83\x85\x03\x12\x15`\x9BW_\x80\xFD[\x82Q` \x84\x01Q\x90\x92P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`\xB8W_\x80\xFD[\x80\x91PP\x92P\x92\x90PV[aIc\x80a\0\xD0_9_\xF3\xFE`\x80`@R`\x046\x10a\x01\x05W_5`\xE0\x1C\x80c\x9F2\x9D\x0B\x11a\0\x92W\x80c\xCA\xC6-\xD9\x11a\0bW\x80c\xCA\xC6-\xD9\x14a\x03\x1FW\x80c\xCC\xFA3o\x14a\x034W\x80c\xD6*\xAD)\x14a\x03GW\x80c\xF4^\x81\x18\x14a\x03\\W\x80c\xF5\xBE\xEA\x8C\x14a\x03qW_\x80\xFD[\x80c\x9F2\x9D\x0B\x14a\x02oW\x80c\xAB\xEE\xB3f\x14a\x02\x84W\x80c\xC1\xE6\x9Bf\x14a\x02\xA3W\x80c\xC29\xEFB\x14a\x03\0W_\x80\xFD[\x80c]\xDC\x9E\x8D\x11a\0\xD8W\x80c]\xDC\x9E\x8D\x14a\x01\xC2W\x80cn\xF8\xD47\x14a\x01\xF9W\x80ct7p)\x14a\x02\x1AW\x80c\x86\xFBa\xED\x14a\x02.W\x80c\x97\x90\xCEq\x14a\x02TW_\x80\xFD[\x80c\x18\x9C\xC9\xD0\x14a\x01\tW\x80c*\x04\xFF\x85\x14a\x010W\x80cIh\xF6\xC5\x14a\x01DW\x80cV\xB4\xA9*\x14a\x01XW[_\x80\xFD[4\x80\x15a\x01\x14W_\x80\xFD[Pa\x01\x1Da\x03\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01;W_\x80\xFD[Pa\x01\x1D`\x0C\x81V[4\x80\x15a\x01OW_\x80\xFD[Pa\x01\x1D_T\x81V[4\x80\x15a\x01cW_\x80\xFD[Pa\x01wa\x01r6`\x04a>\xD4V[a\x03\x9FV[`@Qa\x01'\x91\x90_`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[4\x80\x15a\x01\xCDW_\x80\xFD[P`\x02Ta\x01\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01'V[4\x80\x15a\x02\x04W_\x80\xFD[Pa\x02\x18a\x02\x136`\x04a?4V[a\x04\xB0V[\0[4\x80\x15a\x02%W_\x80\xFD[Pa\x01\x1D` \x81V[4\x80\x15a\x029W_\x80\xFD[Pa\x01\xE1r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x81V[4\x80\x15a\x02_W_\x80\xFD[Pa\x01\x1Dg\r\xE0\xB6\xB3\xA7d\0\0\x81V[4\x80\x15a\x02zW_\x80\xFD[Pa\x01\x1D`\x01T\x81V[4\x80\x15a\x02\x8FW_\x80\xFD[Pa\x01\x1Da\x02\x9E6`\x04a?\xBBV[a\x06\x10V[4\x80\x15a\x02\xAEW_\x80\xFD[Pa\x02\xE1a\x02\xBD6`\x04a?\xBBV[`\x03` R_\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01'V[4\x80\x15a\x03\x0BW_\x80\xFD[Pa\x01\x1Da\x03\x1A6`\x04a?\xE8V[a\x061V[4\x80\x15a\x03*W_\x80\xFD[Pa\x01\x1Da\x1F\xFF\x81V[a\x01\x1Da\x03B6`\x04a@\xB9V[a\x07\xA7V[4\x80\x15a\x03RW_\x80\xFD[Pa\x01\x1Da\x1C \x81V[4\x80\x15a\x03gW_\x80\xFD[Pa\x01\x1Da\x01\0\x81V[4\x80\x15a\x03|W_\x80\xFD[Pa\x01\x1Da\x03\x8B6`\x04a?\xBBV[a\t\x07V[_a\x03\x9ABa\x06\x10V[\x90P\x90V[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\x03\xE1a\x03\xDC\x84a\t V[a\tLV[\x90Pa\x04\x05\x81_\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[` \x02` \x01\x01Qa\x0B7V[\x82R\x80Qa\x04 \x90\x82\x90`\x03\x90\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[` \x83\x01R\x80Qa\x04>\x90\x82\x90`\x04\x90\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[`@\x83\x01R\x80Qa\x04i\x90\x82\x90`\x08\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[` \x02` \x01\x01Qa\x0C4V[``\x83\x01R\x80Qa\x04\x87\x90\x82\x90`\x0B\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x80\x83\x01R\x80Qa\x04\xA5\x90\x82\x90`\x0F\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xA0\x83\x01RP\x91\x90PV[_a\x04\xBB\x83\x80aA,V[\x84`@Q` \x01a\x04\xCD\x92\x91\x90aA\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 _\x81\x81R`\x03\x83R\x83\x90 \x84\x84\x01\x90\x93R\x82T`\x01`\x01`\xA0\x1B\x03\x16\x80\x85R`\x01\x90\x93\x01T\x91\x84\x01\x91\x90\x91R\x92Pa\x05/W`@Qb1&\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x05:\x85\x80aA,V[a\x05H\x90` \x81\x01\x90aB\xF9V[\x81\x01\x90a\x05U\x91\x90aC;V[\x90Pa\x05{\x81a\x05d\x86aD\xF4V[a\x05va\x01 \x8A\x01a\x01\0\x8B\x01aE\xD2V[a\x0C>V[_\x83\x81R`\x03` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01\x82\x90UQ3\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x05\xE0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x05\xE5V[``\x91P[PP\x90P\x80a\x06\x07W`@Qc\r\xB2\xC7\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[_`\x0C`\x01T\x83a\x06!\x91\x90aE\xFFV[a\x06+\x91\x90aF&V[\x92\x91PPV[`\x02T_\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06^W`@Qcc\xE5\x8FW`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x88`@Q` \x01a\x06r\x92\x91\x90aA\xD0V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x03\x90\x93R\x91 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x06\xC4W`@QcuF\xE8\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x03` R`@\x90 `\x01\x01TB\x90a\x06\xE4\x90a\x1C \x90aF9V[\x11\x15a\x07\x03W`@Qc\x1F0L\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x03` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01\x82\x90UQ`\x01`\x01`\xA0\x1B\x03\x85\x16\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x07qW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07vV[``\x91P[PP\x90P\x80a\x07\x98W`@Qc\r\xB2\xC7\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP_T\x97\x96PPPPPPPV[_g\r\xE0\xB6\xB3\xA7d\0\x004\x14a\x07\xD0W`@QcM\xC6`\xA3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x07\xDB\x84\x80aA,V[a\x07\xE9\x90` \x81\x01\x90aB\xF9V[\x81\x01\x90a\x07\xF6\x91\x90aC;V[\x90Pa\x08\x02\x84\x80aA,V[a\x08\x0C\x84\x80aFLV[`@Q` \x01a\x08\x1D\x92\x91\x90aA\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x03\x90\x93R\x91 T\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x08kW`@QcDPCW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`@\x1B\x03\x16a\x08\x80\x84\x80aFLV[a\x08\x93\x90a\x01@\x81\x01\x90a\x01 \x01aFaV[`\x01`\x01`@\x1B\x03\x16\x14a\x08\xBAW`@Qc0\xD3\xBA\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x82R3\x81RB` \x80\x83\x01\x91\x82R_\x85\x81R`\x03\x90\x91R\x92\x90\x92 \x90Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90Q`\x01\x90\x91\x01U\x92\x91PPV[_a\t\x13`\x0C\x83aFzV[`\x01Ta\x06+\x91\x90aF9V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R\x82Q\x82R\x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[``_\x80a\tY\x84a\x0E\\V[\x91\x93P\x90\x91P`\x01\x90P\x81`\x01\x81\x11\x15a\tuWa\tuaF\x91V[\x14a\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid RLP list value.\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\xDFW\x90PP\x90P_\x83[\x86Q\x81\x10\x15a\x0B,W` \x82\x10a\nvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FProvided RLP list exceeds max li`D\x82\x01Ri9\xBA\x1062\xB73\xBA4\x17`\xB1\x1B`d\x82\x01R`\x84\x01a\t\xBEV[_\x80a\n\xB1`@Q\x80`@\x01`@R\x80\x85\x8C_\x01Qa\n\x95\x91\x90aE\xFFV[\x81R` \x01\x85\x8C` \x01Qa\n\xAA\x91\x90aF9V[\x90Ra\x0E\\V[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83a\n\xCD\x91\x90aF9V[\x81R` \x01\x84\x8B` \x01Qa\n\xE2\x91\x90aF9V[\x81RP\x85\x85\x81Q\x81\x10a\n\xF7Wa\n\xF7aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x0B\r`\x01\x85aF9V[\x93Pa\x0B\x19\x81\x83aF9V[a\x0B#\x90\x84aF9V[\x92PPPa\n\nV[P\x81R\x94\x93PPPPV[_`!\x82_\x01Q\x11\x15a\x0B\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP bytes32 value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[_\x80_a\x0B\x98\x85a\x0E\\V[\x91\x94P\x92P\x90P_\x81`\x01\x81\x11\x15a\x0B\xB2Wa\x0B\xB2aF\x91V[\x14a\x0B\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP bytes32 value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[_\x83\x86` \x01Qa\x0C\x10\x91\x90aF9V[\x80Q\x90\x91P` \x84\x10\x15a\x0C*W` \x84\x90\x03a\x01\0\n\x90\x04[\x96\x95PPPPPPV[_a\x06+\x82a\x0B7V[\x82Q`\x01`\x01`@\x1B\x03\x16` a\x0CSa\x03\x90V[a\x0C]\x91\x90aE\xFFV[\x81\x11\x15a\x0C}W`@Qc\xB6\x14K\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\0a\x0C\x88a\x03\x90V[a\x0C\x92\x91\x90aE\xFFV[\x81\x10\x15a\x0C\xB2W`@Qc\x0C\xDC\xEBy`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q_\x90a\x0C\xC2\x90`\x01\x90aE\xFFV[\x90PC\x81\x11\x80a\x0C\xDCWPa\x0C\xD9a\x01\0CaE\xFFV[\x81\x10[\x15a\x0C\xFAW`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q_\x90a\r\n\x90`\x01\x90aE\xFFV[` \x80\x87\x01Q\x80Q\x91\x01 \x90@\x91P\x80\x82\x14a\r9W`@Qc\x1F\x03F[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\rD\x89a\x11VV[\x92P\x92PP\x81`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\r{W`@Qc\xAA\xAA\x91A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\x89\x89`@\x01Qa\x03\x9FV[\x80Q\x90\x91P\x84\x14a\r\xADW`@Qc \xFAl\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\xD4\x8A`\xA0\x01Q_\x81Q\x81\x10a\r\xC7Wa\r\xC7aA\x18V[` \x02` \x01\x01Qa\x11\xDEV[\x90P_\x80a\x0E\x04\x83\x8D`\x80\x01Q_\x81Q\x81\x10a\r\xF2Wa\r\xF2aA\x18V[` \x02` \x01\x01Q\x86`@\x01Qa\x11\xF1V[\x91P\x91P\x81a\x0E&W`@Qc\tL\xEC_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q` \x82\x01 \x85Q\x14a\x0EMW`@QcDb\xB3\x8F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPPPV[_\x80_\x80\x84_\x01Q\x11a\x0E\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FRLP item cannot be null.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[` \x84\x01Q\x80Q_\x1A`\x7F\x81\x11a\x0E\xD3W_`\x01_\x94P\x94P\x94PPPa\x11OV[`\xB7\x81\x11a\x0FBW\x85Q`\x7F\x19\x82\x01\x90\x81\x10a\x0F1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid RLP short string.\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[`\x01\x95P\x93P_\x92Pa\x11O\x91PPV[`\xBF\x81\x11a\x10\x19W\x85Q`\xB6\x19\x82\x01\x90\x81\x10a\x0F\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FInvalid RLP long string length.\0`D\x82\x01R`d\x01a\t\xBEV[_\x81` \x03a\x01\0\n`\x01\x85\x01Q\x04\x90P\x80\x82\x01\x88_\x01Q\x11a\x10\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid RLP long string.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[`\x01\x90\x91\x01\x95P\x93P_\x92Pa\x11O\x91PPV[`\xF7\x81\x11a\x10\x88W\x85Q`\xBF\x19\x82\x01\x90\x81\x10a\x10wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid RLP short list.\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[`\x01\x95P\x93P\x84\x92Pa\x11O\x91PPV[\x85Q`\xF6\x19\x82\x01\x90\x81\x10a\x10\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid RLP long list length.\0\0\0`D\x82\x01R`d\x01a\t\xBEV[_\x81` \x03a\x01\0\n`\x01\x85\x01Q\x04\x90P\x80\x82\x01\x88_\x01Q\x11a\x11<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru$\xB7;0\xB64\xB2\x10)&(\x1067\xB73\x9064\xB9\xBA\x17`Q\x1B`D\x82\x01R`d\x01a\t\xBEV[`\x01\x91\x82\x01\x96P\x94P\x92Pa\x11O\x91PPV[\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x81\x90a\x11\x89a\x11\x7F\x85a\x12\xC6V[\x85` \x01Qa\x13\rV[\x91P_a\x11\x99\x85`@\x01Qa\x135V[\x90Pa\x11\xA4\x81a\x13\xEFV[\x93P`@Q\x80``\x01`@R\x80\x86`@\x01Q\x80Q\x90` \x01 \x81R` \x01\x82``\x01Q\x81R` \x01\x82`\xE0\x01Q\x81RP\x91PP\x91\x93\x90\x92PV[``a\x06+a\x11\xEC\x83a\x14\nV[a\x15'V[_``_a\x11\xFE\x85a\x15\x95V[\x90P_\x80_a\x12\x0E\x84\x8A\x89a\x16\x80V[\x81Q\x92\x95P\x90\x93P\x91P\x15\x80\x80a\x12\"WP\x81[a\x12nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FProvided proof is invalid.\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[_\x81a\x12\x88W`@Q\x80` \x01`@R\x80_\x81RPa\x12\xB4V[a\x12\xB4\x86a\x12\x97`\x01\x88aE\xFFV[\x81Q\x81\x10a\x12\xA7Wa\x12\xA7aA\x18V[` \x02` \x01\x01Qa\x1A\x85V[\x91\x9B\x91\x9AP\x90\x98PPPPPPPPPV[_\x81`@\x01Q\x80Q\x90` \x01 a\x12\xDF\x83_\x01Qa\x1A\xAFV[`@Q` \x01a\x12\xF0\x92\x91\x90aF\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[_\x80_\x80a\x13\x1B\x86\x86a\x1B/V[\x92P\x92P\x92Pa\x13+\x82\x82a\x1BxV[P\x90\x94\x93PPPPV[a\x13=a=nV[_\x82_\x81Q\x81\x10a\x13PWa\x13PaA\x18V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P`\x7F`\xF8\x1B\x81\x10a\x13|Wa\x13u\x83a\x1C4V[\x93\x92PPPV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x01`\xF8\x1B\x03a\x13\x9AWa\x13u\x83a\x1EjV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x01`\xF9\x1B\x03a\x13\xB8Wa\x13u\x83a!*V[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x03`\xF8\x1B\x03a\x13\xD6Wa\x13u\x83a#\xAFV[`@Qco\xC3\xDA\xA3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x06+a\x13\xFC\x83a'<V[a\x14\x05\x84a'TV[a\x13\rV[``_\x82`@Q` \x01a\x14 \x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_[` \x81\x10\x15a\x14kW\x81\x81\x81Q\x81\x10a\x14NWa\x14NaA\x18V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16_\x03a\x14kW`\x01\x01a\x143V[_a\x14w\x82` aE\xFFV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x8EWa\x14\x8Ea=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xB8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a\x15\x1EW\x83\x83a\x14\xD1\x81aF\xCDV[\x94P\x81Q\x81\x10a\x14\xE3Wa\x14\xE3aA\x18V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x15\0Wa\x15\0aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x14\xBDV[P\x94\x93PPPPV[``\x80\x82Q`\x01\x14\x80\x15a\x15TWP`\x80\x83_\x81Q\x81\x10a\x15JWa\x15JaA\x18V[\x01` \x01Q`\xF8\x1C\x10[\x15a\x15`WP\x81a\x06+V[a\x15l\x83Q`\x80a'\xB0V[\x83`@Q` \x01a\x15~\x92\x91\x90aF\xE5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[``_a\x15\xA1\x83a)WV[\x90P_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xBDWa\x15\xBDa=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\x02W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x15\xDBW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x16xW_a\x163\x84\x83\x81Q\x81\x10a\x16&Wa\x16&aA\x18V[` \x02` \x01\x01Qa)eV[\x90P`@Q\x80`@\x01`@R\x80\x82\x81R` \x01a\x16O\x83a)WV[\x81RP\x83\x83\x81Q\x81\x10a\x16dWa\x16daA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x16\x07V[P\x93\x92PPPV[_``\x81\x80\x80a\x16\x8F\x87a)\xF2V[\x90P_\x86\x90P_\x80a\x16\xB4`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[_[\x8CQ\x81\x10\x15a\x1A]W\x8C\x81\x81Q\x81\x10a\x16\xD1Wa\x16\xD1aA\x18V[` \x02` \x01\x01Q\x91P\x82\x84a\x16\xE7\x91\x90aF9V[\x93Pa\x16\xF4`\x01\x88aF9V[\x96P\x83_\x03a\x17NW\x81Q\x80Q` \x90\x91\x01 \x85\x14a\x17IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\t-\xCE\xCC-\x8D,\x84\x0EM\xED\xEE\x84\r\x0C.m`{\x1B`D\x82\x01R`d\x01a\t\xBEV[a\x18\nV[\x81QQ` \x11a\x17\xB0W\x81Q\x80Q` \x90\x91\x01 \x85\x14a\x17IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FInvalid large internal hash\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[\x84a\x17\xBD\x83_\x01Qa+\x02V[\x14a\x18\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid internal node hash\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[a\x18\x16`\x10`\x01aF9V[\x82` \x01QQ\x03a\x18\x86W\x85Q\x84\x14a\x1A]W_\x86\x85\x81Q\x81\x10a\x18<Wa\x18<aA\x18V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_\x83` \x01Q\x82`\xFF\x16\x81Q\x81\x10a\x18fWa\x18faA\x18V[` \x02` \x01\x01Q\x90Pa\x18y\x81a+)V[\x96P`\x01\x94PPPa\x1AUV[`\x02\x82` \x01QQ\x03a\x1A\rW_a\x18\x9D\x83a+]V[\x90P_\x81_\x81Q\x81\x10a\x18\xB2Wa\x18\xB2aA\x18V[\x01` \x01Q`\xF8\x1C\x90P_a\x18\xC8`\x02\x83aF\xF9V[a\x18\xD3\x90`\x02aG\x1AV[\x90P_a\x18\xE3\x84\x83`\xFF\x16a+\x80V[\x90P_a\x18\xF0\x8B\x8Aa+\x80V[\x90P_a\x18\xFD\x83\x83a+\xAEV[\x90P`\xFF\x85\x16`\x02\x14\x80a\x19\x14WP`\xFF\x85\x16`\x03\x14[\x15a\x19NW\x80\x83Q\x14\x80\x15a\x19)WP\x80\x82Q\x14[\x15a\x19;Wa\x198\x81\x8BaF9V[\x99P[P`\x01`\xFF\x1B\x99Pa\x1A]\x94PPPPPV[`\xFF\x85\x16\x15\x80a\x19aWP`\xFF\x85\x16`\x01\x14[\x15a\x19\xB6W\x80_\x03a\x19\x80WP`\x01`\xFF\x1B\x99Pa\x1A]\x94PPPPPV[a\x19\xA7\x88` \x01Q`\x01\x81Q\x81\x10a\x19\x9AWa\x19\x9AaA\x18V[` \x02` \x01\x01Qa+)V[\x9AP\x97Pa\x1AU\x94PPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FReceived a node with an unknown `D\x82\x01Re\x0E\x0EL\xAC\xCD/`\xD3\x1B`d\x82\x01R`\x84\x01a\t\xBEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FReceived an unparsable node.\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[`\x01\x01a\x16\xB6V[P`\x01`\xFF\x1B\x84\x14\x86a\x1Ap\x87\x86a+\x80V[\x90\x9E\x90\x9DP\x90\x9BP\x99PPPPPPPPPPV[` \x81\x01Q\x80Q``\x91a\x06+\x91a\x1A\x9F\x90`\x01\x90aE\xFFV[\x81Q\x81\x10a\x16&Wa\x16&aA\x18V[`@\x80Q`\x08\x80\x82R\x81\x83\x01\x90\x92R``\x91_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`\x08\x81\x10\x15a\x1B(Wa\x1A\xE9\x81`\x08aFzV[\x84`\x01`\x01`@\x1B\x03\x16\x90\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1B\nWa\x1B\naA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1A\xD5V[P\x92\x91PPV[_\x80_\x83Q`A\x03a\x1BfW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa\x1BX\x88\x82\x85\x85a,)V[\x95P\x95P\x95PPPPa\x1BqV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a\x1B\x8BWa\x1B\x8BaF\x91V[\x03a\x1B\x94WPPV[`\x01\x82`\x03\x81\x11\x15a\x1B\xA8Wa\x1B\xA8aF\x91V[\x03a\x1B\xC6W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x1B\xDAWa\x1B\xDAaF\x91V[\x03a\x1B\xFBW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xBEV[`\x03\x82`\x03\x81\x11\x15a\x1C\x0FWa\x1C\x0FaF\x91V[\x03a\x1C0W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xBEV[PPV[a\x1C<a=nV[_\x80\x82Ra\x1CLa\x03\xDC\x84a\t V[\x90P\x80Q`\t\x14\x15\x80\x15a\x1CbWP\x80Q`\x06\x14\x15[\x15a\x1C\x80W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x95\x81_\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[``\x83\x01R\x80Qa\x1C\xB3\x90\x82\x90`\x01\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x80\x83\x01R\x80Qa\x1C\xD1\x90\x82\x90`\x02\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xE0\x83\x01R\x80Qa\x1C\xFC\x90\x82\x90`\x03\x90\x81\x10a\x1C\xEFWa\x1C\xEFaA\x18V[` \x02` \x01\x01Qa,\xF1V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x83\x01R\x80Qa\x1D$\x90\x82\x90`\x04\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[a\x01 \x83\x01R\x80Qa\x1DC\x90\x82\x90`\x05\x90\x81\x10a\x16&Wa\x16&aA\x18V[a\x01@\x83\x01R\x80Q`\x06\x03a\x1DXWP\x91\x90PV[_a\x1Do\x82`\x06\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[\x90P_a\x1D\x88\x83`\x07\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[\x90P_a\x1D\xA1\x84`\x08\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[\x90P\x81\x15\x80\x15a\x1D\xAFWP\x80\x15[\x15a\x1D\xCFW`\x01`\x01`@\x1B\x03\x83\x16` \x86\x01R`\x01`@\x86\x01Ra\x1EaV[`#\x83`\x01`\x01`@\x1B\x03\x16\x10a\x1E\x17W`\x02a\x1D\xED`#\x85aG3V[a\x1D\xF7\x91\x90aGSV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x87\x01R\x83\x16a\x01\xE0\x86\x01R`\x01`@\x86\x01R[_a\x1E&`\x02`\x01\x86\x18aGxV[a\x1E1\x90`\x1BaG\x9DV[`@Q\x90\x91Pa\x1EI\x90\x84\x90\x84\x90\x84\x90` \x01aG\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x87\x01RP[PPPP\x91\x90PV[a\x1Era=nV[`\x01\x80\x82R\x82Q_\x91a\x1E\x92\x91a\x1E\x8A\x90\x82\x90aE\xFFV[\x85\x91\x90a-]V[\x90P_a\x1E\xA1a\x03\xDC\x83a\t V[\x90P\x80Q`\x08\x14\x15\x80\x15a\x1E\xB7WP\x80Q`\x0B\x14\x15[\x15a\x1E\xD5W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xEA\x81_\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa\x1F\x11\x90\x82\x90`\x01\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[``\x84\x01R\x80Qa\x1F/\x90\x82\x90`\x02\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x80\x84\x01R\x80Qa\x1FM\x90\x82\x90`\x03\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xE0\x84\x01R\x80Qa\x1Fk\x90\x82\x90`\x04\x90\x81\x10a\x1C\xEFWa\x1C\xEFaA\x18V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\x1F\x93\x90\x82\x90`\x05\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[a\x01 \x84\x01R\x80Qa\x1F\xB2\x90\x82\x90`\x06\x90\x81\x10a\x16&Wa\x16&aA\x18V[\x83a\x01@\x01\x81\x90RP_a\x1F\xDF\x82`\x07\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aA\x18V[` \x02` \x01\x01Qa\tLV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xFAWa\x1F\xFAa=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a -W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a \x18W\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a \x8BWa a\x82\x82\x81Q\x81\x10a TWa TaA\x18V[` \x02` \x01\x01Qa.\x9DV[\x85a\x01`\x01Q\x82\x81Q\x81\x10a xWa xaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a 6V[P\x81Q`\x08\x03a \x9DWPPP\x91\x90PV[_a \xB4\x83`\x08\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[a \xBF\x90`\x1BaG\xE0V[\x90P_a \xD8\x84`\t\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P_a \xF1\x85`\n\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P\x81\x81\x84`@Q` \x01a!\x08\x93\x92\x91\x90aG\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x88\x01RP\x94\x96\x95PPPPPPV[a!2a=nV[`\x02\x81R\x81Q_\x90a!L\x90`\x01\x90a\x1E\x8A\x90\x82\x90aE\xFFV[\x90P_a![a\x03\xDC\x83a\t V[\x90P\x80Q`\t\x14\x15\x80\x15a!qWP\x80Q`\x0C\x14\x15[\x15a!\x8FW`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\xA4\x81_\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa!\xCB\x90\x82\x90`\x01\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[``\x84\x01R\x80Qa!\xE9\x90\x82\x90`\x02\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xA0\x84\x01R\x80Qa\"\x07\x90\x82\x90`\x03\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xC0\x84\x01R\x80Qa\"%\x90\x82\x90`\x04\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xE0\x84\x01R\x80Qa\"C\x90\x82\x90`\x05\x90\x81\x10a\x1C\xEFWa\x1C\xEFaA\x18V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\"k\x90\x82\x90`\x06\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[a\x01 \x84\x01R\x80Qa\"\x8A\x90\x82\x90`\x07\x90\x81\x10a\x16&Wa\x16&aA\x18V[\x83a\x01@\x01\x81\x90RP_a\"\xAA\x82`\x08\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aA\x18V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC5Wa\"\xC5a=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xF8W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\"\xE3W\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a#IWa#\x1F\x82\x82\x81Q\x81\x10a TWa TaA\x18V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a#6Wa#6aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a#\x01V[P\x81Q`\t\x03a#[WPPP\x91\x90PV[_a#r\x83`\t\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[a#}\x90`\x1BaG\xE0V[\x90P_a#\x96\x84`\n\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P_a \xF1\x85`\x0B\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[a#\xB7a=nV[`\x03\x81R\x81Q_\x90a#\xD1\x90`\x01\x90a\x1E\x8A\x90\x82\x90aE\xFFV[\x90P_a#\xE0a\x03\xDC\x83a\t V[\x90P\x80Q`\x0B\x14\x15\x80\x15a#\xF6WP\x80Q`\x0E\x14\x15[\x15a$\x14W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$)\x81_\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa$P\x90\x82\x90`\x01\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[``\x84\x01R\x80Qa$n\x90\x82\x90`\x02\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xA0\x84\x01R\x80Qa$\x8C\x90\x82\x90`\x03\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xC0\x84\x01R\x80Qa$\xAA\x90\x82\x90`\x04\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xE0\x84\x01R\x80Qa$\xC8\x90\x82\x90`\x05\x90\x81\x10a\x1C\xEFWa\x1C\xEFaA\x18V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa$\xF0\x90\x82\x90`\x06\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[a\x01 \x84\x01R\x80Qa%\x0F\x90\x82\x90`\x07\x90\x81\x10a\x16&Wa\x16&aA\x18V[\x83a\x01@\x01\x81\x90RP_a%/\x82`\x08\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aA\x18V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%JWa%Ja=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%}W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a%hW\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a%\xCEWa%\xA4\x82\x82\x81Q\x81\x10a TWa TaA\x18V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a%\xBBWa%\xBBaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a%\x86V[Pa%\xE5\x82`\t\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[\x84a\x01\x80\x01\x81\x81RPP_a&\x06\x83`\n\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aA\x18V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&!Wa&!a=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&JW\x81` \x01` \x82\x02\x806\x837\x01\x90P[Pa\x01\xA0\x86\x01R_[\x81Q\x81\x10\x15a&\x9BWa&q\x82\x82\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x86a\x01\xA0\x01Q\x82\x81Q\x81\x10a&\x88Wa&\x88aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a&SV[P\x82Q`\x0B\x03a&\xAEWPPPP\x91\x90PV[_a&\xC5\x84`\x0B\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[a&\xD0\x90`\x1BaG\xE0V[\x90P_a&\xE9\x85`\x0C\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P_a'\x02\x86`\r\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P\x81\x81\x84`@Q` \x01a'\x19\x93\x92\x91\x90aG\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x89\x01RP\x95\x97\x96PPPPPPPV[_a'F\x82a.\xA8V[\x80Q\x90` \x01 \x90P\x91\x90PV[``\x81a\x01\xC0\x01QQ_\x03a'|W`@Qc\xD4f\xBD\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x01\xC0\x01QQ`A\x14a'\xA3W`@QcK\xE62\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x01\xC0\x81\x01Q[\x91\x90PV[``\x80`8\x84\x10\x15a(\x15W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90Pa'\xE5\x83\x85aG\xE0V[`\xF8\x1B\x81_\x81Q\x81\x10a'\xFAWa'\xFAaA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa\x13uV[_`\x01[a(#\x81\x87aF&V[\x15a(IW\x81a(2\x81aF\xCDV[\x92Pa(B\x90Pa\x01\0\x82aFzV[\x90Pa(\x19V[a(T\x82`\x01aF9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a(kWa(ka=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a(\x95W` \x82\x01\x81\x806\x837\x01\x90P[P\x92Pa(\xA2\x85\x83aG\xE0V[a(\xAD\x90`7aG\xE0V[`\xF8\x1B\x83_\x81Q\x81\x10a(\xC2Wa(\xC2aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x90P[\x81\x81\x11a)NWa\x01\0a(\xF1\x82\x84aE\xFFV[a(\xFD\x90a\x01\0aH\xD9V[a)\x07\x90\x88aF&V[a)\x11\x91\x90aH\xE4V[`\xF8\x1B\x83\x82\x81Q\x81\x10a)&Wa)&aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x80a)F\x81aF\xCDV[\x91PPa(\xDDV[PP\x93\x92PPPV[``a\x06+a\x03\xDC\x83a\t V[``_\x80_a)s\x85a\x0E\\V[\x91\x94P\x92P\x90P_\x81`\x01\x81\x11\x15a)\x8DWa)\x8DaF\x91V[\x14a)\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid RLP bytes value.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[a)\xE9\x85` \x01Q\x84\x84a/5V[\x95\x94PPPPPV[``_\x82Q`\x02\x02`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x11Wa*\x11a=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a*;W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1B(W`\x04\x84\x82\x81Q\x81\x10a*]Wa*]aA\x18V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x82\x82`\x02\x02\x81Q\x81\x10a*\x89Wa*\x89aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x10\x84\x82\x81Q\x81\x10a*\xB3Wa*\xB3aA\x18V[\x01` \x01Q`\xF8\x1C\x81a*\xC8Wa*\xC8aF\x12V[\x06`\xF8\x1B\x82\x82`\x02\x02`\x01\x01\x81Q\x81\x10a*\xE4Wa*\xE4aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a*@V[_` \x82Q\x10\x15a+\x15WP` \x01Q\x90V[\x81\x80` \x01\x90Q\x81\x01\x90a\x06+\x91\x90aH\xF7V[_``` \x83_\x01Q\x10\x15a+HWa+A\x83a.\x9DV[\x90Pa+TV[a+Q\x83a)eV[\x90P[a\x13u\x81a+\x02V[``a\x06+a+{\x83` \x01Q_\x81Q\x81\x10a\x16&Wa\x16&aA\x18V[a)\xF2V[``\x81\x83Q\x03_\x03a+\xA0WP`@\x80Q` \x81\x01\x90\x91R_\x81Ra\x06+V[a\x13u\x83\x83\x84\x86Q\x03a-]V[_\x80[\x80\x84Q\x11\x80\x15a+\xC1WP\x80\x83Q\x11[\x80\x15a,\x12WP\x82\x81\x81Q\x81\x10a+\xDAWa+\xDAaA\x18V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a,\x01Wa,\x01aA\x18V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14[\x15a\x13uW\x80a,!\x81aF\xCDV[\x91PPa+\xB1V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a,bWP_\x91P`\x03\x90P\x82a,\xE7V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a,\xB3W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a,\xDEWP_\x92P`\x01\x91P\x82\x90Pa,\xE7V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[\x80Q_\x90`\x01\x03a-\x03WP_\x91\x90PV[\x81Q`\x15\x14a-TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP address value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[a\x06+\x82a\x0C4V[``\x81\x82`\x1F\x01\x10\x15a-\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\t\xBEV[\x82\x82\x84\x01\x10\x15a-\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\t\xBEV[\x81\x83\x01\x84Q\x10\x15a.-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\t\xBEV[``\x82\x15\x80\x15a.KW`@Q\x91P_\x82R` \x82\x01`@Ra\x15\x1EV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a.\x84W\x80Q\x83R` \x92\x83\x01\x92\x01a.lV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RPP\x94\x93PPPPV[``a\x06+\x82a/\xE2V[``_\x82Q`\x03\x81\x11\x15a.\xBEWa.\xBEaF\x91V[\x03a.\xCCWa\x06+\x82a/\xF6V[`\x01\x82Q`\x03\x81\x11\x15a.\xE1Wa.\xE1aF\x91V[\x03a.\xEFWa\x06+\x82a2\xE2V[`\x02\x82Q`\x03\x81\x11\x15a/\x04Wa/\x04aF\x91V[\x03a/\x12Wa\x06+\x82a5SV[`\x03\x82Q`\x03\x81\x11\x15a/'Wa/'aF\x91V[\x03a\x13\xD6Wa\x06+\x82a7\xC1V[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a/PWa/Pa=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a/zW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x80Q_\x03a/\x8CW\x90Pa\x13uV[\x84\x84\x01` \x82\x01_[` \x86\x04\x81\x10\x15a/\xB6W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a/\x95V[P_`\x01` \x87\x06` \x03a\x01\0\n\x03\x90P\x80\x82Q\x16\x81\x19\x84Q\x16\x17\x82R\x83\x94PPPPP\x93\x92PPPV[``a\x06+\x82` \x01Q_\x84_\x01Qa/5V[``_\x82` \x01Q`\x01`\x01`@\x1B\x03\x16_\x14a0\x18WP` \x82\x01Qa0[V[a\x01\xC0\x83\x01QQ\x15a0[W`#\x83a\x01\xE0\x01Q`\x01`\x01`@\x1B\x03\x16\x10a0[W`\x02`#\x84a\x01\xE0\x01Qa0N\x91\x90aG3V[a0X\x91\x90aGSV[\x90P[_\x83`@\x01Qa0kW_a0nV[`\x03[a0y\x90`\x06aG\xE0V[`\xFF\x16\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x97Wa0\x97a=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a0\xCAW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a0\xB5W\x90P[P\x90Pa0\xDA\x85``\x01Qa\x11\xDEV[\x81_\x81Q\x81\x10a0\xECWa0\xECaA\x18V[` \x02` \x01\x01\x81\x90RPa1\x04\x85`\x80\x01Qa\x11\xDEV[\x81`\x01\x81Q\x81\x10a1\x17Wa1\x17aA\x18V[` \x02` \x01\x01\x81\x90RPa1/\x85`\xE0\x01Qa\x11\xDEV[\x81`\x02\x81Q\x81\x10a1BWa1BaA\x18V[` \x02` \x01\x01\x81\x90RPa1[\x85a\x01\0\x01Qa;dV[\x81`\x03\x81Q\x81\x10a1nWa1naA\x18V[` \x02` \x01\x01\x81\x90RPa1\x87\x85a\x01 \x01Qa\x11\xDEV[\x81`\x04\x81Q\x81\x10a1\x9AWa1\x9AaA\x18V[` \x02` \x01\x01\x81\x90RPa1\xB3\x85a\x01@\x01Qa\x15'V[\x81`\x05\x81Q\x81\x10a1\xC6Wa1\xC6aA\x18V[` \x02` \x01\x01\x81\x90RP\x84`@\x01Q\x15a2\xD9W\x84` \x01Q`\x01`\x01`@\x1B\x03\x16_\x03a2-W`@Q_` \x82\x01R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81`\x06\x81Q\x81\x10a2\x1DWa2\x1DaA\x18V[` \x02` \x01\x01\x81\x90RPa2^V[a2?\x83`\x01`\x01`@\x1B\x03\x16a\x11\xDEV[\x81`\x06\x81Q\x81\x10a2RWa2RaA\x18V[` \x02` \x01\x01\x81\x90RP[a2\x93_[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a2\x8DW` \x82\x01\x81\x806\x837\x01\x90P[Pa\x15'V[\x81`\x07\x81Q\x81\x10a2\xA6Wa2\xA6aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra2\xBA_a2cV[\x81`\x08\x81Q\x81\x10a2\xCDWa2\xCDaA\x18V[` \x02` \x01\x01\x81\x90RP[a)\xE9\x81a;\x9CV[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a2\xFEW\x90PP\x90Pa3+\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x11\xDEV[\x81_\x81Q\x81\x10a3=Wa3=aA\x18V[` \x02` \x01\x01\x81\x90RPa3U\x83``\x01Qa\x11\xDEV[\x81`\x01\x81Q\x81\x10a3hWa3haA\x18V[` \x02` \x01\x01\x81\x90RPa3\x80\x83`\x80\x01Qa\x11\xDEV[\x81`\x02\x81Q\x81\x10a3\x93Wa3\x93aA\x18V[` \x02` \x01\x01\x81\x90RPa3\xAB\x83`\xE0\x01Qa\x11\xDEV[\x81`\x03\x81Q\x81\x10a3\xBEWa3\xBEaA\x18V[` \x02` \x01\x01\x81\x90RPa3\xD7\x83a\x01\0\x01Qa;dV[\x81`\x04\x81Q\x81\x10a3\xEAWa3\xEAaA\x18V[` \x02` \x01\x01\x81\x90RPa4\x03\x83a\x01 \x01Qa\x11\xDEV[\x81`\x05\x81Q\x81\x10a4\x16Wa4\x16aA\x18V[` \x02` \x01\x01\x81\x90RPa4/\x83a\x01@\x01Qa\x15'V[\x81`\x06\x81Q\x81\x10a4BWa4BaA\x18V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a4lWa4la=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a4\x9FW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a4\x8AW\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a4\xF6W\x84a\x01`\x01Q\x81\x81Q\x81\x10a4\xC9Wa4\xC9aA\x18V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a4\xE3Wa4\xE3aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a4\xA4V[Pa5\0\x81a;\x9CV[\x82`\x07\x81Q\x81\x10a5\x13Wa5\x13aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01[a5*\x83a;\x9CV[`@Q` \x01a5;\x92\x91\x90aI\x0EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`@\x80Q`\t\x80\x82Ra\x01@\x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a5oW\x90PP\x90Pa5\x9C\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x11\xDEV[\x81_\x81Q\x81\x10a5\xAEWa5\xAEaA\x18V[` \x02` \x01\x01\x81\x90RPa5\xC6\x83``\x01Qa\x11\xDEV[\x81`\x01\x81Q\x81\x10a5\xD9Wa5\xD9aA\x18V[` \x02` \x01\x01\x81\x90RPa5\xF1\x83`\xA0\x01Qa\x11\xDEV[\x81`\x02\x81Q\x81\x10a6\x04Wa6\x04aA\x18V[` \x02` \x01\x01\x81\x90RPa6\x1C\x83`\xC0\x01Qa\x11\xDEV[\x81`\x03\x81Q\x81\x10a6/Wa6/aA\x18V[` \x02` \x01\x01\x81\x90RPa6G\x83`\xE0\x01Qa\x11\xDEV[\x81`\x04\x81Q\x81\x10a6ZWa6ZaA\x18V[` \x02` \x01\x01\x81\x90RPa6s\x83a\x01\0\x01Qa;dV[\x81`\x05\x81Q\x81\x10a6\x86Wa6\x86aA\x18V[` \x02` \x01\x01\x81\x90RPa6\x9F\x83a\x01 \x01Qa\x11\xDEV[\x81`\x06\x81Q\x81\x10a6\xB2Wa6\xB2aA\x18V[` \x02` \x01\x01\x81\x90RPa6\xCB\x83a\x01@\x01Qa\x15'V[\x81`\x07\x81Q\x81\x10a6\xDEWa6\xDEaA\x18V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x08Wa7\x08a=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7;W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a7&W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a7\x92W\x84a\x01`\x01Q\x81\x81Q\x81\x10a7eWa7eaA\x18V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a7\x7FWa7\x7FaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a7@V[Pa7\x9C\x81a;\x9CV[\x82`\x08\x81Q\x81\x10a7\xAFWa7\xAFaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x02a5!V[`@\x80Q`\x0B\x80\x82Ra\x01\x80\x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a7\xDDW\x90PP\x90Pa8\n\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x11\xDEV[\x81_\x81Q\x81\x10a8\x1CWa8\x1CaA\x18V[` \x02` \x01\x01\x81\x90RPa84\x83``\x01Qa\x11\xDEV[\x81`\x01\x81Q\x81\x10a8GWa8GaA\x18V[` \x02` \x01\x01\x81\x90RPa8_\x83`\xA0\x01Qa\x11\xDEV[\x81`\x02\x81Q\x81\x10a8rWa8raA\x18V[` \x02` \x01\x01\x81\x90RPa8\x8A\x83`\xC0\x01Qa\x11\xDEV[\x81`\x03\x81Q\x81\x10a8\x9DWa8\x9DaA\x18V[` \x02` \x01\x01\x81\x90RPa8\xB5\x83`\xE0\x01Qa\x11\xDEV[\x81`\x04\x81Q\x81\x10a8\xC8Wa8\xC8aA\x18V[` \x02` \x01\x01\x81\x90RPa8\xE1\x83a\x01\0\x01Qa;dV[\x81`\x05\x81Q\x81\x10a8\xF4Wa8\xF4aA\x18V[` \x02` \x01\x01\x81\x90RPa9\r\x83a\x01 \x01Qa\x11\xDEV[\x81`\x06\x81Q\x81\x10a9 Wa9 aA\x18V[` \x02` \x01\x01\x81\x90RPa99\x83a\x01@\x01Qa\x15'V[\x81`\x07\x81Q\x81\x10a9LWa9LaA\x18V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a9vWa9va=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a9\xA9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a9\x94W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a:\0W\x84a\x01`\x01Q\x81\x81Q\x81\x10a9\xD3Wa9\xD3aA\x18V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a9\xEDWa9\xEDaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a9\xAEV[Pa:\n\x81a;\x9CV[\x82`\x08\x81Q\x81\x10a:\x1DWa:\x1DaA\x18V[` \x02` \x01\x01\x81\x90RPa:6\x84a\x01\x80\x01Qa\x11\xDEV[\x82`\t\x81Q\x81\x10a:IWa:IaA\x18V[` \x02` \x01\x01\x81\x90RP_\x84a\x01\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a:sWa:sa=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a:\xA6W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a:\x91W\x90P[P\x90P_[\x85a\x01\xA0\x01QQ\x81\x10\x15a;\x07Wa:\xE2\x86a\x01\xA0\x01Q\x82\x81Q\x81\x10a:\xD3Wa:\xD3aA\x18V[` \x02` \x01\x01Q_\x1Ca\x11\xDEV[\x82\x82\x81Q\x81\x10a:\xF4Wa:\xF4aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a:\xABV[Pa;\x11\x81a;\x9CV[\x83`\n\x81Q\x81\x10a;$Wa;$aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x03a;:\x84a;\x9CV[`@Q` \x01a;K\x92\x91\x90aI\x0EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x91\x90PV[`@Q``\x82\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x83\x01R\x90a\x06+\x90`4\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x15'V[``_a;\xA8\x83a;\xDFV[\x90Pa;\xB6\x81Q`\xC0a'\xB0V[\x81`@Q` \x01a;\xC8\x92\x91\x90aF\xE5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[``\x81Q_\x03a;\xFEW`@\x80Q_\x80\x82R` \x82\x01\x90\x92R\x90a\x1B(V[_\x80[\x83Q\x81\x10\x15a<:W\x83\x81\x81Q\x81\x10a<\x1CWa<\x1CaA\x18V[` \x02` \x01\x01QQ\x82a<0\x91\x90aF9V[\x91P`\x01\x01a<\x01V[_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a<SWa<Sa=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a<}W` \x82\x01\x81\x806\x837\x01\x90P[P_\x92P\x90P` \x81\x01[\x85Q\x83\x10\x15a\x15\x1EW_\x86\x84\x81Q\x81\x10a<\xA4Wa<\xA4aA\x18V[` \x02` \x01\x01Q\x90P_` \x82\x01\x90Pa<\xC1\x83\x82\x84Qa<\xF7V[\x87\x85\x81Q\x81\x10a<\xD3Wa<\xD3aA\x18V[` \x02` \x01\x01QQ\x83a<\xE7\x91\x90aF9V[`\x01\x90\x95\x01\x94\x92Pa<\x88\x91PPV[\x82\x82\x82[` \x81\x10a=3W\x81Q\x83Ra=\x12` \x84aF9V[\x92Pa=\x1F` \x83aF9V[\x91Pa=,` \x82aE\xFFV[\x90Pa<\xFBV[_`\x01a=A\x83` aE\xFFV[a=M\x90a\x01\0aH\xD9V[a=W\x91\x90aE\xFFV[\x92Q\x84Q\x84\x16\x93\x19\x16\x92\x90\x92\x17\x90\x92RPPPPPV[`@\x80Qa\x02\0\x81\x01\x90\x91R\x80_\x81R` \x01_`\x01`\x01`@\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_`\x01`\x01`@\x1B\x03\x16\x81RP\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>3Wa>3a=\xFDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>aWa>aa=\xFDV[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a>xW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a>\x91Wa>\x91a=\xFDV[a>\xA4`\x1F\x82\x01`\x1F\x19\x16` \x01a>9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a>\xB8W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a>\xE4W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xF9W_\x80\xFD[a?\x05\x84\x82\x85\x01a>iV[\x94\x93PPPPV[_a\x01`\x82\x84\x03\x12\x15a?\x1EW_\x80\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a?\x1EW_\x80\xFD[_\x80_``\x84\x86\x03\x12\x15a?FW_\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?\\W_\x80\xFD[a?h\x87\x83\x88\x01a?\rV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a?}W_\x80\xFD[a?\x89\x87\x83\x88\x01a?$V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a?\x9EW_\x80\xFD[P\x84\x01`\xC0\x81\x87\x03\x12\x15a?\xB0W_\x80\xFD[\x80\x91PP\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a?\xCBW_\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\xABW_\x80\xFD[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a?\xFDW_\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\x13W_\x80\xFD[a@\x1F\x8A\x83\x8B\x01a?\rV[\x97P` \x89\x015\x91P\x80\x82\x11\x15a@4W_\x80\xFD[a@@\x8A\x83\x8B\x01a?$V[\x96Pa@N`@\x8A\x01a?\xD2V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a@cW_\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a@vW_\x80\xFD[\x815\x81\x81\x11\x15a@\x84W_\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a@\x95W_\x80\xFD[` \x83\x01\x95P\x80\x94PPPPa@\xAD`\x80\x88\x01a?\xD2V[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80`@\x83\x85\x03\x12\x15a@\xCAW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\xE0W_\x80\xFD[a@\xEC\x86\x83\x87\x01a?$V[\x93P` \x85\x015\x91P\x80\x82\x11\x15aA\x01W_\x80\xFD[PaA\x0E\x85\x82\x86\x01a?\rV[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`~\x19\x836\x03\x01\x81\x12aA@W_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a'\xABW_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aAuW_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x93W_\x80\xFD[\x806\x03\x82\x13\x15aA\xA1W_\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`@\x81R_`\x01`\x01`@\x1B\x03\x80aA\xE7\x86aAJV[\x16`@\x84\x01RaA\xFA` \x86\x01\x86aA`V[`\x80``\x86\x01RaB\x0F`\xC0\x86\x01\x82\x84aA\xA8V[\x91PP`@\x86\x015`\x80\x85\x01RaB(``\x87\x01a?\xD2V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x86\x01R\x84\x82\x03` \x86\x01Ra\x01`\x90aBo\x83\x88\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[aB\xA0`\x80\x84\x01`\x80\x89\x01\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[a\x01\0\x81aB\xAF\x82\x8A\x01a?\xD2V[\x16\x90\x84\x01RPa\x01 \x83aB\xC4\x88\x83\x01aAJV[\x16\x90\x83\x01Ra\x01@\x92PaB\xDA\x86\x84\x01\x87aA`V[\x82\x85\x85\x01RaB\xEC\x83\x85\x01\x82\x84aA\xA8V[\x99\x98PPPPPPPPPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aC\x0EW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aC'W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aA\xA1W_\x80\xFD[_` \x82\x84\x03\x12\x15aCKW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aCaW_\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15aCtW_\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15aC\x8FWaC\x8Fa=\xFDV[`@RaC\x9B\x83aAJV[\x81R` \x83\x015\x82\x81\x11\x15aC\xAEW_\x80\xFD[aC\xBA\x87\x82\x86\x01a>iV[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15aC\xD1W_\x80\xFD[aC\xDD\x87\x82\x86\x01a>iV[`@\x83\x01RP\x95\x94PPPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aD\x04WaD\x04a=\xFDV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aD\x1DW_\x80\xFD[\x815` aD2aD-\x83aC\xECV[a>9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aDPW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aD\x8DW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aDqW_\x80\xFD[aD\x7F\x89\x86\x83\x8B\x01\x01a>iV[\x84RP\x91\x83\x01\x91\x83\x01aDTV[P\x96\x95PPPPPPV[_\x82`\x1F\x83\x01\x12aD\xA7W_\x80\xFD[\x815` aD\xB7aD-\x83aC\xECV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15aD\xD8W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15aD\x8DW\x805\x83R\x91\x83\x01\x91\x83\x01aD\xDDV[_`\xC0\x826\x03\x12\x15aE\x04W_\x80\xFD[aE\x0Ca>\x11V[\x825\x81R` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aE)W_\x80\xFD[aE56\x83\x87\x01a>iV[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15aEMW_\x80\xFD[aEY6\x83\x87\x01a>iV[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15aEqW_\x80\xFD[aE}6\x83\x87\x01a>iV[``\x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15aE\x95W_\x80\xFD[aE\xA16\x83\x87\x01aD\x0EV[`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15aE\xB9W_\x80\xFD[PaE\xC66\x82\x86\x01aD\x98V[`\xA0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aE\xE2W_\x80\xFD[a\x13u\x82a?\xD2V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06+Wa\x06+aE\xEBV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aF4WaF4aF\x12V[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x06+Wa\x06+aE\xEBV[_\x825a\x01^\x19\x836\x03\x01\x81\x12aA@W_\x80\xFD[_` \x82\x84\x03\x12\x15aFqW_\x80\xFD[a\x13u\x82aAJV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06+Wa\x06+aE\xEBV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x82\x81R_a?\x05` \x83\x01\x84aF\xA5V[_`\x01\x82\x01aF\xDEWaF\xDEaE\xEBV[P`\x01\x01\x90V[_a?\x05aF\xF3\x83\x86aF\xA5V[\x84aF\xA5V[_`\xFF\x83\x16\x80aG\x0BWaG\x0BaF\x12V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x06+Wa\x06+aE\xEBV[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x1B(Wa\x1B(aE\xEBV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80aGlWaGlaF\x12V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80aG\x91WaG\x91aF\x12V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x1B(Wa\x1B(aE\xEBV[\x92\x83R` \x83\x01\x91\x90\x91R`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`@\x82\x01R`A\x01\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x06+Wa\x06+aE\xEBV[`\x01\x81\x81[\x80\x85\x11\x15aH3W\x81_\x19\x04\x82\x11\x15aH\x19WaH\x19aE\xEBV[\x80\x85\x16\x15aH&W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90aG\xFEV[P\x92P\x92\x90PV[_\x82aHIWP`\x01a\x06+V[\x81aHUWP_a\x06+V[\x81`\x01\x81\x14aHkW`\x02\x81\x14aHuWaH\x91V[`\x01\x91PPa\x06+V[`\xFF\x84\x11\x15aH\x86WaH\x86aE\xEBV[PP`\x01\x82\x1Ba\x06+V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aH\xB4WP\x81\x81\na\x06+V[aH\xBE\x83\x83aG\xF9V[\x80_\x19\x04\x82\x11\x15aH\xD1WaH\xD1aE\xEBV[\x02\x93\x92PPPV[_a\x13u\x83\x83aH;V[_\x82aH\xF2WaH\xF2aF\x12V[P\x06\x90V[_` \x82\x84\x03\x12\x15aI\x07W_\x80\xFD[PQ\x91\x90PV[`\xF8\x83\x90\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x81R_a?\x05`\x01\x83\x01\x84aF\xA5V\xFE\xA2dipfsX\"\x12 \x03\xE4Z }\xC3\x9D\xCA\xCB%\x884l '\xA2\xE0jm\x8A\xBD\xA1\xC7\xD4\x87N0\x8E\x01\x81\xC6MdsolcC\0\x08\x19\x003",
    );
	/// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610105575f3560e01c80639f329d0b11610092578063cac62dd911610062578063cac62dd91461031f578063ccfa336f14610334578063d62aad2914610347578063f45e81181461035c578063f5beea8c14610371575f80fd5b80639f329d0b1461026f578063abeeb36614610284578063c1e69b66146102a3578063c239ef4214610300575f80fd5b80635ddc9e8d116100d85780635ddc9e8d146101c25780636ef8d437146101f9578063743770291461021a57806386fb61ed1461022e5780639790ce7114610254575f80fd5b8063189cc9d0146101095780632a04ff85146101305780634968f6c51461014457806356b4a92a14610158575b5f80fd5b348015610114575f80fd5b5061011d610390565b6040519081526020015b60405180910390f35b34801561013b575f80fd5b5061011d600c81565b34801561014f575f80fd5b5061011d5f5481565b348015610163575f80fd5b50610177610172366004613ed4565b61039f565b60405161012791905f60c082019050825182526020830151602083015260408301516040830152606083015160608301526080830151608083015260a083015160a083015292915050565b3480156101cd575f80fd5b506002546101e1906001600160a01b031681565b6040516001600160a01b039091168152602001610127565b348015610204575f80fd5b50610218610213366004613f34565b6104b0565b005b348015610225575f80fd5b5061011d602081565b348015610239575f80fd5b506101e1720f3df6d732807ef1319fb7b8bb8522d0beac0281565b34801561025f575f80fd5b5061011d670de0b6b3a764000081565b34801561027a575f80fd5b5061011d60015481565b34801561028f575f80fd5b5061011d61029e366004613fbb565b610610565b3480156102ae575f80fd5b506102e16102bd366004613fbb565b60036020525f9081526040902080546001909101546001600160a01b039091169082565b604080516001600160a01b039093168352602083019190915201610127565b34801561030b575f80fd5b5061011d61031a366004613fe8565b610631565b34801561032a575f80fd5b5061011d611fff81565b61011d6103423660046140b9565b6107a7565b348015610352575f80fd5b5061011d611c2081565b348015610367575f80fd5b5061011d61010081565b34801561037c575f80fd5b5061011d61038b366004613fbb565b610907565b5f61039a42610610565b905090565b6040805160c0810182525f80825260208201819052918101829052606081018290526080810182905260a08101829052906103e16103dc84610920565b61094c565b9050610405815f815181106103f8576103f8614118565b6020026020010151610b37565b8252805161042090829060039081106103f8576103f8614118565b6020830152805161043e90829060049081106103f8576103f8614118565b60408301528051610469908290600890811061045c5761045c614118565b6020026020010151610c34565b60608301528051610487908290600b90811061045c5761045c614118565b608083015280516104a5908290600f90811061045c5761045c614118565b60a083015250919050565b5f6104bb838061412c565b846040516020016104cd9291906141d0565b60408051601f1981840301815282825280516020918201205f8181526003835283902084840190935282546001600160a01b031680855260019093015491840191909152925061052f57604051623126e360e41b815260040160405180910390fd5b5f61053a858061412c565b6105489060208101906142f9565b810190610555919061433b565b905061057b81610564866144f4565b6105766101208a016101008b016145d2565b610c3e565b5f8381526003602052604080822080546001600160a01b0319168155600101829055513390670de0b6b3a7640000908381818185875af1925050503d805f81146105e0576040519150601f19603f3d011682016040523d82523d5f602084013e6105e5565b606091505b505090508061060757604051630db2c7f160e31b815260040160405180910390fd5b50505050505050565b5f600c6001548361062191906145ff565b61062b9190614626565b92915050565b6002545f906001600160a01b0316331461065e576040516363e58f5760e11b815260040160405180910390fd5b5f86886040516020016106729291906141d0565b60408051808303601f1901815291815281516020928301205f81815260039093529120549091506001600160a01b038481169116146106c457604051637546e8c160e01b815260040160405180910390fd5b5f8181526003602052604090206001015442906106e490611c2090614639565b111561070357604051631f304cc160e01b815260040160405180910390fd5b5f8181526003602052604080822080546001600160a01b0319168155600101829055516001600160a01b03851690670de0b6b3a7640000908381818185875af1925050503d805f8114610771576040519150601f19603f3d011682016040523d82523d5f602084013e610776565b606091505b505090508061079857604051630db2c7f160e31b815260040160405180910390fd5b50505f54979650505050505050565b5f670de0b6b3a764000034146107d057604051634dc660a360e11b815260040160405180910390fd5b5f6107db848061412c565b6107e99060208101906142f9565b8101906107f6919061433b565b9050610802848061412c565b61080c848061464c565b60405160200161081d9291906141d0565b60408051601f1981840301815291815281516020928301205f81815260039093529120549092506001600160a01b03161561086b57604051634450435760e01b815260040160405180910390fd5b80516001600160401b0316610880848061464c565b6108939061014081019061012001614661565b6001600160401b0316146108ba576040516330d3ba0760e01b815260040160405180910390fd5b506040805180820182523381524260208083019182525f858152600390915292909220905181546001600160a01b0319166001600160a01b03909116178155905160019091015592915050565b5f610913600c8361467a565b60015461062b9190614639565b6040805180820182525f8082526020918201528151808301909252825182529182019181019190915290565b60605f8061095984610e5c565b9193509091506001905081600181111561097557610975614691565b146109c75760405162461bcd60e51b815260206004820152601760248201527f496e76616c696420524c50206c6973742076616c75652e00000000000000000060448201526064015b60405180910390fd5b60408051602080825261042082019092525f91816020015b604080518082019091525f80825260208201528152602001906001900390816109df5790505090505f835b8651811015610b2c5760208210610a765760405162461bcd60e51b815260206004820152602a60248201527f50726f766964656420524c50206c6973742065786365656473206d6178206c6960448201526939ba103632b733ba341760b11b60648201526084016109be565b5f80610ab16040518060400160405280858c5f0151610a9591906145ff565b8152602001858c60200151610aaa9190614639565b9052610e5c565b509150915060405180604001604052808383610acd9190614639565b8152602001848b60200151610ae29190614639565b815250858581518110610af757610af7614118565b6020908102919091010152610b0d600185614639565b9350610b198183614639565b610b239084614639565b92505050610a0a565b508152949350505050565b5f6021825f01511115610b8c5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020627974657333322076616c75652e00000000000060448201526064016109be565b5f805f610b9885610e5c565b919450925090505f816001811115610bb257610bb2614691565b14610bff5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020627974657333322076616c75652e00000000000060448201526064016109be565b5f838660200151610c109190614639565b80519091506020841015610c2a5760208490036101000a90045b9695505050505050565b5f61062b82610b37565b82516001600160401b03166020610c53610390565b610c5d91906145ff565b811115610c7d5760405163b6144bff60e01b815260040160405180910390fd5b610100610c88610390565b610c9291906145ff565b811015610cb257604051630cdceb7960e21b815260040160405180910390fd5b82515f90610cc2906001906145ff565b905043811180610cdc5750610cd9610100436145ff565b81105b15610cfa57604051631391e11b60e21b815260040160405180910390fd5b83515f90610d0a906001906145ff565b602080870151805191012090409150808214610d3957604051631f03465b60e11b815260040160405180910390fd5b5f80610d4489611156565b9250925050816001600160a01b0316876001600160a01b031614610d7b5760405163aaaa914160e01b815260040160405180910390fd5b5f610d89896040015161039f565b80519091508414610dad576040516320fa6c8b60e11b815260040160405180910390fd5b5f610dd48a60a001515f81518110610dc757610dc7614118565b60200260200101516111de565b90505f80610e04838d608001515f81518110610df257610df2614118565b602002602001015186604001516111f1565b9150915081610e265760405163094cec5f60e01b815260040160405180910390fd5b80516020820120855114610e4d57604051634462b38f60e11b815260040160405180910390fd5b50505050505050505050505050565b5f805f80845f015111610eb15760405162461bcd60e51b815260206004820152601860248201527f524c50206974656d2063616e6e6f74206265206e756c6c2e000000000000000060448201526064016109be565b602084015180515f1a607f8111610ed3575f60015f945094509450505061114f565b60b78111610f42578551607f198201908110610f315760405162461bcd60e51b815260206004820152601960248201527f496e76616c696420524c502073686f727420737472696e672e0000000000000060448201526064016109be565b6001955093505f925061114f915050565b60bf811161101957855160b6198201908110610fa05760405162461bcd60e51b815260206004820152601f60248201527f496e76616c696420524c50206c6f6e6720737472696e67206c656e6774682e0060448201526064016109be565b5f816020036101000a6001850151049050808201885f0151116110055760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420524c50206c6f6e6720737472696e672e000000000000000060448201526064016109be565b6001909101955093505f925061114f915050565b60f7811161108857855160bf1982019081106110775760405162461bcd60e51b815260206004820152601760248201527f496e76616c696420524c502073686f7274206c6973742e00000000000000000060448201526064016109be565b60019550935084925061114f915050565b855160f61982019081106110de5760405162461bcd60e51b815260206004820152601d60248201527f496e76616c696420524c50206c6f6e67206c697374206c656e6774682e00000060448201526064016109be565b5f816020036101000a6001850151049050808201885f01511161113c5760405162461bcd60e51b815260206004820152601660248201527524b73b30b634b210292628103637b733903634b9ba1760511b60448201526064016109be565b600191820196509450925061114f915050565b9193909250565b604080516060810182525f80825260208201819052918101829052819061118961117f856112c6565b856020015161130d565b91505f6111998560400151611335565b90506111a4816113ef565b935060405180606001604052808660400151805190602001208152602001826060015181526020018260e001518152509150509193909250565b606061062b6111ec8361140a565b611527565b5f60605f6111fe85611595565b90505f805f61120e848a89611680565b815192955090935091501580806112225750815b61126e5760405162461bcd60e51b815260206004820152601a60248201527f50726f76696465642070726f6f6620697320696e76616c69642e00000000000060448201526064016109be565b5f816112885760405180602001604052805f8152506112b4565b6112b4866112976001886145ff565b815181106112a7576112a7614118565b6020026020010151611a85565b919b919a509098505050505050505050565b5f8160400151805190602001206112df835f0151611aaf565b6040516020016112f09291906146bc565b604051602081830303815290604052805190602001209050919050565b5f805f8061131b8686611b2f565b92509250925061132b8282611b78565b5090949350505050565b61133d613d6e565b5f825f8151811061135057611350614118565b01602001516001600160f81b0319169050607f60f81b811061137c5761137583611c34565b9392505050565b6001600160f81b03198116600160f81b0361139a5761137583611e6a565b6001600160f81b03198116600160f91b036113b8576113758361212a565b6001600160f81b03198116600360f81b036113d657611375836123af565b604051636fc3daa360e11b815260040160405180910390fd5b5f61062b6113fc8361273c565b61140584612754565b61130d565b60605f8260405160200161142091815260200190565b60405160208183030381529060405290505f5b602081101561146b5781818151811061144e5761144e614118565b01602001516001600160f81b0319165f0361146b57600101611433565b5f6114778260206145ff565b6001600160401b0381111561148e5761148e613dfd565b6040519080825280601f01601f1916602001820160405280156114b8576020820181803683370190505b5090505f5b815181101561151e5783836114d1816146cd565b9450815181106114e3576114e3614118565b602001015160f81c60f81b82828151811061150057611500614118565b60200101906001600160f81b03191690815f1a9053506001016114bd565b50949350505050565b6060808251600114801561155457506080835f8151811061154a5761154a614118565b016020015160f81c105b1561156057508161062b565b61156c835160806127b0565b8360405160200161157e9291906146e5565b604051602081830303815290604052905092915050565b60605f6115a183612957565b90505f81516001600160401b038111156115bd576115bd613dfd565b60405190808252806020026020018201604052801561160257816020015b60408051808201909152606080825260208201528152602001906001900390816115db5790505b5090505f5b8251811015611678575f61163384838151811061162657611626614118565b6020026020010151612965565b9050604051806040016040528082815260200161164f83612957565b81525083838151811061166457611664614118565b602090810291909101015250600101611607565b509392505050565b5f606081808061168f876129f2565b90505f8690505f806116b4604051806040016040528060608152602001606081525090565b5f5b8c51811015611a5d578c81815181106116d1576116d1614118565b6020026020010151915082846116e79190614639565b93506116f4600188614639565b9650835f0361174e578151805160209091012085146117495760405162461bcd60e51b8152602060048201526011602482015270092dcecc2d8d2c840e4dedee840d0c2e6d607b1b60448201526064016109be565b61180a565b8151516020116117b0578151805160209091012085146117495760405162461bcd60e51b815260206004820152601b60248201527f496e76616c6964206c6172676520696e7465726e616c2068617368000000000060448201526064016109be565b846117bd835f0151612b02565b1461180a5760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420696e7465726e616c206e6f6465206861736800000000000060448201526064016109be565b61181660106001614639565b826020015151036118865785518414611a5d575f86858151811061183c5761183c614118565b602001015160f81c60f81b60f81c90505f83602001518260ff168151811061186657611866614118565b6020026020010151905061187981612b29565b9650600194505050611a55565b600282602001515103611a0d575f61189d83612b5d565b90505f815f815181106118b2576118b2614118565b016020015160f81c90505f6118c86002836146f9565b6118d390600261471a565b90505f6118e3848360ff16612b80565b90505f6118f08b8a612b80565b90505f6118fd8383612bae565b905060ff851660021480611914575060ff85166003145b1561194e578083511480156119295750808251145b1561193b57611938818b614639565b99505b50600160ff1b9950611a5d945050505050565b60ff85161580611961575060ff85166001145b156119b657805f036119805750600160ff1b9950611a5d945050505050565b6119a7886020015160018151811061199a5761199a614118565b6020026020010151612b29565b9a509750611a55945050505050565b60405162461bcd60e51b815260206004820152602660248201527f52656365697665642061206e6f6465207769746820616e20756e6b6e6f776e206044820152650e0e4caccd2f60d31b60648201526084016109be565b60405162461bcd60e51b815260206004820152601c60248201527f526563656976656420616e20756e7061727361626c65206e6f64652e0000000060448201526064016109be565b6001016116b6565b50600160ff1b841486611a708786612b80565b909e909d50909b509950505050505050505050565b6020810151805160609161062b91611a9f906001906145ff565b8151811061162657611626614118565b6040805160088082528183019092526060915f91906020820181803683370190505090505f5b6008811015611b2857611ae981600861467a565b846001600160401b0316901c60f81b828281518110611b0a57611b0a614118565b60200101906001600160f81b03191690815f1a905350600101611ad5565b5092915050565b5f805f8351604103611b66576020840151604085015160608601515f1a611b5888828585612c29565b955095509550505050611b71565b505081515f91506002905b9250925092565b5f826003811115611b8b57611b8b614691565b03611b94575050565b6001826003811115611ba857611ba8614691565b03611bc65760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115611bda57611bda614691565b03611bfb5760405163fce698f760e01b8152600481018290526024016109be565b6003826003811115611c0f57611c0f614691565b03611c30576040516335e2f38360e21b8152600481018290526024016109be565b5050565b611c3c613d6e565b5f808252611c4c6103dc84610920565b90508051600914158015611c6257508051600614155b15611c805760405163c2871a3760e01b815260040160405180910390fd5b611c95815f8151811061045c5761045c614118565b60608301528051611cb3908290600190811061045c5761045c614118565b60808301528051611cd1908290600290811061045c5761045c614118565b60e08301528051611cfc9082906003908110611cef57611cef614118565b6020026020010151612cf1565b6001600160a01b03166101008301528051611d24908290600490811061045c5761045c614118565b6101208301528051611d43908290600590811061162657611626614118565b6101408301528051600603611d585750919050565b5f611d6f8260068151811061045c5761045c614118565b90505f611d888360078151811061045c5761045c614118565b90505f611da18460088151811061045c5761045c614118565b905081158015611daf575080155b15611dcf576001600160401b038316602086015260016040860152611e61565b6023836001600160401b031610611e17576002611ded602385614733565b611df79190614753565b6001600160401b03908116602087015283166101e0860152600160408601525b5f611e26600260018618614778565b611e3190601b61479d565b604051909150611e49908490849084906020016147bd565b60408051601f198184030181529190526101c0870152505b50505050919050565b611e72613d6e565b600180825282515f91611e9291611e8a9082906145ff565b859190612d5d565b90505f611ea16103dc83610920565b90508051600814158015611eb757508051600b14155b15611ed55760405163c2871a3760e01b815260040160405180910390fd5b611eea815f8151811061045c5761045c614118565b6001600160401b031660208401528051611f11908290600190811061045c5761045c614118565b60608401528051611f2f908290600290811061045c5761045c614118565b60808401528051611f4d908290600390811061045c5761045c614118565b60e08401528051611f6b9082906004908110611cef57611cef614118565b6001600160a01b03166101008401528051611f93908290600590811061045c5761045c614118565b6101208401528051611fb2908290600690811061162657611626614118565b8361014001819052505f611fdf82600781518110611fd257611fd2614118565b602002602001015161094c565b905080516001600160401b03811115611ffa57611ffa613dfd565b60405190808252806020026020018201604052801561202d57816020015b60608152602001906001900390816120185790505b506101608501525f5b815181101561208b5761206182828151811061205457612054614118565b6020026020010151612e9d565b856101600151828151811061207857612078614118565b6020908102919091010152600101612036565b50815160080361209d57505050919050565b5f6120b48360088151811061045c5761045c614118565b6120bf90601b6147e0565b90505f6120d8846009815181106103f8576103f8614118565b90505f6120f185600a815181106103f8576103f8614118565b9050818184604051602001612108939291906147bd565b60408051601f198184030181529190526101c088015250949695505050505050565b612132613d6e565b6002815281515f9061214c90600190611e8a9082906145ff565b90505f61215b6103dc83610920565b9050805160091415801561217157508051600c14155b1561218f5760405163c2871a3760e01b815260040160405180910390fd5b6121a4815f8151811061045c5761045c614118565b6001600160401b0316602084015280516121cb908290600190811061045c5761045c614118565b606084015280516121e9908290600290811061045c5761045c614118565b60a08401528051612207908290600390811061045c5761045c614118565b60c08401528051612225908290600490811061045c5761045c614118565b60e084015280516122439082906005908110611cef57611cef614118565b6001600160a01b0316610100840152805161226b908290600690811061045c5761045c614118565b610120840152805161228a908290600790811061162657611626614118565b8361014001819052505f6122aa82600881518110611fd257611fd2614118565b905080516001600160401b038111156122c5576122c5613dfd565b6040519080825280602002602001820160405280156122f857816020015b60608152602001906001900390816122e35790505b506101608501525f5b81518110156123495761231f82828151811061205457612054614118565b856101600151828151811061233657612336614118565b6020908102919091010152600101612301565b50815160090361235b57505050919050565b5f6123728360098151811061045c5761045c614118565b61237d90601b6147e0565b90505f61239684600a815181106103f8576103f8614118565b90505f6120f185600b815181106103f8576103f8614118565b6123b7613d6e565b6003815281515f906123d190600190611e8a9082906145ff565b90505f6123e06103dc83610920565b90508051600b141580156123f657508051600e14155b156124145760405163c2871a3760e01b815260040160405180910390fd5b612429815f8151811061045c5761045c614118565b6001600160401b031660208401528051612450908290600190811061045c5761045c614118565b6060840152805161246e908290600290811061045c5761045c614118565b60a0840152805161248c908290600390811061045c5761045c614118565b60c084015280516124aa908290600490811061045c5761045c614118565b60e084015280516124c89082906005908110611cef57611cef614118565b6001600160a01b031661010084015280516124f0908290600690811061045c5761045c614118565b610120840152805161250f908290600790811061162657611626614118565b8361014001819052505f61252f82600881518110611fd257611fd2614118565b905080516001600160401b0381111561254a5761254a613dfd565b60405190808252806020026020018201604052801561257d57816020015b60608152602001906001900390816125685790505b506101608501525f5b81518110156125ce576125a482828151811061205457612054614118565b85610160015182815181106125bb576125bb614118565b6020908102919091010152600101612586565b506125e58260098151811061045c5761045c614118565b846101800181815250505f61260683600a81518110611fd257611fd2614118565b905080516001600160401b0381111561262157612621613dfd565b60405190808252806020026020018201604052801561264a578160200160208202803683370190505b506101a08601525f5b815181101561269b576126718282815181106103f8576103f8614118565b866101a00151828151811061268857612688614118565b6020908102919091010152600101612653565b508251600b036126ae5750505050919050565b5f6126c584600b8151811061045c5761045c614118565b6126d090601b6147e0565b90505f6126e985600c815181106103f8576103f8614118565b90505f61270286600d815181106103f8576103f8614118565b9050818184604051602001612719939291906147bd565b60408051601f198184030181529190526101c08901525095979650505050505050565b5f61274682612ea8565b805190602001209050919050565b6060816101c00151515f0361277c5760405163d466bd1560e01b815260040160405180910390fd5b816101c00151516041146127a357604051634be6321b60e01b815260040160405180910390fd5b506101c08101515b919050565b606080603884101561281557604080516001808252818301909252906020820181803683370190505090506127e583856147e0565b60f81b815f815181106127fa576127fa614118565b60200101906001600160f81b03191690815f1a905350611375565b5f60015b6128238187614626565b156128495781612832816146cd565b925061284290506101008261467a565b9050612819565b612854826001614639565b6001600160401b0381111561286b5761286b613dfd565b6040519080825280601f01601f191660200182016040528015612895576020820181803683370190505b5092506128a285836147e0565b6128ad9060376147e0565b60f81b835f815181106128c2576128c2614118565b60200101906001600160f81b03191690815f1a905350600190505b81811161294e576101006128f182846145ff565b6128fd906101006148d9565b6129079088614626565b61291191906148e4565b60f81b83828151811061292657612926614118565b60200101906001600160f81b03191690815f1a90535080612946816146cd565b9150506128dd565b50509392505050565b606061062b6103dc83610920565b60605f805f61297385610e5c565b919450925090505f81600181111561298d5761298d614691565b146129da5760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420524c502062797465732076616c75652e000000000000000060448201526064016109be565b6129e985602001518484612f35565b95945050505050565b60605f82516002026001600160401b03811115612a1157612a11613dfd565b6040519080825280601f01601f191660200182016040528015612a3b576020820181803683370190505b5090505f5b8351811015611b28576004848281518110612a5d57612a5d614118565b602001015160f81c60f81b6001600160f81b031916901c828260020281518110612a8957612a89614118565b60200101906001600160f81b03191690815f1a9053506010848281518110612ab357612ab3614118565b016020015160f81c81612ac857612ac8614612565b0660f81b828260020260010181518110612ae457612ae4614118565b60200101906001600160f81b03191690815f1a905350600101612a40565b5f602082511015612b1557506020015190565b8180602001905181019061062b91906148f7565b5f60606020835f01511015612b4857612b4183612e9d565b9050612b54565b612b5183612965565b90505b61137581612b02565b606061062b612b7b83602001515f8151811061162657611626614118565b6129f2565b6060818351035f03612ba0575060408051602081019091525f815261062b565b611375838384865103612d5d565b5f805b808451118015612bc15750808351115b8015612c125750828181518110612bda57612bda614118565b602001015160f81c60f81b6001600160f81b031916848281518110612c0157612c01614118565b01602001516001600160f81b031916145b156113755780612c21816146cd565b915050612bb1565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841115612c6257505f91506003905082612ce7565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015612cb3573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116612cde57505f925060019150829050612ce7565b92505f91508190505b9450945094915050565b80515f90600103612d0357505f919050565b8151601514612d545760405162461bcd60e51b815260206004820152601a60248201527f496e76616c696420524c5020616464726573732076616c75652e00000000000060448201526064016109be565b61062b82610c34565b60608182601f011015612da35760405162461bcd60e51b815260206004820152600e60248201526d736c6963655f6f766572666c6f7760901b60448201526064016109be565b828284011015612de65760405162461bcd60e51b815260206004820152600e60248201526d736c6963655f6f766572666c6f7760901b60448201526064016109be565b81830184511015612e2d5760405162461bcd60e51b8152602060048201526011602482015270736c6963655f6f75744f66426f756e647360781b60448201526064016109be565b606082158015612e4b5760405191505f82526020820160405261151e565b6040519150601f8416801560200281840101858101878315602002848b0101015b81831015612e84578051835260209283019201612e6c565b5050858452601f01601f19166040525050949350505050565b606061062b82612fe2565b60605f82516003811115612ebe57612ebe614691565b03612ecc5761062b82612ff6565b600182516003811115612ee157612ee1614691565b03612eef5761062b826132e2565b600282516003811115612f0457612f04614691565b03612f125761062b82613553565b600382516003811115612f2757612f27614691565b036113d65761062b826137c1565b60605f826001600160401b03811115612f5057612f50613dfd565b6040519080825280601f01601f191660200182016040528015612f7a576020820181803683370190505b50905080515f03612f8c579050611375565b848401602082015f5b60208604811015612fb6578251825260209283019290910190600101612f95565b505f6001602087066020036101000a039050808251168119845116178252839450505050509392505050565b606061062b82602001515f845f0151612f35565b60605f82602001516001600160401b03165f146130185750602082015161305b565b6101c0830151511561305b576023836101e001516001600160401b03161061305b5760026023846101e0015161304e9190614733565b6130589190614753565b90505b5f836040015161306b575f61306e565b60035b6130799060066147e0565b60ff1690505f816001600160401b0381111561309757613097613dfd565b6040519080825280602002602001820160405280156130ca57816020015b60608152602001906001900390816130b55790505b5090506130da85606001516111de565b815f815181106130ec576130ec614118565b602002602001018190525061310485608001516111de565b8160018151811061311757613117614118565b602002602001018190525061312f8560e001516111de565b8160028151811061314257613142614118565b602002602001018190525061315b856101000151613b64565b8160038151811061316e5761316e614118565b60200260200101819052506131878561012001516111de565b8160048151811061319a5761319a614118565b60200260200101819052506131b3856101400151611527565b816005815181106131c6576131c6614118565b60200260200101819052508460400151156132d95784602001516001600160401b03165f0361322d576040515f60208201526021016040516020818303038152906040528160068151811061321d5761321d614118565b602002602001018190525061325e565b61323f836001600160401b03166111de565b8160068151811061325257613252614118565b60200260200101819052505b6132935f5b6040519080825280601f01601f19166020018201604052801561328d576020820181803683370190505b50611527565b816007815181106132a6576132a6614118565b60209081029190910101526132ba5f613263565b816008815181106132cd576132cd614118565b60200260200101819052505b6129e981613b9c565b60408051600880825261012082019092526060915f9190816020015b60608152602001906001900390816132fe57905050905061332b83602001516001600160401b03166111de565b815f8151811061333d5761333d614118565b602002602001018190525061335583606001516111de565b8160018151811061336857613368614118565b602002602001018190525061338083608001516111de565b8160028151811061339357613393614118565b60200260200101819052506133ab8360e001516111de565b816003815181106133be576133be614118565b60200260200101819052506133d7836101000151613b64565b816004815181106133ea576133ea614118565b60200260200101819052506134038361012001516111de565b8160058151811061341657613416614118565b602002602001018190525061342f836101400151611527565b8160068151811061344257613442614118565b60200260200101819052505f836101600151516001600160401b0381111561346c5761346c613dfd565b60405190808252806020026020018201604052801561349f57816020015b606081526020019060019003908161348a5790505b5090505f5b846101600151518110156134f65784610160015181815181106134c9576134c9614118565b60200260200101518282815181106134e3576134e3614118565b60209081029190910101526001016134a4565b5061350081613b9c565b8260078151811061351357613513614118565b602090810291909101015260015b61352a83613b9c565b60405160200161353b92919061490e565b60405160208183030381529060405292505050919050565b60408051600980825261014082019092526060915f9190816020015b606081526020019060019003908161356f57905050905061359c83602001516001600160401b03166111de565b815f815181106135ae576135ae614118565b60200260200101819052506135c683606001516111de565b816001815181106135d9576135d9614118565b60200260200101819052506135f18360a001516111de565b8160028151811061360457613604614118565b602002602001018190525061361c8360c001516111de565b8160038151811061362f5761362f614118565b60200260200101819052506136478360e001516111de565b8160048151811061365a5761365a614118565b6020026020010181905250613673836101000151613b64565b8160058151811061368657613686614118565b602002602001018190525061369f8361012001516111de565b816006815181106136b2576136b2614118565b60200260200101819052506136cb836101400151611527565b816007815181106136de576136de614118565b60200260200101819052505f836101600151516001600160401b0381111561370857613708613dfd565b60405190808252806020026020018201604052801561373b57816020015b60608152602001906001900390816137265790505b5090505f5b8461016001515181101561379257846101600151818151811061376557613765614118565b602002602001015182828151811061377f5761377f614118565b6020908102919091010152600101613740565b5061379c81613b9c565b826008815181106137af576137af614118565b60209081029190910101526002613521565b60408051600b80825261018082019092526060915f9190816020015b60608152602001906001900390816137dd57905050905061380a83602001516001600160401b03166111de565b815f8151811061381c5761381c614118565b602002602001018190525061383483606001516111de565b8160018151811061384757613847614118565b602002602001018190525061385f8360a001516111de565b8160028151811061387257613872614118565b602002602001018190525061388a8360c001516111de565b8160038151811061389d5761389d614118565b60200260200101819052506138b58360e001516111de565b816004815181106138c8576138c8614118565b60200260200101819052506138e1836101000151613b64565b816005815181106138f4576138f4614118565b602002602001018190525061390d8361012001516111de565b8160068151811061392057613920614118565b6020026020010181905250613939836101400151611527565b8160078151811061394c5761394c614118565b60200260200101819052505f836101600151516001600160401b0381111561397657613976613dfd565b6040519080825280602002602001820160405280156139a957816020015b60608152602001906001900390816139945790505b5090505f5b84610160015151811015613a005784610160015181815181106139d3576139d3614118565b60200260200101518282815181106139ed576139ed614118565b60209081029190910101526001016139ae565b50613a0a81613b9c565b82600881518110613a1d57613a1d614118565b6020026020010181905250613a368461018001516111de565b82600981518110613a4957613a49614118565b60200260200101819052505f846101a00151516001600160401b03811115613a7357613a73613dfd565b604051908082528060200260200182016040528015613aa657816020015b6060815260200190600190039081613a915790505b5090505f5b856101a0015151811015613b0757613ae2866101a001518281518110613ad357613ad3614118565b60200260200101515f1c6111de565b828281518110613af457613af4614118565b6020908102919091010152600101613aab565b50613b1181613b9c565b83600a81518110613b2457613b24614118565b60209081029190910101526003613b3a84613b9c565b604051602001613b4b92919061490e565b6040516020818303038152906040529350505050919050565b604051606082811b6bffffffffffffffffffffffff191660208301529061062b90603401604051602081830303815290604052611527565b60605f613ba883613bdf565b9050613bb6815160c06127b0565b81604051602001613bc89291906146e5565b604051602081830303815290604052915050919050565b606081515f03613bfe57604080515f8082526020820190925290611b28565b5f805b8351811015613c3a57838181518110613c1c57613c1c614118565b60200260200101515182613c309190614639565b9150600101613c01565b5f826001600160401b03811115613c5357613c53613dfd565b6040519080825280601f01601f191660200182016040528015613c7d576020820181803683370190505b505f92509050602081015b855183101561151e575f868481518110613ca457613ca4614118565b602002602001015190505f602082019050613cc183828451613cf7565b878581518110613cd357613cd3614118565b60200260200101515183613ce79190614639565b6001909501949250613c88915050565b8282825b60208110613d335781518352613d12602084614639565b9250613d1f602083614639565b9150613d2c6020826145ff565b9050613cfb565b5f6001613d418360206145ff565b613d4d906101006148d9565b613d5791906145ff565b925184518416931916929092179092525050505050565b604080516102008101909152805f81526020015f6001600160401b031681526020015f151581526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f815260200160608152602001606081526020015f815260200160608152602001606081526020015f6001600160401b031681525090565b634e487b7160e01b5f52604160045260245ffd5b60405160c081016001600160401b0381118282101715613e3357613e33613dfd565b60405290565b604051601f8201601f191681016001600160401b0381118282101715613e6157613e61613dfd565b604052919050565b5f82601f830112613e78575f80fd5b81356001600160401b03811115613e9157613e91613dfd565b613ea4601f8201601f1916602001613e39565b818152846020838601011115613eb8575f80fd5b816020850160208301375f918101602001919091529392505050565b5f60208284031215613ee4575f80fd5b81356001600160401b03811115613ef9575f80fd5b613f0584828501613e69565b949350505050565b5f6101608284031215613f1e575f80fd5b50919050565b5f60808284031215613f1e575f80fd5b5f805f60608486031215613f46575f80fd5b83356001600160401b0380821115613f5c575f80fd5b613f6887838801613f0d565b94506020860135915080821115613f7d575f80fd5b613f8987838801613f24565b93506040860135915080821115613f9e575f80fd5b50840160c08187031215613fb0575f80fd5b809150509250925092565b5f60208284031215613fcb575f80fd5b5035919050565b80356001600160a01b03811681146127ab575f80fd5b5f805f805f8060a08789031215613ffd575f80fd5b86356001600160401b0380821115614013575f80fd5b61401f8a838b01613f0d565b97506020890135915080821115614034575f80fd5b6140408a838b01613f24565b965061404e60408a01613fd2565b95506060890135915080821115614063575f80fd5b818901915089601f830112614076575f80fd5b813581811115614084575f80fd5b8a6020828501011115614095575f80fd5b6020830195508094505050506140ad60808801613fd2565b90509295509295509295565b5f80604083850312156140ca575f80fd5b82356001600160401b03808211156140e0575f80fd5b6140ec86838701613f24565b93506020850135915080821115614101575f80fd5b5061410e85828601613f0d565b9150509250929050565b634e487b7160e01b5f52603260045260245ffd5b5f8235607e19833603018112614140575f80fd5b9190910192915050565b80356001600160401b03811681146127ab575f80fd5b5f808335601e19843603018112614175575f80fd5b83016020810192503590506001600160401b03811115614193575f80fd5b8036038213156141a1575f80fd5b9250929050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b604081525f6001600160401b03806141e78661414a565b1660408401526141fa6020860186614160565b6080606086015261420f60c0860182846141a8565b9150506040860135608085015261422860608701613fd2565b6001600160a01b0390811660a086015284820360208601526101609061426f8388803582526020810135602083015260408101356040830152606081013560608301525050565b6142a06080840160808901803582526020810135602083015260408101356040830152606081013560608301525050565b610100816142af828a01613fd2565b169084015250610120836142c488830161414a565b169083015261014092506142da86840187614160565b82858501526142ec83850182846141a8565b9998505050505050505050565b5f808335601e1984360301811261430e575f80fd5b8301803591506001600160401b03821115614327575f80fd5b6020019150368190038213156141a1575f80fd5b5f6020828403121561434b575f80fd5b81356001600160401b0380821115614361575f80fd5b9083019060608286031215614374575f80fd5b60405160608101818110838211171561438f5761438f613dfd565b60405261439b8361414a565b81526020830135828111156143ae575f80fd5b6143ba87828601613e69565b6020830152506040830135828111156143d1575f80fd5b6143dd87828601613e69565b60408301525095945050505050565b5f6001600160401b0382111561440457614404613dfd565b5060051b60200190565b5f82601f83011261441d575f80fd5b8135602061443261442d836143ec565b613e39565b82815260059290921b84018101918181019086841115614450575f80fd5b8286015b8481101561448d5780356001600160401b03811115614471575f80fd5b61447f8986838b0101613e69565b845250918301918301614454565b509695505050505050565b5f82601f8301126144a7575f80fd5b813560206144b761442d836143ec565b8083825260208201915060208460051b8701019350868411156144d8575f80fd5b602086015b8481101561448d57803583529183019183016144dd565b5f60c08236031215614504575f80fd5b61450c613e11565b8235815260208301356001600160401b0380821115614529575f80fd5b61453536838701613e69565b6020840152604085013591508082111561454d575f80fd5b61455936838701613e69565b60408401526060850135915080821115614571575f80fd5b61457d36838701613e69565b60608401526080850135915080821115614595575f80fd5b6145a13683870161440e565b608084015260a08501359150808211156145b9575f80fd5b506145c636828601614498565b60a08301525092915050565b5f602082840312156145e2575f80fd5b61137582613fd2565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561062b5761062b6145eb565b634e487b7160e01b5f52601260045260245ffd5b5f8261463457614634614612565b500490565b8082018082111561062b5761062b6145eb565b5f823561015e19833603018112614140575f80fd5b5f60208284031215614671575f80fd5b6113758261414a565b808202811582820484141761062b5761062b6145eb565b634e487b7160e01b5f52602160045260245ffd5b5f81518060208401855e5f93019283525090919050565b8281525f613f0560208301846146a5565b5f600182016146de576146de6145eb565b5060010190565b5f613f056146f383866146a5565b846146a5565b5f60ff83168061470b5761470b614612565b8060ff84160691505092915050565b60ff828116828216039081111561062b5761062b6145eb565b6001600160401b03828116828216039080821115611b2857611b286145eb565b5f6001600160401b038084168061476c5761476c614612565b92169190910492915050565b5f6001600160401b038084168061479157614791614612565b92169190910692915050565b6001600160401b03818116838216019080821115611b2857611b286145eb565b928352602083019190915260f81b6001600160f81b031916604082015260410190565b60ff818116838216019081111561062b5761062b6145eb565b600181815b8085111561483357815f1904821115614819576148196145eb565b8085161561482657918102915b93841c93908002906147fe565b509250929050565b5f826148495750600161062b565b8161485557505f61062b565b816001811461486b576002811461487557614891565b600191505061062b565b60ff841115614886576148866145eb565b50506001821b61062b565b5060208310610133831016604e8410600b84101617156148b4575081810a61062b565b6148be83836147f9565b805f19048211156148d1576148d16145eb565b029392505050565b5f611375838361483b565b5f826148f2576148f2614612565b500690565b5f60208284031215614907575f80fd5b5051919050565b60f883901b6001600160f81b03191681525f613f0560018301846146a556fea264697066735822122003e45a207dc39dcacb2588346c2027a2e06a6d8abda1c7d4874e308e0181c64d64736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\x05W_5`\xE0\x1C\x80c\x9F2\x9D\x0B\x11a\0\x92W\x80c\xCA\xC6-\xD9\x11a\0bW\x80c\xCA\xC6-\xD9\x14a\x03\x1FW\x80c\xCC\xFA3o\x14a\x034W\x80c\xD6*\xAD)\x14a\x03GW\x80c\xF4^\x81\x18\x14a\x03\\W\x80c\xF5\xBE\xEA\x8C\x14a\x03qW_\x80\xFD[\x80c\x9F2\x9D\x0B\x14a\x02oW\x80c\xAB\xEE\xB3f\x14a\x02\x84W\x80c\xC1\xE6\x9Bf\x14a\x02\xA3W\x80c\xC29\xEFB\x14a\x03\0W_\x80\xFD[\x80c]\xDC\x9E\x8D\x11a\0\xD8W\x80c]\xDC\x9E\x8D\x14a\x01\xC2W\x80cn\xF8\xD47\x14a\x01\xF9W\x80ct7p)\x14a\x02\x1AW\x80c\x86\xFBa\xED\x14a\x02.W\x80c\x97\x90\xCEq\x14a\x02TW_\x80\xFD[\x80c\x18\x9C\xC9\xD0\x14a\x01\tW\x80c*\x04\xFF\x85\x14a\x010W\x80cIh\xF6\xC5\x14a\x01DW\x80cV\xB4\xA9*\x14a\x01XW[_\x80\xFD[4\x80\x15a\x01\x14W_\x80\xFD[Pa\x01\x1Da\x03\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01;W_\x80\xFD[Pa\x01\x1D`\x0C\x81V[4\x80\x15a\x01OW_\x80\xFD[Pa\x01\x1D_T\x81V[4\x80\x15a\x01cW_\x80\xFD[Pa\x01wa\x01r6`\x04a>\xD4V[a\x03\x9FV[`@Qa\x01'\x91\x90_`\xC0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[4\x80\x15a\x01\xCDW_\x80\xFD[P`\x02Ta\x01\xE1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01'V[4\x80\x15a\x02\x04W_\x80\xFD[Pa\x02\x18a\x02\x136`\x04a?4V[a\x04\xB0V[\0[4\x80\x15a\x02%W_\x80\xFD[Pa\x01\x1D` \x81V[4\x80\x15a\x029W_\x80\xFD[Pa\x01\xE1r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x81V[4\x80\x15a\x02_W_\x80\xFD[Pa\x01\x1Dg\r\xE0\xB6\xB3\xA7d\0\0\x81V[4\x80\x15a\x02zW_\x80\xFD[Pa\x01\x1D`\x01T\x81V[4\x80\x15a\x02\x8FW_\x80\xFD[Pa\x01\x1Da\x02\x9E6`\x04a?\xBBV[a\x06\x10V[4\x80\x15a\x02\xAEW_\x80\xFD[Pa\x02\xE1a\x02\xBD6`\x04a?\xBBV[`\x03` R_\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01'V[4\x80\x15a\x03\x0BW_\x80\xFD[Pa\x01\x1Da\x03\x1A6`\x04a?\xE8V[a\x061V[4\x80\x15a\x03*W_\x80\xFD[Pa\x01\x1Da\x1F\xFF\x81V[a\x01\x1Da\x03B6`\x04a@\xB9V[a\x07\xA7V[4\x80\x15a\x03RW_\x80\xFD[Pa\x01\x1Da\x1C \x81V[4\x80\x15a\x03gW_\x80\xFD[Pa\x01\x1Da\x01\0\x81V[4\x80\x15a\x03|W_\x80\xFD[Pa\x01\x1Da\x03\x8B6`\x04a?\xBBV[a\t\x07V[_a\x03\x9ABa\x06\x10V[\x90P\x90V[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R\x90a\x03\xE1a\x03\xDC\x84a\t V[a\tLV[\x90Pa\x04\x05\x81_\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[` \x02` \x01\x01Qa\x0B7V[\x82R\x80Qa\x04 \x90\x82\x90`\x03\x90\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[` \x83\x01R\x80Qa\x04>\x90\x82\x90`\x04\x90\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[`@\x83\x01R\x80Qa\x04i\x90\x82\x90`\x08\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[` \x02` \x01\x01Qa\x0C4V[``\x83\x01R\x80Qa\x04\x87\x90\x82\x90`\x0B\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x80\x83\x01R\x80Qa\x04\xA5\x90\x82\x90`\x0F\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xA0\x83\x01RP\x91\x90PV[_a\x04\xBB\x83\x80aA,V[\x84`@Q` \x01a\x04\xCD\x92\x91\x90aA\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 _\x81\x81R`\x03\x83R\x83\x90 \x84\x84\x01\x90\x93R\x82T`\x01`\x01`\xA0\x1B\x03\x16\x80\x85R`\x01\x90\x93\x01T\x91\x84\x01\x91\x90\x91R\x92Pa\x05/W`@Qb1&\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x05:\x85\x80aA,V[a\x05H\x90` \x81\x01\x90aB\xF9V[\x81\x01\x90a\x05U\x91\x90aC;V[\x90Pa\x05{\x81a\x05d\x86aD\xF4V[a\x05va\x01 \x8A\x01a\x01\0\x8B\x01aE\xD2V[a\x0C>V[_\x83\x81R`\x03` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01\x82\x90UQ3\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x05\xE0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x05\xE5V[``\x91P[PP\x90P\x80a\x06\x07W`@Qc\r\xB2\xC7\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[_`\x0C`\x01T\x83a\x06!\x91\x90aE\xFFV[a\x06+\x91\x90aF&V[\x92\x91PPV[`\x02T_\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06^W`@Qcc\xE5\x8FW`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x88`@Q` \x01a\x06r\x92\x91\x90aA\xD0V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x03\x90\x93R\x91 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x16\x14a\x06\xC4W`@QcuF\xE8\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x03` R`@\x90 `\x01\x01TB\x90a\x06\xE4\x90a\x1C \x90aF9V[\x11\x15a\x07\x03W`@Qc\x1F0L\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x03` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x01\x82\x90UQ`\x01`\x01`\xA0\x1B\x03\x85\x16\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x07qW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07vV[``\x91P[PP\x90P\x80a\x07\x98W`@Qc\r\xB2\xC7\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP_T\x97\x96PPPPPPPV[_g\r\xE0\xB6\xB3\xA7d\0\x004\x14a\x07\xD0W`@QcM\xC6`\xA3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x07\xDB\x84\x80aA,V[a\x07\xE9\x90` \x81\x01\x90aB\xF9V[\x81\x01\x90a\x07\xF6\x91\x90aC;V[\x90Pa\x08\x02\x84\x80aA,V[a\x08\x0C\x84\x80aFLV[`@Q` \x01a\x08\x1D\x92\x91\x90aA\xD0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x03\x90\x93R\x91 T\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x08kW`@QcDPCW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x01`\x01`@\x1B\x03\x16a\x08\x80\x84\x80aFLV[a\x08\x93\x90a\x01@\x81\x01\x90a\x01 \x01aFaV[`\x01`\x01`@\x1B\x03\x16\x14a\x08\xBAW`@Qc0\xD3\xBA\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x82R3\x81RB` \x80\x83\x01\x91\x82R_\x85\x81R`\x03\x90\x91R\x92\x90\x92 \x90Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90Q`\x01\x90\x91\x01U\x92\x91PPV[_a\t\x13`\x0C\x83aFzV[`\x01Ta\x06+\x91\x90aF9V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R\x82Q\x82R\x91\x82\x01\x91\x81\x01\x91\x90\x91R\x90V[``_\x80a\tY\x84a\x0E\\V[\x91\x93P\x90\x91P`\x01\x90P\x81`\x01\x81\x11\x15a\tuWa\tuaF\x91V[\x14a\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid RLP list value.\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R_\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\xDFW\x90PP\x90P_\x83[\x86Q\x81\x10\x15a\x0B,W` \x82\x10a\nvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FProvided RLP list exceeds max li`D\x82\x01Ri9\xBA\x1062\xB73\xBA4\x17`\xB1\x1B`d\x82\x01R`\x84\x01a\t\xBEV[_\x80a\n\xB1`@Q\x80`@\x01`@R\x80\x85\x8C_\x01Qa\n\x95\x91\x90aE\xFFV[\x81R` \x01\x85\x8C` \x01Qa\n\xAA\x91\x90aF9V[\x90Ra\x0E\\V[P\x91P\x91P`@Q\x80`@\x01`@R\x80\x83\x83a\n\xCD\x91\x90aF9V[\x81R` \x01\x84\x8B` \x01Qa\n\xE2\x91\x90aF9V[\x81RP\x85\x85\x81Q\x81\x10a\n\xF7Wa\n\xF7aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x0B\r`\x01\x85aF9V[\x93Pa\x0B\x19\x81\x83aF9V[a\x0B#\x90\x84aF9V[\x92PPPa\n\nV[P\x81R\x94\x93PPPPV[_`!\x82_\x01Q\x11\x15a\x0B\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP bytes32 value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[_\x80_a\x0B\x98\x85a\x0E\\V[\x91\x94P\x92P\x90P_\x81`\x01\x81\x11\x15a\x0B\xB2Wa\x0B\xB2aF\x91V[\x14a\x0B\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP bytes32 value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[_\x83\x86` \x01Qa\x0C\x10\x91\x90aF9V[\x80Q\x90\x91P` \x84\x10\x15a\x0C*W` \x84\x90\x03a\x01\0\n\x90\x04[\x96\x95PPPPPPV[_a\x06+\x82a\x0B7V[\x82Q`\x01`\x01`@\x1B\x03\x16` a\x0CSa\x03\x90V[a\x0C]\x91\x90aE\xFFV[\x81\x11\x15a\x0C}W`@Qc\xB6\x14K\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\0a\x0C\x88a\x03\x90V[a\x0C\x92\x91\x90aE\xFFV[\x81\x10\x15a\x0C\xB2W`@Qc\x0C\xDC\xEBy`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q_\x90a\x0C\xC2\x90`\x01\x90aE\xFFV[\x90PC\x81\x11\x80a\x0C\xDCWPa\x0C\xD9a\x01\0CaE\xFFV[\x81\x10[\x15a\x0C\xFAW`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q_\x90a\r\n\x90`\x01\x90aE\xFFV[` \x80\x87\x01Q\x80Q\x91\x01 \x90@\x91P\x80\x82\x14a\r9W`@Qc\x1F\x03F[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\rD\x89a\x11VV[\x92P\x92PP\x81`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\r{W`@Qc\xAA\xAA\x91A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\x89\x89`@\x01Qa\x03\x9FV[\x80Q\x90\x91P\x84\x14a\r\xADW`@Qc \xFAl\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\xD4\x8A`\xA0\x01Q_\x81Q\x81\x10a\r\xC7Wa\r\xC7aA\x18V[` \x02` \x01\x01Qa\x11\xDEV[\x90P_\x80a\x0E\x04\x83\x8D`\x80\x01Q_\x81Q\x81\x10a\r\xF2Wa\r\xF2aA\x18V[` \x02` \x01\x01Q\x86`@\x01Qa\x11\xF1V[\x91P\x91P\x81a\x0E&W`@Qc\tL\xEC_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q` \x82\x01 \x85Q\x14a\x0EMW`@QcDb\xB3\x8F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPPPV[_\x80_\x80\x84_\x01Q\x11a\x0E\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FRLP item cannot be null.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[` \x84\x01Q\x80Q_\x1A`\x7F\x81\x11a\x0E\xD3W_`\x01_\x94P\x94P\x94PPPa\x11OV[`\xB7\x81\x11a\x0FBW\x85Q`\x7F\x19\x82\x01\x90\x81\x10a\x0F1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid RLP short string.\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[`\x01\x95P\x93P_\x92Pa\x11O\x91PPV[`\xBF\x81\x11a\x10\x19W\x85Q`\xB6\x19\x82\x01\x90\x81\x10a\x0F\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FInvalid RLP long string length.\0`D\x82\x01R`d\x01a\t\xBEV[_\x81` \x03a\x01\0\n`\x01\x85\x01Q\x04\x90P\x80\x82\x01\x88_\x01Q\x11a\x10\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid RLP long string.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[`\x01\x90\x91\x01\x95P\x93P_\x92Pa\x11O\x91PPV[`\xF7\x81\x11a\x10\x88W\x85Q`\xBF\x19\x82\x01\x90\x81\x10a\x10wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FInvalid RLP short list.\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[`\x01\x95P\x93P\x84\x92Pa\x11O\x91PPV[\x85Q`\xF6\x19\x82\x01\x90\x81\x10a\x10\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FInvalid RLP long list length.\0\0\0`D\x82\x01R`d\x01a\t\xBEV[_\x81` \x03a\x01\0\n`\x01\x85\x01Q\x04\x90P\x80\x82\x01\x88_\x01Q\x11a\x11<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru$\xB7;0\xB64\xB2\x10)&(\x1067\xB73\x9064\xB9\xBA\x17`Q\x1B`D\x82\x01R`d\x01a\t\xBEV[`\x01\x91\x82\x01\x96P\x94P\x92Pa\x11O\x91PPV[\x91\x93\x90\x92PV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x81\x90a\x11\x89a\x11\x7F\x85a\x12\xC6V[\x85` \x01Qa\x13\rV[\x91P_a\x11\x99\x85`@\x01Qa\x135V[\x90Pa\x11\xA4\x81a\x13\xEFV[\x93P`@Q\x80``\x01`@R\x80\x86`@\x01Q\x80Q\x90` \x01 \x81R` \x01\x82``\x01Q\x81R` \x01\x82`\xE0\x01Q\x81RP\x91PP\x91\x93\x90\x92PV[``a\x06+a\x11\xEC\x83a\x14\nV[a\x15'V[_``_a\x11\xFE\x85a\x15\x95V[\x90P_\x80_a\x12\x0E\x84\x8A\x89a\x16\x80V[\x81Q\x92\x95P\x90\x93P\x91P\x15\x80\x80a\x12\"WP\x81[a\x12nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FProvided proof is invalid.\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[_\x81a\x12\x88W`@Q\x80` \x01`@R\x80_\x81RPa\x12\xB4V[a\x12\xB4\x86a\x12\x97`\x01\x88aE\xFFV[\x81Q\x81\x10a\x12\xA7Wa\x12\xA7aA\x18V[` \x02` \x01\x01Qa\x1A\x85V[\x91\x9B\x91\x9AP\x90\x98PPPPPPPPPV[_\x81`@\x01Q\x80Q\x90` \x01 a\x12\xDF\x83_\x01Qa\x1A\xAFV[`@Q` \x01a\x12\xF0\x92\x91\x90aF\xBCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[_\x80_\x80a\x13\x1B\x86\x86a\x1B/V[\x92P\x92P\x92Pa\x13+\x82\x82a\x1BxV[P\x90\x94\x93PPPPV[a\x13=a=nV[_\x82_\x81Q\x81\x10a\x13PWa\x13PaA\x18V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90P`\x7F`\xF8\x1B\x81\x10a\x13|Wa\x13u\x83a\x1C4V[\x93\x92PPPV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x01`\xF8\x1B\x03a\x13\x9AWa\x13u\x83a\x1EjV[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x01`\xF9\x1B\x03a\x13\xB8Wa\x13u\x83a!*V[`\x01`\x01`\xF8\x1B\x03\x19\x81\x16`\x03`\xF8\x1B\x03a\x13\xD6Wa\x13u\x83a#\xAFV[`@Qco\xC3\xDA\xA3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x06+a\x13\xFC\x83a'<V[a\x14\x05\x84a'TV[a\x13\rV[``_\x82`@Q` \x01a\x14 \x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_[` \x81\x10\x15a\x14kW\x81\x81\x81Q\x81\x10a\x14NWa\x14NaA\x18V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16_\x03a\x14kW`\x01\x01a\x143V[_a\x14w\x82` aE\xFFV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x8EWa\x14\x8Ea=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xB8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a\x15\x1EW\x83\x83a\x14\xD1\x81aF\xCDV[\x94P\x81Q\x81\x10a\x14\xE3Wa\x14\xE3aA\x18V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x15\0Wa\x15\0aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x14\xBDV[P\x94\x93PPPPV[``\x80\x82Q`\x01\x14\x80\x15a\x15TWP`\x80\x83_\x81Q\x81\x10a\x15JWa\x15JaA\x18V[\x01` \x01Q`\xF8\x1C\x10[\x15a\x15`WP\x81a\x06+V[a\x15l\x83Q`\x80a'\xB0V[\x83`@Q` \x01a\x15~\x92\x91\x90aF\xE5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[``_a\x15\xA1\x83a)WV[\x90P_\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xBDWa\x15\xBDa=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\x02W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x15\xDBW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x16xW_a\x163\x84\x83\x81Q\x81\x10a\x16&Wa\x16&aA\x18V[` \x02` \x01\x01Qa)eV[\x90P`@Q\x80`@\x01`@R\x80\x82\x81R` \x01a\x16O\x83a)WV[\x81RP\x83\x83\x81Q\x81\x10a\x16dWa\x16daA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x16\x07V[P\x93\x92PPPV[_``\x81\x80\x80a\x16\x8F\x87a)\xF2V[\x90P_\x86\x90P_\x80a\x16\xB4`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[_[\x8CQ\x81\x10\x15a\x1A]W\x8C\x81\x81Q\x81\x10a\x16\xD1Wa\x16\xD1aA\x18V[` \x02` \x01\x01Q\x91P\x82\x84a\x16\xE7\x91\x90aF9V[\x93Pa\x16\xF4`\x01\x88aF9V[\x96P\x83_\x03a\x17NW\x81Q\x80Q` \x90\x91\x01 \x85\x14a\x17IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\t-\xCE\xCC-\x8D,\x84\x0EM\xED\xEE\x84\r\x0C.m`{\x1B`D\x82\x01R`d\x01a\t\xBEV[a\x18\nV[\x81QQ` \x11a\x17\xB0W\x81Q\x80Q` \x90\x91\x01 \x85\x14a\x17IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FInvalid large internal hash\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[\x84a\x17\xBD\x83_\x01Qa+\x02V[\x14a\x18\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid internal node hash\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[a\x18\x16`\x10`\x01aF9V[\x82` \x01QQ\x03a\x18\x86W\x85Q\x84\x14a\x1A]W_\x86\x85\x81Q\x81\x10a\x18<Wa\x18<aA\x18V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_\x83` \x01Q\x82`\xFF\x16\x81Q\x81\x10a\x18fWa\x18faA\x18V[` \x02` \x01\x01Q\x90Pa\x18y\x81a+)V[\x96P`\x01\x94PPPa\x1AUV[`\x02\x82` \x01QQ\x03a\x1A\rW_a\x18\x9D\x83a+]V[\x90P_\x81_\x81Q\x81\x10a\x18\xB2Wa\x18\xB2aA\x18V[\x01` \x01Q`\xF8\x1C\x90P_a\x18\xC8`\x02\x83aF\xF9V[a\x18\xD3\x90`\x02aG\x1AV[\x90P_a\x18\xE3\x84\x83`\xFF\x16a+\x80V[\x90P_a\x18\xF0\x8B\x8Aa+\x80V[\x90P_a\x18\xFD\x83\x83a+\xAEV[\x90P`\xFF\x85\x16`\x02\x14\x80a\x19\x14WP`\xFF\x85\x16`\x03\x14[\x15a\x19NW\x80\x83Q\x14\x80\x15a\x19)WP\x80\x82Q\x14[\x15a\x19;Wa\x198\x81\x8BaF9V[\x99P[P`\x01`\xFF\x1B\x99Pa\x1A]\x94PPPPPV[`\xFF\x85\x16\x15\x80a\x19aWP`\xFF\x85\x16`\x01\x14[\x15a\x19\xB6W\x80_\x03a\x19\x80WP`\x01`\xFF\x1B\x99Pa\x1A]\x94PPPPPV[a\x19\xA7\x88` \x01Q`\x01\x81Q\x81\x10a\x19\x9AWa\x19\x9AaA\x18V[` \x02` \x01\x01Qa+)V[\x9AP\x97Pa\x1AU\x94PPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FReceived a node with an unknown `D\x82\x01Re\x0E\x0EL\xAC\xCD/`\xD3\x1B`d\x82\x01R`\x84\x01a\t\xBEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FReceived an unparsable node.\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[`\x01\x01a\x16\xB6V[P`\x01`\xFF\x1B\x84\x14\x86a\x1Ap\x87\x86a+\x80V[\x90\x9E\x90\x9DP\x90\x9BP\x99PPPPPPPPPPV[` \x81\x01Q\x80Q``\x91a\x06+\x91a\x1A\x9F\x90`\x01\x90aE\xFFV[\x81Q\x81\x10a\x16&Wa\x16&aA\x18V[`@\x80Q`\x08\x80\x82R\x81\x83\x01\x90\x92R``\x91_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`\x08\x81\x10\x15a\x1B(Wa\x1A\xE9\x81`\x08aFzV[\x84`\x01`\x01`@\x1B\x03\x16\x90\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1B\nWa\x1B\naA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x1A\xD5V[P\x92\x91PPV[_\x80_\x83Q`A\x03a\x1BfW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa\x1BX\x88\x82\x85\x85a,)V[\x95P\x95P\x95PPPPa\x1BqV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a\x1B\x8BWa\x1B\x8BaF\x91V[\x03a\x1B\x94WPPV[`\x01\x82`\x03\x81\x11\x15a\x1B\xA8Wa\x1B\xA8aF\x91V[\x03a\x1B\xC6W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x1B\xDAWa\x1B\xDAaF\x91V[\x03a\x1B\xFBW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xBEV[`\x03\x82`\x03\x81\x11\x15a\x1C\x0FWa\x1C\x0FaF\x91V[\x03a\x1C0W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xBEV[PPV[a\x1C<a=nV[_\x80\x82Ra\x1CLa\x03\xDC\x84a\t V[\x90P\x80Q`\t\x14\x15\x80\x15a\x1CbWP\x80Q`\x06\x14\x15[\x15a\x1C\x80W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x95\x81_\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[``\x83\x01R\x80Qa\x1C\xB3\x90\x82\x90`\x01\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x80\x83\x01R\x80Qa\x1C\xD1\x90\x82\x90`\x02\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xE0\x83\x01R\x80Qa\x1C\xFC\x90\x82\x90`\x03\x90\x81\x10a\x1C\xEFWa\x1C\xEFaA\x18V[` \x02` \x01\x01Qa,\xF1V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x83\x01R\x80Qa\x1D$\x90\x82\x90`\x04\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[a\x01 \x83\x01R\x80Qa\x1DC\x90\x82\x90`\x05\x90\x81\x10a\x16&Wa\x16&aA\x18V[a\x01@\x83\x01R\x80Q`\x06\x03a\x1DXWP\x91\x90PV[_a\x1Do\x82`\x06\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[\x90P_a\x1D\x88\x83`\x07\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[\x90P_a\x1D\xA1\x84`\x08\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[\x90P\x81\x15\x80\x15a\x1D\xAFWP\x80\x15[\x15a\x1D\xCFW`\x01`\x01`@\x1B\x03\x83\x16` \x86\x01R`\x01`@\x86\x01Ra\x1EaV[`#\x83`\x01`\x01`@\x1B\x03\x16\x10a\x1E\x17W`\x02a\x1D\xED`#\x85aG3V[a\x1D\xF7\x91\x90aGSV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x87\x01R\x83\x16a\x01\xE0\x86\x01R`\x01`@\x86\x01R[_a\x1E&`\x02`\x01\x86\x18aGxV[a\x1E1\x90`\x1BaG\x9DV[`@Q\x90\x91Pa\x1EI\x90\x84\x90\x84\x90\x84\x90` \x01aG\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x87\x01RP[PPPP\x91\x90PV[a\x1Era=nV[`\x01\x80\x82R\x82Q_\x91a\x1E\x92\x91a\x1E\x8A\x90\x82\x90aE\xFFV[\x85\x91\x90a-]V[\x90P_a\x1E\xA1a\x03\xDC\x83a\t V[\x90P\x80Q`\x08\x14\x15\x80\x15a\x1E\xB7WP\x80Q`\x0B\x14\x15[\x15a\x1E\xD5W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1E\xEA\x81_\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa\x1F\x11\x90\x82\x90`\x01\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[``\x84\x01R\x80Qa\x1F/\x90\x82\x90`\x02\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x80\x84\x01R\x80Qa\x1FM\x90\x82\x90`\x03\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xE0\x84\x01R\x80Qa\x1Fk\x90\x82\x90`\x04\x90\x81\x10a\x1C\xEFWa\x1C\xEFaA\x18V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\x1F\x93\x90\x82\x90`\x05\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[a\x01 \x84\x01R\x80Qa\x1F\xB2\x90\x82\x90`\x06\x90\x81\x10a\x16&Wa\x16&aA\x18V[\x83a\x01@\x01\x81\x90RP_a\x1F\xDF\x82`\x07\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aA\x18V[` \x02` \x01\x01Qa\tLV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xFAWa\x1F\xFAa=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a -W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a \x18W\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a \x8BWa a\x82\x82\x81Q\x81\x10a TWa TaA\x18V[` \x02` \x01\x01Qa.\x9DV[\x85a\x01`\x01Q\x82\x81Q\x81\x10a xWa xaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a 6V[P\x81Q`\x08\x03a \x9DWPPP\x91\x90PV[_a \xB4\x83`\x08\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[a \xBF\x90`\x1BaG\xE0V[\x90P_a \xD8\x84`\t\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P_a \xF1\x85`\n\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P\x81\x81\x84`@Q` \x01a!\x08\x93\x92\x91\x90aG\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x88\x01RP\x94\x96\x95PPPPPPV[a!2a=nV[`\x02\x81R\x81Q_\x90a!L\x90`\x01\x90a\x1E\x8A\x90\x82\x90aE\xFFV[\x90P_a![a\x03\xDC\x83a\t V[\x90P\x80Q`\t\x14\x15\x80\x15a!qWP\x80Q`\x0C\x14\x15[\x15a!\x8FW`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\xA4\x81_\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa!\xCB\x90\x82\x90`\x01\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[``\x84\x01R\x80Qa!\xE9\x90\x82\x90`\x02\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xA0\x84\x01R\x80Qa\"\x07\x90\x82\x90`\x03\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xC0\x84\x01R\x80Qa\"%\x90\x82\x90`\x04\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xE0\x84\x01R\x80Qa\"C\x90\x82\x90`\x05\x90\x81\x10a\x1C\xEFWa\x1C\xEFaA\x18V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa\"k\x90\x82\x90`\x06\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[a\x01 \x84\x01R\x80Qa\"\x8A\x90\x82\x90`\x07\x90\x81\x10a\x16&Wa\x16&aA\x18V[\x83a\x01@\x01\x81\x90RP_a\"\xAA\x82`\x08\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aA\x18V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC5Wa\"\xC5a=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xF8W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\"\xE3W\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a#IWa#\x1F\x82\x82\x81Q\x81\x10a TWa TaA\x18V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a#6Wa#6aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a#\x01V[P\x81Q`\t\x03a#[WPPP\x91\x90PV[_a#r\x83`\t\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[a#}\x90`\x1BaG\xE0V[\x90P_a#\x96\x84`\n\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P_a \xF1\x85`\x0B\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[a#\xB7a=nV[`\x03\x81R\x81Q_\x90a#\xD1\x90`\x01\x90a\x1E\x8A\x90\x82\x90aE\xFFV[\x90P_a#\xE0a\x03\xDC\x83a\t V[\x90P\x80Q`\x0B\x14\x15\x80\x15a#\xF6WP\x80Q`\x0E\x14\x15[\x15a$\x14W`@Qc\xC2\x87\x1A7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$)\x81_\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x80Qa$P\x90\x82\x90`\x01\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[``\x84\x01R\x80Qa$n\x90\x82\x90`\x02\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xA0\x84\x01R\x80Qa$\x8C\x90\x82\x90`\x03\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xC0\x84\x01R\x80Qa$\xAA\x90\x82\x90`\x04\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[`\xE0\x84\x01R\x80Qa$\xC8\x90\x82\x90`\x05\x90\x81\x10a\x1C\xEFWa\x1C\xEFaA\x18V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x84\x01R\x80Qa$\xF0\x90\x82\x90`\x06\x90\x81\x10a\x04\\Wa\x04\\aA\x18V[a\x01 \x84\x01R\x80Qa%\x0F\x90\x82\x90`\x07\x90\x81\x10a\x16&Wa\x16&aA\x18V[\x83a\x01@\x01\x81\x90RP_a%/\x82`\x08\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aA\x18V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%JWa%Ja=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%}W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a%hW\x90P[Pa\x01`\x85\x01R_[\x81Q\x81\x10\x15a%\xCEWa%\xA4\x82\x82\x81Q\x81\x10a TWa TaA\x18V[\x85a\x01`\x01Q\x82\x81Q\x81\x10a%\xBBWa%\xBBaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a%\x86V[Pa%\xE5\x82`\t\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[\x84a\x01\x80\x01\x81\x81RPP_a&\x06\x83`\n\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aA\x18V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&!Wa&!a=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&JW\x81` \x01` \x82\x02\x806\x837\x01\x90P[Pa\x01\xA0\x86\x01R_[\x81Q\x81\x10\x15a&\x9BWa&q\x82\x82\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x86a\x01\xA0\x01Q\x82\x81Q\x81\x10a&\x88Wa&\x88aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a&SV[P\x82Q`\x0B\x03a&\xAEWPPPP\x91\x90PV[_a&\xC5\x84`\x0B\x81Q\x81\x10a\x04\\Wa\x04\\aA\x18V[a&\xD0\x90`\x1BaG\xE0V[\x90P_a&\xE9\x85`\x0C\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P_a'\x02\x86`\r\x81Q\x81\x10a\x03\xF8Wa\x03\xF8aA\x18V[\x90P\x81\x81\x84`@Q` \x01a'\x19\x93\x92\x91\x90aG\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90Ra\x01\xC0\x89\x01RP\x95\x97\x96PPPPPPPV[_a'F\x82a.\xA8V[\x80Q\x90` \x01 \x90P\x91\x90PV[``\x81a\x01\xC0\x01QQ_\x03a'|W`@Qc\xD4f\xBD\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x01\xC0\x01QQ`A\x14a'\xA3W`@QcK\xE62\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x01\xC0\x81\x01Q[\x91\x90PV[``\x80`8\x84\x10\x15a(\x15W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90Pa'\xE5\x83\x85aG\xE0V[`\xF8\x1B\x81_\x81Q\x81\x10a'\xFAWa'\xFAaA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SPa\x13uV[_`\x01[a(#\x81\x87aF&V[\x15a(IW\x81a(2\x81aF\xCDV[\x92Pa(B\x90Pa\x01\0\x82aFzV[\x90Pa(\x19V[a(T\x82`\x01aF9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a(kWa(ka=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a(\x95W` \x82\x01\x81\x806\x837\x01\x90P[P\x92Pa(\xA2\x85\x83aG\xE0V[a(\xAD\x90`7aG\xE0V[`\xF8\x1B\x83_\x81Q\x81\x10a(\xC2Wa(\xC2aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x90P[\x81\x81\x11a)NWa\x01\0a(\xF1\x82\x84aE\xFFV[a(\xFD\x90a\x01\0aH\xD9V[a)\x07\x90\x88aF&V[a)\x11\x91\x90aH\xE4V[`\xF8\x1B\x83\x82\x81Q\x81\x10a)&Wa)&aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x80a)F\x81aF\xCDV[\x91PPa(\xDDV[PP\x93\x92PPPV[``a\x06+a\x03\xDC\x83a\t V[``_\x80_a)s\x85a\x0E\\V[\x91\x94P\x92P\x90P_\x81`\x01\x81\x11\x15a)\x8DWa)\x8DaF\x91V[\x14a)\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid RLP bytes value.\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[a)\xE9\x85` \x01Q\x84\x84a/5V[\x95\x94PPPPPV[``_\x82Q`\x02\x02`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x11Wa*\x11a=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a*;W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x1B(W`\x04\x84\x82\x81Q\x81\x10a*]Wa*]aA\x18V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x82\x82`\x02\x02\x81Q\x81\x10a*\x89Wa*\x89aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x10\x84\x82\x81Q\x81\x10a*\xB3Wa*\xB3aA\x18V[\x01` \x01Q`\xF8\x1C\x81a*\xC8Wa*\xC8aF\x12V[\x06`\xF8\x1B\x82\x82`\x02\x02`\x01\x01\x81Q\x81\x10a*\xE4Wa*\xE4aA\x18V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a*@V[_` \x82Q\x10\x15a+\x15WP` \x01Q\x90V[\x81\x80` \x01\x90Q\x81\x01\x90a\x06+\x91\x90aH\xF7V[_``` \x83_\x01Q\x10\x15a+HWa+A\x83a.\x9DV[\x90Pa+TV[a+Q\x83a)eV[\x90P[a\x13u\x81a+\x02V[``a\x06+a+{\x83` \x01Q_\x81Q\x81\x10a\x16&Wa\x16&aA\x18V[a)\xF2V[``\x81\x83Q\x03_\x03a+\xA0WP`@\x80Q` \x81\x01\x90\x91R_\x81Ra\x06+V[a\x13u\x83\x83\x84\x86Q\x03a-]V[_\x80[\x80\x84Q\x11\x80\x15a+\xC1WP\x80\x83Q\x11[\x80\x15a,\x12WP\x82\x81\x81Q\x81\x10a+\xDAWa+\xDAaA\x18V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x84\x82\x81Q\x81\x10a,\x01Wa,\x01aA\x18V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x14[\x15a\x13uW\x80a,!\x81aF\xCDV[\x91PPa+\xB1V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a,bWP_\x91P`\x03\x90P\x82a,\xE7V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a,\xB3W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a,\xDEWP_\x92P`\x01\x91P\x82\x90Pa,\xE7V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[\x80Q_\x90`\x01\x03a-\x03WP_\x91\x90PV[\x81Q`\x15\x14a-TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FInvalid RLP address value.\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xBEV[a\x06+\x82a\x0C4V[``\x81\x82`\x1F\x01\x10\x15a-\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\t\xBEV[\x82\x82\x84\x01\x10\x15a-\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\t\xBEV[\x81\x83\x01\x84Q\x10\x15a.-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\t\xBEV[``\x82\x15\x80\x15a.KW`@Q\x91P_\x82R` \x82\x01`@Ra\x15\x1EV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a.\x84W\x80Q\x83R` \x92\x83\x01\x92\x01a.lV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RPP\x94\x93PPPPV[``a\x06+\x82a/\xE2V[``_\x82Q`\x03\x81\x11\x15a.\xBEWa.\xBEaF\x91V[\x03a.\xCCWa\x06+\x82a/\xF6V[`\x01\x82Q`\x03\x81\x11\x15a.\xE1Wa.\xE1aF\x91V[\x03a.\xEFWa\x06+\x82a2\xE2V[`\x02\x82Q`\x03\x81\x11\x15a/\x04Wa/\x04aF\x91V[\x03a/\x12Wa\x06+\x82a5SV[`\x03\x82Q`\x03\x81\x11\x15a/'Wa/'aF\x91V[\x03a\x13\xD6Wa\x06+\x82a7\xC1V[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a/PWa/Pa=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a/zW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x80Q_\x03a/\x8CW\x90Pa\x13uV[\x84\x84\x01` \x82\x01_[` \x86\x04\x81\x10\x15a/\xB6W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a/\x95V[P_`\x01` \x87\x06` \x03a\x01\0\n\x03\x90P\x80\x82Q\x16\x81\x19\x84Q\x16\x17\x82R\x83\x94PPPPP\x93\x92PPPV[``a\x06+\x82` \x01Q_\x84_\x01Qa/5V[``_\x82` \x01Q`\x01`\x01`@\x1B\x03\x16_\x14a0\x18WP` \x82\x01Qa0[V[a\x01\xC0\x83\x01QQ\x15a0[W`#\x83a\x01\xE0\x01Q`\x01`\x01`@\x1B\x03\x16\x10a0[W`\x02`#\x84a\x01\xE0\x01Qa0N\x91\x90aG3V[a0X\x91\x90aGSV[\x90P[_\x83`@\x01Qa0kW_a0nV[`\x03[a0y\x90`\x06aG\xE0V[`\xFF\x16\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x97Wa0\x97a=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a0\xCAW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a0\xB5W\x90P[P\x90Pa0\xDA\x85``\x01Qa\x11\xDEV[\x81_\x81Q\x81\x10a0\xECWa0\xECaA\x18V[` \x02` \x01\x01\x81\x90RPa1\x04\x85`\x80\x01Qa\x11\xDEV[\x81`\x01\x81Q\x81\x10a1\x17Wa1\x17aA\x18V[` \x02` \x01\x01\x81\x90RPa1/\x85`\xE0\x01Qa\x11\xDEV[\x81`\x02\x81Q\x81\x10a1BWa1BaA\x18V[` \x02` \x01\x01\x81\x90RPa1[\x85a\x01\0\x01Qa;dV[\x81`\x03\x81Q\x81\x10a1nWa1naA\x18V[` \x02` \x01\x01\x81\x90RPa1\x87\x85a\x01 \x01Qa\x11\xDEV[\x81`\x04\x81Q\x81\x10a1\x9AWa1\x9AaA\x18V[` \x02` \x01\x01\x81\x90RPa1\xB3\x85a\x01@\x01Qa\x15'V[\x81`\x05\x81Q\x81\x10a1\xC6Wa1\xC6aA\x18V[` \x02` \x01\x01\x81\x90RP\x84`@\x01Q\x15a2\xD9W\x84` \x01Q`\x01`\x01`@\x1B\x03\x16_\x03a2-W`@Q_` \x82\x01R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81`\x06\x81Q\x81\x10a2\x1DWa2\x1DaA\x18V[` \x02` \x01\x01\x81\x90RPa2^V[a2?\x83`\x01`\x01`@\x1B\x03\x16a\x11\xDEV[\x81`\x06\x81Q\x81\x10a2RWa2RaA\x18V[` \x02` \x01\x01\x81\x90RP[a2\x93_[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a2\x8DW` \x82\x01\x81\x806\x837\x01\x90P[Pa\x15'V[\x81`\x07\x81Q\x81\x10a2\xA6Wa2\xA6aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra2\xBA_a2cV[\x81`\x08\x81Q\x81\x10a2\xCDWa2\xCDaA\x18V[` \x02` \x01\x01\x81\x90RP[a)\xE9\x81a;\x9CV[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a2\xFEW\x90PP\x90Pa3+\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x11\xDEV[\x81_\x81Q\x81\x10a3=Wa3=aA\x18V[` \x02` \x01\x01\x81\x90RPa3U\x83``\x01Qa\x11\xDEV[\x81`\x01\x81Q\x81\x10a3hWa3haA\x18V[` \x02` \x01\x01\x81\x90RPa3\x80\x83`\x80\x01Qa\x11\xDEV[\x81`\x02\x81Q\x81\x10a3\x93Wa3\x93aA\x18V[` \x02` \x01\x01\x81\x90RPa3\xAB\x83`\xE0\x01Qa\x11\xDEV[\x81`\x03\x81Q\x81\x10a3\xBEWa3\xBEaA\x18V[` \x02` \x01\x01\x81\x90RPa3\xD7\x83a\x01\0\x01Qa;dV[\x81`\x04\x81Q\x81\x10a3\xEAWa3\xEAaA\x18V[` \x02` \x01\x01\x81\x90RPa4\x03\x83a\x01 \x01Qa\x11\xDEV[\x81`\x05\x81Q\x81\x10a4\x16Wa4\x16aA\x18V[` \x02` \x01\x01\x81\x90RPa4/\x83a\x01@\x01Qa\x15'V[\x81`\x06\x81Q\x81\x10a4BWa4BaA\x18V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a4lWa4la=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a4\x9FW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a4\x8AW\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a4\xF6W\x84a\x01`\x01Q\x81\x81Q\x81\x10a4\xC9Wa4\xC9aA\x18V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a4\xE3Wa4\xE3aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a4\xA4V[Pa5\0\x81a;\x9CV[\x82`\x07\x81Q\x81\x10a5\x13Wa5\x13aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01[a5*\x83a;\x9CV[`@Q` \x01a5;\x92\x91\x90aI\x0EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`@\x80Q`\t\x80\x82Ra\x01@\x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a5oW\x90PP\x90Pa5\x9C\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x11\xDEV[\x81_\x81Q\x81\x10a5\xAEWa5\xAEaA\x18V[` \x02` \x01\x01\x81\x90RPa5\xC6\x83``\x01Qa\x11\xDEV[\x81`\x01\x81Q\x81\x10a5\xD9Wa5\xD9aA\x18V[` \x02` \x01\x01\x81\x90RPa5\xF1\x83`\xA0\x01Qa\x11\xDEV[\x81`\x02\x81Q\x81\x10a6\x04Wa6\x04aA\x18V[` \x02` \x01\x01\x81\x90RPa6\x1C\x83`\xC0\x01Qa\x11\xDEV[\x81`\x03\x81Q\x81\x10a6/Wa6/aA\x18V[` \x02` \x01\x01\x81\x90RPa6G\x83`\xE0\x01Qa\x11\xDEV[\x81`\x04\x81Q\x81\x10a6ZWa6ZaA\x18V[` \x02` \x01\x01\x81\x90RPa6s\x83a\x01\0\x01Qa;dV[\x81`\x05\x81Q\x81\x10a6\x86Wa6\x86aA\x18V[` \x02` \x01\x01\x81\x90RPa6\x9F\x83a\x01 \x01Qa\x11\xDEV[\x81`\x06\x81Q\x81\x10a6\xB2Wa6\xB2aA\x18V[` \x02` \x01\x01\x81\x90RPa6\xCB\x83a\x01@\x01Qa\x15'V[\x81`\x07\x81Q\x81\x10a6\xDEWa6\xDEaA\x18V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x08Wa7\x08a=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7;W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a7&W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a7\x92W\x84a\x01`\x01Q\x81\x81Q\x81\x10a7eWa7eaA\x18V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a7\x7FWa7\x7FaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a7@V[Pa7\x9C\x81a;\x9CV[\x82`\x08\x81Q\x81\x10a7\xAFWa7\xAFaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x02a5!V[`@\x80Q`\x0B\x80\x82Ra\x01\x80\x82\x01\x90\x92R``\x91_\x91\x90\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a7\xDDW\x90PP\x90Pa8\n\x83` \x01Q`\x01`\x01`@\x1B\x03\x16a\x11\xDEV[\x81_\x81Q\x81\x10a8\x1CWa8\x1CaA\x18V[` \x02` \x01\x01\x81\x90RPa84\x83``\x01Qa\x11\xDEV[\x81`\x01\x81Q\x81\x10a8GWa8GaA\x18V[` \x02` \x01\x01\x81\x90RPa8_\x83`\xA0\x01Qa\x11\xDEV[\x81`\x02\x81Q\x81\x10a8rWa8raA\x18V[` \x02` \x01\x01\x81\x90RPa8\x8A\x83`\xC0\x01Qa\x11\xDEV[\x81`\x03\x81Q\x81\x10a8\x9DWa8\x9DaA\x18V[` \x02` \x01\x01\x81\x90RPa8\xB5\x83`\xE0\x01Qa\x11\xDEV[\x81`\x04\x81Q\x81\x10a8\xC8Wa8\xC8aA\x18V[` \x02` \x01\x01\x81\x90RPa8\xE1\x83a\x01\0\x01Qa;dV[\x81`\x05\x81Q\x81\x10a8\xF4Wa8\xF4aA\x18V[` \x02` \x01\x01\x81\x90RPa9\r\x83a\x01 \x01Qa\x11\xDEV[\x81`\x06\x81Q\x81\x10a9 Wa9 aA\x18V[` \x02` \x01\x01\x81\x90RPa99\x83a\x01@\x01Qa\x15'V[\x81`\x07\x81Q\x81\x10a9LWa9LaA\x18V[` \x02` \x01\x01\x81\x90RP_\x83a\x01`\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a9vWa9va=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a9\xA9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a9\x94W\x90P[P\x90P_[\x84a\x01`\x01QQ\x81\x10\x15a:\0W\x84a\x01`\x01Q\x81\x81Q\x81\x10a9\xD3Wa9\xD3aA\x18V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a9\xEDWa9\xEDaA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a9\xAEV[Pa:\n\x81a;\x9CV[\x82`\x08\x81Q\x81\x10a:\x1DWa:\x1DaA\x18V[` \x02` \x01\x01\x81\x90RPa:6\x84a\x01\x80\x01Qa\x11\xDEV[\x82`\t\x81Q\x81\x10a:IWa:IaA\x18V[` \x02` \x01\x01\x81\x90RP_\x84a\x01\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a:sWa:sa=\xFDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a:\xA6W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a:\x91W\x90P[P\x90P_[\x85a\x01\xA0\x01QQ\x81\x10\x15a;\x07Wa:\xE2\x86a\x01\xA0\x01Q\x82\x81Q\x81\x10a:\xD3Wa:\xD3aA\x18V[` \x02` \x01\x01Q_\x1Ca\x11\xDEV[\x82\x82\x81Q\x81\x10a:\xF4Wa:\xF4aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a:\xABV[Pa;\x11\x81a;\x9CV[\x83`\n\x81Q\x81\x10a;$Wa;$aA\x18V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x03a;:\x84a;\x9CV[`@Q` \x01a;K\x92\x91\x90aI\x0EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x91\x90PV[`@Q``\x82\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x83\x01R\x90a\x06+\x90`4\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x15'V[``_a;\xA8\x83a;\xDFV[\x90Pa;\xB6\x81Q`\xC0a'\xB0V[\x81`@Q` \x01a;\xC8\x92\x91\x90aF\xE5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[``\x81Q_\x03a;\xFEW`@\x80Q_\x80\x82R` \x82\x01\x90\x92R\x90a\x1B(V[_\x80[\x83Q\x81\x10\x15a<:W\x83\x81\x81Q\x81\x10a<\x1CWa<\x1CaA\x18V[` \x02` \x01\x01QQ\x82a<0\x91\x90aF9V[\x91P`\x01\x01a<\x01V[_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a<SWa<Sa=\xFDV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a<}W` \x82\x01\x81\x806\x837\x01\x90P[P_\x92P\x90P` \x81\x01[\x85Q\x83\x10\x15a\x15\x1EW_\x86\x84\x81Q\x81\x10a<\xA4Wa<\xA4aA\x18V[` \x02` \x01\x01Q\x90P_` \x82\x01\x90Pa<\xC1\x83\x82\x84Qa<\xF7V[\x87\x85\x81Q\x81\x10a<\xD3Wa<\xD3aA\x18V[` \x02` \x01\x01QQ\x83a<\xE7\x91\x90aF9V[`\x01\x90\x95\x01\x94\x92Pa<\x88\x91PPV[\x82\x82\x82[` \x81\x10a=3W\x81Q\x83Ra=\x12` \x84aF9V[\x92Pa=\x1F` \x83aF9V[\x91Pa=,` \x82aE\xFFV[\x90Pa<\xFBV[_`\x01a=A\x83` aE\xFFV[a=M\x90a\x01\0aH\xD9V[a=W\x91\x90aE\xFFV[\x92Q\x84Q\x84\x16\x93\x19\x16\x92\x90\x92\x17\x90\x92RPPPPPV[`@\x80Qa\x02\0\x81\x01\x90\x91R\x80_\x81R` \x01_`\x01`\x01`@\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_`\x01`\x01`@\x1B\x03\x16\x81RP\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>3Wa>3a=\xFDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>aWa>aa=\xFDV[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a>xW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a>\x91Wa>\x91a=\xFDV[a>\xA4`\x1F\x82\x01`\x1F\x19\x16` \x01a>9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a>\xB8W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a>\xE4W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xF9W_\x80\xFD[a?\x05\x84\x82\x85\x01a>iV[\x94\x93PPPPV[_a\x01`\x82\x84\x03\x12\x15a?\x1EW_\x80\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a?\x1EW_\x80\xFD[_\x80_``\x84\x86\x03\x12\x15a?FW_\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?\\W_\x80\xFD[a?h\x87\x83\x88\x01a?\rV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a?}W_\x80\xFD[a?\x89\x87\x83\x88\x01a?$V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a?\x9EW_\x80\xFD[P\x84\x01`\xC0\x81\x87\x03\x12\x15a?\xB0W_\x80\xFD[\x80\x91PP\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a?\xCBW_\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\xABW_\x80\xFD[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a?\xFDW_\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\x13W_\x80\xFD[a@\x1F\x8A\x83\x8B\x01a?\rV[\x97P` \x89\x015\x91P\x80\x82\x11\x15a@4W_\x80\xFD[a@@\x8A\x83\x8B\x01a?$V[\x96Pa@N`@\x8A\x01a?\xD2V[\x95P``\x89\x015\x91P\x80\x82\x11\x15a@cW_\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a@vW_\x80\xFD[\x815\x81\x81\x11\x15a@\x84W_\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a@\x95W_\x80\xFD[` \x83\x01\x95P\x80\x94PPPPa@\xAD`\x80\x88\x01a?\xD2V[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80`@\x83\x85\x03\x12\x15a@\xCAW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\xE0W_\x80\xFD[a@\xEC\x86\x83\x87\x01a?$V[\x93P` \x85\x015\x91P\x80\x82\x11\x15aA\x01W_\x80\xFD[PaA\x0E\x85\x82\x86\x01a?\rV[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`~\x19\x836\x03\x01\x81\x12aA@W_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a'\xABW_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aAuW_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x93W_\x80\xFD[\x806\x03\x82\x13\x15aA\xA1W_\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`@\x81R_`\x01`\x01`@\x1B\x03\x80aA\xE7\x86aAJV[\x16`@\x84\x01RaA\xFA` \x86\x01\x86aA`V[`\x80``\x86\x01RaB\x0F`\xC0\x86\x01\x82\x84aA\xA8V[\x91PP`@\x86\x015`\x80\x85\x01RaB(``\x87\x01a?\xD2V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x86\x01R\x84\x82\x03` \x86\x01Ra\x01`\x90aBo\x83\x88\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[aB\xA0`\x80\x84\x01`\x80\x89\x01\x805\x82R` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01RPPV[a\x01\0\x81aB\xAF\x82\x8A\x01a?\xD2V[\x16\x90\x84\x01RPa\x01 \x83aB\xC4\x88\x83\x01aAJV[\x16\x90\x83\x01Ra\x01@\x92PaB\xDA\x86\x84\x01\x87aA`V[\x82\x85\x85\x01RaB\xEC\x83\x85\x01\x82\x84aA\xA8V[\x99\x98PPPPPPPPPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aC\x0EW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aC'W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aA\xA1W_\x80\xFD[_` \x82\x84\x03\x12\x15aCKW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aCaW_\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15aCtW_\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15aC\x8FWaC\x8Fa=\xFDV[`@RaC\x9B\x83aAJV[\x81R` \x83\x015\x82\x81\x11\x15aC\xAEW_\x80\xFD[aC\xBA\x87\x82\x86\x01a>iV[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15aC\xD1W_\x80\xFD[aC\xDD\x87\x82\x86\x01a>iV[`@\x83\x01RP\x95\x94PPPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aD\x04WaD\x04a=\xFDV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aD\x1DW_\x80\xFD[\x815` aD2aD-\x83aC\xECV[a>9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aDPW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aD\x8DW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aDqW_\x80\xFD[aD\x7F\x89\x86\x83\x8B\x01\x01a>iV[\x84RP\x91\x83\x01\x91\x83\x01aDTV[P\x96\x95PPPPPPV[_\x82`\x1F\x83\x01\x12aD\xA7W_\x80\xFD[\x815` aD\xB7aD-\x83aC\xECV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15aD\xD8W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15aD\x8DW\x805\x83R\x91\x83\x01\x91\x83\x01aD\xDDV[_`\xC0\x826\x03\x12\x15aE\x04W_\x80\xFD[aE\x0Ca>\x11V[\x825\x81R` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aE)W_\x80\xFD[aE56\x83\x87\x01a>iV[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15aEMW_\x80\xFD[aEY6\x83\x87\x01a>iV[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15aEqW_\x80\xFD[aE}6\x83\x87\x01a>iV[``\x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15aE\x95W_\x80\xFD[aE\xA16\x83\x87\x01aD\x0EV[`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15aE\xB9W_\x80\xFD[PaE\xC66\x82\x86\x01aD\x98V[`\xA0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aE\xE2W_\x80\xFD[a\x13u\x82a?\xD2V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06+Wa\x06+aE\xEBV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aF4WaF4aF\x12V[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x06+Wa\x06+aE\xEBV[_\x825a\x01^\x19\x836\x03\x01\x81\x12aA@W_\x80\xFD[_` \x82\x84\x03\x12\x15aFqW_\x80\xFD[a\x13u\x82aAJV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06+Wa\x06+aE\xEBV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x82\x81R_a?\x05` \x83\x01\x84aF\xA5V[_`\x01\x82\x01aF\xDEWaF\xDEaE\xEBV[P`\x01\x01\x90V[_a?\x05aF\xF3\x83\x86aF\xA5V[\x84aF\xA5V[_`\xFF\x83\x16\x80aG\x0BWaG\x0BaF\x12V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x06+Wa\x06+aE\xEBV[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x1B(Wa\x1B(aE\xEBV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80aGlWaGlaF\x12V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80aG\x91WaG\x91aF\x12V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x1B(Wa\x1B(aE\xEBV[\x92\x83R` \x83\x01\x91\x90\x91R`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`@\x82\x01R`A\x01\x90V[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x06+Wa\x06+aE\xEBV[`\x01\x81\x81[\x80\x85\x11\x15aH3W\x81_\x19\x04\x82\x11\x15aH\x19WaH\x19aE\xEBV[\x80\x85\x16\x15aH&W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90aG\xFEV[P\x92P\x92\x90PV[_\x82aHIWP`\x01a\x06+V[\x81aHUWP_a\x06+V[\x81`\x01\x81\x14aHkW`\x02\x81\x14aHuWaH\x91V[`\x01\x91PPa\x06+V[`\xFF\x84\x11\x15aH\x86WaH\x86aE\xEBV[PP`\x01\x82\x1Ba\x06+V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aH\xB4WP\x81\x81\na\x06+V[aH\xBE\x83\x83aG\xF9V[\x80_\x19\x04\x82\x11\x15aH\xD1WaH\xD1aE\xEBV[\x02\x93\x92PPPV[_a\x13u\x83\x83aH;V[_\x82aH\xF2WaH\xF2aF\x12V[P\x06\x90V[_` \x82\x84\x03\x12\x15aI\x07W_\x80\xFD[PQ\x91\x90PV[`\xF8\x83\x90\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x81R_a?\x05`\x01\x83\x01\x84aF\xA5V\xFE\xA2dipfsX\"\x12 \x03\xE4Z }\xC3\x9D\xCA\xCB%\x884l '\xA2\xE0jm\x8A\xBD\xA1\xC7\xD4\x87N0\x8E\x01\x81\xC6MdsolcC\0\x08\x19\x003",
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
	constructor(uint256 _slashAmountWei, address _urc);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct constructorCall {
		#[allow(missing_docs)]
		pub _slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub _urc: alloy::sol_types::private::Address,
	}
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Address);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> =
				(alloy::sol_types::private::primitives::aliases::U256, alloy::sol_types::private::Address);
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
					(value._slashAmountWei, value._urc)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _slashAmountWei: tuple.0, _urc: tuple.1 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolConstructor for constructorCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Address);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
						&self._slashAmountWei,
					),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self._urc),
				)
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
	/**Function with signature `CHALLENGE_BOND()` and selector `0x9790ce71`.
	```solidity
	function CHALLENGE_BOND() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct CHALLENGE_BONDCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`CHALLENGE_BOND()`](CHALLENGE_BONDCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct CHALLENGE_BONDReturn {
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
			impl ::core::convert::From<CHALLENGE_BONDCall> for UnderlyingRustTuple<'_> {
				fn from(value: CHALLENGE_BONDCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for CHALLENGE_BONDCall {
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
			impl ::core::convert::From<CHALLENGE_BONDReturn> for UnderlyingRustTuple<'_> {
				fn from(value: CHALLENGE_BONDReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for CHALLENGE_BONDReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for CHALLENGE_BONDCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "CHALLENGE_BOND()";
			const SELECTOR: [u8; 4] = [151u8, 144u8, 206u8, 113u8];
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
					let r: CHALLENGE_BONDReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: CHALLENGE_BONDReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `CHALLENGE_WINDOW()` and selector `0xd62aad29`.
	```solidity
	function CHALLENGE_WINDOW() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct CHALLENGE_WINDOWCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`CHALLENGE_WINDOW()`](CHALLENGE_WINDOWCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct CHALLENGE_WINDOWReturn {
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
			impl ::core::convert::From<CHALLENGE_WINDOWCall> for UnderlyingRustTuple<'_> {
				fn from(value: CHALLENGE_WINDOWCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for CHALLENGE_WINDOWCall {
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
			impl ::core::convert::From<CHALLENGE_WINDOWReturn> for UnderlyingRustTuple<'_> {
				fn from(value: CHALLENGE_WINDOWReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for CHALLENGE_WINDOWReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for CHALLENGE_WINDOWCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "CHALLENGE_WINDOW()";
			const SELECTOR: [u8; 4] = [214u8, 42u8, 173u8, 41u8];
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
					let r: CHALLENGE_WINDOWReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: CHALLENGE_WINDOWReturn = r.into();
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
	/**Function with signature `challenges(bytes32)` and selector `0xc1e69b66`.
	```solidity
	function challenges(bytes32 challengeID) external view returns (address challenger, uint256 challengeTimestamp);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct challengesCall {
		#[allow(missing_docs)]
		pub challengeID: alloy::sol_types::private::FixedBytes<32>,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`challenges(bytes32)`](challengesCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct challengesReturn {
		#[allow(missing_docs)]
		pub challenger: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub challengeTimestamp: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
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
			impl ::core::convert::From<challengesCall> for UnderlyingRustTuple<'_> {
				fn from(value: challengesCall) -> Self {
					(value.challengeID,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengesCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { challengeID: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> =
				(alloy::sol_types::private::Address, alloy::sol_types::private::primitives::aliases::U256);
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
			impl ::core::convert::From<challengesReturn> for UnderlyingRustTuple<'_> {
				fn from(value: challengesReturn) -> Self {
					(value.challenger, value.challengeTimestamp)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for challengesReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { challenger: tuple.0, challengeTimestamp: tuple.1 }
				}
			}
		}
		impl challengesReturn {
			fn _tokenize(&self) -> <challengesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				(
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.challenger),
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
						&self.challengeTimestamp,
					),
				)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for challengesCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = challengesReturn;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "challenges(bytes32)";
			const SELECTOR: [u8; 4] = [193u8, 230u8, 155u8, 102u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.challengeID),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				challengesReturn::_tokenize(ret)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(Into::into)
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Into::into)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize)]
	/**Function with signature `createChallenge(((uint64,bytes,bytes32,address),uint64,bytes32,bytes),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))` and selector `0xccfa336f`.
	```solidity
	function createChallenge(ISlasher.SignedCommitment memory commitment, ISlasher.SignedDelegation memory signedDelegation) external payable returns (bytes32 challengeID);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct createChallengeCall {
		#[allow(missing_docs)]
		pub commitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub signedDelegation: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`createChallenge(((uint64,bytes,bytes32,address),uint64,bytes32,bytes),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))`](createChallengeCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct createChallengeReturn {
		#[allow(missing_docs)]
		pub challengeID: alloy::sol_types::private::FixedBytes<32>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (ISlasher::SignedCommitment, ISlasher::SignedDelegation);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				<ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
				<ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
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
			impl ::core::convert::From<createChallengeCall> for UnderlyingRustTuple<'_> {
				fn from(value: createChallengeCall) -> Self {
					(value.commitment, value.signedDelegation)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for createChallengeCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { commitment: tuple.0, signedDelegation: tuple.1 }
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
			impl ::core::convert::From<createChallengeReturn> for UnderlyingRustTuple<'_> {
				fn from(value: createChallengeReturn) -> Self {
					(value.challengeID,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for createChallengeReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { challengeID: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for createChallengeCall {
			type Parameters<'a> = (ISlasher::SignedCommitment, ISlasher::SignedDelegation);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::FixedBytes<32>;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "createChallenge(((uint64,bytes,bytes32,address),uint64,bytes32,bytes),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))";
			const SELECTOR: [u8; 4] = [204u8, 250u8, 51u8, 111u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<ISlasher::SignedCommitment as alloy_sol_types::SolType>::tokenize(&self.commitment),
					<ISlasher::SignedDelegation as alloy_sol_types::SolType>::tokenize(&self.signedDelegation),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: createChallengeReturn = r.into();
					r.challengeID
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: createChallengeReturn = r.into();
					r.challengeID
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize)]
	/**Function with signature `proveChallengeFraudulent(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),((uint64,bytes,bytes32,address),uint64,bytes32,bytes),(uint256,bytes,bytes,bytes,bytes[],uint256[]))` and selector `0x6ef8d437`.
	```solidity
	function proveChallengeFraudulent(ISlasher.Delegation memory delegation, ISlasher.SignedCommitment memory commitment, PreconfStructs.InclusionProof memory proof) external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct proveChallengeFraudulentCall {
		#[allow(missing_docs)]
		pub delegation: <ISlasher::Delegation as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub commitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub proof: <PreconfStructs::InclusionProof as alloy::sol_types::SolType>::RustType,
	}
	///Container type for the return parameters of the [`proveChallengeFraudulent(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),((uint64,bytes,bytes32,address),uint64,bytes32,bytes),(uint256,bytes,bytes,bytes,bytes[],uint256[]))`](proveChallengeFraudulentCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct proveChallengeFraudulentReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> =
				(ISlasher::Delegation, ISlasher::SignedCommitment, PreconfStructs::InclusionProof);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				<ISlasher::Delegation as alloy::sol_types::SolType>::RustType,
				<ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
				<PreconfStructs::InclusionProof as alloy::sol_types::SolType>::RustType,
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
			impl ::core::convert::From<proveChallengeFraudulentCall> for UnderlyingRustTuple<'_> {
				fn from(value: proveChallengeFraudulentCall) -> Self {
					(value.delegation, value.commitment, value.proof)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for proveChallengeFraudulentCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { delegation: tuple.0, commitment: tuple.1, proof: tuple.2 }
				}
			}
		}
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
			impl ::core::convert::From<proveChallengeFraudulentReturn> for UnderlyingRustTuple<'_> {
				fn from(value: proveChallengeFraudulentReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for proveChallengeFraudulentReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl proveChallengeFraudulentReturn {
			fn _tokenize(&self) -> <proveChallengeFraudulentCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for proveChallengeFraudulentCall {
			type Parameters<'a> = (ISlasher::Delegation, ISlasher::SignedCommitment, PreconfStructs::InclusionProof);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = proveChallengeFraudulentReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "proveChallengeFraudulent(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),((uint64,bytes,bytes32,address),uint64,bytes32,bytes),(uint256,bytes,bytes,bytes,bytes[],uint256[]))";
			const SELECTOR: [u8; 4] = [110u8, 248u8, 212u8, 55u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<ISlasher::Delegation as alloy_sol_types::SolType>::tokenize(&self.delegation),
					<ISlasher::SignedCommitment as alloy_sol_types::SolType>::tokenize(&self.commitment),
					<PreconfStructs::InclusionProof as alloy_sol_types::SolType>::tokenize(&self.proof),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				proveChallengeFraudulentReturn::_tokenize(ret)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(Into::into)
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Into::into)
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
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `urc()` and selector `0x5ddc9e8d`.
	```solidity
	function urc() external view returns (address);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct urcCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`urc()`](urcCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct urcReturn {
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
			impl ::core::convert::From<urcCall> for UnderlyingRustTuple<'_> {
				fn from(value: urcCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for urcCall {
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
			impl ::core::convert::From<urcReturn> for UnderlyingRustTuple<'_> {
				fn from(value: urcReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for urcReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for urcCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::Address;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "urc()";
			const SELECTOR: [u8; 4] = [93u8, 220u8, 158u8, 141u8];
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
					let r: urcReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: urcReturn = r.into();
					r._0
				})
			}
		}
	};
	///Container for all the [`InclusionPreconfSlasher`](self) function calls.
	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum InclusionPreconfSlasherCalls {
		#[allow(missing_docs)]
		BEACON_ROOTS_CONTRACT(BEACON_ROOTS_CONTRACTCall),
		#[allow(missing_docs)]
		BLOCKHASH_EVM_LOOKBACK(BLOCKHASH_EVM_LOOKBACKCall),
		#[allow(missing_docs)]
		CHALLENGE_BOND(CHALLENGE_BONDCall),
		#[allow(missing_docs)]
		CHALLENGE_WINDOW(CHALLENGE_WINDOWCall),
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
		challenges(challengesCall),
		#[allow(missing_docs)]
		createChallenge(createChallengeCall),
		#[allow(missing_docs)]
		proveChallengeFraudulent(proveChallengeFraudulentCall),
		#[allow(missing_docs)]
		slash(slashCall),
		#[allow(missing_docs)]
		urc(urcCall),
	}
	impl InclusionPreconfSlasherCalls {
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
			[93u8, 220u8, 158u8, 141u8],
			[110u8, 248u8, 212u8, 55u8],
			[116u8, 55u8, 112u8, 41u8],
			[134u8, 251u8, 97u8, 237u8],
			[151u8, 144u8, 206u8, 113u8],
			[159u8, 50u8, 157u8, 11u8],
			[171u8, 238u8, 179u8, 102u8],
			[193u8, 230u8, 155u8, 102u8],
			[194u8, 57u8, 239u8, 66u8],
			[202u8, 198u8, 45u8, 217u8],
			[204u8, 250u8, 51u8, 111u8],
			[214u8, 42u8, 173u8, 41u8],
			[244u8, 94u8, 129u8, 24u8],
			[245u8, 190u8, 234u8, 140u8],
		];
		/// The names of the variants in the same order as `SELECTORS`.
		pub const VARIANT_NAMES: &'static [&'static str] = &[
			::core::stringify!(_getCurrentSlot),
			::core::stringify!(SLOT_TIME),
			::core::stringify!(SLASH_AMOUNT_WEI),
			::core::stringify!(_decodeBlockHeaderRLP),
			::core::stringify!(urc),
			::core::stringify!(proveChallengeFraudulent),
			::core::stringify!(JUSTIFICATION_DELAY),
			::core::stringify!(BEACON_ROOTS_CONTRACT),
			::core::stringify!(CHALLENGE_BOND),
			::core::stringify!(ETH2_GENESIS_TIMESTAMP),
			::core::stringify!(_getSlotFromTimestamp),
			::core::stringify!(challenges),
			::core::stringify!(slash),
			::core::stringify!(EIP4788_WINDOW),
			::core::stringify!(createChallenge),
			::core::stringify!(CHALLENGE_WINDOW),
			::core::stringify!(BLOCKHASH_EVM_LOOKBACK),
			::core::stringify!(_getTimestampFromSlot),
		];
		/// The signatures in the same order as `SELECTORS`.
		pub const SIGNATURES: &'static [&'static str] = &[
			<_getCurrentSlotCall as alloy_sol_types::SolCall>::SIGNATURE,
			<SLOT_TIMECall as alloy_sol_types::SolCall>::SIGNATURE,
			<SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::SIGNATURE,
			<_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::SIGNATURE,
			<urcCall as alloy_sol_types::SolCall>::SIGNATURE,
			<proveChallengeFraudulentCall as alloy_sol_types::SolCall>::SIGNATURE,
			<JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::SIGNATURE,
			<BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::SIGNATURE,
			<CHALLENGE_BONDCall as alloy_sol_types::SolCall>::SIGNATURE,
			<ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::SIGNATURE,
			<_getSlotFromTimestampCall as alloy_sol_types::SolCall>::SIGNATURE,
			<challengesCall as alloy_sol_types::SolCall>::SIGNATURE,
			<slashCall as alloy_sol_types::SolCall>::SIGNATURE,
			<EIP4788_WINDOWCall as alloy_sol_types::SolCall>::SIGNATURE,
			<createChallengeCall as alloy_sol_types::SolCall>::SIGNATURE,
			<CHALLENGE_WINDOWCall as alloy_sol_types::SolCall>::SIGNATURE,
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
	impl alloy_sol_types::SolInterface for InclusionPreconfSlasherCalls {
		const NAME: &'static str = "InclusionPreconfSlasherCalls";
		const MIN_DATA_LENGTH: usize = 0usize;
		const COUNT: usize = 18usize;
		#[inline]
		fn selector(&self) -> [u8; 4] {
			match self {
				Self::BEACON_ROOTS_CONTRACT(_) => <BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::BLOCKHASH_EVM_LOOKBACK(_) => <BLOCKHASH_EVM_LOOKBACKCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::CHALLENGE_BOND(_) => <CHALLENGE_BONDCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::CHALLENGE_WINDOW(_) => <CHALLENGE_WINDOWCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::EIP4788_WINDOW(_) => <EIP4788_WINDOWCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::ETH2_GENESIS_TIMESTAMP(_) => <ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::JUSTIFICATION_DELAY(_) => <JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::SLASH_AMOUNT_WEI(_) => <SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::SELECTOR,
				Self::SLOT_TIME(_) => <SLOT_TIMECall as alloy_sol_types::SolCall>::SELECTOR,
				Self::_decodeBlockHeaderRLP(_) => <_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::_getCurrentSlot(_) => <_getCurrentSlotCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::_getSlotFromTimestamp(_) => <_getSlotFromTimestampCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::_getTimestampFromSlot(_) => <_getTimestampFromSlotCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::challenges(_) => <challengesCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::createChallenge(_) => <createChallengeCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::proveChallengeFraudulent(_) => {
					<proveChallengeFraudulentCall as alloy_sol_types::SolCall>::SELECTOR
				}
				Self::slash(_) => <slashCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::urc(_) => <urcCall as alloy_sol_types::SolCall>::SELECTOR,
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
			static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls>] = &[
				{
					fn _getCurrentSlot(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<_getCurrentSlotCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::_getCurrentSlot)
					}
					_getCurrentSlot
				},
				{
					fn SLOT_TIME(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<SLOT_TIMECall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::SLOT_TIME)
					}
					SLOT_TIME
				},
				{
					fn SLASH_AMOUNT_WEI(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::SLASH_AMOUNT_WEI)
					}
					SLASH_AMOUNT_WEI
				},
				{
					fn _decodeBlockHeaderRLP(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::_decodeBlockHeaderRLP)
					}
					_decodeBlockHeaderRLP
				},
				{
					fn urc(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<urcCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::urc)
					}
					urc
				},
				{
					fn proveChallengeFraudulent(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<proveChallengeFraudulentCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::proveChallengeFraudulent)
					}
					proveChallengeFraudulent
				},
				{
					fn JUSTIFICATION_DELAY(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::JUSTIFICATION_DELAY)
					}
					JUSTIFICATION_DELAY
				},
				{
					fn BEACON_ROOTS_CONTRACT(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::BEACON_ROOTS_CONTRACT)
					}
					BEACON_ROOTS_CONTRACT
				},
				{
					fn CHALLENGE_BOND(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<CHALLENGE_BONDCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::CHALLENGE_BOND)
					}
					CHALLENGE_BOND
				},
				{
					fn ETH2_GENESIS_TIMESTAMP(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::ETH2_GENESIS_TIMESTAMP)
					}
					ETH2_GENESIS_TIMESTAMP
				},
				{
					fn _getSlotFromTimestamp(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<_getSlotFromTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::_getSlotFromTimestamp)
					}
					_getSlotFromTimestamp
				},
				{
					fn challenges(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<challengesCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::challenges)
					}
					challenges
				},
				{
					fn slash(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<slashCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::slash)
					}
					slash
				},
				{
					fn EIP4788_WINDOW(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<EIP4788_WINDOWCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::EIP4788_WINDOW)
					}
					EIP4788_WINDOW
				},
				{
					fn createChallenge(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<createChallengeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::createChallenge)
					}
					createChallenge
				},
				{
					fn CHALLENGE_WINDOW(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<CHALLENGE_WINDOWCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::CHALLENGE_WINDOW)
					}
					CHALLENGE_WINDOW
				},
				{
					fn BLOCKHASH_EVM_LOOKBACK(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<BLOCKHASH_EVM_LOOKBACKCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::BLOCKHASH_EVM_LOOKBACK)
					}
					BLOCKHASH_EVM_LOOKBACK
				},
				{
					fn _getTimestampFromSlot(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<_getTimestampFromSlotCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherCalls::_getTimestampFromSlot)
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
			static DECODE_VALIDATE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls>] = &[
				{
					fn _getCurrentSlot(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<_getCurrentSlotCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::_getCurrentSlot)
					}
					_getCurrentSlot
				},
				{
					fn SLOT_TIME(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<SLOT_TIMECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::SLOT_TIME)
					}
					SLOT_TIME
				},
				{
					fn SLASH_AMOUNT_WEI(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<SLASH_AMOUNT_WEICall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::SLASH_AMOUNT_WEI)
					}
					SLASH_AMOUNT_WEI
				},
				{
					fn _decodeBlockHeaderRLP(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<_decodeBlockHeaderRLPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::_decodeBlockHeaderRLP)
					}
					_decodeBlockHeaderRLP
				},
				{
					fn urc(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<urcCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::urc)
					}
					urc
				},
				{
					fn proveChallengeFraudulent(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<proveChallengeFraudulentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::proveChallengeFraudulent)
					}
					proveChallengeFraudulent
				},
				{
					fn JUSTIFICATION_DELAY(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<JUSTIFICATION_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::JUSTIFICATION_DELAY)
					}
					JUSTIFICATION_DELAY
				},
				{
					fn BEACON_ROOTS_CONTRACT(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<BEACON_ROOTS_CONTRACTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::BEACON_ROOTS_CONTRACT)
					}
					BEACON_ROOTS_CONTRACT
				},
				{
					fn CHALLENGE_BOND(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<CHALLENGE_BONDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::CHALLENGE_BOND)
					}
					CHALLENGE_BOND
				},
				{
					fn ETH2_GENESIS_TIMESTAMP(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<ETH2_GENESIS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::ETH2_GENESIS_TIMESTAMP)
					}
					ETH2_GENESIS_TIMESTAMP
				},
				{
					fn _getSlotFromTimestamp(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<_getSlotFromTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::_getSlotFromTimestamp)
					}
					_getSlotFromTimestamp
				},
				{
					fn challenges(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<challengesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::challenges)
					}
					challenges
				},
				{
					fn slash(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<slashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::slash)
					}
					slash
				},
				{
					fn EIP4788_WINDOW(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<EIP4788_WINDOWCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::EIP4788_WINDOW)
					}
					EIP4788_WINDOW
				},
				{
					fn createChallenge(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<createChallengeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::createChallenge)
					}
					createChallenge
				},
				{
					fn CHALLENGE_WINDOW(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<CHALLENGE_WINDOWCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::CHALLENGE_WINDOW)
					}
					CHALLENGE_WINDOW
				},
				{
					fn BLOCKHASH_EVM_LOOKBACK(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<BLOCKHASH_EVM_LOOKBACKCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::BLOCKHASH_EVM_LOOKBACK)
					}
					BLOCKHASH_EVM_LOOKBACK
				},
				{
					fn _getTimestampFromSlot(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherCalls> {
						<_getTimestampFromSlotCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherCalls::_getTimestampFromSlot)
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
				Self::CHALLENGE_BOND(inner) => {
					<CHALLENGE_BONDCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::CHALLENGE_WINDOW(inner) => {
					<CHALLENGE_WINDOWCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
				Self::challenges(inner) => <challengesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::createChallenge(inner) => {
					<createChallengeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::proveChallengeFraudulent(inner) => {
					<proveChallengeFraudulentCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::slash(inner) => <slashCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::urc(inner) => <urcCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
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
				Self::CHALLENGE_BOND(inner) => {
					<CHALLENGE_BONDCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::CHALLENGE_WINDOW(inner) => {
					<CHALLENGE_WINDOWCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
				Self::challenges(inner) => <challengesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::createChallenge(inner) => {
					<createChallengeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::proveChallengeFraudulent(inner) => {
					<proveChallengeFraudulentCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::slash(inner) => <slashCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::urc(inner) => <urcCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
			}
		}
	}
	///Container for all the [`InclusionPreconfSlasher`](self) custom errors.
	#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
	pub enum InclusionPreconfSlasherErrors {
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
	impl InclusionPreconfSlasherErrors {
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
	impl alloy_sol_types::SolInterface for InclusionPreconfSlasherErrors {
		const NAME: &'static str = "InclusionPreconfSlasherErrors";
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
			static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors>] = &[
				{
					fn ChallengeDoesNotExist(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::ChallengeDoesNotExist)
					}
					ChallengeDoesNotExist
				},
				{
					fn TransactionExcluded(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<TransactionExcluded as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::TransactionExcluded)
					}
					TransactionExcluded
				},
				{
					fn FraudProofWindowActive(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<FraudProofWindowActive as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::FraudProofWindowActive)
					}
					FraudProofWindowActive
				},
				{
					fn DelegationExpired(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<DelegationExpired as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::DelegationExpired)
					}
					DelegationExpired
				},
				{
					fn BlockIsTooOld(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<BlockIsTooOld as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::BlockIsTooOld)
					}
					BlockIsTooOld
				},
				{
					fn InvalidBlockHash(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidBlockHash as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::InvalidBlockHash)
					}
					InvalidBlockHash
				},
				{
					fn InvalidParentBlockHash(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidParentBlockHash as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::InvalidParentBlockHash)
					}
					InvalidParentBlockHash
				},
				{
					fn ChallengeAlreadyExists(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::ChallengeAlreadyExists)
					}
					ChallengeAlreadyExists
				},
				{
					fn InvalidSignatureLength(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::InvalidSignatureLength)
					}
					InvalidSignatureLength
				},
				{
					fn InvalidBlockNumber(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::InvalidBlockNumber)
					}
					InvalidBlockNumber
				},
				{
					fn EthTransferFailed(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<EthTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::EthTransferFailed)
					}
					EthTransferFailed
				},
				{
					fn WrongChallengerAddress(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<WrongChallengerAddress as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::WrongChallengerAddress)
					}
					WrongChallengerAddress
				},
				{
					fn WrongTransactionHashProof(
						data: &[u8],
					) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<WrongTransactionHashProof as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::WrongTransactionHashProof)
					}
					WrongTransactionHashProof
				},
				{
					fn IncorrectChallengeBond(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<IncorrectChallengeBond as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::IncorrectChallengeBond)
					}
					IncorrectChallengeBond
				},
				{
					fn UnexpectedSigner(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<UnexpectedSigner as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::UnexpectedSigner)
					}
					UnexpectedSigner
				},
				{
					fn BlockIsNotFinalized(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<BlockIsNotFinalized as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::BlockIsNotFinalized)
					}
					BlockIsNotFinalized
				},
				{
					fn InvalidFieldCount(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidFieldCount as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::InvalidFieldCount)
					}
					InvalidFieldCount
				},
				{
					fn NotURC(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<NotURC as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::NotURC)
					}
					NotURC
				},
				{
					fn NoSignature(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<NoSignature as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::NoSignature)
					}
					NoSignature
				},
				{
					fn ECDSAInvalidSignatureS(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::ECDSAInvalidSignatureS)
					}
					ECDSAInvalidSignatureS
				},
				{
					fn UnsupportedTxType(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<UnsupportedTxType as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::UnsupportedTxType)
					}
					UnsupportedTxType
				},
				{
					fn BeaconRootNotFound(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<BeaconRootNotFound as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::BeaconRootNotFound)
					}
					BeaconRootNotFound
				},
				{
					fn ECDSAInvalidSignature(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::ECDSAInvalidSignature)
					}
					ECDSAInvalidSignature
				},
				{
					fn ECDSAInvalidSignatureLength(
						data: &[u8],
					) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(InclusionPreconfSlasherErrors::ECDSAInvalidSignatureLength)
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
			static DECODE_VALIDATE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors>] = &[
				{
					fn ChallengeDoesNotExist(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ChallengeDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::ChallengeDoesNotExist)
					}
					ChallengeDoesNotExist
				},
				{
					fn TransactionExcluded(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<TransactionExcluded as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::TransactionExcluded)
					}
					TransactionExcluded
				},
				{
					fn FraudProofWindowActive(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<FraudProofWindowActive as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::FraudProofWindowActive)
					}
					FraudProofWindowActive
				},
				{
					fn DelegationExpired(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<DelegationExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::DelegationExpired)
					}
					DelegationExpired
				},
				{
					fn BlockIsTooOld(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<BlockIsTooOld as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::BlockIsTooOld)
					}
					BlockIsTooOld
				},
				{
					fn InvalidBlockHash(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidBlockHash as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::InvalidBlockHash)
					}
					InvalidBlockHash
				},
				{
					fn InvalidParentBlockHash(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidParentBlockHash as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::InvalidParentBlockHash)
					}
					InvalidParentBlockHash
				},
				{
					fn ChallengeAlreadyExists(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ChallengeAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::ChallengeAlreadyExists)
					}
					ChallengeAlreadyExists
				},
				{
					fn InvalidSignatureLength(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::InvalidSignatureLength)
					}
					InvalidSignatureLength
				},
				{
					fn InvalidBlockNumber(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidBlockNumber as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::InvalidBlockNumber)
					}
					InvalidBlockNumber
				},
				{
					fn EthTransferFailed(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<EthTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::EthTransferFailed)
					}
					EthTransferFailed
				},
				{
					fn WrongChallengerAddress(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<WrongChallengerAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::WrongChallengerAddress)
					}
					WrongChallengerAddress
				},
				{
					fn WrongTransactionHashProof(
						data: &[u8],
					) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<WrongTransactionHashProof as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::WrongTransactionHashProof)
					}
					WrongTransactionHashProof
				},
				{
					fn IncorrectChallengeBond(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<IncorrectChallengeBond as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::IncorrectChallengeBond)
					}
					IncorrectChallengeBond
				},
				{
					fn UnexpectedSigner(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<UnexpectedSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::UnexpectedSigner)
					}
					UnexpectedSigner
				},
				{
					fn BlockIsNotFinalized(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<BlockIsNotFinalized as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::BlockIsNotFinalized)
					}
					BlockIsNotFinalized
				},
				{
					fn InvalidFieldCount(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<InvalidFieldCount as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::InvalidFieldCount)
					}
					InvalidFieldCount
				},
				{
					fn NotURC(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<NotURC as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::NotURC)
					}
					NotURC
				},
				{
					fn NoSignature(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<NoSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::NoSignature)
					}
					NoSignature
				},
				{
					fn ECDSAInvalidSignatureS(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::ECDSAInvalidSignatureS)
					}
					ECDSAInvalidSignatureS
				},
				{
					fn UnsupportedTxType(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<UnsupportedTxType as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::UnsupportedTxType)
					}
					UnsupportedTxType
				},
				{
					fn BeaconRootNotFound(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<BeaconRootNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::BeaconRootNotFound)
					}
					BeaconRootNotFound
				},
				{
					fn ECDSAInvalidSignature(data: &[u8]) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::ECDSAInvalidSignature)
					}
					ECDSAInvalidSignature
				},
				{
					fn ECDSAInvalidSignatureLength(
						data: &[u8],
					) -> alloy_sol_types::Result<InclusionPreconfSlasherErrors> {
						<ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(InclusionPreconfSlasherErrors::ECDSAInvalidSignatureLength)
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
	/**Creates a new wrapper around an on-chain [`InclusionPreconfSlasher`](self) contract instance.

	See the [wrapper's documentation](`InclusionPreconfSlasherInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> InclusionPreconfSlasherInstance<P, N> {
		InclusionPreconfSlasherInstance::<P, N>::new(address, __provider)
	}
	/**Deploys this contract using the given `provider` and constructor arguments, if any.

	Returns a new instance of the contract, if the deployment was successful.

	For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
	#[inline]
	pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		__provider: P,
		_slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
		_urc: alloy::sol_types::private::Address,
	) -> impl ::core::future::Future<Output = alloy_contract::Result<InclusionPreconfSlasherInstance<P, N>>> {
		InclusionPreconfSlasherInstance::<P, N>::deploy(__provider, _slashAmountWei, _urc)
	}
	/**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
	and constructor arguments, if any.

	This is a simple wrapper around creating a `RawCallBuilder` with the data set to
	the bytecode concatenated with the constructor's ABI-encoded arguments.*/
	#[inline]
	pub fn deploy_builder<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		__provider: P,
		_slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
		_urc: alloy::sol_types::private::Address,
	) -> alloy_contract::RawCallBuilder<P, N> {
		InclusionPreconfSlasherInstance::<P, N>::deploy_builder(__provider, _slashAmountWei, _urc)
	}
	/**A [`InclusionPreconfSlasher`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`InclusionPreconfSlasher`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct InclusionPreconfSlasherInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for InclusionPreconfSlasherInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("InclusionPreconfSlasherInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
		InclusionPreconfSlasherInstance<P, N>
	{
		/**Creates a new wrapper around an on-chain [`InclusionPreconfSlasher`](self) contract instance.

		See the [wrapper's documentation](`InclusionPreconfSlasherInstance`) for more details.*/
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
			_urc: alloy::sol_types::private::Address,
		) -> alloy_contract::Result<InclusionPreconfSlasherInstance<P, N>> {
			let call_builder = Self::deploy_builder(__provider, _slashAmountWei, _urc);
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
			_urc: alloy::sol_types::private::Address,
		) -> alloy_contract::RawCallBuilder<P, N> {
			alloy_contract::RawCallBuilder::new_raw_deploy(
				__provider,
				[
					&BYTECODE[..],
					&alloy_sol_types::SolConstructor::abi_encode(&constructorCall { _slashAmountWei, _urc })[..],
				]
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
	impl<P: ::core::clone::Clone, N> InclusionPreconfSlasherInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> InclusionPreconfSlasherInstance<P, N> {
			InclusionPreconfSlasherInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
		InclusionPreconfSlasherInstance<P, N>
	{
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
		///Creates a new call builder for the [`CHALLENGE_BOND`] function.
		pub fn CHALLENGE_BOND(&self) -> alloy_contract::SolCallBuilder<&P, CHALLENGE_BONDCall, N> {
			self.call_builder(&CHALLENGE_BONDCall)
		}
		///Creates a new call builder for the [`CHALLENGE_WINDOW`] function.
		pub fn CHALLENGE_WINDOW(&self) -> alloy_contract::SolCallBuilder<&P, CHALLENGE_WINDOWCall, N> {
			self.call_builder(&CHALLENGE_WINDOWCall)
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
		///Creates a new call builder for the [`challenges`] function.
		pub fn challenges(
			&self,
			challengeID: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, challengesCall, N> {
			self.call_builder(&challengesCall { challengeID })
		}
		///Creates a new call builder for the [`createChallenge`] function.
		pub fn createChallenge(
			&self,
			commitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
			signedDelegation: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
		) -> alloy_contract::SolCallBuilder<&P, createChallengeCall, N> {
			self.call_builder(&createChallengeCall { commitment, signedDelegation })
		}
		///Creates a new call builder for the [`proveChallengeFraudulent`] function.
		pub fn proveChallengeFraudulent(
			&self,
			delegation: <ISlasher::Delegation as alloy::sol_types::SolType>::RustType,
			commitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
			proof: <PreconfStructs::InclusionProof as alloy::sol_types::SolType>::RustType,
		) -> alloy_contract::SolCallBuilder<&P, proveChallengeFraudulentCall, N> {
			self.call_builder(&proveChallengeFraudulentCall { delegation, commitment, proof })
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
		///Creates a new call builder for the [`urc`] function.
		pub fn urc(&self) -> alloy_contract::SolCallBuilder<&P, urcCall, N> {
			self.call_builder(&urcCall)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
		InclusionPreconfSlasherInstance<P, N>
	{
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
			alloy_contract::Event::new_sol(&self.provider, &self.address)
		}
	}
}
