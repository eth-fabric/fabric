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
library IRegistry {
	struct SignedRegistration { BLS.G1Point pubkey; BLS.G2Point signature; uint64 nonce; }
}
```*/
#[allow(
	non_camel_case_types,
	non_snake_case,
	clippy::pub_underscore_fields,
	clippy::style,
	clippy::empty_structs_with_brackets
)]
pub mod IRegistry {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct SignedRegistration { BLS.G1Point pubkey; BLS.G2Point signature; uint64 nonce; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SignedRegistration {
		#[allow(missing_docs)]
		pub pubkey: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub signature: <BLS::G2Point as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub nonce: u64,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (BLS::G1Point, BLS::G2Point, alloy::sol_types::sol_data::Uint<64>);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			<BLS::G1Point as alloy::sol_types::SolType>::RustType,
			<BLS::G2Point as alloy::sol_types::SolType>::RustType,
			u64,
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
		impl ::core::convert::From<SignedRegistration> for UnderlyingRustTuple<'_> {
			fn from(value: SignedRegistration) -> Self {
				(value.pubkey, value.signature, value.nonce)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedRegistration {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { pubkey: tuple.0, signature: tuple.1, nonce: tuple.2 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for SignedRegistration {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for SignedRegistration {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.pubkey),
					<BLS::G2Point as alloy_sol_types::SolType>::tokenize(&self.signature),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.nonce),
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
		impl alloy_sol_types::SolType for SignedRegistration {
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
		impl alloy_sol_types::SolStruct for SignedRegistration {
			const NAME: &'static str = "SignedRegistration";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"SignedRegistration(G1Point pubkey,G2Point signature,uint64 nonce)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(2);
				components.push(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components());
				components.push(<BLS::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G2Point as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.pubkey).0,
					<BLS::G2Point as alloy_sol_types::SolType>::eip712_data_word(&self.signature).0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.nonce).0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for SignedRegistration {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.pubkey)
					+ <BLS::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.signature)
					+ <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.nonce,
					)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.pubkey, out);
				<BLS::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.signature, out);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.nonce,
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
	/**Creates a new wrapper around an on-chain [`IRegistry`](self) contract instance.

	See the [wrapper's documentation](`IRegistryInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> IRegistryInstance<P, N> {
		IRegistryInstance::<P, N>::new(address, __provider)
	}
	/**A [`IRegistry`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`IRegistry`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct IRegistryInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for IRegistryInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("IRegistryInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> IRegistryInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`IRegistry`](self) contract instance.

		See the [wrapper's documentation](`IRegistryInstance`) for more details.*/
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
	impl<P: ::core::clone::Clone, N> IRegistryInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> IRegistryInstance<P, N> {
			IRegistryInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> IRegistryInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> IRegistryInstance<P, N> {
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
library UnitTestHelper {
	struct RegisterAndDelegateParams { uint256 proposerSecretKey; uint256 collateral; address owner; uint256 delegateSecretKey; uint256 committerSecretKey; address committer; address slasher; bytes metadata; uint64 slot; bytes32 signingId; uint64 nonce; }
	struct RegisterAndDelegateResult { bytes32 registrationRoot; IRegistry.SignedRegistration[] registrations; ISlasher.SignedDelegation signedDelegation; }
}
```*/
#[allow(
	non_camel_case_types,
	non_snake_case,
	clippy::pub_underscore_fields,
	clippy::style,
	clippy::empty_structs_with_brackets
)]
pub mod UnitTestHelper {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct RegisterAndDelegateParams { uint256 proposerSecretKey; uint256 collateral; address owner; uint256 delegateSecretKey; uint256 committerSecretKey; address committer; address slasher; bytes metadata; uint64 slot; bytes32 signingId; uint64 nonce; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct RegisterAndDelegateParams {
		#[allow(missing_docs)]
		pub proposerSecretKey: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub collateral: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub owner: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub delegateSecretKey: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub committerSecretKey: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub committer: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub metadata: alloy::sol_types::private::Bytes,
		#[allow(missing_docs)]
		pub slot: u64,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub nonce: u64,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::Uint<256>,
			alloy::sol_types::sol_data::Uint<256>,
			alloy::sol_types::sol_data::Address,
			alloy::sol_types::sol_data::Uint<256>,
			alloy::sol_types::sol_data::Uint<256>,
			alloy::sol_types::sol_data::Address,
			alloy::sol_types::sol_data::Address,
			alloy::sol_types::sol_data::Bytes,
			alloy::sol_types::sol_data::Uint<64>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::Uint<64>,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::primitives::aliases::U256,
			alloy::sol_types::private::primitives::aliases::U256,
			alloy::sol_types::private::Address,
			alloy::sol_types::private::primitives::aliases::U256,
			alloy::sol_types::private::primitives::aliases::U256,
			alloy::sol_types::private::Address,
			alloy::sol_types::private::Address,
			alloy::sol_types::private::Bytes,
			u64,
			alloy::sol_types::private::FixedBytes<32>,
			u64,
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
		impl ::core::convert::From<RegisterAndDelegateParams> for UnderlyingRustTuple<'_> {
			fn from(value: RegisterAndDelegateParams) -> Self {
				(
					value.proposerSecretKey,
					value.collateral,
					value.owner,
					value.delegateSecretKey,
					value.committerSecretKey,
					value.committer,
					value.slasher,
					value.metadata,
					value.slot,
					value.signingId,
					value.nonce,
				)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for RegisterAndDelegateParams {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self {
					proposerSecretKey: tuple.0,
					collateral: tuple.1,
					owner: tuple.2,
					delegateSecretKey: tuple.3,
					committerSecretKey: tuple.4,
					committer: tuple.5,
					slasher: tuple.6,
					metadata: tuple.7,
					slot: tuple.8,
					signingId: tuple.9,
					nonce: tuple.10,
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for RegisterAndDelegateParams {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for RegisterAndDelegateParams {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
						&self.proposerSecretKey,
					),
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.collateral),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.owner),
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
						&self.delegateSecretKey,
					),
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
						&self.committerSecretKey,
					),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.committer),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.slasher),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.metadata),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.slot),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.signingId),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.nonce),
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
		impl alloy_sol_types::SolType for RegisterAndDelegateParams {
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
		impl alloy_sol_types::SolStruct for RegisterAndDelegateParams {
			const NAME: &'static str = "RegisterAndDelegateParams";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"RegisterAndDelegateParams(uint256 proposerSecretKey,uint256 collateral,address owner,uint256 delegateSecretKey,uint256 committerSecretKey,address committer,address slasher,bytes metadata,uint64 slot,bytes32 signingId,uint64 nonce)",
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
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(
						&self.proposerSecretKey,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(
						&self.collateral,
					)
					.0,
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(&self.owner).0,
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(
						&self.delegateSecretKey,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::eip712_data_word(
						&self.committerSecretKey,
					)
					.0,
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
						&self.committer,
					)
					.0,
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(&self.slasher)
						.0,
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(&self.metadata).0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.slot).0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.signingId,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.nonce).0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for RegisterAndDelegateParams {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.proposerSecretKey,
					) + <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.collateral,
				) + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.owner,
				) + <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.delegateSecretKey,
				) + <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.committerSecretKey,
				) + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.committer,
				) + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.slasher,
				) + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.metadata,
				) + <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.slot,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.signingId,
				) + <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.nonce,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.proposerSecretKey,
					out,
				);
				<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.collateral,
					out,
				);
				<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.owner,
					out,
				);
				<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.delegateSecretKey,
					out,
				);
				<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.committerSecretKey,
					out,
				);
				<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.committer,
					out,
				);
				<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.slasher,
					out,
				);
				<alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.metadata,
					out,
				);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.slot, out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signingId,
					out,
				);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.nonce,
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
	#[derive(serde::Serialize, serde::Deserialize)]
	/**```solidity
	struct RegisterAndDelegateResult { bytes32 registrationRoot; IRegistry.SignedRegistration[] registrations; ISlasher.SignedDelegation signedDelegation; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct RegisterAndDelegateResult {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub registrations:
			alloy::sol_types::private::Vec<<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType>,
		#[allow(missing_docs)]
		pub signedDelegation: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
			ISlasher::SignedDelegation,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::Vec<<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType>,
			<ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
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
		impl ::core::convert::From<RegisterAndDelegateResult> for UnderlyingRustTuple<'_> {
			fn from(value: RegisterAndDelegateResult) -> Self {
				(value.registrationRoot, value.registrations, value.signedDelegation)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for RegisterAndDelegateResult {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { registrationRoot: tuple.0, registrations: tuple.1, signedDelegation: tuple.2 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for RegisterAndDelegateResult {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for RegisterAndDelegateResult {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.registrationRoot),
                    <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::SolType>::tokenize(&self.registrations),
                    <ISlasher::SignedDelegation as alloy_sol_types::SolType>::tokenize(
                        &self.signedDelegation,
                    ),
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
		impl alloy_sol_types::SolType for RegisterAndDelegateResult {
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
		impl alloy_sol_types::SolStruct for RegisterAndDelegateResult {
			const NAME: &'static str = "RegisterAndDelegateResult";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"RegisterAndDelegateResult(bytes32 registrationRoot,IRegistry.SignedRegistration[] registrations,SignedDelegation signedDelegation)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(2);
				components.push(<IRegistry::SignedRegistration as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<IRegistry::SignedRegistration as alloy_sol_types::SolStruct>::eip712_components());
				components.push(<ISlasher::SignedDelegation as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<ISlasher::SignedDelegation as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.registrationRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.registrations)
                        .0,
                    <ISlasher::SignedDelegation as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signedDelegation,
                        )
                        .0,
                ]
                    .concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for RegisterAndDelegateResult {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.registrationRoot,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.registrations,
                    )
                    + <ISlasher::SignedDelegation as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signedDelegation,
                    )
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.registrationRoot,
					out,
				);
				<alloy::sol_types::sol_data::Array<
                    IRegistry::SignedRegistration,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.registrations,
                    out,
                );
				<ISlasher::SignedDelegation as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signedDelegation,
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
	/**Creates a new wrapper around an on-chain [`UnitTestHelper`](self) contract instance.

	See the [wrapper's documentation](`UnitTestHelperInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> UnitTestHelperInstance<P, N> {
		UnitTestHelperInstance::<P, N>::new(address, __provider)
	}
	/**A [`UnitTestHelper`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`UnitTestHelper`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct UnitTestHelperInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for UnitTestHelperInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("UnitTestHelperInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> UnitTestHelperInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`UnitTestHelper`](self) contract instance.

		See the [wrapper's documentation](`UnitTestHelperInstance`) for more details.*/
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
	impl<P: ::core::clone::Clone, N> UnitTestHelperInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> UnitTestHelperInstance<P, N> {
			UnitTestHelperInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> UnitTestHelperInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> UnitTestHelperInstance<P, N> {
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

library IRegistry {
	struct SignedRegistration {
		BLS.G1Point pubkey;
		BLS.G2Point signature;
		uint64 nonce;
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

library UnitTestHelper {
	struct RegisterAndDelegateParams {
		uint256 proposerSecretKey;
		uint256 collateral;
		address owner;
		uint256 delegateSecretKey;
		uint256 committerSecretKey;
		address committer;
		address slasher;
		bytes metadata;
		uint64 slot;
		bytes32 signingId;
		uint64 nonce;
	}
	struct RegisterAndDelegateResult {
		bytes32 registrationRoot;
		IRegistry.SignedRegistration[] registrations;
		ISlasher.SignedDelegation signedDelegation;
	}
}

interface ReentrantSlashableRegistrationContract {
	constructor(address registryAddress);

	receive() external payable;

	function claimCollateral() external;
	function collateral() external view returns (uint256);
	function errors() external view returns (uint256);
	function register(IRegistry.SignedRegistration[] memory _registrations, bytes32 signingId) external;
	function registrationRoot() external view returns (bytes32);
	function registry() external view returns (address);
	function saveResult(UnitTestHelper.RegisterAndDelegateParams memory _params, UnitTestHelper.RegisterAndDelegateResult memory _result, ISlasher.SignedCommitment memory _signedCommitment, ISlasher.SignedDelegation memory _signedDelegationTwo) external;
	function unregister() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
	"type": "constructor",
	"inputs": [
	  {
		"name": "registryAddress",
		"type": "address",
		"internalType": "address"
	  }
	],
	"stateMutability": "nonpayable"
  },
  {
	"type": "receive",
	"stateMutability": "payable"
  },
  {
	"type": "function",
	"name": "claimCollateral",
	"inputs": [],
	"outputs": [],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "collateral",
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
	"name": "errors",
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
	"name": "register",
	"inputs": [
	  {
		"name": "_registrations",
		"type": "tuple[]",
		"internalType": "struct IRegistry.SignedRegistration[]",
		"components": [
		  {
			"name": "pubkey",
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
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  }
		]
	  },
	  {
		"name": "signingId",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "registrationRoot",
	"inputs": [],
	"outputs": [
	  {
		"name": "",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "registry",
	"inputs": [],
	"outputs": [
	  {
		"name": "",
		"type": "address",
		"internalType": "contract IRegistry"
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "saveResult",
	"inputs": [
	  {
		"name": "_params",
		"type": "tuple",
		"internalType": "struct UnitTestHelper.RegisterAndDelegateParams",
		"components": [
		  {
			"name": "proposerSecretKey",
			"type": "uint256",
			"internalType": "uint256"
		  },
		  {
			"name": "collateral",
			"type": "uint256",
			"internalType": "uint256"
		  },
		  {
			"name": "owner",
			"type": "address",
			"internalType": "address"
		  },
		  {
			"name": "delegateSecretKey",
			"type": "uint256",
			"internalType": "uint256"
		  },
		  {
			"name": "committerSecretKey",
			"type": "uint256",
			"internalType": "uint256"
		  },
		  {
			"name": "committer",
			"type": "address",
			"internalType": "address"
		  },
		  {
			"name": "slasher",
			"type": "address",
			"internalType": "address"
		  },
		  {
			"name": "metadata",
			"type": "bytes",
			"internalType": "bytes"
		  },
		  {
			"name": "slot",
			"type": "uint64",
			"internalType": "uint64"
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
		  }
		]
	  },
	  {
		"name": "_result",
		"type": "tuple",
		"internalType": "struct UnitTestHelper.RegisterAndDelegateResult",
		"components": [
		  {
			"name": "registrationRoot",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "registrations",
			"type": "tuple[]",
			"internalType": "struct IRegistry.SignedRegistration[]",
			"components": [
			  {
				"name": "pubkey",
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
				"name": "nonce",
				"type": "uint64",
				"internalType": "uint64"
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
		]
	  },
	  {
		"name": "_signedCommitment",
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
		"name": "_signedDelegationTwo",
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
	"outputs": [],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "unregister",
	"inputs": [],
	"outputs": [],
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
pub mod ReentrantSlashableRegistrationContract {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	/// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052671bc16d674ec80000600155348015601a575f80fd5b50604051611db4380380611db4833981016040819052603791605a565b5f80546001600160a01b0319166001600160a01b03929092169190911790556085565b5f602082840312156069575f80fd5b81516001600160a01b0381168114607e575f80fd5b9392505050565b611d22806100925f395ff3fe60806040526004361061007c575f3560e01c80637b1039991161004c5780637b103999146109345780637e3780ac1461096a578063d8dfeb451461097f578063e79a198f14610994575f80fd5b80631f3fc639146108ba5780633db5d8bd146108d95780636471a855146108f85780636f0b0c1c14610920575f80fd5b366108b6575f5460025460405163791ea01360e11b815260048101919091526001600160a01b039091169063f23d40269034906024015f604051808303818588803b1580156100c9575f80fd5b505af1935050505080156100db575060015b61012c573d808015610108576040519150601f19603f3d011682016040523d82523d5f602084013e61010d565b606091505b50600160035f8282546101209190611069565b90915550610185915050565b60405162461bcd60e51b8152602060048201526024808201527f73686f756c64206e6f742062652061626c6520746f2061646420636f6c6c6174604482015263195c985b60e21b60648201526084015b60405180910390fd5b5f54600254604051630682467760e21b81526001600160a01b0390921691631a0919dc916101b99160040190815260200190565b5f604051808303815f87803b1580156101d0575f80fd5b505af19250505080156101e1575060015b610232573d80801561020e576040519150601f19603f3d011682016040523d82523d5f602084013e610213565b606091505b50600160035f8282546102269190611069565b9091555061027a915050565b60405162461bcd60e51b815260206004820181905260248201527f73686f756c64206e6f742062652061626c6520746f20756e7265676973746572604482015260640161017c565b5f5460025460405163e3fc028d60e01b81526001600160a01b039092169163e3fc028d916102ae9160040190815260200190565b5f604051808303815f87803b1580156102c5575f80fd5b505af19250505080156102d6575060015b610327573d808015610303576040519150601f19603f3d011682016040523d82523d5f602084013e610308565b606091505b50600160035f82825461031b9190611069565b9091555061037e915050565b60405162461bcd60e51b815260206004820152602660248201527f73686f756c64206e6f742062652061626c6520746f20636c61696d20636f6c6c604482015265185d195c985b60d21b606482015260840161017c565b6040805160018082528183019092525f91816020015b6104116040805160e081019091525f606082018181526080830182905260a0830182905260c08301919091528190815260408051610100810182525f8082526020828101829052928201819052606082018190526080820181905260a0820181905260c0820181905260e082015291019081525f60209091015290565b8152602001906001900390816103945750506040805160e08082018352602354606080840191825260245460808086019190915260255460a08087019190915260265460c0808801919091529386528651610100810188526027548152602854602080830191909152602954828a0152602a5494820194909452602b5492810192909252602c5490820152602d5492810192909252602e549282019290925290820152602f546001600160401b03169181019190915281519192509082905f906104dd576104dd6110a2565b60209081029190910101525f8054600d54604051634a3f55c960e11b81526001600160a01b039092169163947eab92916105209186913091879190600401611197565b5f60405180830381865afa15801561053a573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261056191908101906113d0565b5f5460405163fc0ce59b60e01b81529192506001600160a01b03169063fc0ce59b906105919084906004016114d8565b6020604051808303815f875af19250505080156105cb575060408051601f3d908101601f191682019092526105c891810190611556565b60015b61061c573d8080156105f8576040519150601f19603f3d011682016040523d82523d5f602084013e6105fd565b606091505b50600160035f8282546106109190611069565b9091555061067c915050565b5060405162461bcd60e51b815260206004820152602e60248201527f73686f756c64206e6f742062652061626c6520746f20736c617368207265676960448201526d39ba3930ba34b7b71030b3b0b4b760911b606482015260840161017c565b6040805160e08082018352602354606080840191825260245460808086019190915260255460a08087019190915260265460c0808801919091529386528651610100810188526027548152602854602082810191909152602954828a0152602a5494820194909452602b5492810192909252602c5490820152602d5492810192909252602e549282019290925290820152602f546001600160401b031691810191909152825183905f90610732576107326110a2565b6020026020010181905250815160011461075e5760405162461bcd60e51b815260040161017c9061156d565b5f54600154600d5460405163dce4369560e01b81526001600160a01b039093169263dce43695929161079691879130916004016115b6565b60206040518083038185885af1935050505080156107d1575060408051601f3d908101601f191682019092526107ce91810190611556565b60015b610822573d8080156107fe576040519150601f19603f3d011682016040523d82523d5f602084013e610803565b606091505b50600160035f8282546108169190611069565b9091555061086b915050565b5060405162461bcd60e51b815260206004820152601e60248201527f73686f756c64206e6f742062652061626c6520746f2072656769737465720000604482015260640161017c565b6003546005146108b45760405162461bcd60e51b815260206004820152601460248201527373686f756c6420686176652035206572726f727360601b604482015260640161017c565b005b5f80fd5b3480156108c5575f80fd5b506108b46108d436600461174b565b6109a8565b3480156108e4575f80fd5b506108b46108f3366004611a4e565b610b22565b348015610903575f80fd5b5061090d60035481565b6040519081526020015b60405180910390f35b34801561092b575f80fd5b506108b4610fd2565b34801561093f575f80fd5b505f54610952906001600160a01b031681565b6040516001600160a01b039091168152602001610917565b348015610975575f80fd5b5061090d60025481565b34801561098a575f80fd5b5061090d60015481565b34801561099f575f80fd5b506108b4611035565b81516001146109c95760405162461bcd60e51b815260040161017c9061156d565b815f815181106109db576109db6110a2565b602002602001015160235f600181106109f6576109f66110a2565b82518051600d9290920292909201908155602080830151600180840191909155604080850151600285015560609485015160038501558286015180516004808701919091559381015160058601558082015160068601559485015160078501556080850151600885015560a0850151600985015560c0850151600a85015560e090940151600b84015593830151600c909201805467ffffffffffffffff19166001600160401b03909316929092179091555f549254915163dce4369560e01b81526001600160a01b039093169263dce436959291610ada91879130918891016115b6565b60206040518083038185885af1158015610af6573d5f803e3d5ffd5b50505050506040513d601f19601f82011682018060405250810190610b1b9190611556565b6002555050565b8351600490815560208501516005556040850151600680546001600160a01b039283166001600160a01b0319918216179091556060870151600755608087015160085560a08701516009805491841691831691909117905560c0870151600a805491909316911617905560e0850151859190600b90610ba19082611c2d565b506101008201516008820180546001600160401b0392831667ffffffffffffffff1991821617909155610120840151600984015561014090930151600a90920180549282169290931691909117909155604080850151805180518051600f908155602080830151601055828601516011556060928301516012558084015180516013559081015160145580860151601555820151601655938201516017805492840151909616600160a01b026001600160e01b03199092166001600160a01b03909116171790935560808301519092908290601890610c809082611c2d565b505050602082810151600a8301805467ffffffffffffffff19166001600160401b03909216919091179055604080840151600b8401556060938401518051600c85015591820151600d840155810151600e83015591820151600f8201556080820151601082015560a0820151601182015560c0820151601282015560e0909101516013909101555f5b836020015151811015610dfb5783602001518181518110610d2c57610d2c6110a2565b602002602001015160238260018110610d4757610d476110a2565b82518051600d92909202929092019081556020808301516001808401919091556040808501516002850155606094850151600385015582860151805160048601559283015160058501558281015160068501559382015160078401556080820151600884015560a0820151600984015560c0820151600a84015560e090910151600b8301559190920151600c909201805467ffffffffffffffff19166001600160401b039093169290921790915501610d09565b50815180516031805467ffffffffffffffff19166001600160401b0390921691909117815560208201518492908290603290610e379082611c2d565b506040828101516002830155606092830151600390920180546001600160a01b0319166001600160a01b0390931692909217909155602084015160048401805467ffffffffffffffff19166001600160401b0390921691909117905583015160058301558201516006820190610ead9082611c2d565b50508151805180516038908155602080830151603955604080840151603a55606093840151603b55818501518051603c5591820151603d5581810151603e5590830151603f55808401518154938501516001600160401b0316600160a01b026001600160e01b03199094166001600160a01b03909116179290921790915560808201518493509091908290604190610f459082611c2d565b505050602082810151600a8301805467ffffffffffffffff19166001600160401b03909216919091179055604080840151600b8401556060938401518051600c85015591820151600d840155810151600e83015591820151600f8201556080820151601082015560a0820151601182015560c0820151601282015560e09091015160139091015550505050565b5f5460025460405163e3fc028d60e01b81526001600160a01b039092169163e3fc028d916110069160040190815260200190565b5f604051808303815f87803b15801561101d575f80fd5b505af115801561102f573d5f803e3d5ffd5b50505050565b5f54600254604051630682467760e21b81526001600160a01b0390921691631a0919dc916110069160040190815260200190565b8082018082111561108857634e487b7160e01b5f52601160045260245ffd5b92915050565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b80518051835260208101516020840152604081015160408401526060810151606084015250602081015180516080840152602081015160a0840152604081015160c0840152606081015160e0840152608081015161010084015260a081015161012084015260c081015161014084015260e081015161016084015250604081015161114d6101808401826001600160401b03169052565b505050565b5f815180845260208085019450602084015f5b8381101561118c576111788783516110b6565b6101a0969096019590820190600101611165565b509495945050505050565b608081525f6111a96080830187611152565b6001600160a01b03959095166020830152506040810192909252606090910152919050565b60405161010081016001600160401b03811182821017156111f1576111f161108e565b60405290565b604051608081016001600160401b03811182821017156111f1576111f161108e565b604051606081016001600160401b03811182821017156111f1576111f161108e565b60405160a081016001600160401b03811182821017156111f1576111f161108e565b60405161016081016001600160401b03811182821017156111f1576111f161108e565b604051601f8201601f191681016001600160401b03811182821017156112a8576112a861108e565b604052919050565b5f61010082840312156112c1575f80fd5b6112c96111ce565b9050815181526020820151602082015260408201516040820152606082015160608201526080820151608082015260a082015160a082015260c082015160c082015260e082015160e082015292915050565b6001600160401b038116811461132f575f80fd5b50565b805161133d8161131b565b919050565b5f6001600160401b0382111561135a5761135a61108e565b5060051b60200190565b5f82601f830112611373575f80fd5b8151602061138861138383611342565b611280565b8083825260208201915060208460051b8701019350868411156113a9575f80fd5b602086015b848110156113c557805183529183019183016113ae565b509695505050505050565b5f602082840312156113e0575f80fd5b81516001600160401b03808211156113f6575f80fd5b9083019081850361020081121561140b575f80fd5b6114136111f7565b83518152601f19820191506101a08083121561142d575f80fd5b611435611219565b6080841215611442575f80fd5b61144a6111f7565b9350602086015184526040860151602085015260608601516040850152608086015160608501528381526114818960a088016112b0565b6020820152611491828701611332565b60408201526020830152506101c08401519150828211156114b0575f80fd5b6114bc87838601611364565b60408201526101e0939093015160608401525090949350505050565b5f60208083526102208301845182850152818501516114fa60408601826110b6565b5060408501516102006101e08601819052815192839052908301915f91906102408701905b8084101561153f578451825293850193600193909301929085019061151f565b506060979097015195019490945250929392505050565b5f60208284031215611566575f80fd5b5051919050565b60208082526029908201527f74657374206861726e65737320737570706f727473206f6e6c7920312072656760408201526834b9ba3930ba34b7b760b91b606082015260800190565b606081525f6115c86060830186611152565b6001600160a01b039490941660208301525060400152919050565b5f608082840312156115f3575f80fd5b6115fb6111f7565b90508135815260208201356020820152604082013560408201526060820135606082015292915050565b5f6101008284031215611636575f80fd5b61163e6111ce565b9050813581526020820135602082015260408201356040820152606082013560608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e082015292915050565b803561133d8161131b565b5f82601f8301126116aa575f80fd5b813560206116ba61138383611342565b808382526020820191506101a060208186028801019450878511156116dd575f80fd5b602087015b8581101561173e5781818a0312156116f8575f80fd5b611700611219565b61170a8a836115e3565b81526117198a60808401611625565b8682015261018082013561172c8161131b565b604082015284529284019281016116e2565b5090979650505050505050565b5f806040838503121561175c575f80fd5b82356001600160401b03811115611771575f80fd5b61177d8582860161169b565b95602094909401359450505050565b80356001600160a01b038116811461133d575f80fd5b5f82601f8301126117b1575f80fd5b81356001600160401b038111156117ca576117ca61108e565b6117dd601f8201601f1916602001611280565b8181528460208386010111156117f1575f80fd5b816020850160208301375f918101602001919091529392505050565b5f61016080838503121561181f575f80fd5b6118276111f7565b915082356001600160401b038082111561183f575f80fd5b8185019150828287031215611852575f80fd5b61185a61123b565b925061186686836115e3565b835261187586608084016115e3565b6020840152611887610100830161178c565b604084015261012082013561189b8161131b565b6060840152610140820135818111156118b2575f80fd5b6118be878285016117a2565b608085015250505081526118d460208301611690565b6020820152604082013560408201526118f08360608401611625565b606082015292915050565b5f6060828403121561190b575f80fd5b611913611219565b90508135815260208201356001600160401b0380821115611932575f80fd5b61193e8583860161169b565b60208401526040840135915080821115611956575f80fd5b506119638482850161180d565b60408301525092915050565b5f6080828403121561197f575f80fd5b6119876111f7565b905081356001600160401b038082111561199f575f80fd5b90830190608082860312156119b2575f80fd5b6119ba6111f7565b82356119c58161131b565b81526020830135828111156119d8575f80fd5b6119e4878286016117a2565b60208301525060408301356040820152611a006060840161178c565b60608201528352611a1360208501611690565b6020840152604084013560408401526060840135915080821115611a35575f80fd5b50611a42848285016117a2565b60608301525092915050565b5f805f8060808587031215611a61575f80fd5b84356001600160401b0380821115611a77575f80fd5b908601906101608289031215611a8b575f80fd5b611a9361125d565b8235815260208301356020820152611aad6040840161178c565b60408201526060830135606082015260808301356080820152611ad260a0840161178c565b60a0820152611ae360c0840161178c565b60c082015260e083013582811115611af9575f80fd5b611b058a8286016117a2565b60e083015250610100611b19818501611690565b908201526101208381013590820152610140611b36818501611690565b9082015295506020870135915080821115611b4f575f80fd5b611b5b888389016118fb565b94506040870135915080821115611b70575f80fd5b611b7c8883890161196f565b93506060870135915080821115611b91575f80fd5b50611b9e8782880161180d565b91505092959194509250565b600181811c90821680611bbe57607f821691505b602082108103611bdc57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561114d57805f5260205f20601f840160051c81016020851015611c075750805b601f840160051c820191505b81811015611c26575f8155600101611c13565b5050505050565b81516001600160401b03811115611c4657611c4661108e565b611c5a81611c548454611baa565b84611be2565b602080601f831160018114611c8d575f8415611c765750858301515b5f19600386901b1c1916600185901b178555611ce4565b5f85815260208120601f198616915b82811015611cbb57888601518255948401946001909101908401611c9c565b5085821015611cd857878501515f19600388901b60f8161c191681555b505060018460011b0185555b50505050505056fea2646970667358221220b789078602d68440f078754b7e3fe42ee4279ea67491b412c357da6cc10faee964736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@Rg\x1B\xC1mgN\xC8\0\0`\x01U4\x80\x15`\x1AW_\x80\xFD[P`@Qa\x1D\xB48\x03\x80a\x1D\xB4\x839\x81\x01`@\x81\x90R`7\x91`ZV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\x85V[_` \x82\x84\x03\x12\x15`iW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`~W_\x80\xFD[\x93\x92PPPV[a\x1D\"\x80a\0\x92_9_\xF3\xFE`\x80`@R`\x046\x10a\0|W_5`\xE0\x1C\x80c{\x109\x99\x11a\0LW\x80c{\x109\x99\x14a\t4W\x80c~7\x80\xAC\x14a\tjW\x80c\xD8\xDF\xEBE\x14a\t\x7FW\x80c\xE7\x9A\x19\x8F\x14a\t\x94W_\x80\xFD[\x80c\x1F?\xC69\x14a\x08\xBAW\x80c=\xB5\xD8\xBD\x14a\x08\xD9W\x80cdq\xA8U\x14a\x08\xF8W\x80co\x0B\x0C\x1C\x14a\t W_\x80\xFD[6a\x08\xB6W_T`\x02T`@Qcy\x1E\xA0\x13`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF2=@&\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\0\xC9W_\x80\xFD[PZ\xF1\x93PPPP\x80\x15a\0\xDBWP`\x01[a\x01,W=\x80\x80\x15a\x01\x08W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x01\rV[``\x91P[P`\x01`\x03_\x82\x82Ta\x01 \x91\x90a\x10iV[\x90\x91UPa\x01\x85\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fshould not be able to add collat`D\x82\x01Rc\x19\\\x98[`\xE2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_T`\x02T`@Qc\x06\x82Fw`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1A\t\x19\xDC\x91a\x01\xB9\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xD0W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01\xE1WP`\x01[a\x022W=\x80\x80\x15a\x02\x0EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02\x13V[``\x91P[P`\x01`\x03_\x82\x82Ta\x02&\x91\x90a\x10iV[\x90\x91UPa\x02z\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Fshould not be able to unregister`D\x82\x01R`d\x01a\x01|V[_T`\x02T`@Qc\xE3\xFC\x02\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE3\xFC\x02\x8D\x91a\x02\xAE\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xC5W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x02\xD6WP`\x01[a\x03'W=\x80\x80\x15a\x03\x03W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\x08V[``\x91P[P`\x01`\x03_\x82\x82Ta\x03\x1B\x91\x90a\x10iV[\x90\x91UPa\x03~\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fshould not be able to claim coll`D\x82\x01Re\x18]\x19\\\x98[`\xD2\x1B`d\x82\x01R`\x84\x01a\x01|V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[a\x04\x11`@\x80Q`\xE0\x81\x01\x90\x91R_``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R`\xA0\x83\x01\x82\x90R`\xC0\x83\x01\x91\x90\x91R\x81\x90\x81R`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01\x81\x90R`\xE0\x82\x01R\x91\x01\x90\x81R_` \x90\x91\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\x94WPP`@\x80Q`\xE0\x80\x82\x01\x83R`#T``\x80\x84\x01\x91\x82R`$T`\x80\x80\x86\x01\x91\x90\x91R`%T`\xA0\x80\x87\x01\x91\x90\x91R`&T`\xC0\x80\x88\x01\x91\x90\x91R\x93\x86R\x86Qa\x01\0\x81\x01\x88R`'T\x81R`(T` \x80\x83\x01\x91\x90\x91R`)T\x82\x8A\x01R`*T\x94\x82\x01\x94\x90\x94R`+T\x92\x81\x01\x92\x90\x92R`,T\x90\x82\x01R`-T\x92\x81\x01\x92\x90\x92R`.T\x92\x82\x01\x92\x90\x92R\x90\x82\x01R`/T`\x01`\x01`@\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x81Q\x91\x92P\x90\x82\x90_\x90a\x04\xDDWa\x04\xDDa\x10\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x01R_\x80T`\rT`@QcJ?U\xC9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x94~\xAB\x92\x91a\x05 \x91\x86\x910\x91\x87\x91\x90`\x04\x01a\x11\x97V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05:W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05a\x91\x90\x81\x01\x90a\x13\xD0V[_T`@Qc\xFC\x0C\xE5\x9B`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFC\x0C\xE5\x9B\x90a\x05\x91\x90\x84\x90`\x04\x01a\x14\xD8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x05\xCBWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x05\xC8\x91\x81\x01\x90a\x15VV[`\x01[a\x06\x1CW=\x80\x80\x15a\x05\xF8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x05\xFDV[``\x91P[P`\x01`\x03_\x82\x82Ta\x06\x10\x91\x90a\x10iV[\x90\x91UPa\x06|\x91PPV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7Fshould not be able to slash regi`D\x82\x01Rm9\xBA90\xBA4\xB7\xB7\x100\xB3\xB0\xB4\xB7`\x91\x1B`d\x82\x01R`\x84\x01a\x01|V[`@\x80Q`\xE0\x80\x82\x01\x83R`#T``\x80\x84\x01\x91\x82R`$T`\x80\x80\x86\x01\x91\x90\x91R`%T`\xA0\x80\x87\x01\x91\x90\x91R`&T`\xC0\x80\x88\x01\x91\x90\x91R\x93\x86R\x86Qa\x01\0\x81\x01\x88R`'T\x81R`(T` \x82\x81\x01\x91\x90\x91R`)T\x82\x8A\x01R`*T\x94\x82\x01\x94\x90\x94R`+T\x92\x81\x01\x92\x90\x92R`,T\x90\x82\x01R`-T\x92\x81\x01\x92\x90\x92R`.T\x92\x82\x01\x92\x90\x92R\x90\x82\x01R`/T`\x01`\x01`@\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x82Q\x83\x90_\x90a\x072Wa\x072a\x10\xA2V[` \x02` \x01\x01\x81\x90RP\x81Q`\x01\x14a\x07^W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01|\x90a\x15mV[_T`\x01T`\rT`@Qc\xDC\xE46\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xDC\xE46\x95\x92\x91a\x07\x96\x91\x87\x910\x91`\x04\x01a\x15\xB6V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x93PPPP\x80\x15a\x07\xD1WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x07\xCE\x91\x81\x01\x90a\x15VV[`\x01[a\x08\"W=\x80\x80\x15a\x07\xFEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x08\x03V[``\x91P[P`\x01`\x03_\x82\x82Ta\x08\x16\x91\x90a\x10iV[\x90\x91UPa\x08k\x91PPV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fshould not be able to register\0\0`D\x82\x01R`d\x01a\x01|V[`\x03T`\x05\x14a\x08\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsshould have 5 errors``\x1B`D\x82\x01R`d\x01a\x01|V[\0[_\x80\xFD[4\x80\x15a\x08\xC5W_\x80\xFD[Pa\x08\xB4a\x08\xD46`\x04a\x17KV[a\t\xA8V[4\x80\x15a\x08\xE4W_\x80\xFD[Pa\x08\xB4a\x08\xF36`\x04a\x1ANV[a\x0B\"V[4\x80\x15a\t\x03W_\x80\xFD[Pa\t\r`\x03T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t+W_\x80\xFD[Pa\x08\xB4a\x0F\xD2V[4\x80\x15a\t?W_\x80\xFD[P_Ta\tR\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\t\x17V[4\x80\x15a\tuW_\x80\xFD[Pa\t\r`\x02T\x81V[4\x80\x15a\t\x8AW_\x80\xFD[Pa\t\r`\x01T\x81V[4\x80\x15a\t\x9FW_\x80\xFD[Pa\x08\xB4a\x105V[\x81Q`\x01\x14a\t\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01|\x90a\x15mV[\x81_\x81Q\x81\x10a\t\xDBWa\t\xDBa\x10\xA2V[` \x02` \x01\x01Q`#_`\x01\x81\x10a\t\xF6Wa\t\xF6a\x10\xA2V[\x82Q\x80Q`\r\x92\x90\x92\x02\x92\x90\x92\x01\x90\x81U` \x80\x83\x01Q`\x01\x80\x84\x01\x91\x90\x91U`@\x80\x85\x01Q`\x02\x85\x01U``\x94\x85\x01Q`\x03\x85\x01U\x82\x86\x01Q\x80Q`\x04\x80\x87\x01\x91\x90\x91U\x93\x81\x01Q`\x05\x86\x01U\x80\x82\x01Q`\x06\x86\x01U\x94\x85\x01Q`\x07\x85\x01U`\x80\x85\x01Q`\x08\x85\x01U`\xA0\x85\x01Q`\t\x85\x01U`\xC0\x85\x01Q`\n\x85\x01U`\xE0\x90\x94\x01Q`\x0B\x84\x01U\x93\x83\x01Q`\x0C\x90\x92\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U_T\x92T\x91Qc\xDC\xE46\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xDC\xE46\x95\x92\x91a\n\xDA\x91\x87\x910\x91\x88\x91\x01a\x15\xB6V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\n\xF6W=_\x80>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x1B\x91\x90a\x15VV[`\x02UPPV[\x83Q`\x04\x90\x81U` \x85\x01Q`\x05U`@\x85\x01Q`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U``\x87\x01Q`\x07U`\x80\x87\x01Q`\x08U`\xA0\x87\x01Q`\t\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`\xC0\x87\x01Q`\n\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xE0\x85\x01Q\x85\x91\x90`\x0B\x90a\x0B\xA1\x90\x82a\x1C-V[Pa\x01\0\x82\x01Q`\x08\x82\x01\x80T`\x01`\x01`@\x1B\x03\x92\x83\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x17\x90\x91Ua\x01 \x84\x01Q`\t\x84\x01Ua\x01@\x90\x93\x01Q`\n\x90\x92\x01\x80T\x92\x82\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x91U`@\x80\x85\x01Q\x80Q\x80Q\x80Q`\x0F\x90\x81U` \x80\x83\x01Q`\x10U\x82\x86\x01Q`\x11U``\x92\x83\x01Q`\x12U\x80\x84\x01Q\x80Q`\x13U\x90\x81\x01Q`\x14U\x80\x86\x01Q`\x15U\x82\x01Q`\x16U\x93\x82\x01Q`\x17\x80T\x92\x84\x01Q\x90\x96\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x17\x90\x93U`\x80\x83\x01Q\x90\x92\x90\x82\x90`\x18\x90a\x0C\x80\x90\x82a\x1C-V[PPP` \x82\x81\x01Q`\n\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80\x84\x01Q`\x0B\x84\x01U``\x93\x84\x01Q\x80Q`\x0C\x85\x01U\x91\x82\x01Q`\r\x84\x01U\x81\x01Q`\x0E\x83\x01U\x91\x82\x01Q`\x0F\x82\x01U`\x80\x82\x01Q`\x10\x82\x01U`\xA0\x82\x01Q`\x11\x82\x01U`\xC0\x82\x01Q`\x12\x82\x01U`\xE0\x90\x91\x01Q`\x13\x90\x91\x01U_[\x83` \x01QQ\x81\x10\x15a\r\xFBW\x83` \x01Q\x81\x81Q\x81\x10a\r,Wa\r,a\x10\xA2V[` \x02` \x01\x01Q`#\x82`\x01\x81\x10a\rGWa\rGa\x10\xA2V[\x82Q\x80Q`\r\x92\x90\x92\x02\x92\x90\x92\x01\x90\x81U` \x80\x83\x01Q`\x01\x80\x84\x01\x91\x90\x91U`@\x80\x85\x01Q`\x02\x85\x01U``\x94\x85\x01Q`\x03\x85\x01U\x82\x86\x01Q\x80Q`\x04\x86\x01U\x92\x83\x01Q`\x05\x85\x01U\x82\x81\x01Q`\x06\x85\x01U\x93\x82\x01Q`\x07\x84\x01U`\x80\x82\x01Q`\x08\x84\x01U`\xA0\x82\x01Q`\t\x84\x01U`\xC0\x82\x01Q`\n\x84\x01U`\xE0\x90\x91\x01Q`\x0B\x83\x01U\x91\x90\x92\x01Q`\x0C\x90\x92\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x01a\r\tV[P\x81Q\x80Q`1\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x81U` \x82\x01Q\x84\x92\x90\x82\x90`2\x90a\x0E7\x90\x82a\x1C-V[P`@\x82\x81\x01Q`\x02\x83\x01U``\x92\x83\x01Q`\x03\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U` \x84\x01Q`\x04\x84\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83\x01Q`\x05\x83\x01U\x82\x01Q`\x06\x82\x01\x90a\x0E\xAD\x90\x82a\x1C-V[PP\x81Q\x80Q\x80Q`8\x90\x81U` \x80\x83\x01Q`9U`@\x80\x84\x01Q`:U``\x93\x84\x01Q`;U\x81\x85\x01Q\x80Q`<U\x91\x82\x01Q`=U\x81\x81\x01Q`>U\x90\x83\x01Q`?U\x80\x84\x01Q\x81T\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x92\x90\x92\x17\x90\x91U`\x80\x82\x01Q\x84\x93P\x90\x91\x90\x82\x90`A\x90a\x0FE\x90\x82a\x1C-V[PPP` \x82\x81\x01Q`\n\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80\x84\x01Q`\x0B\x84\x01U``\x93\x84\x01Q\x80Q`\x0C\x85\x01U\x91\x82\x01Q`\r\x84\x01U\x81\x01Q`\x0E\x83\x01U\x91\x82\x01Q`\x0F\x82\x01U`\x80\x82\x01Q`\x10\x82\x01U`\xA0\x82\x01Q`\x11\x82\x01U`\xC0\x82\x01Q`\x12\x82\x01U`\xE0\x90\x91\x01Q`\x13\x90\x91\x01UPPPPV[_T`\x02T`@Qc\xE3\xFC\x02\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE3\xFC\x02\x8D\x91a\x10\x06\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x1DW_\x80\xFD[PZ\xF1\x15\x80\x15a\x10/W=_\x80>=_\xFD[PPPPV[_T`\x02T`@Qc\x06\x82Fw`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1A\t\x19\xDC\x91a\x10\x06\x91`\x04\x01\x90\x81R` \x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x10\x88WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01RP` \x81\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01R`@\x81\x01Q`\xC0\x84\x01R``\x81\x01Q`\xE0\x84\x01R`\x80\x81\x01Qa\x01\0\x84\x01R`\xA0\x81\x01Qa\x01 \x84\x01R`\xC0\x81\x01Qa\x01@\x84\x01R`\xE0\x81\x01Qa\x01`\x84\x01RP`@\x81\x01Qa\x11Ma\x01\x80\x84\x01\x82`\x01`\x01`@\x1B\x03\x16\x90RV[PPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x11\x8CWa\x11x\x87\x83Qa\x10\xB6V[a\x01\xA0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x11eV[P\x94\x95\x94PPPPPV[`\x80\x81R_a\x11\xA9`\x80\x83\x01\x87a\x11RV[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16` \x83\x01RP`@\x81\x01\x92\x90\x92R``\x90\x91\x01R\x91\x90PV[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@Qa\x01`\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x12\xA8Wa\x12\xA8a\x10\x8EV[`@R\x91\x90PV[_a\x01\0\x82\x84\x03\x12\x15a\x12\xC1W_\x80\xFD[a\x12\xC9a\x11\xCEV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Q`\x80\x82\x01R`\xA0\x82\x01Q`\xA0\x82\x01R`\xC0\x82\x01Q`\xC0\x82\x01R`\xE0\x82\x01Q`\xE0\x82\x01R\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x13/W_\x80\xFD[PV[\x80Qa\x13=\x81a\x13\x1BV[\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x13ZWa\x13Za\x10\x8EV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x13sW_\x80\xFD[\x81Q` a\x13\x88a\x13\x83\x83a\x13BV[a\x12\x80V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x13\xA9W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x13\xC5W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x13\xAEV[P\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x13\xE0W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\xF6W_\x80\xFD[\x90\x83\x01\x90\x81\x85\x03a\x02\0\x81\x12\x15a\x14\x0BW_\x80\xFD[a\x14\x13a\x11\xF7V[\x83Q\x81R`\x1F\x19\x82\x01\x91Pa\x01\xA0\x80\x83\x12\x15a\x14-W_\x80\xFD[a\x145a\x12\x19V[`\x80\x84\x12\x15a\x14BW_\x80\xFD[a\x14Ja\x11\xF7V[\x93P` \x86\x01Q\x84R`@\x86\x01Q` \x85\x01R``\x86\x01Q`@\x85\x01R`\x80\x86\x01Q``\x85\x01R\x83\x81Ra\x14\x81\x89`\xA0\x88\x01a\x12\xB0V[` \x82\x01Ra\x14\x91\x82\x87\x01a\x132V[`@\x82\x01R` \x83\x01RPa\x01\xC0\x84\x01Q\x91P\x82\x82\x11\x15a\x14\xB0W_\x80\xFD[a\x14\xBC\x87\x83\x86\x01a\x13dV[`@\x82\x01Ra\x01\xE0\x93\x90\x93\x01Q``\x84\x01RP\x90\x94\x93PPPPV[_` \x80\x83Ra\x02 \x83\x01\x84Q\x82\x85\x01R\x81\x85\x01Qa\x14\xFA`@\x86\x01\x82a\x10\xB6V[P`@\x85\x01Qa\x02\0a\x01\xE0\x86\x01\x81\x90R\x81Q\x92\x83\x90R\x90\x83\x01\x91_\x91\x90a\x02@\x87\x01\x90[\x80\x84\x10\x15a\x15?W\x84Q\x82R\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x85\x01\x90a\x15\x1FV[P``\x97\x90\x97\x01Q\x95\x01\x94\x90\x94RP\x92\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x15fW_\x80\xFD[PQ\x91\x90PV[` \x80\x82R`)\x90\x82\x01R\x7Ftest harness supports only 1 reg`@\x82\x01Rh4\xB9\xBA90\xBA4\xB7\xB7`\xB9\x1B``\x82\x01R`\x80\x01\x90V[``\x81R_a\x15\xC8``\x83\x01\x86a\x11RV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16` \x83\x01RP`@\x01R\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\x15\xF3W_\x80\xFD[a\x15\xFBa\x11\xF7V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[_a\x01\0\x82\x84\x03\x12\x15a\x166W_\x80\xFD[a\x16>a\x11\xCEV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01R\x92\x91PPV[\x805a\x13=\x81a\x13\x1BV[_\x82`\x1F\x83\x01\x12a\x16\xAAW_\x80\xFD[\x815` a\x16\xBAa\x13\x83\x83a\x13BV[\x80\x83\x82R` \x82\x01\x91Pa\x01\xA0` \x81\x86\x02\x88\x01\x01\x94P\x87\x85\x11\x15a\x16\xDDW_\x80\xFD[` \x87\x01[\x85\x81\x10\x15a\x17>W\x81\x81\x8A\x03\x12\x15a\x16\xF8W_\x80\xFD[a\x17\0a\x12\x19V[a\x17\n\x8A\x83a\x15\xE3V[\x81Ra\x17\x19\x8A`\x80\x84\x01a\x16%V[\x86\x82\x01Ra\x01\x80\x82\x015a\x17,\x81a\x13\x1BV[`@\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x16\xE2V[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x17\\W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17qW_\x80\xFD[a\x17}\x85\x82\x86\x01a\x16\x9BV[\x95` \x94\x90\x94\x015\x94PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13=W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x17\xB1W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xCAWa\x17\xCAa\x10\x8EV[a\x17\xDD`\x1F\x82\x01`\x1F\x19\x16` \x01a\x12\x80V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x17\xF1W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_a\x01`\x80\x83\x85\x03\x12\x15a\x18\x1FW_\x80\xFD[a\x18'a\x11\xF7V[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18?W_\x80\xFD[\x81\x85\x01\x91P\x82\x82\x87\x03\x12\x15a\x18RW_\x80\xFD[a\x18Za\x12;V[\x92Pa\x18f\x86\x83a\x15\xE3V[\x83Ra\x18u\x86`\x80\x84\x01a\x15\xE3V[` \x84\x01Ra\x18\x87a\x01\0\x83\x01a\x17\x8CV[`@\x84\x01Ra\x01 \x82\x015a\x18\x9B\x81a\x13\x1BV[``\x84\x01Ra\x01@\x82\x015\x81\x81\x11\x15a\x18\xB2W_\x80\xFD[a\x18\xBE\x87\x82\x85\x01a\x17\xA2V[`\x80\x85\x01RPPP\x81Ra\x18\xD4` \x83\x01a\x16\x90V[` \x82\x01R`@\x82\x015`@\x82\x01Ra\x18\xF0\x83``\x84\x01a\x16%V[``\x82\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x19\x0BW_\x80\xFD[a\x19\x13a\x12\x19V[\x90P\x815\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x192W_\x80\xFD[a\x19>\x85\x83\x86\x01a\x16\x9BV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x19VW_\x80\xFD[Pa\x19c\x84\x82\x85\x01a\x18\rV[`@\x83\x01RP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a\x19\x7FW_\x80\xFD[a\x19\x87a\x11\xF7V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\x9FW_\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15a\x19\xB2W_\x80\xFD[a\x19\xBAa\x11\xF7V[\x825a\x19\xC5\x81a\x13\x1BV[\x81R` \x83\x015\x82\x81\x11\x15a\x19\xD8W_\x80\xFD[a\x19\xE4\x87\x82\x86\x01a\x17\xA2V[` \x83\x01RP`@\x83\x015`@\x82\x01Ra\x1A\0``\x84\x01a\x17\x8CV[``\x82\x01R\x83Ra\x1A\x13` \x85\x01a\x16\x90V[` \x84\x01R`@\x84\x015`@\x84\x01R``\x84\x015\x91P\x80\x82\x11\x15a\x1A5W_\x80\xFD[Pa\x1AB\x84\x82\x85\x01a\x17\xA2V[``\x83\x01RP\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x1AaW_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1AwW_\x80\xFD[\x90\x86\x01\x90a\x01`\x82\x89\x03\x12\x15a\x1A\x8BW_\x80\xFD[a\x1A\x93a\x12]V[\x825\x81R` \x83\x015` \x82\x01Ra\x1A\xAD`@\x84\x01a\x17\x8CV[`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01Ra\x1A\xD2`\xA0\x84\x01a\x17\x8CV[`\xA0\x82\x01Ra\x1A\xE3`\xC0\x84\x01a\x17\x8CV[`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x1A\xF9W_\x80\xFD[a\x1B\x05\x8A\x82\x86\x01a\x17\xA2V[`\xE0\x83\x01RPa\x01\0a\x1B\x19\x81\x85\x01a\x16\x90V[\x90\x82\x01Ra\x01 \x83\x81\x015\x90\x82\x01Ra\x01@a\x1B6\x81\x85\x01a\x16\x90V[\x90\x82\x01R\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x1BOW_\x80\xFD[a\x1B[\x88\x83\x89\x01a\x18\xFBV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x1BpW_\x80\xFD[a\x1B|\x88\x83\x89\x01a\x19oV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1B\x91W_\x80\xFD[Pa\x1B\x9E\x87\x82\x88\x01a\x18\rV[\x91PP\x92\x95\x91\x94P\x92PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1B\xBEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1B\xDCWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x11MW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x1C\x07WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1C&W_\x81U`\x01\x01a\x1C\x13V[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1CFWa\x1CFa\x10\x8EV[a\x1CZ\x81a\x1CT\x84Ta\x1B\xAAV[\x84a\x1B\xE2V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1C\x8DW_\x84\x15a\x1CvWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1C\xE4V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x1C\xBBW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1C\x9CV[P\x85\x82\x10\x15a\x1C\xD8W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV\xFE\xA2dipfsX\"\x12 \xB7\x89\x07\x86\x02\xD6\x84@\xF0xuK~?\xE4.\xE4'\x9E\xA6t\x91\xB4\x12\xC3W\xDAl\xC1\x0F\xAE\xE9dsolcC\0\x08\x19\x003",
    );
	/// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061007c575f3560e01c80637b1039991161004c5780637b103999146109345780637e3780ac1461096a578063d8dfeb451461097f578063e79a198f14610994575f80fd5b80631f3fc639146108ba5780633db5d8bd146108d95780636471a855146108f85780636f0b0c1c14610920575f80fd5b366108b6575f5460025460405163791ea01360e11b815260048101919091526001600160a01b039091169063f23d40269034906024015f604051808303818588803b1580156100c9575f80fd5b505af1935050505080156100db575060015b61012c573d808015610108576040519150601f19603f3d011682016040523d82523d5f602084013e61010d565b606091505b50600160035f8282546101209190611069565b90915550610185915050565b60405162461bcd60e51b8152602060048201526024808201527f73686f756c64206e6f742062652061626c6520746f2061646420636f6c6c6174604482015263195c985b60e21b60648201526084015b60405180910390fd5b5f54600254604051630682467760e21b81526001600160a01b0390921691631a0919dc916101b99160040190815260200190565b5f604051808303815f87803b1580156101d0575f80fd5b505af19250505080156101e1575060015b610232573d80801561020e576040519150601f19603f3d011682016040523d82523d5f602084013e610213565b606091505b50600160035f8282546102269190611069565b9091555061027a915050565b60405162461bcd60e51b815260206004820181905260248201527f73686f756c64206e6f742062652061626c6520746f20756e7265676973746572604482015260640161017c565b5f5460025460405163e3fc028d60e01b81526001600160a01b039092169163e3fc028d916102ae9160040190815260200190565b5f604051808303815f87803b1580156102c5575f80fd5b505af19250505080156102d6575060015b610327573d808015610303576040519150601f19603f3d011682016040523d82523d5f602084013e610308565b606091505b50600160035f82825461031b9190611069565b9091555061037e915050565b60405162461bcd60e51b815260206004820152602660248201527f73686f756c64206e6f742062652061626c6520746f20636c61696d20636f6c6c604482015265185d195c985b60d21b606482015260840161017c565b6040805160018082528183019092525f91816020015b6104116040805160e081019091525f606082018181526080830182905260a0830182905260c08301919091528190815260408051610100810182525f8082526020828101829052928201819052606082018190526080820181905260a0820181905260c0820181905260e082015291019081525f60209091015290565b8152602001906001900390816103945750506040805160e08082018352602354606080840191825260245460808086019190915260255460a08087019190915260265460c0808801919091529386528651610100810188526027548152602854602080830191909152602954828a0152602a5494820194909452602b5492810192909252602c5490820152602d5492810192909252602e549282019290925290820152602f546001600160401b03169181019190915281519192509082905f906104dd576104dd6110a2565b60209081029190910101525f8054600d54604051634a3f55c960e11b81526001600160a01b039092169163947eab92916105209186913091879190600401611197565b5f60405180830381865afa15801561053a573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261056191908101906113d0565b5f5460405163fc0ce59b60e01b81529192506001600160a01b03169063fc0ce59b906105919084906004016114d8565b6020604051808303815f875af19250505080156105cb575060408051601f3d908101601f191682019092526105c891810190611556565b60015b61061c573d8080156105f8576040519150601f19603f3d011682016040523d82523d5f602084013e6105fd565b606091505b50600160035f8282546106109190611069565b9091555061067c915050565b5060405162461bcd60e51b815260206004820152602e60248201527f73686f756c64206e6f742062652061626c6520746f20736c617368207265676960448201526d39ba3930ba34b7b71030b3b0b4b760911b606482015260840161017c565b6040805160e08082018352602354606080840191825260245460808086019190915260255460a08087019190915260265460c0808801919091529386528651610100810188526027548152602854602082810191909152602954828a0152602a5494820194909452602b5492810192909252602c5490820152602d5492810192909252602e549282019290925290820152602f546001600160401b031691810191909152825183905f90610732576107326110a2565b6020026020010181905250815160011461075e5760405162461bcd60e51b815260040161017c9061156d565b5f54600154600d5460405163dce4369560e01b81526001600160a01b039093169263dce43695929161079691879130916004016115b6565b60206040518083038185885af1935050505080156107d1575060408051601f3d908101601f191682019092526107ce91810190611556565b60015b610822573d8080156107fe576040519150601f19603f3d011682016040523d82523d5f602084013e610803565b606091505b50600160035f8282546108169190611069565b9091555061086b915050565b5060405162461bcd60e51b815260206004820152601e60248201527f73686f756c64206e6f742062652061626c6520746f2072656769737465720000604482015260640161017c565b6003546005146108b45760405162461bcd60e51b815260206004820152601460248201527373686f756c6420686176652035206572726f727360601b604482015260640161017c565b005b5f80fd5b3480156108c5575f80fd5b506108b46108d436600461174b565b6109a8565b3480156108e4575f80fd5b506108b46108f3366004611a4e565b610b22565b348015610903575f80fd5b5061090d60035481565b6040519081526020015b60405180910390f35b34801561092b575f80fd5b506108b4610fd2565b34801561093f575f80fd5b505f54610952906001600160a01b031681565b6040516001600160a01b039091168152602001610917565b348015610975575f80fd5b5061090d60025481565b34801561098a575f80fd5b5061090d60015481565b34801561099f575f80fd5b506108b4611035565b81516001146109c95760405162461bcd60e51b815260040161017c9061156d565b815f815181106109db576109db6110a2565b602002602001015160235f600181106109f6576109f66110a2565b82518051600d9290920292909201908155602080830151600180840191909155604080850151600285015560609485015160038501558286015180516004808701919091559381015160058601558082015160068601559485015160078501556080850151600885015560a0850151600985015560c0850151600a85015560e090940151600b84015593830151600c909201805467ffffffffffffffff19166001600160401b03909316929092179091555f549254915163dce4369560e01b81526001600160a01b039093169263dce436959291610ada91879130918891016115b6565b60206040518083038185885af1158015610af6573d5f803e3d5ffd5b50505050506040513d601f19601f82011682018060405250810190610b1b9190611556565b6002555050565b8351600490815560208501516005556040850151600680546001600160a01b039283166001600160a01b0319918216179091556060870151600755608087015160085560a08701516009805491841691831691909117905560c0870151600a805491909316911617905560e0850151859190600b90610ba19082611c2d565b506101008201516008820180546001600160401b0392831667ffffffffffffffff1991821617909155610120840151600984015561014090930151600a90920180549282169290931691909117909155604080850151805180518051600f908155602080830151601055828601516011556060928301516012558084015180516013559081015160145580860151601555820151601655938201516017805492840151909616600160a01b026001600160e01b03199092166001600160a01b03909116171790935560808301519092908290601890610c809082611c2d565b505050602082810151600a8301805467ffffffffffffffff19166001600160401b03909216919091179055604080840151600b8401556060938401518051600c85015591820151600d840155810151600e83015591820151600f8201556080820151601082015560a0820151601182015560c0820151601282015560e0909101516013909101555f5b836020015151811015610dfb5783602001518181518110610d2c57610d2c6110a2565b602002602001015160238260018110610d4757610d476110a2565b82518051600d92909202929092019081556020808301516001808401919091556040808501516002850155606094850151600385015582860151805160048601559283015160058501558281015160068501559382015160078401556080820151600884015560a0820151600984015560c0820151600a84015560e090910151600b8301559190920151600c909201805467ffffffffffffffff19166001600160401b039093169290921790915501610d09565b50815180516031805467ffffffffffffffff19166001600160401b0390921691909117815560208201518492908290603290610e379082611c2d565b506040828101516002830155606092830151600390920180546001600160a01b0319166001600160a01b0390931692909217909155602084015160048401805467ffffffffffffffff19166001600160401b0390921691909117905583015160058301558201516006820190610ead9082611c2d565b50508151805180516038908155602080830151603955604080840151603a55606093840151603b55818501518051603c5591820151603d5581810151603e5590830151603f55808401518154938501516001600160401b0316600160a01b026001600160e01b03199094166001600160a01b03909116179290921790915560808201518493509091908290604190610f459082611c2d565b505050602082810151600a8301805467ffffffffffffffff19166001600160401b03909216919091179055604080840151600b8401556060938401518051600c85015591820151600d840155810151600e83015591820151600f8201556080820151601082015560a0820151601182015560c0820151601282015560e09091015160139091015550505050565b5f5460025460405163e3fc028d60e01b81526001600160a01b039092169163e3fc028d916110069160040190815260200190565b5f604051808303815f87803b15801561101d575f80fd5b505af115801561102f573d5f803e3d5ffd5b50505050565b5f54600254604051630682467760e21b81526001600160a01b0390921691631a0919dc916110069160040190815260200190565b8082018082111561108857634e487b7160e01b5f52601160045260245ffd5b92915050565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b80518051835260208101516020840152604081015160408401526060810151606084015250602081015180516080840152602081015160a0840152604081015160c0840152606081015160e0840152608081015161010084015260a081015161012084015260c081015161014084015260e081015161016084015250604081015161114d6101808401826001600160401b03169052565b505050565b5f815180845260208085019450602084015f5b8381101561118c576111788783516110b6565b6101a0969096019590820190600101611165565b509495945050505050565b608081525f6111a96080830187611152565b6001600160a01b03959095166020830152506040810192909252606090910152919050565b60405161010081016001600160401b03811182821017156111f1576111f161108e565b60405290565b604051608081016001600160401b03811182821017156111f1576111f161108e565b604051606081016001600160401b03811182821017156111f1576111f161108e565b60405160a081016001600160401b03811182821017156111f1576111f161108e565b60405161016081016001600160401b03811182821017156111f1576111f161108e565b604051601f8201601f191681016001600160401b03811182821017156112a8576112a861108e565b604052919050565b5f61010082840312156112c1575f80fd5b6112c96111ce565b9050815181526020820151602082015260408201516040820152606082015160608201526080820151608082015260a082015160a082015260c082015160c082015260e082015160e082015292915050565b6001600160401b038116811461132f575f80fd5b50565b805161133d8161131b565b919050565b5f6001600160401b0382111561135a5761135a61108e565b5060051b60200190565b5f82601f830112611373575f80fd5b8151602061138861138383611342565b611280565b8083825260208201915060208460051b8701019350868411156113a9575f80fd5b602086015b848110156113c557805183529183019183016113ae565b509695505050505050565b5f602082840312156113e0575f80fd5b81516001600160401b03808211156113f6575f80fd5b9083019081850361020081121561140b575f80fd5b6114136111f7565b83518152601f19820191506101a08083121561142d575f80fd5b611435611219565b6080841215611442575f80fd5b61144a6111f7565b9350602086015184526040860151602085015260608601516040850152608086015160608501528381526114818960a088016112b0565b6020820152611491828701611332565b60408201526020830152506101c08401519150828211156114b0575f80fd5b6114bc87838601611364565b60408201526101e0939093015160608401525090949350505050565b5f60208083526102208301845182850152818501516114fa60408601826110b6565b5060408501516102006101e08601819052815192839052908301915f91906102408701905b8084101561153f578451825293850193600193909301929085019061151f565b506060979097015195019490945250929392505050565b5f60208284031215611566575f80fd5b5051919050565b60208082526029908201527f74657374206861726e65737320737570706f727473206f6e6c7920312072656760408201526834b9ba3930ba34b7b760b91b606082015260800190565b606081525f6115c86060830186611152565b6001600160a01b039490941660208301525060400152919050565b5f608082840312156115f3575f80fd5b6115fb6111f7565b90508135815260208201356020820152604082013560408201526060820135606082015292915050565b5f6101008284031215611636575f80fd5b61163e6111ce565b9050813581526020820135602082015260408201356040820152606082013560608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e082015292915050565b803561133d8161131b565b5f82601f8301126116aa575f80fd5b813560206116ba61138383611342565b808382526020820191506101a060208186028801019450878511156116dd575f80fd5b602087015b8581101561173e5781818a0312156116f8575f80fd5b611700611219565b61170a8a836115e3565b81526117198a60808401611625565b8682015261018082013561172c8161131b565b604082015284529284019281016116e2565b5090979650505050505050565b5f806040838503121561175c575f80fd5b82356001600160401b03811115611771575f80fd5b61177d8582860161169b565b95602094909401359450505050565b80356001600160a01b038116811461133d575f80fd5b5f82601f8301126117b1575f80fd5b81356001600160401b038111156117ca576117ca61108e565b6117dd601f8201601f1916602001611280565b8181528460208386010111156117f1575f80fd5b816020850160208301375f918101602001919091529392505050565b5f61016080838503121561181f575f80fd5b6118276111f7565b915082356001600160401b038082111561183f575f80fd5b8185019150828287031215611852575f80fd5b61185a61123b565b925061186686836115e3565b835261187586608084016115e3565b6020840152611887610100830161178c565b604084015261012082013561189b8161131b565b6060840152610140820135818111156118b2575f80fd5b6118be878285016117a2565b608085015250505081526118d460208301611690565b6020820152604082013560408201526118f08360608401611625565b606082015292915050565b5f6060828403121561190b575f80fd5b611913611219565b90508135815260208201356001600160401b0380821115611932575f80fd5b61193e8583860161169b565b60208401526040840135915080821115611956575f80fd5b506119638482850161180d565b60408301525092915050565b5f6080828403121561197f575f80fd5b6119876111f7565b905081356001600160401b038082111561199f575f80fd5b90830190608082860312156119b2575f80fd5b6119ba6111f7565b82356119c58161131b565b81526020830135828111156119d8575f80fd5b6119e4878286016117a2565b60208301525060408301356040820152611a006060840161178c565b60608201528352611a1360208501611690565b6020840152604084013560408401526060840135915080821115611a35575f80fd5b50611a42848285016117a2565b60608301525092915050565b5f805f8060808587031215611a61575f80fd5b84356001600160401b0380821115611a77575f80fd5b908601906101608289031215611a8b575f80fd5b611a9361125d565b8235815260208301356020820152611aad6040840161178c565b60408201526060830135606082015260808301356080820152611ad260a0840161178c565b60a0820152611ae360c0840161178c565b60c082015260e083013582811115611af9575f80fd5b611b058a8286016117a2565b60e083015250610100611b19818501611690565b908201526101208381013590820152610140611b36818501611690565b9082015295506020870135915080821115611b4f575f80fd5b611b5b888389016118fb565b94506040870135915080821115611b70575f80fd5b611b7c8883890161196f565b93506060870135915080821115611b91575f80fd5b50611b9e8782880161180d565b91505092959194509250565b600181811c90821680611bbe57607f821691505b602082108103611bdc57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561114d57805f5260205f20601f840160051c81016020851015611c075750805b601f840160051c820191505b81811015611c26575f8155600101611c13565b5050505050565b81516001600160401b03811115611c4657611c4661108e565b611c5a81611c548454611baa565b84611be2565b602080601f831160018114611c8d575f8415611c765750858301515b5f19600386901b1c1916600185901b178555611ce4565b5f85815260208120601f198616915b82811015611cbb57888601518255948401946001909101908401611c9c565b5085821015611cd857878501515f19600388901b60f8161c191681555b505060018460011b0185555b50505050505056fea2646970667358221220b789078602d68440f078754b7e3fe42ee4279ea67491b412c357da6cc10faee964736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0|W_5`\xE0\x1C\x80c{\x109\x99\x11a\0LW\x80c{\x109\x99\x14a\t4W\x80c~7\x80\xAC\x14a\tjW\x80c\xD8\xDF\xEBE\x14a\t\x7FW\x80c\xE7\x9A\x19\x8F\x14a\t\x94W_\x80\xFD[\x80c\x1F?\xC69\x14a\x08\xBAW\x80c=\xB5\xD8\xBD\x14a\x08\xD9W\x80cdq\xA8U\x14a\x08\xF8W\x80co\x0B\x0C\x1C\x14a\t W_\x80\xFD[6a\x08\xB6W_T`\x02T`@Qcy\x1E\xA0\x13`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF2=@&\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\0\xC9W_\x80\xFD[PZ\xF1\x93PPPP\x80\x15a\0\xDBWP`\x01[a\x01,W=\x80\x80\x15a\x01\x08W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x01\rV[``\x91P[P`\x01`\x03_\x82\x82Ta\x01 \x91\x90a\x10iV[\x90\x91UPa\x01\x85\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fshould not be able to add collat`D\x82\x01Rc\x19\\\x98[`\xE2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_T`\x02T`@Qc\x06\x82Fw`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1A\t\x19\xDC\x91a\x01\xB9\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xD0W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01\xE1WP`\x01[a\x022W=\x80\x80\x15a\x02\x0EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02\x13V[``\x91P[P`\x01`\x03_\x82\x82Ta\x02&\x91\x90a\x10iV[\x90\x91UPa\x02z\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Fshould not be able to unregister`D\x82\x01R`d\x01a\x01|V[_T`\x02T`@Qc\xE3\xFC\x02\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE3\xFC\x02\x8D\x91a\x02\xAE\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xC5W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x02\xD6WP`\x01[a\x03'W=\x80\x80\x15a\x03\x03W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\x08V[``\x91P[P`\x01`\x03_\x82\x82Ta\x03\x1B\x91\x90a\x10iV[\x90\x91UPa\x03~\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fshould not be able to claim coll`D\x82\x01Re\x18]\x19\\\x98[`\xD2\x1B`d\x82\x01R`\x84\x01a\x01|V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[a\x04\x11`@\x80Q`\xE0\x81\x01\x90\x91R_``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R`\xA0\x83\x01\x82\x90R`\xC0\x83\x01\x91\x90\x91R\x81\x90\x81R`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01\x81\x90R`\xE0\x82\x01R\x91\x01\x90\x81R_` \x90\x91\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\x94WPP`@\x80Q`\xE0\x80\x82\x01\x83R`#T``\x80\x84\x01\x91\x82R`$T`\x80\x80\x86\x01\x91\x90\x91R`%T`\xA0\x80\x87\x01\x91\x90\x91R`&T`\xC0\x80\x88\x01\x91\x90\x91R\x93\x86R\x86Qa\x01\0\x81\x01\x88R`'T\x81R`(T` \x80\x83\x01\x91\x90\x91R`)T\x82\x8A\x01R`*T\x94\x82\x01\x94\x90\x94R`+T\x92\x81\x01\x92\x90\x92R`,T\x90\x82\x01R`-T\x92\x81\x01\x92\x90\x92R`.T\x92\x82\x01\x92\x90\x92R\x90\x82\x01R`/T`\x01`\x01`@\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x81Q\x91\x92P\x90\x82\x90_\x90a\x04\xDDWa\x04\xDDa\x10\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x01R_\x80T`\rT`@QcJ?U\xC9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x94~\xAB\x92\x91a\x05 \x91\x86\x910\x91\x87\x91\x90`\x04\x01a\x11\x97V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05:W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05a\x91\x90\x81\x01\x90a\x13\xD0V[_T`@Qc\xFC\x0C\xE5\x9B`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFC\x0C\xE5\x9B\x90a\x05\x91\x90\x84\x90`\x04\x01a\x14\xD8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x05\xCBWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x05\xC8\x91\x81\x01\x90a\x15VV[`\x01[a\x06\x1CW=\x80\x80\x15a\x05\xF8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x05\xFDV[``\x91P[P`\x01`\x03_\x82\x82Ta\x06\x10\x91\x90a\x10iV[\x90\x91UPa\x06|\x91PPV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7Fshould not be able to slash regi`D\x82\x01Rm9\xBA90\xBA4\xB7\xB7\x100\xB3\xB0\xB4\xB7`\x91\x1B`d\x82\x01R`\x84\x01a\x01|V[`@\x80Q`\xE0\x80\x82\x01\x83R`#T``\x80\x84\x01\x91\x82R`$T`\x80\x80\x86\x01\x91\x90\x91R`%T`\xA0\x80\x87\x01\x91\x90\x91R`&T`\xC0\x80\x88\x01\x91\x90\x91R\x93\x86R\x86Qa\x01\0\x81\x01\x88R`'T\x81R`(T` \x82\x81\x01\x91\x90\x91R`)T\x82\x8A\x01R`*T\x94\x82\x01\x94\x90\x94R`+T\x92\x81\x01\x92\x90\x92R`,T\x90\x82\x01R`-T\x92\x81\x01\x92\x90\x92R`.T\x92\x82\x01\x92\x90\x92R\x90\x82\x01R`/T`\x01`\x01`@\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x82Q\x83\x90_\x90a\x072Wa\x072a\x10\xA2V[` \x02` \x01\x01\x81\x90RP\x81Q`\x01\x14a\x07^W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01|\x90a\x15mV[_T`\x01T`\rT`@Qc\xDC\xE46\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xDC\xE46\x95\x92\x91a\x07\x96\x91\x87\x910\x91`\x04\x01a\x15\xB6V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x93PPPP\x80\x15a\x07\xD1WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x07\xCE\x91\x81\x01\x90a\x15VV[`\x01[a\x08\"W=\x80\x80\x15a\x07\xFEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x08\x03V[``\x91P[P`\x01`\x03_\x82\x82Ta\x08\x16\x91\x90a\x10iV[\x90\x91UPa\x08k\x91PPV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fshould not be able to register\0\0`D\x82\x01R`d\x01a\x01|V[`\x03T`\x05\x14a\x08\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsshould have 5 errors``\x1B`D\x82\x01R`d\x01a\x01|V[\0[_\x80\xFD[4\x80\x15a\x08\xC5W_\x80\xFD[Pa\x08\xB4a\x08\xD46`\x04a\x17KV[a\t\xA8V[4\x80\x15a\x08\xE4W_\x80\xFD[Pa\x08\xB4a\x08\xF36`\x04a\x1ANV[a\x0B\"V[4\x80\x15a\t\x03W_\x80\xFD[Pa\t\r`\x03T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t+W_\x80\xFD[Pa\x08\xB4a\x0F\xD2V[4\x80\x15a\t?W_\x80\xFD[P_Ta\tR\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\t\x17V[4\x80\x15a\tuW_\x80\xFD[Pa\t\r`\x02T\x81V[4\x80\x15a\t\x8AW_\x80\xFD[Pa\t\r`\x01T\x81V[4\x80\x15a\t\x9FW_\x80\xFD[Pa\x08\xB4a\x105V[\x81Q`\x01\x14a\t\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01|\x90a\x15mV[\x81_\x81Q\x81\x10a\t\xDBWa\t\xDBa\x10\xA2V[` \x02` \x01\x01Q`#_`\x01\x81\x10a\t\xF6Wa\t\xF6a\x10\xA2V[\x82Q\x80Q`\r\x92\x90\x92\x02\x92\x90\x92\x01\x90\x81U` \x80\x83\x01Q`\x01\x80\x84\x01\x91\x90\x91U`@\x80\x85\x01Q`\x02\x85\x01U``\x94\x85\x01Q`\x03\x85\x01U\x82\x86\x01Q\x80Q`\x04\x80\x87\x01\x91\x90\x91U\x93\x81\x01Q`\x05\x86\x01U\x80\x82\x01Q`\x06\x86\x01U\x94\x85\x01Q`\x07\x85\x01U`\x80\x85\x01Q`\x08\x85\x01U`\xA0\x85\x01Q`\t\x85\x01U`\xC0\x85\x01Q`\n\x85\x01U`\xE0\x90\x94\x01Q`\x0B\x84\x01U\x93\x83\x01Q`\x0C\x90\x92\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U_T\x92T\x91Qc\xDC\xE46\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xDC\xE46\x95\x92\x91a\n\xDA\x91\x87\x910\x91\x88\x91\x01a\x15\xB6V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\n\xF6W=_\x80>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x1B\x91\x90a\x15VV[`\x02UPPV[\x83Q`\x04\x90\x81U` \x85\x01Q`\x05U`@\x85\x01Q`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U``\x87\x01Q`\x07U`\x80\x87\x01Q`\x08U`\xA0\x87\x01Q`\t\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`\xC0\x87\x01Q`\n\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xE0\x85\x01Q\x85\x91\x90`\x0B\x90a\x0B\xA1\x90\x82a\x1C-V[Pa\x01\0\x82\x01Q`\x08\x82\x01\x80T`\x01`\x01`@\x1B\x03\x92\x83\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x17\x90\x91Ua\x01 \x84\x01Q`\t\x84\x01Ua\x01@\x90\x93\x01Q`\n\x90\x92\x01\x80T\x92\x82\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x91U`@\x80\x85\x01Q\x80Q\x80Q\x80Q`\x0F\x90\x81U` \x80\x83\x01Q`\x10U\x82\x86\x01Q`\x11U``\x92\x83\x01Q`\x12U\x80\x84\x01Q\x80Q`\x13U\x90\x81\x01Q`\x14U\x80\x86\x01Q`\x15U\x82\x01Q`\x16U\x93\x82\x01Q`\x17\x80T\x92\x84\x01Q\x90\x96\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x17\x90\x93U`\x80\x83\x01Q\x90\x92\x90\x82\x90`\x18\x90a\x0C\x80\x90\x82a\x1C-V[PPP` \x82\x81\x01Q`\n\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80\x84\x01Q`\x0B\x84\x01U``\x93\x84\x01Q\x80Q`\x0C\x85\x01U\x91\x82\x01Q`\r\x84\x01U\x81\x01Q`\x0E\x83\x01U\x91\x82\x01Q`\x0F\x82\x01U`\x80\x82\x01Q`\x10\x82\x01U`\xA0\x82\x01Q`\x11\x82\x01U`\xC0\x82\x01Q`\x12\x82\x01U`\xE0\x90\x91\x01Q`\x13\x90\x91\x01U_[\x83` \x01QQ\x81\x10\x15a\r\xFBW\x83` \x01Q\x81\x81Q\x81\x10a\r,Wa\r,a\x10\xA2V[` \x02` \x01\x01Q`#\x82`\x01\x81\x10a\rGWa\rGa\x10\xA2V[\x82Q\x80Q`\r\x92\x90\x92\x02\x92\x90\x92\x01\x90\x81U` \x80\x83\x01Q`\x01\x80\x84\x01\x91\x90\x91U`@\x80\x85\x01Q`\x02\x85\x01U``\x94\x85\x01Q`\x03\x85\x01U\x82\x86\x01Q\x80Q`\x04\x86\x01U\x92\x83\x01Q`\x05\x85\x01U\x82\x81\x01Q`\x06\x85\x01U\x93\x82\x01Q`\x07\x84\x01U`\x80\x82\x01Q`\x08\x84\x01U`\xA0\x82\x01Q`\t\x84\x01U`\xC0\x82\x01Q`\n\x84\x01U`\xE0\x90\x91\x01Q`\x0B\x83\x01U\x91\x90\x92\x01Q`\x0C\x90\x92\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x01a\r\tV[P\x81Q\x80Q`1\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x81U` \x82\x01Q\x84\x92\x90\x82\x90`2\x90a\x0E7\x90\x82a\x1C-V[P`@\x82\x81\x01Q`\x02\x83\x01U``\x92\x83\x01Q`\x03\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U` \x84\x01Q`\x04\x84\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83\x01Q`\x05\x83\x01U\x82\x01Q`\x06\x82\x01\x90a\x0E\xAD\x90\x82a\x1C-V[PP\x81Q\x80Q\x80Q`8\x90\x81U` \x80\x83\x01Q`9U`@\x80\x84\x01Q`:U``\x93\x84\x01Q`;U\x81\x85\x01Q\x80Q`<U\x91\x82\x01Q`=U\x81\x81\x01Q`>U\x90\x83\x01Q`?U\x80\x84\x01Q\x81T\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x92\x90\x92\x17\x90\x91U`\x80\x82\x01Q\x84\x93P\x90\x91\x90\x82\x90`A\x90a\x0FE\x90\x82a\x1C-V[PPP` \x82\x81\x01Q`\n\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80\x84\x01Q`\x0B\x84\x01U``\x93\x84\x01Q\x80Q`\x0C\x85\x01U\x91\x82\x01Q`\r\x84\x01U\x81\x01Q`\x0E\x83\x01U\x91\x82\x01Q`\x0F\x82\x01U`\x80\x82\x01Q`\x10\x82\x01U`\xA0\x82\x01Q`\x11\x82\x01U`\xC0\x82\x01Q`\x12\x82\x01U`\xE0\x90\x91\x01Q`\x13\x90\x91\x01UPPPPV[_T`\x02T`@Qc\xE3\xFC\x02\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE3\xFC\x02\x8D\x91a\x10\x06\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x1DW_\x80\xFD[PZ\xF1\x15\x80\x15a\x10/W=_\x80>=_\xFD[PPPPV[_T`\x02T`@Qc\x06\x82Fw`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1A\t\x19\xDC\x91a\x10\x06\x91`\x04\x01\x90\x81R` \x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x10\x88WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01RP` \x81\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01R`@\x81\x01Q`\xC0\x84\x01R``\x81\x01Q`\xE0\x84\x01R`\x80\x81\x01Qa\x01\0\x84\x01R`\xA0\x81\x01Qa\x01 \x84\x01R`\xC0\x81\x01Qa\x01@\x84\x01R`\xE0\x81\x01Qa\x01`\x84\x01RP`@\x81\x01Qa\x11Ma\x01\x80\x84\x01\x82`\x01`\x01`@\x1B\x03\x16\x90RV[PPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x11\x8CWa\x11x\x87\x83Qa\x10\xB6V[a\x01\xA0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x11eV[P\x94\x95\x94PPPPPV[`\x80\x81R_a\x11\xA9`\x80\x83\x01\x87a\x11RV[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16` \x83\x01RP`@\x81\x01\x92\x90\x92R``\x90\x91\x01R\x91\x90PV[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@Qa\x01`\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11\xF1Wa\x11\xF1a\x10\x8EV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x12\xA8Wa\x12\xA8a\x10\x8EV[`@R\x91\x90PV[_a\x01\0\x82\x84\x03\x12\x15a\x12\xC1W_\x80\xFD[a\x12\xC9a\x11\xCEV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Q`\x80\x82\x01R`\xA0\x82\x01Q`\xA0\x82\x01R`\xC0\x82\x01Q`\xC0\x82\x01R`\xE0\x82\x01Q`\xE0\x82\x01R\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x13/W_\x80\xFD[PV[\x80Qa\x13=\x81a\x13\x1BV[\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x13ZWa\x13Za\x10\x8EV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x13sW_\x80\xFD[\x81Q` a\x13\x88a\x13\x83\x83a\x13BV[a\x12\x80V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x13\xA9W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x13\xC5W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x13\xAEV[P\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x13\xE0W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\xF6W_\x80\xFD[\x90\x83\x01\x90\x81\x85\x03a\x02\0\x81\x12\x15a\x14\x0BW_\x80\xFD[a\x14\x13a\x11\xF7V[\x83Q\x81R`\x1F\x19\x82\x01\x91Pa\x01\xA0\x80\x83\x12\x15a\x14-W_\x80\xFD[a\x145a\x12\x19V[`\x80\x84\x12\x15a\x14BW_\x80\xFD[a\x14Ja\x11\xF7V[\x93P` \x86\x01Q\x84R`@\x86\x01Q` \x85\x01R``\x86\x01Q`@\x85\x01R`\x80\x86\x01Q``\x85\x01R\x83\x81Ra\x14\x81\x89`\xA0\x88\x01a\x12\xB0V[` \x82\x01Ra\x14\x91\x82\x87\x01a\x132V[`@\x82\x01R` \x83\x01RPa\x01\xC0\x84\x01Q\x91P\x82\x82\x11\x15a\x14\xB0W_\x80\xFD[a\x14\xBC\x87\x83\x86\x01a\x13dV[`@\x82\x01Ra\x01\xE0\x93\x90\x93\x01Q``\x84\x01RP\x90\x94\x93PPPPV[_` \x80\x83Ra\x02 \x83\x01\x84Q\x82\x85\x01R\x81\x85\x01Qa\x14\xFA`@\x86\x01\x82a\x10\xB6V[P`@\x85\x01Qa\x02\0a\x01\xE0\x86\x01\x81\x90R\x81Q\x92\x83\x90R\x90\x83\x01\x91_\x91\x90a\x02@\x87\x01\x90[\x80\x84\x10\x15a\x15?W\x84Q\x82R\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x85\x01\x90a\x15\x1FV[P``\x97\x90\x97\x01Q\x95\x01\x94\x90\x94RP\x92\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x15fW_\x80\xFD[PQ\x91\x90PV[` \x80\x82R`)\x90\x82\x01R\x7Ftest harness supports only 1 reg`@\x82\x01Rh4\xB9\xBA90\xBA4\xB7\xB7`\xB9\x1B``\x82\x01R`\x80\x01\x90V[``\x81R_a\x15\xC8``\x83\x01\x86a\x11RV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16` \x83\x01RP`@\x01R\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\x15\xF3W_\x80\xFD[a\x15\xFBa\x11\xF7V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[_a\x01\0\x82\x84\x03\x12\x15a\x166W_\x80\xFD[a\x16>a\x11\xCEV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015`\x80\x82\x01R`\xA0\x82\x015`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R`\xE0\x82\x015`\xE0\x82\x01R\x92\x91PPV[\x805a\x13=\x81a\x13\x1BV[_\x82`\x1F\x83\x01\x12a\x16\xAAW_\x80\xFD[\x815` a\x16\xBAa\x13\x83\x83a\x13BV[\x80\x83\x82R` \x82\x01\x91Pa\x01\xA0` \x81\x86\x02\x88\x01\x01\x94P\x87\x85\x11\x15a\x16\xDDW_\x80\xFD[` \x87\x01[\x85\x81\x10\x15a\x17>W\x81\x81\x8A\x03\x12\x15a\x16\xF8W_\x80\xFD[a\x17\0a\x12\x19V[a\x17\n\x8A\x83a\x15\xE3V[\x81Ra\x17\x19\x8A`\x80\x84\x01a\x16%V[\x86\x82\x01Ra\x01\x80\x82\x015a\x17,\x81a\x13\x1BV[`@\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x16\xE2V[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x17\\W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17qW_\x80\xFD[a\x17}\x85\x82\x86\x01a\x16\x9BV[\x95` \x94\x90\x94\x015\x94PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13=W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x17\xB1W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xCAWa\x17\xCAa\x10\x8EV[a\x17\xDD`\x1F\x82\x01`\x1F\x19\x16` \x01a\x12\x80V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x17\xF1W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_a\x01`\x80\x83\x85\x03\x12\x15a\x18\x1FW_\x80\xFD[a\x18'a\x11\xF7V[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18?W_\x80\xFD[\x81\x85\x01\x91P\x82\x82\x87\x03\x12\x15a\x18RW_\x80\xFD[a\x18Za\x12;V[\x92Pa\x18f\x86\x83a\x15\xE3V[\x83Ra\x18u\x86`\x80\x84\x01a\x15\xE3V[` \x84\x01Ra\x18\x87a\x01\0\x83\x01a\x17\x8CV[`@\x84\x01Ra\x01 \x82\x015a\x18\x9B\x81a\x13\x1BV[``\x84\x01Ra\x01@\x82\x015\x81\x81\x11\x15a\x18\xB2W_\x80\xFD[a\x18\xBE\x87\x82\x85\x01a\x17\xA2V[`\x80\x85\x01RPPP\x81Ra\x18\xD4` \x83\x01a\x16\x90V[` \x82\x01R`@\x82\x015`@\x82\x01Ra\x18\xF0\x83``\x84\x01a\x16%V[``\x82\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x19\x0BW_\x80\xFD[a\x19\x13a\x12\x19V[\x90P\x815\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x192W_\x80\xFD[a\x19>\x85\x83\x86\x01a\x16\x9BV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x19VW_\x80\xFD[Pa\x19c\x84\x82\x85\x01a\x18\rV[`@\x83\x01RP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a\x19\x7FW_\x80\xFD[a\x19\x87a\x11\xF7V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\x9FW_\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15a\x19\xB2W_\x80\xFD[a\x19\xBAa\x11\xF7V[\x825a\x19\xC5\x81a\x13\x1BV[\x81R` \x83\x015\x82\x81\x11\x15a\x19\xD8W_\x80\xFD[a\x19\xE4\x87\x82\x86\x01a\x17\xA2V[` \x83\x01RP`@\x83\x015`@\x82\x01Ra\x1A\0``\x84\x01a\x17\x8CV[``\x82\x01R\x83Ra\x1A\x13` \x85\x01a\x16\x90V[` \x84\x01R`@\x84\x015`@\x84\x01R``\x84\x015\x91P\x80\x82\x11\x15a\x1A5W_\x80\xFD[Pa\x1AB\x84\x82\x85\x01a\x17\xA2V[``\x83\x01RP\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x1AaW_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1AwW_\x80\xFD[\x90\x86\x01\x90a\x01`\x82\x89\x03\x12\x15a\x1A\x8BW_\x80\xFD[a\x1A\x93a\x12]V[\x825\x81R` \x83\x015` \x82\x01Ra\x1A\xAD`@\x84\x01a\x17\x8CV[`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01Ra\x1A\xD2`\xA0\x84\x01a\x17\x8CV[`\xA0\x82\x01Ra\x1A\xE3`\xC0\x84\x01a\x17\x8CV[`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x1A\xF9W_\x80\xFD[a\x1B\x05\x8A\x82\x86\x01a\x17\xA2V[`\xE0\x83\x01RPa\x01\0a\x1B\x19\x81\x85\x01a\x16\x90V[\x90\x82\x01Ra\x01 \x83\x81\x015\x90\x82\x01Ra\x01@a\x1B6\x81\x85\x01a\x16\x90V[\x90\x82\x01R\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x1BOW_\x80\xFD[a\x1B[\x88\x83\x89\x01a\x18\xFBV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x1BpW_\x80\xFD[a\x1B|\x88\x83\x89\x01a\x19oV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1B\x91W_\x80\xFD[Pa\x1B\x9E\x87\x82\x88\x01a\x18\rV[\x91PP\x92\x95\x91\x94P\x92PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1B\xBEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1B\xDCWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x11MW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x1C\x07WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1C&W_\x81U`\x01\x01a\x1C\x13V[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1CFWa\x1CFa\x10\x8EV[a\x1CZ\x81a\x1CT\x84Ta\x1B\xAAV[\x84a\x1B\xE2V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x1C\x8DW_\x84\x15a\x1CvWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x1C\xE4V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x1C\xBBW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x1C\x9CV[P\x85\x82\x10\x15a\x1C\xD8W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV\xFE\xA2dipfsX\"\x12 \xB7\x89\x07\x86\x02\xD6\x84@\xF0xuK~?\xE4.\xE4'\x9E\xA6t\x91\xB4\x12\xC3W\xDAl\xC1\x0F\xAE\xE9dsolcC\0\x08\x19\x003",
    );
	/**Constructor`.
	```solidity
	constructor(address registryAddress);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct constructorCall {
		#[allow(missing_docs)]
		pub registryAddress: alloy::sol_types::private::Address,
	}
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
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
			impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
				fn from(value: constructorCall) -> Self {
					(value.registryAddress,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registryAddress: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolConstructor for constructorCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.registryAddress),)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `claimCollateral()` and selector `0x6f0b0c1c`.
	```solidity
	function claimCollateral() external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct claimCollateralCall;
	///Container type for the return parameters of the [`claimCollateral()`](claimCollateralCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct claimCollateralReturn {}
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
			impl ::core::convert::From<claimCollateralCall> for UnderlyingRustTuple<'_> {
				fn from(value: claimCollateralCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCollateralCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self
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
			impl ::core::convert::From<claimCollateralReturn> for UnderlyingRustTuple<'_> {
				fn from(value: claimCollateralReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCollateralReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl claimCollateralReturn {
			fn _tokenize(&self) -> <claimCollateralCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for claimCollateralCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = claimCollateralReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "claimCollateral()";
			const SELECTOR: [u8; 4] = [111u8, 11u8, 12u8, 28u8];
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
				claimCollateralReturn::_tokenize(ret)
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
	/**Function with signature `collateral()` and selector `0xd8dfeb45`.
	```solidity
	function collateral() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct collateralCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`collateral()`](collateralCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct collateralReturn {
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
			impl ::core::convert::From<collateralCall> for UnderlyingRustTuple<'_> {
				fn from(value: collateralCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for collateralCall {
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
			impl ::core::convert::From<collateralReturn> for UnderlyingRustTuple<'_> {
				fn from(value: collateralReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for collateralReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for collateralCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "collateral()";
			const SELECTOR: [u8; 4] = [216u8, 223u8, 235u8, 69u8];
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
					let r: collateralReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: collateralReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `errors()` and selector `0x6471a855`.
	```solidity
	function errors() external view returns (uint256);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct errorsCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`errors()`](errorsCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct errorsReturn {
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
			impl ::core::convert::From<errorsCall> for UnderlyingRustTuple<'_> {
				fn from(value: errorsCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for errorsCall {
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
			impl ::core::convert::From<errorsReturn> for UnderlyingRustTuple<'_> {
				fn from(value: errorsReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for errorsReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for errorsCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "errors()";
			const SELECTOR: [u8; 4] = [100u8, 113u8, 168u8, 85u8];
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
					let r: errorsReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: errorsReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `register(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],bytes32)` and selector `0x1f3fc639`.
	```solidity
	function register(IRegistry.SignedRegistration[] memory _registrations, bytes32 signingId) external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct registerCall {
		#[allow(missing_docs)]
		pub _registrations:
			alloy::sol_types::private::Vec<<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType>,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
	}
	///Container type for the return parameters of the [`register(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],bytes32)`](registerCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct registerReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (
				alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
				alloy::sol_types::sol_data::FixedBytes<32>,
			);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				alloy::sol_types::private::Vec<<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType>,
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
			impl ::core::convert::From<registerCall> for UnderlyingRustTuple<'_> {
				fn from(value: registerCall) -> Self {
					(value._registrations, value.signingId)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _registrations: tuple.0, signingId: tuple.1 }
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
			impl ::core::convert::From<registerReturn> for UnderlyingRustTuple<'_> {
				fn from(value: registerReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl registerReturn {
			fn _tokenize(&self) -> <registerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for registerCall {
			type Parameters<'a> = (
				alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
				alloy::sol_types::sol_data::FixedBytes<32>,
			);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = registerReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "register(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],bytes32)";
			const SELECTOR: [u8; 4] = [31u8, 63u8, 198u8, 57u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
                    <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::SolType>::tokenize(&self._registrations),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.signingId),
                )
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				registerReturn::_tokenize(ret)
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
	/**Function with signature `registrationRoot()` and selector `0x7e3780ac`.
	```solidity
	function registrationRoot() external view returns (bytes32);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct registrationRootCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`registrationRoot()`](registrationRootCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct registrationRootReturn {
		#[allow(missing_docs)]
		pub _0: alloy::sol_types::private::FixedBytes<32>,
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
			impl ::core::convert::From<registrationRootCall> for UnderlyingRustTuple<'_> {
				fn from(value: registrationRootCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for registrationRootCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self
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
			impl ::core::convert::From<registrationRootReturn> for UnderlyingRustTuple<'_> {
				fn from(value: registrationRootReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for registrationRootReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for registrationRootCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::FixedBytes<32>;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "registrationRoot()";
			const SELECTOR: [u8; 4] = [126u8, 55u8, 128u8, 172u8];
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
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: registrationRootReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: registrationRootReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `registry()` and selector `0x7b103999`.
	```solidity
	function registry() external view returns (address);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct registryCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`registry()`](registryCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct registryReturn {
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
			impl ::core::convert::From<registryCall> for UnderlyingRustTuple<'_> {
				fn from(value: registryCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCall {
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
			impl ::core::convert::From<registryReturn> for UnderlyingRustTuple<'_> {
				fn from(value: registryReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for registryCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::Address;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "registry()";
			const SELECTOR: [u8; 4] = [123u8, 16u8, 57u8, 153u8];
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
					let r: registryReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: registryReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize)]
	/**Function with signature `saveResult((uint256,uint256,address,uint256,uint256,address,address,bytes,uint64,bytes32,uint64),(bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32))),((uint64,bytes,bytes32,address),uint64,bytes32,bytes),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))` and selector `0x3db5d8bd`.
	```solidity
	function saveResult(UnitTestHelper.RegisterAndDelegateParams memory _params, UnitTestHelper.RegisterAndDelegateResult memory _result, ISlasher.SignedCommitment memory _signedCommitment, ISlasher.SignedDelegation memory _signedDelegationTwo) external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct saveResultCall {
		#[allow(missing_docs)]
		pub _params: <UnitTestHelper::RegisterAndDelegateParams as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub _result: <UnitTestHelper::RegisterAndDelegateResult as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub _signedCommitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub _signedDelegationTwo: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
	}
	///Container type for the return parameters of the [`saveResult((uint256,uint256,address,uint256,uint256,address,address,bytes,uint64,bytes32,uint64),(bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32))),((uint64,bytes,bytes32,address),uint64,bytes32,bytes),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))`](saveResultCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct saveResultReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (
				UnitTestHelper::RegisterAndDelegateParams,
				UnitTestHelper::RegisterAndDelegateResult,
				ISlasher::SignedCommitment,
				ISlasher::SignedDelegation,
			);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				<UnitTestHelper::RegisterAndDelegateParams as alloy::sol_types::SolType>::RustType,
				<UnitTestHelper::RegisterAndDelegateResult as alloy::sol_types::SolType>::RustType,
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
			impl ::core::convert::From<saveResultCall> for UnderlyingRustTuple<'_> {
				fn from(value: saveResultCall) -> Self {
					(value._params, value._result, value._signedCommitment, value._signedDelegationTwo)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for saveResultCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {
						_params: tuple.0,
						_result: tuple.1,
						_signedCommitment: tuple.2,
						_signedDelegationTwo: tuple.3,
					}
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
			impl ::core::convert::From<saveResultReturn> for UnderlyingRustTuple<'_> {
				fn from(value: saveResultReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for saveResultReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl saveResultReturn {
			fn _tokenize(&self) -> <saveResultCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for saveResultCall {
			type Parameters<'a> = (
				UnitTestHelper::RegisterAndDelegateParams,
				UnitTestHelper::RegisterAndDelegateResult,
				ISlasher::SignedCommitment,
				ISlasher::SignedDelegation,
			);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = saveResultReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "saveResult((uint256,uint256,address,uint256,uint256,address,address,bytes,uint64,bytes32,uint64),(bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32))),((uint64,bytes,bytes32,address),uint64,bytes32,bytes),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))";
			const SELECTOR: [u8; 4] = [61u8, 181u8, 216u8, 189u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<UnitTestHelper::RegisterAndDelegateParams as alloy_sol_types::SolType>::tokenize(&self._params),
					<UnitTestHelper::RegisterAndDelegateResult as alloy_sol_types::SolType>::tokenize(&self._result),
					<ISlasher::SignedCommitment as alloy_sol_types::SolType>::tokenize(&self._signedCommitment),
					<ISlasher::SignedDelegation as alloy_sol_types::SolType>::tokenize(&self._signedDelegationTwo),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				saveResultReturn::_tokenize(ret)
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
	/**Function with signature `unregister()` and selector `0xe79a198f`.
	```solidity
	function unregister() external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct unregisterCall;
	///Container type for the return parameters of the [`unregister()`](unregisterCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct unregisterReturn {}
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
			impl ::core::convert::From<unregisterCall> for UnderlyingRustTuple<'_> {
				fn from(value: unregisterCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for unregisterCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self
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
			impl ::core::convert::From<unregisterReturn> for UnderlyingRustTuple<'_> {
				fn from(value: unregisterReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for unregisterReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl unregisterReturn {
			fn _tokenize(&self) -> <unregisterCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for unregisterCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = unregisterReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "unregister()";
			const SELECTOR: [u8; 4] = [231u8, 154u8, 25u8, 143u8];
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
				unregisterReturn::_tokenize(ret)
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
	///Container for all the [`ReentrantSlashableRegistrationContract`](self) function calls.
	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum ReentrantSlashableRegistrationContractCalls {
		#[allow(missing_docs)]
		claimCollateral(claimCollateralCall),
		#[allow(missing_docs)]
		collateral(collateralCall),
		#[allow(missing_docs)]
		errors(errorsCall),
		#[allow(missing_docs)]
		register(registerCall),
		#[allow(missing_docs)]
		registrationRoot(registrationRootCall),
		#[allow(missing_docs)]
		registry(registryCall),
		#[allow(missing_docs)]
		saveResult(saveResultCall),
		#[allow(missing_docs)]
		unregister(unregisterCall),
	}
	impl ReentrantSlashableRegistrationContractCalls {
		/// All the selectors of this enum.
		///
		/// Note that the selectors might not be in the same order as the variants.
		/// No guarantees are made about the order of the selectors.
		///
		/// Prefer using `SolInterface` methods instead.
		pub const SELECTORS: &'static [[u8; 4usize]] = &[
			[31u8, 63u8, 198u8, 57u8],
			[61u8, 181u8, 216u8, 189u8],
			[100u8, 113u8, 168u8, 85u8],
			[111u8, 11u8, 12u8, 28u8],
			[123u8, 16u8, 57u8, 153u8],
			[126u8, 55u8, 128u8, 172u8],
			[216u8, 223u8, 235u8, 69u8],
			[231u8, 154u8, 25u8, 143u8],
		];
		/// The names of the variants in the same order as `SELECTORS`.
		pub const VARIANT_NAMES: &'static [&'static str] = &[
			::core::stringify!(register),
			::core::stringify!(saveResult),
			::core::stringify!(errors),
			::core::stringify!(claimCollateral),
			::core::stringify!(registry),
			::core::stringify!(registrationRoot),
			::core::stringify!(collateral),
			::core::stringify!(unregister),
		];
		/// The signatures in the same order as `SELECTORS`.
		pub const SIGNATURES: &'static [&'static str] = &[
			<registerCall as alloy_sol_types::SolCall>::SIGNATURE,
			<saveResultCall as alloy_sol_types::SolCall>::SIGNATURE,
			<errorsCall as alloy_sol_types::SolCall>::SIGNATURE,
			<claimCollateralCall as alloy_sol_types::SolCall>::SIGNATURE,
			<registryCall as alloy_sol_types::SolCall>::SIGNATURE,
			<registrationRootCall as alloy_sol_types::SolCall>::SIGNATURE,
			<collateralCall as alloy_sol_types::SolCall>::SIGNATURE,
			<unregisterCall as alloy_sol_types::SolCall>::SIGNATURE,
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
	impl alloy_sol_types::SolInterface for ReentrantSlashableRegistrationContractCalls {
		const NAME: &'static str = "ReentrantSlashableRegistrationContractCalls";
		const MIN_DATA_LENGTH: usize = 0usize;
		const COUNT: usize = 8usize;
		#[inline]
		fn selector(&self) -> [u8; 4] {
			match self {
				Self::claimCollateral(_) => <claimCollateralCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::collateral(_) => <collateralCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::errors(_) => <errorsCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::register(_) => <registerCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::registrationRoot(_) => <registrationRootCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::registry(_) => <registryCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::saveResult(_) => <saveResultCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::unregister(_) => <unregisterCall as alloy_sol_types::SolCall>::SELECTOR,
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
			static DECODE_SHIMS: &[fn(
				&[u8],
			) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls>] = &[
				{
					fn register(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<registerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(ReentrantSlashableRegistrationContractCalls::register)
					}
					register
				},
				{
					fn saveResult(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<saveResultCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(ReentrantSlashableRegistrationContractCalls::saveResult)
					}
					saveResult
				},
				{
					fn errors(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<errorsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(ReentrantSlashableRegistrationContractCalls::errors)
					}
					errors
				},
				{
					fn claimCollateral(
						data: &[u8],
					) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<claimCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(ReentrantSlashableRegistrationContractCalls::claimCollateral)
					}
					claimCollateral
				},
				{
					fn registry(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<registryCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(ReentrantSlashableRegistrationContractCalls::registry)
					}
					registry
				},
				{
					fn registrationRoot(
						data: &[u8],
					) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<registrationRootCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(ReentrantSlashableRegistrationContractCalls::registrationRoot)
					}
					registrationRoot
				},
				{
					fn collateral(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<collateralCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(ReentrantSlashableRegistrationContractCalls::collateral)
					}
					collateral
				},
				{
					fn unregister(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<unregisterCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(ReentrantSlashableRegistrationContractCalls::unregister)
					}
					unregister
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
			static DECODE_VALIDATE_SHIMS: &[fn(
				&[u8],
			) -> alloy_sol_types::Result<
				ReentrantSlashableRegistrationContractCalls,
			>] = &[
				{
					fn register(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<registerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(ReentrantSlashableRegistrationContractCalls::register)
					}
					register
				},
				{
					fn saveResult(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<saveResultCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(ReentrantSlashableRegistrationContractCalls::saveResult)
					}
					saveResult
				},
				{
					fn errors(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<errorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(ReentrantSlashableRegistrationContractCalls::errors)
					}
					errors
				},
				{
					fn claimCollateral(
						data: &[u8],
					) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<claimCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(ReentrantSlashableRegistrationContractCalls::claimCollateral)
					}
					claimCollateral
				},
				{
					fn registry(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<registryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(ReentrantSlashableRegistrationContractCalls::registry)
					}
					registry
				},
				{
					fn registrationRoot(
						data: &[u8],
					) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<registrationRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(ReentrantSlashableRegistrationContractCalls::registrationRoot)
					}
					registrationRoot
				},
				{
					fn collateral(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<collateralCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(ReentrantSlashableRegistrationContractCalls::collateral)
					}
					collateral
				},
				{
					fn unregister(data: &[u8]) -> alloy_sol_types::Result<ReentrantSlashableRegistrationContractCalls> {
						<unregisterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(ReentrantSlashableRegistrationContractCalls::unregister)
					}
					unregister
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
				Self::claimCollateral(inner) => {
					<claimCollateralCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::collateral(inner) => <collateralCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::errors(inner) => <errorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::register(inner) => <registerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::registrationRoot(inner) => {
					<registrationRootCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::registry(inner) => <registryCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::saveResult(inner) => <saveResultCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::unregister(inner) => <unregisterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
			}
		}
		#[inline]
		fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
			match self {
				Self::claimCollateral(inner) => {
					<claimCollateralCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::collateral(inner) => <collateralCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::errors(inner) => <errorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::register(inner) => <registerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::registrationRoot(inner) => {
					<registrationRootCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::registry(inner) => <registryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::saveResult(inner) => <saveResultCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::unregister(inner) => <unregisterCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
			}
		}
	}
	use alloy::contract as alloy_contract;
	/**Creates a new wrapper around an on-chain [`ReentrantSlashableRegistrationContract`](self) contract instance.

	See the [wrapper's documentation](`ReentrantSlashableRegistrationContractInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> ReentrantSlashableRegistrationContractInstance<P, N> {
		ReentrantSlashableRegistrationContractInstance::<P, N>::new(address, __provider)
	}
	/**Deploys this contract using the given `provider` and constructor arguments, if any.

	Returns a new instance of the contract, if the deployment was successful.

	For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
	#[inline]
	pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		__provider: P,
		registryAddress: alloy::sol_types::private::Address,
	) -> impl ::core::future::Future<Output = alloy_contract::Result<ReentrantSlashableRegistrationContractInstance<P, N>>>
	{
		ReentrantSlashableRegistrationContractInstance::<P, N>::deploy(__provider, registryAddress)
	}
	/**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
	and constructor arguments, if any.

	This is a simple wrapper around creating a `RawCallBuilder` with the data set to
	the bytecode concatenated with the constructor's ABI-encoded arguments.*/
	#[inline]
	pub fn deploy_builder<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		__provider: P,
		registryAddress: alloy::sol_types::private::Address,
	) -> alloy_contract::RawCallBuilder<P, N> {
		ReentrantSlashableRegistrationContractInstance::<P, N>::deploy_builder(__provider, registryAddress)
	}
	/**A [`ReentrantSlashableRegistrationContract`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`ReentrantSlashableRegistrationContract`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct ReentrantSlashableRegistrationContractInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for ReentrantSlashableRegistrationContractInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("ReentrantSlashableRegistrationContractInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
		ReentrantSlashableRegistrationContractInstance<P, N>
	{
		/**Creates a new wrapper around an on-chain [`ReentrantSlashableRegistrationContract`](self) contract instance.

		See the [wrapper's documentation](`ReentrantSlashableRegistrationContractInstance`) for more details.*/
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
			registryAddress: alloy::sol_types::private::Address,
		) -> alloy_contract::Result<ReentrantSlashableRegistrationContractInstance<P, N>> {
			let call_builder = Self::deploy_builder(__provider, registryAddress);
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
			registryAddress: alloy::sol_types::private::Address,
		) -> alloy_contract::RawCallBuilder<P, N> {
			alloy_contract::RawCallBuilder::new_raw_deploy(
				__provider,
				[&BYTECODE[..], &alloy_sol_types::SolConstructor::abi_encode(&constructorCall { registryAddress })[..]]
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
	impl<P: ::core::clone::Clone, N> ReentrantSlashableRegistrationContractInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> ReentrantSlashableRegistrationContractInstance<P, N> {
			ReentrantSlashableRegistrationContractInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
		ReentrantSlashableRegistrationContractInstance<P, N>
	{
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
		///Creates a new call builder for the [`claimCollateral`] function.
		pub fn claimCollateral(&self) -> alloy_contract::SolCallBuilder<&P, claimCollateralCall, N> {
			self.call_builder(&claimCollateralCall)
		}
		///Creates a new call builder for the [`collateral`] function.
		pub fn collateral(&self) -> alloy_contract::SolCallBuilder<&P, collateralCall, N> {
			self.call_builder(&collateralCall)
		}
		///Creates a new call builder for the [`errors`] function.
		pub fn errors(&self) -> alloy_contract::SolCallBuilder<&P, errorsCall, N> {
			self.call_builder(&errorsCall)
		}
		///Creates a new call builder for the [`register`] function.
		pub fn register(
			&self,
			_registrations: alloy::sol_types::private::Vec<
				<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType,
			>,
			signingId: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, registerCall, N> {
			self.call_builder(&registerCall { _registrations, signingId })
		}
		///Creates a new call builder for the [`registrationRoot`] function.
		pub fn registrationRoot(&self) -> alloy_contract::SolCallBuilder<&P, registrationRootCall, N> {
			self.call_builder(&registrationRootCall)
		}
		///Creates a new call builder for the [`registry`] function.
		pub fn registry(&self) -> alloy_contract::SolCallBuilder<&P, registryCall, N> {
			self.call_builder(&registryCall)
		}
		///Creates a new call builder for the [`saveResult`] function.
		pub fn saveResult(
			&self,
			_params: <UnitTestHelper::RegisterAndDelegateParams as alloy::sol_types::SolType>::RustType,
			_result: <UnitTestHelper::RegisterAndDelegateResult as alloy::sol_types::SolType>::RustType,
			_signedCommitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
			_signedDelegationTwo: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
		) -> alloy_contract::SolCallBuilder<&P, saveResultCall, N> {
			self.call_builder(&saveResultCall { _params, _result, _signedCommitment, _signedDelegationTwo })
		}
		///Creates a new call builder for the [`unregister`] function.
		pub fn unregister(&self) -> alloy_contract::SolCallBuilder<&P, unregisterCall, N> {
			self.call_builder(&unregisterCall)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
		ReentrantSlashableRegistrationContractInstance<P, N>
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
